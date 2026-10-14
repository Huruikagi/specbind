//! Contract validation, graph projections, ownership, and shared agreement reads.

use std::{fmt::Write as _, path::Path};

use super::super::{
    CommandOutput,
    output::{escape, push_field, yes_no},
    render_issue,
};
use crate::{
    config,
    contract_graph::{self, GraphIssueSeverity},
};

#[must_use]
pub fn check_contracts(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let graph = contract_graph::resolve(&paths.specbind_root);
    let errors = contract_graph_errors(&graph);
    if !errors.is_empty() {
        return CommandOutput::failure(
            "CONTRACTS_FAILED",
            "Contract graph has structural diagnostics.",
            errors,
        );
    }
    let warnings = graph
        .report
        .issues
        .iter()
        .filter(|issue| issue.severity == GraphIssueSeverity::Warning)
        .map(render_graph_issue)
        .collect::<Vec<_>>();
    let mut output = format!(
        "OK CONTRACTS_VERIFIED: Verified {} contract(s) and {} dependency reference(s).\n",
        graph.report.contracts.len(),
        graph.report.dependencies.len()
    );
    push_field(
        &mut output,
        "Ownership findings",
        &graph.report.ownership_findings.len().to_string(),
    );
    push_field(
        &mut output,
        "Dependency cycles",
        &graph.report.dependency_cycles.len().to_string(),
    );
    if warnings.is_empty() {
        push_field(&mut output, "Warnings", "none");
    } else {
        output.push_str("  Warnings:\n");
        for warning in warnings {
            writeln!(output, "    - {warning}").expect("writing to a String cannot fail");
        }
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn contract_graph(start: &Path) -> CommandOutput {
    let graph = match readable_contract_graph(start) {
        Ok(graph) => graph,
        Err(output) => return output,
    };
    let mut output = format!(
        "OK CONTRACT_GRAPH_REPORTED: Reported {} contract(s) and {} direct dependency reference(s).\n",
        graph.report.contracts.len(),
        graph.report.dependencies.len()
    );
    push_dependency_list(&mut output, "Dependencies", &graph.report.dependencies);
    if let Some(shared) = &graph.report.shared {
        push_field(
            &mut output,
            "Shared resources",
            &shared.as_wire().resources.len().to_string(),
        );
    }
    push_field(
        &mut output,
        "Graph warnings",
        &graph
            .report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Warning)
            .count()
            .to_string(),
    );
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn contract_dependencies(start: &Path, canonical_spec: &str) -> CommandOutput {
    let graph = match readable_contract_graph(start) {
        Ok(graph) => graph,
        Err(output) => return output,
    };
    if !graph.report.contracts.contains_key(canonical_spec) {
        return contract_spec_not_found(canonical_spec);
    }
    let dependencies = graph
        .report
        .dependencies
        .iter()
        .filter(|dependency| dependency.consumer.owner.spec() == Some(canonical_spec))
        .cloned()
        .collect::<Vec<_>>();
    let mut output = format!(
        "OK CONTRACT_DEPENDENCIES_REPORTED: Reported {} direct provider reference(s) for spec {}.\n",
        dependencies.len(),
        escape(canonical_spec)
    );
    push_dependency_list(&mut output, "Dependencies", &dependencies);
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn contract_consumers(start: &Path, canonical_spec: &str) -> CommandOutput {
    let graph = match readable_contract_graph(start) {
        Ok(graph) => graph,
        Err(output) => return output,
    };
    if !graph.report.contracts.contains_key(canonical_spec) {
        return contract_spec_not_found(canonical_spec);
    }
    let consumers = graph
        .report
        .dependencies
        .iter()
        .filter(|dependency| dependency.provider.owner.spec() == Some(canonical_spec))
        .cloned()
        .collect::<Vec<_>>();
    let mut output = format!(
        "OK CONTRACT_CONSUMERS_REPORTED: Reported {} direct consumer reference(s) for spec {}.\n",
        consumers.len(),
        escape(canonical_spec)
    );
    push_dependency_list(&mut output, "Consumers", &consumers);
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn contract_owners(start: &Path, path: &str) -> CommandOutput {
    let graph = match readable_contract_graph(start) {
        Ok(graph) => graph,
        Err(output) => return output,
    };
    let owners = match contract_graph::owners_for_path(&graph.report, path) {
        Ok(owners) => owners,
        Err(contract_graph::InvalidOwnershipQuery) => {
            return CommandOutput::failure(
                "CONTRACT_OWNERS_PATH_INVALID",
                "File Ownership lookup requires one concrete project-relative portable path.",
                vec![format!("Path: {}", escape(path))],
            );
        }
    };
    let owning_specs = owners
        .iter()
        .filter_map(|owner| owner.owner.owner.spec())
        .collect::<std::collections::BTreeSet<_>>();
    let has_shared = owners
        .iter()
        .any(|owner| owner.owner.owner == crate::contract::ContractOwner::Shared);
    let mut output = if has_shared {
        format!(
            "OK CONTRACT_OWNERS_REPORTED: Reported {} Spec/shared management declaration(s) for path {}.\n",
            owners.len(),
            escape(path)
        )
    } else {
        format!(
            "OK CONTRACT_OWNERS_REPORTED: Reported {} File Ownership declaration(s) from {} Spec(s) for path {}.\n",
            owners.len(),
            owning_specs.len(),
            escape(path)
        )
    };
    if owners.is_empty() {
        push_field(&mut output, "Owners", "none");
    } else {
        output.push_str("  Owners:\n");
        for owner in &owners {
            writeln!(
                output,
                "    - {} path={}",
                render_contract_entry(&owner.owner),
                escape(&owner.declared_path)
            )
            .expect("writing to a String cannot fail");
        }
    }
    push_field(
        &mut output,
        "Ambiguous across Specs",
        yes_no(owning_specs.len() > 1),
    );
    if has_shared {
        let boundaries = owners
            .iter()
            .map(|value| &value.owner)
            .collect::<std::collections::BTreeSet<_>>();
        push_field(
            &mut output,
            "Ambiguous management boundaries",
            yes_no(boundaries.len() > 1),
        );
    }
    CommandOutput::success(output.into_bytes())
}

fn readable_contract_graph(
    start: &Path,
) -> Result<contract_graph::ContractGraphResolution, CommandOutput> {
    let paths = config::resolve_from(start)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))?;
    let graph = contract_graph::resolve(&paths.specbind_root);
    let errors = contract_graph_errors(&graph);
    if errors.is_empty() {
        Ok(graph)
    } else {
        Err(CommandOutput::failure(
            "CONTRACT_GRAPH_READ_FAILED",
            "Cannot report an incomplete Contract graph.",
            errors,
        ))
    }
}

fn contract_graph_errors(graph: &contract_graph::ContractGraphResolution) -> Vec<String> {
    let mut errors = graph
        .project_issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    for (spec, inventory) in &graph.inventories {
        errors.extend(
            inventory
                .issues
                .iter()
                .map(|issue| format!("{} spec {spec}", render_issue(issue))),
        );
    }
    errors.extend(
        graph
            .report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Error)
            .map(render_graph_issue),
    );
    errors
}

fn contract_spec_not_found(canonical_spec: &str) -> CommandOutput {
    CommandOutput::failure(
        "CONTRACT_SPEC_NOT_FOUND",
        format!(
            "Persistent spec {} does not have a resolved Contract in the project graph.",
            escape(canonical_spec)
        ),
        vec![],
    )
}

fn push_dependency_list(
    output: &mut String,
    label: &str,
    dependencies: &[contract_graph::ContractDependency],
) {
    if dependencies.is_empty() {
        push_field(output, label, "none");
        return;
    }
    writeln!(output, "  {label}:").expect("writing to a String cannot fail");
    for dependency in dependencies {
        writeln!(output, "    - {}", render_dependency(dependency))
            .expect("writing to a String cannot fail");
    }
}

fn render_dependency(dependency: &contract_graph::ContractDependency) -> String {
    format!(
        "{} -> {}",
        render_contract_entry(&dependency.consumer),
        render_contract_entry(&dependency.provider)
    )
}

fn render_contract_entry(entry: &contract_graph::ContractEntryRef) -> String {
    escape(&contract_graph::entry_selector(entry))
}

fn render_graph_issue(issue: &contract_graph::ContractGraphIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn contract_shared(start: &Path, consumers: bool, resource_id: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let shared = match crate::read_model::shared_contract::read(&paths.specbind_root) {
        Ok(shared) => shared,
        Err(message) => {
            return CommandOutput::failure("SHARED_CONTRACT_READ_INVALID", message, vec![]);
        }
    };
    if let Some(id) = resource_id
        && !shared.as_ref().is_some_and(|value| {
            value
                .as_wire()
                .resources
                .iter()
                .any(|resource| resource.id.0 == id)
        })
    {
        return CommandOutput::failure(
            "SHARED_CONTRACT_RESOURCE_NOT_FOUND",
            "Unknown shared resource.",
            vec![escape(id)],
        );
    }
    let Some(shared) = shared else {
        return CommandOutput::success(
            b"OK SHARED_CONTRACT_ABSENT: Project shared Contract is not in use.\n".to_vec(),
        );
    };
    if consumers {
        let graph = match readable_contract_graph(start) {
            Ok(value) => value,
            Err(output) => return output,
        };
        let entries = graph
            .report
            .dependencies
            .iter()
            .filter(|dependency| {
                dependency.provider.owner == crate::contract::ContractOwner::Shared
                    && resource_id.is_none_or(|id| dependency.provider.entry_id == id)
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut output = String::from(
            "OK SHARED_CONTRACT_CONSUMERS_REPORTED: Reported declared shared resource consumers.\n",
        );
        push_dependency_list(&mut output, "Consumers", &entries);
        CommandOutput::success(output.into_bytes())
    } else {
        match serde_saphyr::to_string(shared.as_wire()) {
            Ok(content) => CommandOutput::success(content.into_bytes()),
            Err(error) => {
                CommandOutput::failure("SHARED_CONTRACT_RENDER_FAILED", error.to_string(), vec![])
            }
        }
    }
}
