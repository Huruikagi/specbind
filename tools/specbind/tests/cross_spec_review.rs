use std::fs;
use std::path::Path;
use std::process::Command;

use specbind::{
    cross_spec_review::{self, ReviewBoundary, ReviewFreshnessStatus},
    fingerprint::Fingerprint,
};
use tempfile::TempDir;

const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture");
}

fn fixture() -> TempDir {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  new_specs:\n    - spec: provider\n      summary: Add provider\n  spec_updates:\n    - spec: consumer\n      summary: Update consumer\n      depends_on:\n        - spec: provider\n---\n# Roadmap\n"
        ),
    );
    write(
        root.path(),
        "specs/provider/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `value` → `provider/exports/value`\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/consumer/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Consume value\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root.path(),
        "specs/consumer/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n",
    );
    root
}

fn git(root: &Path, arguments: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .expect("start Git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git output")
        .trim()
        .to_owned()
}

fn acceptance_fixture() -> TempDir {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init", "--quiet"]);
    git(
        root.path(),
        &["config", "user.email", "specbind@example.com"],
    );
    git(root.path(), &["config", "user.name", "SpecBind Tests"]);
    write(root.path(), "baseline.txt", "baseline\n");
    git(root.path(), &["add", "baseline.txt"]);
    git(root.path(), &["commit", "--quiet", "-m", "baseline"]);
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);

    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    let requirements = "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n";
    let contract = "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n";
    let design = "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n";
    write(root.path(), "specs/checkout/requirements.md", requirements);
    write(root.path(), "specs/checkout/contract.md", contract);
    write(root.path(), "specs/checkout/design.md", design);
    write(
        root.path(),
        "specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {MILESTONE}\n  state: tasks\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        contract: {}\n        design/main: {}\n",
            Fingerprint::markdown(requirements.as_bytes()),
            Fingerprint::markdown(contract.as_bytes()),
            Fingerprint::markdown(design.as_bytes()),
        ),
    );
    root
}

#[test]
fn resolves_authoritative_contract_first_input_revisions() {
    let root = fixture();
    let candidate = r##"{"schemaVersion":1,"assessment":"# Assessment\n\nCompatible.","deepInputs":["specs/consumer#requirements","specs/consumer#design/main"]}"##;

    let resolution = cross_spec_review::resolve_inputs(root.path(), candidate)
        .expect("valid review input resolution");

    assert_eq!(resolution.roadmap.milestone_id, MILESTONE);
    assert_eq!(resolution.graph.report.dependencies.len(), 1);
    assert_eq!(
        resolution
            .input_revisions
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        [
            "specs/consumer#contract",
            "specs/consumer#design/main",
            "specs/consumer#requirements",
            "specs/provider#contract",
            "steering/roadmap.md#cross-spec-scope",
        ]
    );
    let requirements =
        fs::read(root.path().join("specs/consumer/requirements.md")).expect("requirements bytes");
    assert_eq!(
        resolution.input_revisions["specs/consumer#requirements"],
        Fingerprint::markdown(&requirements)
    );
}

#[test]
fn rejects_invalid_and_duplicate_deep_inputs() {
    let root = fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":["specs/consumer#contract","specs/consumer#requirements","specs/consumer#requirements"]}"#;

    let error =
        cross_spec_review::resolve_inputs(root.path(), candidate).expect_err("invalid deep inputs");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"CROSS_SPEC_REVIEW_DEEP_INPUT_INVALID"));
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_DEEP_INPUT_DUPLICATE"));
}

#[test]
fn rejects_invalid_candidates_and_direct_only_roadmaps() {
    let root = fixture();
    let invalid = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":2,"assessment":" ","deepInputs":[]}"#,
    )
    .expect_err("invalid candidate");
    assert!(
        invalid
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_CANDIDATE_VERSION_UNSUPPORTED")
    );

    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n"
        ),
    );
    let direct = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#,
    )
    .expect_err("Direct-only milestone");
    assert!(
        direct
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_DIRECT_ONLY")
    );
}

