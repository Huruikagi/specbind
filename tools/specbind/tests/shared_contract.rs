use specbind::{
    contract::{Contract, ContractOwner},
    contract_graph,
    cross_spec_review::{self, ReviewFreshnessStatus},
    domain::shared_contract::SharedContract,
    fingerprint::Fingerprint,
    schema::runtime,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
use tempfile::TempDir;

const SHARED: &str = "schema_version: 1\nresources:\n  - id: translations\n    description: Translation catalogs\n    paths: [locales/ja.json, locales/en.json]\n    change_policy: Each feature updates its namespace.\n    invariants: [Matching keys, Matching variables]\n";
const CONSUMER: &str = "schema_version: 2\nowns: []\nexports: []\nconsumes:\n  - id: catalog\n    target: {shared: true, section: resources, id: translations}\ninvariants: []\nfile_ownership: []\n";
const CANDIDATE: &str = r#"{"schemaVersion":1,"assessment":"Shared resources and all consumers remain coherent.","deepInputs":[]}"#;
const MID: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn shared(input: &str) -> SharedContract {
    SharedContract::try_from(runtime::load_shared_contract(input).unwrap()).unwrap()
}
fn write(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
fn commit(root: &Path) -> String {
    git(root, &["add", "."]);
    git(
        root,
        &["commit", "--quiet", "--allow-empty", "-m", "fixture"],
    );
    git(root, &["rev-parse", "HEAD"])
}
fn fixture(with_shared: bool, declared: bool) -> TempDir {
    fixture_at(
        with_shared.then_some("specs/shared-contract.yaml"),
        declared,
    )
}
fn fixture_at(shared_path: Option<&str>, declared: bool) -> TempDir {
    let root = tempfile::tempdir().unwrap();
    git(root.path(), &["init", "--quiet"]);
    git(root.path(), &["config", "user.email", "test@example.com"]);
    git(root.path(), &["config", "user.name", "Test"]);
    write(root.path(), "baseline.txt", "baseline");
    if let Some(shared_path) = shared_path {
        write(root.path(), shared_path, SHARED);
    }
    let baseline = commit(root.path());
    let declaration = if declared {
        "\n      shared_contract_changes: [translations]"
    } else {
        ""
    };
    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MID}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: catalogs\n      summary: Maintain catalogs{declaration}\n---\n# Shared change\nPreserve matching keys.\n"
        ),
    );
    root
}

#[test]
fn reads_the_v1_5_0_path_and_treats_a_path_only_move_as_compatible() {
    let root = fixture_at(Some("shared-contract.yaml"), false);
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::NotRequired
    );
    fs::create_dir_all(root.path().join("specs")).unwrap();
    fs::rename(
        root.path().join("shared-contract.yaml"),
        root.path().join("specs/shared-contract.yaml"),
    )
    .unwrap();
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::NotRequired
    );
}

#[test]
fn rejects_ambiguous_shared_contract_paths() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "shared-contract.yaml", SHARED);
    write(root.path(), "specs/shared-contract.yaml", SHARED);
    let graph = contract_graph::resolve(root.path());
    assert!(graph.project_issues.iter().any(|issue| {
        issue.code == "SHARED_CONTRACT_INVALID" && issue.message.contains("exists at both")
    }));
}

#[test]
fn validates_shared_structure_and_semantics_and_preserves_v1() {
    shared(SHARED);
    for invalid in [
        SHARED.replace("change_policy:", "unknown:"),
        SHARED.replace("paths: [locales/ja.json, locales/en.json]", "paths: []"),
        SHARED.replace("schema_version: 1", "schema_version: 2"),
    ] {
        assert!(runtime::load_shared_contract(&invalid).is_err());
    }
    for invalid in [
        SHARED.replace("Each feature updates its namespace.", "'  '"),
        SHARED.replace("locales/ja.json", "../ja.json"),
        SHARED.replace("locales/en.json", "LOCALES/JA.JSON"),
    ] {
        assert!(
            SharedContract::try_from(runtime::load_shared_contract(&invalid).unwrap()).is_err()
        );
    }
    for invalid in [
        CONSUMER.replace("shared: true", "shared: false"),
        CONSUMER.replace("shared: true", "shared: true, spec: search"),
        CONSUMER.replace("schema_version: 2", "schema_version: 1"),
    ] {
        assert!(runtime::load_contract(&invalid).is_err());
    }
    let v1 = "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n";
    let contract = Contract::try_from(runtime::load_contract(v1).unwrap()).unwrap();
    assert_eq!(contract.as_wire().schema_version, 1);
}

