use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use specbind::contract;
use specbind::contract_graph::{self, GraphIssueSeverity, OwnershipFindingKind};
use tempfile::TempDir;

const EMPTY: &str =
    "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n";

fn document(body: &str) -> contract::ContractDocument {
    contract::parse(body).expect("valid Contract fixture")
}

fn contracts(values: &[(&str, &str)]) -> BTreeMap<String, contract::ContractDocument> {
    values
        .iter()
        .map(|(spec, body)| ((*spec).to_owned(), document(body)))
        .collect()
}

fn specs(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture");
}

#[test]
fn resolves_typed_consumes_edges() {
    let report = contract_graph::evaluate(
        &specs(&["catalog", "checkout"]),
        contracts(&[
            (
                "catalog",
                "# Contract\n\n## Owns\n\n## Exports\n\n- `stock-status` — Stock status.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
            ),
            (
                "checkout",
                "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `inventory` → `catalog/exports/stock-status`\n\n## Invariants\n\n## File Ownership\n",
            ),
        ]),
    );

    assert!(report.issues.is_empty(), "{:?}", report.issues);
    assert_eq!(report.dependencies.len(), 1);
    assert_eq!(report.dependencies[0].consumer.canonical_spec, "checkout");
    assert_eq!(report.dependencies[0].provider.canonical_spec, "catalog");
    assert_eq!(report.dependencies[0].provider.entry_id, "stock-status");
}

#[test]
fn distinguishes_unavailable_specs_contracts_entries_and_self_consumes() {
    let report = contract_graph::evaluate(
        &specs(&["consumer", "missing-contract"]),
        contracts(&[(
            "consumer",
            "# Contract\n\n## Owns\n\n- `local` — Local boundary.\n\n## Exports\n\n## Consumes\n\n- `unknown-spec` → `absent/exports/value`\n- `unknown-contract` → `missing-contract/exports/value`\n- `unknown-entry` → `consumer/exports/value`\n- `self` → `consumer/owns/local`\n\n## Invariants\n\n## File Ownership\n",
        )]),
    );
    let codes = report
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<BTreeSet<_>>();

    assert!(codes.contains("CONTRACT_GRAPH_CONTRACT_UNAVAILABLE"));
    assert!(codes.contains("CONTRACT_GRAPH_TARGET_SPEC_MISSING"));
    assert!(codes.contains("CONTRACT_GRAPH_TARGET_CONTRACT_UNAVAILABLE"));
    assert!(codes.contains("CONTRACT_GRAPH_SELF_CONSUME"));
    assert!(
        report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Error)
            .all(|issue| issue.code != "CONTRACT_GRAPH_TARGET_ENTRY_MISSING")
    );
}

#[test]
fn reports_missing_target_entries_in_valid_target_contracts() {
    let report = contract_graph::evaluate(
        &specs(&["consumer", "provider"]),
        contracts(&[
            ("provider", EMPTY),
            (
                "consumer",
                "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `missing` → `provider/exports/value`\n\n## Invariants\n\n## File Ownership\n",
            ),
        ]),
    );

    assert!(report.issues.iter().any(|issue| {
        issue.code == "CONTRACT_GRAPH_TARGET_ENTRY_MISSING"
            && issue.source.as_deref() == Some("specs/consumer#contract/consumes/missing")
    }));
}

#[test]
fn reports_cross_spec_duplicate_and_overlapping_ownership_as_warnings() {
    let report = contract_graph::evaluate(
        &specs(&["alpha", "beta", "gamma"]),
        contracts(&[
            (
                "alpha",
                "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `api` — `src/API/**`\n",
            ),
            (
                "beta",
                "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `api-file` — `src/api/handler.rs`\n",
            ),
            (
                "gamma",
                "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `api-copy` — `src/api/**`\n",
            ),
        ]),
    );

    assert!(report.ownership_findings.iter().any(|finding| {
        finding.kind == OwnershipFindingKind::Duplicate
            && finding.left.entry_id == "api"
            && finding.right.entry_id == "api-copy"
    }));
    assert!(report.ownership_findings.iter().any(|finding| {
        finding.kind == OwnershipFindingKind::Overlap && finding.right.entry_id == "api-file"
    }));
    assert!(
        report
            .issues
            .iter()
            .filter(|issue| issue.code.starts_with("CONTRACT_GRAPH_OWNERSHIP_"))
            .all(|issue| issue.severity == GraphIssueSeverity::Warning)
    );
}

#[test]
fn reports_a_deterministic_dependency_cycle_path() {
    let report = contract_graph::evaluate(
        &specs(&["alpha", "beta", "gamma"]),
        contracts(&[
            (
                "alpha",
                "# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n- `beta-value` → `beta/exports/value`\n\n## Invariants\n\n## File Ownership\n",
            ),
            (
                "beta",
                "# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n- `gamma-value` → `gamma/exports/value`\n\n## Invariants\n\n## File Ownership\n",
            ),
            (
                "gamma",
                "# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n- `alpha-value` → `alpha/exports/value`\n\n## Invariants\n\n## File Ownership\n",
            ),
        ]),
    );

    assert_eq!(
        report.dependency_cycles,
        [vec!["alpha", "beta", "gamma", "alpha"]]
    );
    let warning = report
        .issues
        .iter()
        .find(|issue| issue.code == "CONTRACT_GRAPH_DEPENDENCY_CYCLE")
        .expect("cycle warning");
    assert_eq!(warning.severity, GraphIssueSeverity::Warning);
}

#[test]
fn resolves_every_immediate_persistent_spec_and_keeps_partial_inventories() {
    let root = TempDir::new().expect("temporary SpecBind root");
    write(
        root.path(),
        "specs/catalog/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `stock` — Stock.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `stock` → `catalog/exports/stock`\n\n## Invariants\n\n## File Ownership\n",
    );
    write(root.path(), "specs/missing/index.md", "# Reserved index\n");
    write(root.path(), "specs/not-a-directory", "invalid\n");

    let resolution = contract_graph::resolve(root.path());

    assert_eq!(
        resolution
            .inventories
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["catalog", "checkout", "missing"]
    );
    assert_eq!(resolution.report.dependencies.len(), 1);
    assert!(resolution.report.issues.iter().any(|issue| {
        issue.code == "CONTRACT_GRAPH_CONTRACT_UNAVAILABLE"
            && issue.source.as_deref() == Some("specs/missing#contract")
    }));
    assert!(
        resolution
            .project_issues
            .iter()
            .any(|issue| issue.code == "CONTRACT_GRAPH_SPEC_PATH_INVALID")
    );
}
