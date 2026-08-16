use specbind::contract::{self, ContractSection};

const EMPTY: &str =
    "# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n";

#[test]
fn parses_the_canonical_contract_profile() {
    let document = contract::parse(
        "# Contract\n\n## Owns\n\n- `checkout-flow` — Owns checkout orchestration.\n\n## Exports\n\n- `checkout-result` — Public result.\n\n## Consumes\n\n- `inventory` → `catalog/exports/stock-status` — Reads availability.\n\n## Invariants\n\n- `no-double-charge` — A payment is captured once.\n\n## File Ownership\n\n- `checkout-api` — `src/api/checkout.rs`, `schemas/checkout/**`\n",
    )
    .expect("valid Contract");

    assert_eq!(document.owns[0].id, "checkout-flow");
    assert_eq!(document.exports[0].description, "Public result.");
    assert_eq!(document.consumes[0].id, "inventory");
    assert_eq!(document.consumes[0].target.canonical_spec, "catalog");
    assert_eq!(
        document.consumes[0].target.section,
        ContractSection::Exports
    );
    assert_eq!(document.consumes[0].target.entry_id, "stock-status");
    assert_eq!(
        document.file_ownership[0].paths,
        ["src/api/checkout.rs", "schemas/checkout/**"]
    );
}

#[test]
fn accepts_the_explicit_empty_contract() {
    let document = contract::parse(EMPTY).expect("empty Contract");

    assert!(document.owns.is_empty());
    assert!(document.exports.is_empty());
    assert!(document.consumes.is_empty());
    assert!(document.invariants.is_empty());
    assert!(document.file_ownership.is_empty());
}

#[test]
fn rejects_missing_reordered_and_non_list_sections() {
    let error = contract::parse(
        "# Wrong\n\n## Exports\n\nNothing here.\n\n## Owns\n\n1. `owned` — Description\n",
    )
    .expect_err("invalid structure");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"CONTRACT_ROOT_HEADING_INVALID"));
    assert!(codes.contains(&"CONTRACT_SECTION_HEADING_INVALID"));
    assert!(codes.contains(&"CONTRACT_SECTION_CONTENT_INVALID"));
    assert!(codes.contains(&"CONTRACT_SECTION_LIST_ORDERED"));
    assert!(codes.contains(&"CONTRACT_SECTION_HEADING_MISSING"));
}

#[test]
fn rejects_invalid_entry_ids_targets_and_paths() {
    let error = contract::parse(
        "# Contract\n\n## Owns\n\n- `Bad_ID` — Description\n- `same-id` — First\n- `same-id` — Second\n\n## Exports\n\n## Consumes\n\n- `dependency` → `catalog/consumes/value`\n\n## Invariants\n\n## File Ownership\n\n- `source` — `/absolute/path`, `src/**/nested`\n",
    )
    .expect_err("invalid entries");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"CONTRACT_DESCRIBED_ENTRY_INVALID"));
    assert!(codes.contains(&"CONTRACT_ENTRY_ID_DUPLICATE"));
    assert!(codes.contains(&"CONTRACT_CONSUMES_ENTRY_INVALID"));
    assert!(codes.contains(&"CONTRACT_FILE_OWNERSHIP_ENTRY_INVALID"));
}

#[test]
fn rejects_nested_lists_and_duplicate_case_insensitive_paths() {
    let error = contract::parse(
        "# Contract\n\n## Owns\n\n- `owner` — Description\n  - nested\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `source` — `Src/API.rs`, `src/api.rs`\n",
    )
    .expect_err("invalid flat-list and path set");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"CONTRACT_ENTRY_STRUCTURE_INVALID"));
    assert!(codes.contains(&"CONTRACT_FILE_OWNERSHIP_ENTRY_INVALID"));
}