#[test]
fn resolves_shared_consumers_owners_and_real_overlaps() {
    let consumer = Contract::try_from(runtime::load_contract(CONSUMER).unwrap()).unwrap();
    let report = contract_graph::evaluate_with_shared(
        &BTreeSet::from(["search".into(), "checkout".into()]),
        BTreeMap::from([
            ("search".into(), consumer.clone()),
            ("checkout".into(), consumer),
        ]),
        Some(shared(SHARED)),
    );
    assert_eq!(report.dependencies.len(), 2);
    assert!(report.ownership_findings.is_empty());
    assert!(
        report
            .dependencies
            .iter()
            .all(|edge| edge.provider.owner == ContractOwner::Shared)
    );
    let owners = contract_graph::owners_for_path(&report, "LOCALES/JA.JSON").unwrap();
    assert_eq!(owners.len(), 1);
    assert_eq!(
        contract_graph::entry_selector(&owners[0].owner),
        "shared-contract#resources/translations"
    );
    let claimed = CONSUMER.replace(
        "file_ownership: []",
        "file_ownership: [{id: files, paths: [locales/**]}]",
    );
    let report = contract_graph::evaluate_with_shared(
        &BTreeSet::from(["search".into()]),
        BTreeMap::from([(
            "search".into(),
            Contract::try_from(runtime::load_contract(&claimed).unwrap()).unwrap(),
        )]),
        Some(shared(SHARED)),
    );
    assert_eq!(report.ownership_findings.len(), 2);
    assert_eq!(
        contract_graph::owners_for_path(&report, "locales/ja.json")
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn missing_and_invalid_shared_contracts_are_not_partial_success() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "specs/search/contract.yaml", CONSUMER);
    let graph = contract_graph::resolve(root.path());
    assert!(
        graph
            .report
            .issues
            .iter()
            .any(|issue| issue.code == "CONTRACT_GRAPH_SHARED_RESOURCE_MISSING")
    );
    write(root.path(), "specs/shared-contract.yaml", "broken");
    assert!(
        contract_graph::resolve(root.path())
            .project_issues
            .iter()
            .any(|issue| issue.code == "SHARED_CONTRACT_INVALID")
    );
}

#[test]
fn shared_fingerprint_normalizes_order_but_detects_rules_and_absence() {
    let first = Fingerprint::shared_contract(Some(&shared(SHARED))).unwrap();
    let reordered = SHARED
        .replace(
            "locales/ja.json, locales/en.json",
            "locales/en.json, locales/ja.json",
        )
        .replace(
            "Matching keys, Matching variables",
            "Matching variables, Matching keys",
        );
    assert_eq!(
        first,
        Fingerprint::shared_contract(Some(&shared(&reordered))).unwrap()
    );
    assert_ne!(
        first,
        Fingerprint::shared_contract(Some(&shared(
            &SHARED.replace("Matching keys", "Different rule")
        )))
        .unwrap()
    );
    assert_ne!(
        Fingerprint::shared_contract(None).unwrap(),
        Fingerprint::shared_contract(Some(&shared("schema_version: 1\nresources: []\n"))).unwrap()
    );
}

