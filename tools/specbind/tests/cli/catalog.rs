use super::*;

#[test]
fn reads_embedded_protocols_without_a_project() {
    let outside = tempfile::tempdir().expect("directory without a SpecBind project");

    let mut list = specbind_command();
    list.current_dir(outside.path())
        .args(["protocol", "list"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK PROTOCOL_LISTED: Found ").and(
                predicate::str::contains("selector=okf-authoring purpose=\""),
            ),
        )
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(outside.path())
        .args(["protocol", "read", "okf-authoring"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("# OKF authoring protocol\n")
                .and(predicate::str::contains("Open Knowledge Format v0.2"))
                .and(predicate::str::contains("OK PROTOCOL").not()),
        )
        .stderr("");

    let mut unknown = specbind_command();
    unknown
        .current_dir(outside.path())
        .args(["protocol", "read", "absent-protocol"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR PROTOCOL_SELECTOR_NOT_FOUND:")
                .and(predicate::str::contains("available selector okf-authoring")),
        );
}

#[test]
fn verifies_traceability_and_fails_closed_on_missing_coverage() {
    let root = project_fixture();
    write_status_fixture(root.path());

    let mut pass = specbind_command();
    pass.current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TRACEABILITY_VERIFIED: Verified traceability for spec checkout.\n  Requirements: 1\n  Active requirement IDs: 1\n  Design coverage: 1/1\n  Task coverage: 1/1 (required)\n",
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['9.9']\n",
    );
    let mut fail = specbind_command();
    fail.current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR TRACEABILITY_FAILED: Traceability for spec checkout has diagnostics.",
            )
            .and(predicate::str::contains(
                "TRACEABILITY_TASK_COVERAGE_MISSING",
            ))
            .and(predicate::str::contains(
                "TRACEABILITY_TASK_REQUIREMENT_UNKNOWN",
            )),
        );
}

#[test]
fn reports_an_idle_spec_without_active_coverage_ratios() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TRACEABILITY_VERIFIED: Verified traceability for spec checkout.\n  Requirements: 1\n  Active requirement IDs: none\n",
        )
        .stderr("");
}

#[test]
fn verifies_the_contract_graph_and_keeps_review_warnings_non_fatal() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        ".specbind/specs/provider/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `shared-tree` — `src/shared/**`\n",
    );
    write(
        root.path(),
        ".specbind/specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `value` → `provider/exports/value`\n\n## Invariants\n\n## File Ownership\n\n- `shared-tree` — `src/shared/**`\n",
    );

    let mut warned = specbind_command();
    warned
        .current_dir(root.path())
        .args(["check", "contracts"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK CONTRACTS_VERIFIED: Verified 3 contract(s) and 1 dependency reference(s).\n",
            )
            .and(predicate::str::contains("\n  Dependency cycles: 0\n"))
            .and(predicate::str::contains("\n  Warnings:\n")),
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `missing` → `provider/exports/missing`\n\n## Invariants\n\n## File Ownership\n",
    );
    let mut failed = specbind_command();
    failed
        .current_dir(root.path())
        .args(["check", "contracts"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR CONTRACTS_FAILED: Contract graph has structural diagnostics.",
            )
            .and(predicate::str::contains(
                "CONTRACT_GRAPH_TARGET_ENTRY_MISSING",
            )),
        );
}

#[test]
fn lists_and_reads_project_owned_spec_templates() {
    let root = project_fixture();
    write_template_fixture(root.path());
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );

    let mut list = specbind_command();
    list.current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK TEMPLATE_LISTED: Found 7 recognized spec template(s).\n")
                .and(predicate::str::contains(
                    "selector=brief source=project type=\"SpecBind Brief\" template_path=settings/templates/specs/brief.md output_path=brief.md\n",
                ))
                .and(predicate::str::contains(
                    "selector=design/main source=project type=\"SpecBind Design\" artifact_id=main template_path=settings/templates/specs/technical-design/main.md output_path=technical-design/main.md\n",
                ))
                .and(predicate::str::contains(
                    "selector=requirements source=embedded type=\"SpecBind Requirements\"",
                ))
                .and(predicate::str::contains(
                    "selector=implementation-notes/main source=embedded",
                )),
        )
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(root.path())
        .args(["template", "read", "spec", "design/main"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "<!-- specbind:instruction maintain Describe one owned decision. -->",
        ))
        .stderr("");

    let mut resolve = specbind_command();
    resolve
        .current_dir(root.path())
        .args(["template", "resolve", "spec", "checkout", "design/main"])
        .assert()
        .success()
        .stdout(concat!(
            "OK TEMPLATE_RESOLVED: Resolved template design/main for spec checkout.\n",
            "  Selector: design/main\n",
            "  Source: project\n",
            "  Type: SpecBind Design\n",
            "  Artifact ID: main\n",
            "  Template path: settings/templates/specs/technical-design/main.md\n",
            "  Output path: technical-design/main.md\n",
            "  Project path: .specbind/specs/checkout/technical-design/main.md\n",
        ))
        .stderr("");

    let embedded_root = project_fixture();
    write(
        embedded_root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    let mut embedded = specbind_command();
    embedded
        .current_dir(embedded_root.path())
        .args(["template", "resolve", "spec", "checkout", "contract"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Source: embedded\n").and(predicate::str::contains(
                "  Project path: .specbind/specs/checkout/contract.md\n",
            )),
        )
        .stderr("");
}