#[test]
fn blocks_input_resolution_on_contract_graph_errors() {
    let root = fixture();
    write(
        root.path(),
        "specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `missing` → `provider/exports/missing`\n\n## Invariants\n\n## File Ownership\n",
    );

    let error = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#,
    )
    .expect_err("dangling Contract target");
    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "CONTRACT_GRAPH_TARGET_ENTRY_MISSING")
    );
}

#[test]
fn atomically_accepts_and_replaces_a_guarded_review() {
    let root = acceptance_fixture();
    let first =
        r##"{"schemaVersion":1,"assessment":"# Assessment\n\nCompatible.","deepInputs":[]}"##;
    let accepted =
        cross_spec_review::accept(root.path(), root.path(), first).expect("accepted review");
    assert_eq!(accepted.path, "state/cross-spec-review.md");
    let path = root.path().join(&accepted.path);
    let content = fs::read_to_string(&path).expect("accepted review content");
    assert!(content.contains("type: SpecBind Cross-Spec Review"));
    assert!(content.contains("steering/roadmap.md#cross-spec-scope"));
    assert!(content.ends_with("Compatible.\n"));

    let second = r#"{"schemaVersion":1,"assessment":"Replacement assessment.","deepInputs":[]}"#;
    cross_spec_review::accept(root.path(), root.path(), second).expect("replacement review");
    let replaced = fs::read_to_string(path).expect("replaced review content");
    assert!(replaced.ends_with("Replacement assessment.\n"));
    assert!(!replaced.contains("Compatible."));
}

#[test]
fn rejects_acceptance_when_tasks_exist_or_design_is_stale() {
    let root = acceptance_fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#;
    write(root.path(), "specs/checkout/tasks.yaml", "invalid\n");
    let tasks = cross_spec_review::accept(root.path(), root.path(), candidate)
        .expect_err("tasks must not exist");
    assert!(
        tasks
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_TASKS_ALREADY_EXIST")
    );
    fs::remove_file(root.path().join("specs/checkout/tasks.yaml")).expect("remove fixture task");
    write(
        root.path(),
        "specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Changed Design\n\n_Requirements: 1.1_\n",
    );
    let stale = cross_spec_review::accept(root.path(), root.path(), candidate)
        .expect_err("Design must be fresh");
    assert!(
        stale
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_DESIGN_NOT_FRESH")
    );
}

#[test]
fn reports_fresh_and_then_stale_authoritative_inputs() {
    let root = acceptance_fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#;
    cross_spec_review::accept(root.path(), root.path(), candidate).expect("accepted review");

    let fresh = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(fresh.status, ReviewFreshnessStatus::Fresh);
    assert!(fresh.issues.is_empty());
    assert!(fresh.accepted.is_some());
    assert!(fresh.current_input_revisions.is_some());

    write(
        root.path(),
        "specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n",
    );
    let stale = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(stale.status, ReviewFreshnessStatus::Stale);
    assert!(
        stale
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_INPUTS_STALE"),
        "{:?}",
        stale.issues
    );
}

#[test]
fn reconstructs_and_checks_persisted_deep_inputs() {
    let root = acceptance_fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed deeply.","deepInputs":["specs/checkout#requirements","specs/checkout#design/main"]}"#;
    cross_spec_review::accept(root.path(), root.path(), candidate).expect("accepted deep review");

    let accepted = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(accepted.status, ReviewFreshnessStatus::Fresh);
    write(
        root.path(),
        "specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Changed Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );

    let stale = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(stale.status, ReviewFreshnessStatus::Stale);
    assert!(
        stale
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_INPUTS_STALE")
    );
}

#[test]
fn distinguishes_not_required_missing_and_unexpected_review() {
    let root = fixture();
    let missing = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(missing.status, ReviewFreshnessStatus::Missing);

    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n"
        ),
    );
    let not_required = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(not_required.status, ReviewFreshnessStatus::NotRequired);

    write(
        root.path(),
        "state/cross-spec-review.md",
        "leftover review\n",
    );
    let unexpected = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(unexpected.status, ReviewFreshnessStatus::Invalid);
    assert!(
        unexpected
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_UNEXPECTED_FOR_DIRECT_ONLY")
    );
}

