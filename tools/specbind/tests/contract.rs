use specbind::{
    contract::{Contract, ContractSection},
    schema::runtime,
};

const EMPTY: &str =
    "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n";

fn contract(input: &str) -> Result<Contract, specbind::domain::SemanticIssues> {
    runtime::load_contract(input)
        .expect("structurally valid Contract")
        .try_into()
}

#[test]
fn loads_the_strict_contract_profile() {
    let document = contract(
        "schema_version: 1\nowns:\n  - id: checkout-flow\n    description: Owns checkout orchestration.\nexports:\n  - id: checkout-result\n    description: Public result.\nconsumes:\n  - id: inventory\n    target:\n      spec: catalog\n      section: exports\n      id: stock-status\n    description: Reads availability.\ninvariants:\n  - id: no-double-charge\n    description: A payment is captured once.\nfile_ownership:\n  - id: checkout-api\n    paths: [src/api/checkout.rs, schemas/checkout/**]\n",
    )
    .expect("valid Contract");

    assert_eq!(document.owns[0].id, "checkout-flow");
    assert_eq!(document.exports[0].description, "Public result.");
    assert_eq!(document.consumes[0].target.canonical_spec, "catalog");
    assert_eq!(
        document.consumes[0].target.section,
        ContractSection::Exports
    );
    assert_eq!(
        document.file_ownership[0].paths,
        ["src/api/checkout.rs", "schemas/checkout/**"]
    );
}

#[test]
fn accepts_the_explicit_empty_contract() {
    let document = contract(EMPTY).expect("empty Contract");
    assert!(document.owns.is_empty());
    assert!(document.exports.is_empty());
    assert!(document.consumes.is_empty());
    assert!(document.invariants.is_empty());
    assert!(document.file_ownership.is_empty());
}

#[test]
fn rejects_unknown_fields_and_invalid_target_sections_structurally() {
    assert!(runtime::load_contract(&EMPTY.replace("owns: []", "notes: prose\nowns: []")).is_err());
    assert!(runtime::load_contract(&EMPTY.replace(
        "consumes: []",
        "consumes:\n  - id: dependency\n    target: { spec: catalog, section: consumes, id: value }"
    ))
    .is_err());
}

#[test]
fn rejects_duplicate_ids_empty_descriptions_and_invalid_paths_semantically() {
    let error = contract(
        "schema_version: 1\nowns:\n  - { id: owner, description: ' ' }\n  - { id: owner, description: Duplicate }\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership:\n  - id: source\n    paths: [/absolute/path, Src/API.rs, src/api.rs]\n",
    )
    .expect_err("invalid semantics");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert!(codes.contains(&"CONTRACT_ENTRY_ID_DUPLICATE"));
    assert!(codes.contains(&"CONTRACT_DESCRIPTION_EMPTY"));
    assert!(codes.contains(&"CONTRACT_FILE_OWNERSHIP_PATH_INVALID"));
    assert!(codes.contains(&"CONTRACT_FILE_OWNERSHIP_PATH_DUPLICATE"));
}