#[test]
fn lists_and_reads_the_project_owned_milestone_template() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/templates/roadmap.md",
        "---\ntype: SpecBind Roadmap\n---\n# Project Roadmap\n\n## Change request\n",
    );

    let mut list = specbind_command();
    list.current_dir(root.path())
        .args(["template", "list", "milestone"])
        .assert()
        .success()
        .stdout(concat!(
            "OK TEMPLATE_LISTED: Found 1 recognized milestone template(s).\n",
            "  selector=roadmap source=project type=\"SpecBind Roadmap\" template_path=settings/templates/roadmap.md body_target=steering/roadmap.md\n",
        ))
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(root.path())
        .args(["template", "read", "milestone", "roadmap"])
        .assert()
        .success()
        .stdout("---\ntype: SpecBind Roadmap\n---\n# Project Roadmap\n\n## Change request\n")
        .stderr("");
}

#[test]
fn lists_and_reads_the_steering_template_scope() {
    let root = project_fixture();

    let mut list = specbind_command();
    list.current_dir(root.path())
        .args(["template", "list", "steering"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK TEMPLATE_LISTED: Found 4 recognized steering template(s).\n",
            )
            .and(predicate::str::contains(
                "selector=product source=embedded type=\"SpecBind Steering\" artifact_id=product template_path=en/steering/product.md output_path=steering/product.md\n",
            ))
            .and(predicate::str::contains(
                "selector=document source=embedded type=\"SpecBind Steering\" template_path=en/steering/document.md output_path=<authored>\n",
            )),
        )
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(root.path())
        .args(["template", "read", "steering", "document"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("type: SpecBind Steering")
                .and(predicate::str::contains("artifact_id:").not())
                .and(predicate::str::contains("specbind:instruction")),
        )
        .stderr("");

    let mut missing = specbind_command();
    missing
        .current_dir(root.path())
        .args(["template", "read", "steering", "product-overview"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "ERROR TEMPLATE_SELECTOR_NOT_FOUND",
        ));
}

#[test]
fn falls_back_to_embedded_defaults_in_the_configured_language() {
    let root = project_fixture();

    let mut english = specbind_command();
    english
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK TEMPLATE_LISTED: Found 7 recognized spec template(s).\n",
            )
            .and(predicate::str::contains("template_path=en/specs/brief.md"))
            .and(predicate::str::contains("source=project").not()),
        )
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(root.path())
        .args(["template", "read", "spec", "requirements"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("type: SpecBind Requirements")
                .and(predicate::str::contains("requirement: Requirement"))
                .and(predicate::str::contains("### Requirement 1:").not())
                .and(predicate::str::contains(
                    "deliberately not a valid live Requirements artifact",
                ))
                .and(predicate::str::contains("specbind:instruction")),
        );

    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"ja","agents":["codex"]}"#,
    );
    let mut japanese = specbind_command();
    japanese
        .current_dir(root.path())
        .args(["template", "read", "spec", "requirements"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("requirement: 要件")
                .and(predicate::str::contains("### 要件 1:").not())
                .and(predicate::str::contains(
                    "意図的に有効なlive Requirementsではない",
                )),
        );
}

#[test]
fn rejects_unknown_template_selectors_and_invalid_template_profiles() {
    let root = project_fixture();
    write_template_fixture(root.path());

    let mut missing = specbind_command();
    missing
        .current_dir(root.path())
        .args(["template", "read", "spec", "design/absent"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR TEMPLATE_SELECTOR_NOT_FOUND:",
        ));

    write(
        root.path(),
        ".specbind/settings/templates/specs/design-live.md",
        "---\ntype: SpecBind Design\nartifact_id: live\nrequirement_ids: ['1.1']\n---\n# Design\n",
    );
    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR TEMPLATE_LIST_FAILED:").and(
                predicate::str::contains("TEMPLATE_DESIGN_REQUIREMENT_IDS_FORBIDDEN"),
            ),
        );
}

#[test]
fn reports_an_unreadable_template_root_without_falling_back_silently() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/templates/specs",
        "not a directory\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR TEMPLATE_LIST_FAILED:")
                .and(predicate::str::contains("TEMPLATE_ROOT_NOT_DIRECTORY")),
        );
}