#[test]
fn reports_malformed_accepted_review_as_invalid() {
    let root = fixture();
    write(
        root.path(),
        "state/cross-spec-review.md",
        &format!(
            "---\ntype: Wrong Type\nmilestone_id: {MILESTONE}\npassed_at: yesterday\ninput_revisions:\n  steering/roadmap.md#cross-spec-scope: SHA256:BAD\nunknown: true\n---\n"
        ),
    );

    let invalid = cross_spec_review::evaluate_freshness(root.path(), root.path());
    assert_eq!(invalid.status, ReviewFreshnessStatus::Invalid);
    assert!(
        invalid
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_FRONTMATTER_INVALID")
    );

    write(
        root.path(),
        "state/cross-spec-review.md",
        &format!(
            "---\ntype: Wrong Type\nmilestone_id: {MILESTONE}\npassed_at: yesterday\ninput_revisions:\n  steering/roadmap.md#cross-spec-scope: SHA256:BAD\n---\n"
        ),
    );
    let invalid_values = cross_spec_review::evaluate_freshness(root.path(), root.path());
    let codes = invalid_values
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert_eq!(invalid_values.status, ReviewFreshnessStatus::Invalid);
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_TYPE_INVALID"));
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_PASSED_AT_INVALID"));
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_FINGERPRINT_INVALID"));
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_ASSESSMENT_EMPTY"));
}

#[test]
fn enforces_fresh_review_for_spec_local_later_boundaries() {
    let root = acceptance_fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#;
    cross_spec_review::accept(root.path(), root.path(), candidate).expect("accepted review");

    for boundary in [
        ReviewBoundary::TasksApproval {
            canonical_spec: "checkout",
        },
        ReviewBoundary::ImplementationValidation {
            canonical_spec: "checkout",
        },
    ] {
        let report = cross_spec_review::require_for_boundary(root.path(), root.path(), boundary)
            .expect("fresh participating review");
        assert_eq!(report.status, ReviewFreshnessStatus::Fresh);
    }
    let release = cross_spec_review::require_for_boundary(
        root.path(),
        root.path(),
        ReviewBoundary::ReleasePreflight,
    )
    .expect("Spec-backed release requires and accepts the fresh review");
    assert_eq!(release.status, ReviewFreshnessStatus::Fresh);

    let unrelated = cross_spec_review::require_for_boundary(
        root.path(),
        root.path(),
        ReviewBoundary::TasksApproval {
            canonical_spec: "unrelated",
        },
    )
    .expect_err("non-participant must be rejected");
    assert!(
        unrelated
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_SPEC_NOT_IN_MILESTONE")
    );

    write(
        root.path(),
        "specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n",
    );
    let stale = cross_spec_review::require_for_boundary(
        root.path(),
        root.path(),
        ReviewBoundary::ImplementationValidation {
            canonical_spec: "checkout",
        },
    )
    .expect_err("stale review must block validation");
    assert!(
        stale
            .issues
            .iter()
            .any(|issue| { issue.code == "CROSS_SPEC_REVIEW_IMPLEMENTATION_VALIDATION_BLOCKED" })
    );
}

#[test]
fn release_review_guard_accepts_direct_only_without_review() {
    let root = fixture();
    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n"
        ),
    );

    let report = cross_spec_review::require_for_boundary(
        root.path(),
        root.path(),
        ReviewBoundary::ReleasePreflight,
    )
    .expect("Direct-only release does not require a review");
    assert_eq!(report.status, ReviewFreshnessStatus::NotRequired);

    let invalid_target = cross_spec_review::require_for_boundary(
        root.path(),
        root.path(),
        ReviewBoundary::TasksApproval {
            canonical_spec: "Not-Canonical",
        },
    )
    .expect_err("Spec target must be canonical");
    assert!(
        invalid_target
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_SPEC_TARGET_INVALID")
    );
}
