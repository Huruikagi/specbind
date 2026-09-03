//! Read model for project-wide persistent `SpecBind` Contract validation.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use camino::Utf8PathBuf;
use petgraph::{algo::tarjan_scc, graphmap::DiGraphMap};

use crate::artifacts::{
    self, ArtifactInventory, ArtifactKind, DiscoveryIssue, SpecEntryFault, discover_spec,
    resolve_contract_projection,
};
use crate::contract::{ContractDocument, ContractSection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphIssueSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractGraphIssue {
    pub severity: GraphIssueSeverity,
    pub code: &'static str,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractEntryRef {
    pub canonical_spec: String,
    pub section: ContractSection,
    pub entry_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractDependency {
    pub consumer: ContractEntryRef,
    pub provider: ContractEntryRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OwnershipFindingKind {
    Duplicate,
    Overlap,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnershipFinding {
    pub kind: OwnershipFindingKind,
    pub left: ContractEntryRef,
    pub left_path: String,
    pub right: ContractEntryRef,
    pub right_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnershipMatch {
    pub owner: ContractEntryRef,
    pub declared_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidOwnershipQuery;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractGraphReport {
    pub contracts: BTreeMap<String, ContractDocument>,
    pub dependencies: Vec<ContractDependency>,
    pub ownership_findings: Vec<OwnershipFinding>,
    pub dependency_cycles: Vec<Vec<String>>,
    pub issues: Vec<ContractGraphIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractGraphResolution {
    pub inventories: BTreeMap<String, ArtifactInventory>,
    pub project_issues: Vec<DiscoveryIssue>,
    pub report: ContractGraphReport,
}

#[derive(Debug)]
struct OwnershipClaim {
    entry: ContractEntryRef,
    path: String,
    normalized: String,
    subtree: bool,
}

/// Discovers every immediate canonical Spec directory and resolves the complete
/// persistent Contract graph below one `SpecBind` root.
#[must_use]
pub fn resolve(specbind_root: &Path) -> ContractGraphResolution {
    let (expected_specs, mut project_issues) = discover_spec_ids(specbind_root);
    let mut inventories = BTreeMap::new();
    let mut contracts = BTreeMap::new();

    for canonical_spec in &expected_specs {
        let mut inventory = discover_spec(specbind_root, canonical_spec);
        let contract = inventory
            .artifacts
            .iter()
            .find(|artifact| artifact.kind == ArtifactKind::Contract)
            .cloned();
        if let Some(artifact) = contract
            && !inventory
                .issues
                .iter()
                .any(|issue| issue.path.as_ref() == Some(&artifact.path))
            && let Some(document) =
                resolve_contract_projection(specbind_root, &artifact, &mut inventory.issues)
        {
            contracts.insert(canonical_spec.clone(), document);
        }
        inventory.issues.sort();
        inventory.issues.dedup();
        inventories.insert(canonical_spec.clone(), inventory);
    }

    project_issues.sort();
    project_issues.dedup();
    let report = evaluate(&expected_specs, contracts);
    ContractGraphResolution {
        inventories,
        project_issues,
        report,
    }
}

/// Validates typed Contract manifests against the complete persistent Spec set.
#[must_use]
pub fn evaluate(
    expected_specs: &BTreeSet<String>,
    contracts: BTreeMap<String, ContractDocument>,
) -> ContractGraphReport {
    let mut issues = Vec::new();
    for canonical_spec in expected_specs {
        if !contracts.contains_key(canonical_spec) {
            issues.push(graph_issue(
                GraphIssueSeverity::Error,
                "CONTRACT_GRAPH_CONTRACT_UNAVAILABLE",
                Some(format!("specs/{canonical_spec}#contract")),
                format!("persistent spec {canonical_spec} requires one valid discovered Contract"),
            ));
        }
    }

    let dependencies = resolve_dependencies(expected_specs, &contracts, &mut issues);
    let ownership_findings = ownership_findings(&contracts);
    for finding in &ownership_findings {
        let (code, relation) = match finding.kind {
            OwnershipFindingKind::Duplicate => ("CONTRACT_GRAPH_OWNERSHIP_DUPLICATE", "duplicates"),
            OwnershipFindingKind::Overlap => ("CONTRACT_GRAPH_OWNERSHIP_OVERLAP", "overlaps"),
        };
        issues.push(graph_issue(
            GraphIssueSeverity::Warning,
            code,
            Some(entry_selector(&finding.left)),
            format!(
                "File Ownership path {} {relation} {} at {}",
                finding.left_path,
                finding.right_path,
                entry_selector(&finding.right)
            ),
        ));
    }

    for entry in unconsumed_exports(&contracts, &dependencies) {
        issues.push(graph_issue(
            GraphIssueSeverity::Warning,
            "CONTRACT_GRAPH_EXPORT_UNCONSUMED",
            Some(entry_selector(&entry)),
            format!(
                "Exports entry {} is consumed by no managed spec; confirm an external consumer or retire the seam",
                entry.entry_id
            ),
        ));
    }

    let dependency_cycles = dependency_cycles(&contracts, &dependencies);
    for cycle in &dependency_cycles {
        issues.push(graph_issue(
            GraphIssueSeverity::Warning,
            "CONTRACT_GRAPH_DEPENDENCY_CYCLE",
            cycle.first().map(|spec| format!("specs/{spec}#contract")),
            format!("Contract dependency cycle: {}", cycle.join(" -> ")),
        ));
    }

    issues.sort();
    issues.dedup();
    ContractGraphReport {
        contracts,
        dependencies,
        ownership_findings,
        dependency_cycles,
        issues,
    }
}

/// Resolves the File Ownership declarations that contain one concrete
/// project-relative portable path.
///
/// # Errors
///
/// Returns [`InvalidOwnershipQuery`] when `path` is empty, absolute, contains
/// native separators or traversal, or uses declaration-only wildcard syntax.
pub fn owners_for_path(
    report: &ContractGraphReport,
    path: &str,
) -> Result<Vec<OwnershipMatch>, InvalidOwnershipQuery> {
    if !crate::contract::valid_file_ownership_query(path) {
        return Err(InvalidOwnershipQuery);
    }

    let normalized = path.to_ascii_lowercase();
    let mut matches = Vec::new();
    for (canonical_spec, contract) in &report.contracts {
        for entry in &contract.file_ownership {
            for declared_path in &entry.paths {
                if ownership_path_matches(declared_path, &normalized) {
                    matches.push(OwnershipMatch {
                        owner: ContractEntryRef {
                            canonical_spec: canonical_spec.clone(),
                            section: ContractSection::FileOwnership,
                            entry_id: entry.id.clone(),
                        },
                        declared_path: declared_path.clone(),
                    });
                }
            }
        }
    }
    matches.sort();
    matches.dedup();
    Ok(matches)
}

fn ownership_path_matches(declared_path: &str, normalized_query: &str) -> bool {
    let normalized_declaration = declared_path.to_ascii_lowercase();
    if let Some(directory) = normalized_declaration.strip_suffix("/**") {
        path_contains(directory, normalized_query)
    } else {
        normalized_declaration == normalized_query
    }
}

/// Reports every exported seam no managed spec consumes.
///
/// An export exists to be depended on, so one nothing reaches is either a
/// boundary cut for a consumer that never arrived or a seam whose only consumer
/// is outside the graph. The graph cannot tell those apart, which is why this is
/// a warning the reviewer resolves rather than an error.
fn unconsumed_exports(
    contracts: &BTreeMap<String, ContractDocument>,
    dependencies: &[ContractDependency],
) -> Vec<ContractEntryRef> {
    let consumed = dependencies
        .iter()
        .map(|dependency| dependency.provider.clone())
        .collect::<BTreeSet<_>>();
    let mut unconsumed = contracts
        .iter()
        .flat_map(|(canonical_spec, contract)| {
            contract.exports.iter().map(move |export| ContractEntryRef {
                canonical_spec: canonical_spec.clone(),
                section: ContractSection::Exports,
                entry_id: export.id.clone(),
            })
        })
        .filter(|entry| !consumed.contains(entry))
        .collect::<Vec<_>>();
    unconsumed.sort();
    unconsumed
}

fn discover_spec_ids(specbind_root: &Path) -> (BTreeSet<String>, Vec<DiscoveryIssue>) {
    let discovery = match artifacts::discover_spec_ids(specbind_root) {
        Ok(discovery) => discovery,
        Err(error) => {
            return (
                BTreeSet::new(),
                vec![discovery_issue(
                    "CONTRACT_GRAPH_SPECS_READ_FAILED",
                    Some(Utf8PathBuf::from("specs")),
                    format!("cannot enumerate persistent specs: {error}"),
                )],
            );
        }
    };
    let issues = discovery
        .faults
        .into_iter()
        .map(|(path, fault)| {
            let path = path.or_else(|| Some(Utf8PathBuf::from("specs")));
            match fault {
                SpecEntryFault::Unreadable(error) => discovery_issue(
                    "CONTRACT_GRAPH_SPEC_ENTRY_READ_FAILED",
                    path,
                    format!("cannot inspect persistent spec entry: {error}"),
                ),
                SpecEntryFault::NonUtf8Name => discovery_issue(
                    "CONTRACT_GRAPH_SPEC_ID_INVALID",
                    path,
                    "persistent spec directory name must be UTF-8 lowercase kebab-case",
                ),
                SpecEntryFault::NotADirectory => discovery_issue(
                    "CONTRACT_GRAPH_SPEC_PATH_INVALID",
                    path,
                    "persistent spec path must be a regular non-symlink directory",
                ),
                SpecEntryFault::InvalidId => discovery_issue(
                    "CONTRACT_GRAPH_SPEC_ID_INVALID",
                    path,
                    "persistent spec directory name must be lowercase kebab-case",
                ),
            }
        })
        .collect();
    (discovery.specs, issues)
}

fn resolve_dependencies(
    expected_specs: &BTreeSet<String>,
    contracts: &BTreeMap<String, ContractDocument>,
    issues: &mut Vec<ContractGraphIssue>,
) -> Vec<ContractDependency> {
    let mut dependencies = Vec::new();
    for (consumer_spec, contract) in contracts {
        for consume in &contract.consumes {
            let consumer = ContractEntryRef {
                canonical_spec: consumer_spec.clone(),
                section: ContractSection::Consumes,
                entry_id: consume.id.clone(),
            };
            let provider = ContractEntryRef {
                canonical_spec: consume.target.canonical_spec.clone(),
                section: consume.target.section,
                entry_id: consume.target.entry_id.clone(),
            };
            let source = Some(entry_selector(&consumer));
            if consumer_spec == &provider.canonical_spec {
                issues.push(graph_issue(
                    GraphIssueSeverity::Error,
                    "CONTRACT_GRAPH_SELF_CONSUME",
                    source,
                    format!(
                        "Consumes entry {} must target a boundary owned by another spec",
                        consume.id
                    ),
                ));
            } else if !expected_specs.contains(&provider.canonical_spec) {
                issues.push(graph_issue(
                    GraphIssueSeverity::Error,
                    "CONTRACT_GRAPH_TARGET_SPEC_MISSING",
                    source,
                    format!(
                        "Consumes target spec {} does not exist",
                        provider.canonical_spec
                    ),
                ));
            } else if let Some(target_contract) = contracts.get(&provider.canonical_spec) {
                if entry_exists(target_contract, provider.section, &provider.entry_id) {
                    dependencies.push(ContractDependency { consumer, provider });
                } else {
                    issues.push(graph_issue(
                        GraphIssueSeverity::Error,
                        "CONTRACT_GRAPH_TARGET_ENTRY_MISSING",
                        source,
                        format!(
                            "Consumes target {} does not resolve",
                            entry_selector(&provider)
                        ),
                    ));
                }
            } else {
                issues.push(graph_issue(
                    GraphIssueSeverity::Error,
                    "CONTRACT_GRAPH_TARGET_CONTRACT_UNAVAILABLE",
                    source,
                    format!(
                        "Consumes target spec {} has no valid Contract",
                        provider.canonical_spec
                    ),
                ));
            }
        }
    }
    dependencies.sort();
    dependencies.dedup();
    dependencies
}

fn entry_exists(document: &ContractDocument, section: ContractSection, id: &str) -> bool {
    match section {
        ContractSection::Owns => document.owns.iter().any(|entry| entry.id == id),
        ContractSection::Exports => document.exports.iter().any(|entry| entry.id == id),
        ContractSection::Invariants => document.invariants.iter().any(|entry| entry.id == id),
        ContractSection::FileOwnership => {
            document.file_ownership.iter().any(|entry| entry.id == id)
        }
        ContractSection::Consumes => false,
    }
}

fn ownership_findings(contracts: &BTreeMap<String, ContractDocument>) -> Vec<OwnershipFinding> {
    let mut claims = Vec::new();
    for (canonical_spec, contract) in contracts {
        for entry in &contract.file_ownership {
            for path in &entry.paths {
                claims.push(OwnershipClaim {
                    entry: ContractEntryRef {
                        canonical_spec: canonical_spec.clone(),
                        section: ContractSection::FileOwnership,
                        entry_id: entry.id.clone(),
                    },
                    path: path.clone(),
                    normalized: path.to_ascii_lowercase(),
                    subtree: path.ends_with("/**"),
                });
            }
        }
    }
    claims.sort_by(|left, right| {
        (&left.entry, &left.normalized).cmp(&(&right.entry, &right.normalized))
    });

    let mut findings = Vec::new();
    for (index, left) in claims.iter().enumerate() {
        for right in &claims[index + 1..] {
            if left.entry.canonical_spec == right.entry.canonical_spec {
                continue;
            }
            let kind = if left.normalized == right.normalized {
                Some(OwnershipFindingKind::Duplicate)
            } else if patterns_overlap(left, right) {
                Some(OwnershipFindingKind::Overlap)
            } else {
                None
            };
            if let Some(kind) = kind {
                findings.push(OwnershipFinding {
                    kind,
                    left: left.entry.clone(),
                    left_path: left.path.clone(),
                    right: right.entry.clone(),
                    right_path: right.path.clone(),
                });
            }
        }
    }
    findings.sort();
    findings.dedup();
    findings
}

fn patterns_overlap(left: &OwnershipClaim, right: &OwnershipClaim) -> bool {
    let left_base = left
        .normalized
        .strip_suffix("/**")
        .unwrap_or(&left.normalized);
    let right_base = right
        .normalized
        .strip_suffix("/**")
        .unwrap_or(&right.normalized);
    match (left.subtree, right.subtree) {
        (false, false) => false,
        (true, false) => path_contains(left_base, right_base),
        (false, true) => path_contains(right_base, left_base),
        (true, true) => {
            path_contains(left_base, right_base) || path_contains(right_base, left_base)
        }
    }
}

fn path_contains(directory: &str, candidate: &str) -> bool {
    candidate == directory || candidate.starts_with(&format!("{directory}/"))
}

fn dependency_cycles(
    contracts: &BTreeMap<String, ContractDocument>,
    dependencies: &[ContractDependency],
) -> Vec<Vec<String>> {
    let mut graph = DiGraphMap::<&str, ()>::new();
    let mut adjacency = BTreeMap::<&str, BTreeSet<&str>>::new();
    for canonical_spec in contracts.keys() {
        graph.add_node(canonical_spec);
        adjacency.entry(canonical_spec).or_default();
    }
    for dependency in dependencies {
        let consumer = dependency.consumer.canonical_spec.as_str();
        let provider = dependency.provider.canonical_spec.as_str();
        graph.add_edge(consumer, provider, ());
        adjacency.entry(consumer).or_default().insert(provider);
    }

    let mut cycles = Vec::new();
    for component in tarjan_scc(&graph) {
        if component.len() < 2 {
            continue;
        }
        let allowed = component.into_iter().collect::<BTreeSet<_>>();
        if let Some(start) = allowed.first().copied() {
            let mut path = vec![start];
            let mut seen = BTreeSet::from([start]);
            if let Some(cycle) =
                find_cycle(start, start, &allowed, &adjacency, &mut path, &mut seen)
            {
                cycles.push(cycle.into_iter().map(str::to_owned).collect());
            }
        }
    }
    cycles.sort();
    cycles.dedup();
    cycles
}

fn find_cycle<'a>(
    current: &'a str,
    start: &'a str,
    allowed: &BTreeSet<&'a str>,
    adjacency: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    path: &mut Vec<&'a str>,
    seen: &mut BTreeSet<&'a str>,
) -> Option<Vec<&'a str>> {
    for next in adjacency.get(current).into_iter().flatten() {
        if !allowed.contains(next) {
            continue;
        }
        if *next == start {
            let mut cycle = path.clone();
            cycle.push(start);
            return Some(cycle);
        }
        if seen.insert(next) {
            path.push(next);
            if let Some(cycle) = find_cycle(next, start, allowed, adjacency, path, seen) {
                return Some(cycle);
            }
            path.pop();
            seen.remove(next);
        }
    }
    None
}

fn entry_selector(entry: &ContractEntryRef) -> String {
    format!(
        "specs/{}#contract/{}/{}",
        entry.canonical_spec,
        entry.section.token(),
        entry.entry_id
    )
}

fn graph_issue(
    severity: GraphIssueSeverity,
    code: &'static str,
    source: Option<String>,
    message: impl Into<String>,
) -> ContractGraphIssue {
    ContractGraphIssue {
        severity,
        code,
        source,
        message: message.into(),
    }
}

fn discovery_issue(
    code: &'static str,
    path: Option<Utf8PathBuf>,
    message: impl Into<String>,
) -> DiscoveryIssue {
    DiscoveryIssue {
        code,
        path,
        message: message.into(),
    }
}
