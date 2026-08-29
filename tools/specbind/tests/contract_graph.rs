use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use specbind::contract_graph::{self, GraphIssueSeverity, OwnershipFindingKind};
use specbind::{contract, schema::runtime};
use tempfile::TempDir;

const EMPTY: &str =
    "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n";

fn document(body: &str) -> contract::ContractDocument {
    runtime::load_contract(body)
        .expect("structurally valid Contract fixture")
        .try_into()
        .expect("semantically valid Contract fixture")
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
                "schema_version: 1\nowns: []\nexports:\n  - { id: stock-status, description: Stock status. }\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
            ),
            (
                "checkout",
                "schema_version: 1\nowns: []\nexports: []\nconsumes:\n  - id: inventory\n    target: { spec: catalog, section: exports, id: stock-status }\ninvariants: []\nfile_ownership: []\n",
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
            "schema_version: 1\nowns:\n  - { id: local, description: Local boundary. }\nexports: []\nconsumes:\n  - { id: unknown-spec, target: { spec: absent, section: exports, id: value } }\n  - { id: unknown-contract, target: { spec: missing-contract, section: exports, id: value } }\n  - { id: unknown-entry, target: { spec: consumer, section: exports, id: value } }\n  - { id: self, target: { spec: consumer, section: owns, id: local } }\ninvariants: []\nfile_ownership: []\n",
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
                "schema_version: 1\nowns: []\nexports: []\nconsumes:\n  - { id: missing, target: { spec: provider, section: exports, id: value } }\ninvariants: []\nfile_ownership: []\n",
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
                "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership:\n  - { id: api, paths: [src/API/**] }\n",
            ),
            (
                "beta",
                "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership:\n  - { id: api-file, paths: [src/api/handler.rs] }\n",
            ),
            (
                "gamma",
                "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership:\n  - { id: api-copy, paths: [src/api/**] }\n",
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
                "schema_version: 1\nowns: []\nexports:\n  - { id: value, description: Value. }\nconsumes:\n  - { id: beta-value, target: { spec: beta, section: exports, id: value } }\ninvariants: []\nfile_ownership: []\n",
            ),
            (
                "beta",
                "schema_version: 1\nowns: []\nexports:\n  - { id: value, description: Value. }\nconsumes:\n  - { id: gamma-value, target: { spec: gamma, section: exports, id: value } }\ninvariants: []\nfile_ownership: []\n",
            ),
            (
                "gamma",
                "schema_version: 1\nowns: []\nexports:\n  - { id: value, description: Value. }\nconsumes:\n  - { id: alpha-value, target: { spec: alpha, section: exports, id: value } }\ninvariants: []\nfile_ownership: []\n",
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
        "specs/catalog/contract.yaml",
        "schema_version: 1\nowns: []\nexports:\n  - { id: stock, description: Stock. }\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
    );
    write(
        root.path(),
        "specs/checkout/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes:\n  - { id: stock, target: { spec: catalog, section: exports, id: stock } }\ninvariants: []\nfile_ownership: []\n",
    );
    write(root.path(), "specs/missing/index.md", "# Reserved index\n");
    write(
        root.path(),
        "specs/invalid/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\nforbidden: true\n",
    );
    write(root.path(), "specs/not-a-directory", "invalid\n");

    let resolution = contract_graph::resolve(root.path());

    assert_eq!(
        resolution
            .inventories
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["catalog", "checkout", "invalid", "missing"]
    );
    assert_eq!(resolution.report.dependencies.len(), 1);
    assert!(resolution.report.issues.iter().any(|issue| {
        issue.code == "CONTRACT_GRAPH_CONTRACT_UNAVAILABLE"
            && issue.source.as_deref() == Some("specs/missing#contract")
    }));
    assert!(resolution.report.issues.iter().any(|issue| {
        issue.code == "CONTRACT_GRAPH_CONTRACT_UNAVAILABLE"
            && issue.source.as_deref() == Some("specs/invalid#contract")
    }));
    assert!(
        resolution
            .project_issues
            .iter()
            .any(|issue| issue.code == "CONTRACT_GRAPH_SPEC_PATH_INVALID")
    );
}

#[test]
fn warns_when_an_exported_seam_reaches_no_managed_consumer() {
    let report = contract_graph::evaluate(
        &specs(&["catalog", "checkout"]),
        contracts(&[
            (
                "catalog",
                "schema_version: 1\nowns: []\nexports:\n  - { id: stock-status, description: Stock status. }\n  - { id: restock-forecast, description: Forecast nobody consumes. }\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
            ),
            (
                "checkout",
                "schema_version: 1\nowns: []\nexports: []\nconsumes:\n  - { id: inventory, target: { spec: catalog, section: exports, id: stock-status } }\ninvariants: []\nfile_ownership: []\n",
            ),
        ]),
    );
    let unconsumed = report
        .issues
        .iter()
        .filter(|issue| issue.code == "CONTRACT_GRAPH_EXPORT_UNCONSUMED")
        .collect::<Vec<_>>();

    // The graph cannot tell a premature seam from one whose only consumer is
    // outside it, so this is a warning a reviewer resolves, not an error.
    assert_eq!(unconsumed.len(), 1);
    assert_eq!(unconsumed[0].severity, GraphIssueSeverity::Warning);
    assert_eq!(
        unconsumed[0].source.as_deref(),
        Some("specs/catalog#contract/exports/restock-forecast")
    );
}