#[test]
fn direct_only_shared_change_is_reviewed_before_completion_and_rechecked_after_completion() {
    let root = fixture(false, true);
    write(root.path(), "specs/shared-contract.yaml", SHARED);
    commit(root.path());
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Missing
    );
    assert!(specbind::completion::direct_preflight(root.path(), root.path(), "catalogs").is_err());
    cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap();
    let revision = commit(root.path());
    specbind::completion::direct_complete(root.path(), root.path(), "catalogs", &revision).unwrap();
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Fresh
    );
    write(root.path(), "locales/ja.json", "{}");
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Fresh
    );
    write(
        root.path(),
        "specs/shared-contract.yaml",
        &SHARED.replace("Matching keys", "New promise"),
    );
    commit(root.path());
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Stale
    );
    assert!(specbind::completion::direct_preflight(root.path(), root.path(), "catalogs").is_err());
    assert!(
        cross_spec_review::require_for_boundary(
            root.path(),
            root.path(),
            cross_spec_review::ReviewBoundary::ReleasePreflight
        )
        .is_err()
    );
}

#[test]
fn unscoped_creation_and_deletion_require_review_and_scope_repair() {
    for original in [false, true] {
        let root = fixture(original, false);
        if original {
            fs::remove_file(root.path().join("specs/shared-contract.yaml")).unwrap();
        } else {
            write(root.path(), "specs/shared-contract.yaml", SHARED);
        }
        assert_eq!(
            cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
            ReviewFreshnessStatus::Missing
        );
        let error = cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap_err();
        assert!(error.issues.iter().any(|issue| matches!(
            issue.code,
            "SHARED_CONTRACT_CHANGE_UNSCOPED" | "CONTRACT_REVIEW_DIRECT_ONLY"
        )));
        assert!(cross_spec_review::require_shared_for_direct(root.path(), root.path()).is_err());
    }
}

#[test]
fn completed_shared_obligations_are_locked_but_followup_direct_work_is_allowed() {
    let root = fixture(false, true);
    write(root.path(), "specs/shared-contract.yaml", SHARED);
    commit(root.path());
    cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap();
    let revision = commit(root.path());
    specbind::completion::direct_complete(root.path(), root.path(), "catalogs", &revision).unwrap();
    commit(root.path());
    let input = specbind::milestone_scope::resolve(root.path(), true)
        .unwrap()
        .unwrap();
    let mut scope: serde_json::Value = serde_json::from_str(&input).unwrap();
    let original = scope["workItems"]["directChanges"][0].clone();
    scope["workItems"]["directChanges"][0]["summary"] = "Replace the completed obligation".into();
    assert!(
        specbind::milestone::update_scope(root.path(), root.path(), &scope.to_string()).is_err()
    );
    scope["workItems"]["directChanges"][0] = original.clone();
    let mut followup = original;
    followup["id"] = "catalog-followup".into();
    followup["summary"] = "Refine catalog guarantees".into();
    scope["workItems"]["directChanges"]
        .as_array_mut()
        .unwrap()
        .push(followup);
    specbind::milestone::update_scope(root.path(), root.path(), &scope.to_string()).unwrap();
    let roadmap = specbind::roadmap::parse(
        &fs::read_to_string(root.path().join("steering/roadmap.md")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        roadmap.direct_changes[0].status,
        Some(specbind::roadmap::DirectStatus::Completed)
    );
    assert_ne!(
        roadmap.direct_changes[1].status,
        Some(specbind::roadmap::DirectStatus::Completed)
    );
    assert_ne!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Fresh
    );
}

#[test]
fn declared_deletion_is_reviewable_and_manifest_is_not_a_spec() {
    let root = fixture(true, true);
    fs::remove_file(root.path().join("specs/shared-contract.yaml")).unwrap();
    cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap();
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Fresh
    );
    assert!(
        contract_graph::resolve(root.path())
            .report
            .contracts
            .is_empty()
    );
}

#[test]
fn ordinary_direct_with_unchanged_shared_catalog_needs_no_review() {
    let root = fixture(true, false);
    write(
        root.path(),
        "locales/ja.json",
        "{\"search.title\":\"Search\"}",
    );
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::NotRequired
    );
    assert!(cross_spec_review::accept(root.path(), root.path(), CANDIDATE).is_err());
}

#[test]
fn malformed_shared_presence_is_invalid_even_before_the_first_review() {
    for declared in [false, true] {
        let root = fixture(false, declared);
        write(
            root.path(),
            "specs/shared-contract.yaml",
            "resources: [broken",
        );
        assert_eq!(
            cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
            ReviewFreshnessStatus::Invalid
        );
        assert!(cross_spec_review::require_shared_for_direct(root.path(), root.path()).is_err());
    }
}

#[test]
fn shared_scope_roundtrip_and_scheduler_keep_preparation_before_direct_execution() {
    let root = fixture(false, true);
    let scope = specbind::milestone_scope::resolve(root.path(), true)
        .unwrap()
        .unwrap();
    let json: serde_json::Value = serde_json::from_str(&scope).unwrap();
    assert_eq!(
        json["workItems"]["directChanges"][0]["sharedContractChanges"][0],
        "translations"
    );
    let model = specbind::milestone_status::resolve(root.path(), root.path())
        .unwrap()
        .unwrap();
    assert!(model.actionable.iter().any(|action| action.action
        == specbind::milestone_status::MilestoneActionKind::SharedContractPlan));
    assert!(
        !model.actionable.iter().any(|action| action.action
            == specbind::milestone_status::MilestoneActionKind::Implementation)
    );
}

#[test]
fn shared_direct_release_archives_review_and_retains_the_agreement() {
    let root = fixture(false, true);
    write(root.path(), "specs/shared-contract.yaml", SHARED);
    commit(root.path());
    cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap();
    let revision = commit(root.path());
    specbind::completion::direct_complete(root.path(), root.path(), "catalogs", &revision).unwrap();
    commit(root.path());
    specbind::milestone::bind_release(root.path(), root.path(), "v1.0.0", false).unwrap();
    commit(root.path());
    specbind::release_readiness::resolve(root.path(), root.path()).unwrap();
    specbind::release_finalize::finalize(
        root.path(),
        root.path(),
        specbind::config::ProjectLanguage::En,
        None,
    )
    .unwrap();
    assert!(root.path().join("specs/shared-contract.yaml").is_file());
    assert!(!root.path().join("state/contract-review.md").exists());
    let archive = specbind::release::archive_targets("v1.0.0").unwrap();
    assert!(root.path().join(&archive.cross_spec_review).is_file());
    assert!(root.path().join(&archive.roadmap).is_file());
    assert!(matches!(
        specbind::release_finalize::finalize(
            root.path(),
            root.path(),
            specbind::config::ProjectLanguage::En,
            None
        )
        .unwrap(),
        specbind::release_finalize::FinalizeOutcome::AlreadyFinalized { .. }
    ));
}

#[test]
fn review_rejects_an_unprepared_resource_and_empty_manifest_uses_explicit_scope() {
    let root = fixture(false, true);
    assert!(
        cross_spec_review::accept(root.path(), root.path(), CANDIDATE)
            .unwrap_err()
            .issues
            .iter()
            .any(|issue| issue.code == "SHARED_CONTRACT_PLANNED_RESOURCE_MISSING")
    );
    let roadmap_path = root.path().join("steering/roadmap.md");
    let roadmap = fs::read_to_string(&roadmap_path)
        .unwrap()
        .replace("[translations]", "['*']");
    fs::write(roadmap_path, roadmap).unwrap();
    write(
        root.path(),
        "specs/shared-contract.yaml",
        "schema_version: 1\nresources: []\n",
    );
    cross_spec_review::accept(root.path(), root.path(), CANDIDATE).unwrap();
    assert_eq!(
        cross_spec_review::evaluate_freshness(root.path(), root.path()).status,
        ReviewFreshnessStatus::Fresh
    );
}
