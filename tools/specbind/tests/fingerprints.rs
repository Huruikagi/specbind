use specbind::{
    domain::{contract::Contract, tasks::Tasks},
    fingerprint::Fingerprint,
    schema::runtime,
};

fn tasks(input: &str) -> Tasks {
    runtime::load_tasks(input)
        .expect("task document is structurally valid")
        .try_into()
        .expect("task document is semantically valid")
}

fn contract(input: &str) -> Contract {
    runtime::load_contract(input)
        .expect("Contract is structurally valid")
        .try_into()
        .expect("Contract is semantically valid")
}

#[test]
fn contract_fingerprint_ignores_entry_path_and_yaml_order_but_keeps_descriptions() {
    let first = contract(
        "schema_version: 1\nowns: []\nexports:\n  - { id: zed, description: Zed. }\n  - { id: alpha, description: Alpha. }\nconsumes: []\ninvariants: []\nfile_ownership:\n  - { id: source, paths: [src/z, src/a] }\n",
    );
    let reordered = contract(
        "file_ownership:\n  - { paths: [src/a, src/z], id: source }\ninvariants: []\nconsumes: []\nexports:\n  - { description: Alpha., id: alpha }\n  - { description: Zed., id: zed }\nowns: []\nschema_version: 1\n",
    );
    let changed = contract(
        "schema_version: 1\nowns: []\nexports:\n  - { id: alpha, description: Changed. }\n  - { id: zed, description: Zed. }\nconsumes: []\ninvariants: []\nfile_ownership:\n  - { id: source, paths: [src/a, src/z] }\n",
    );
    let fingerprint = Fingerprint::contract(&first).expect("Contract canonicalizes");
    assert_eq!(
        fingerprint,
        Fingerprint::contract(&reordered).expect("Contract canonicalizes")
    );
    assert_ne!(
        fingerprint,
        Fingerprint::contract(&changed).expect("Contract canonicalizes")
    );
}

#[test]
fn markdown_normalizes_only_line_endings() {
    let lf = Fingerprint::markdown(b"alpha\nbeta\n");
    let crlf = Fingerprint::markdown(b"alpha\r\nbeta\r\n");
    let bare_cr = Fingerprint::markdown(b"alpha\rbeta\r");

    assert_eq!(lf, crlf);
    assert_eq!(lf, bare_cr);
    assert_ne!(lf, Fingerprint::markdown(b"alpha\nbeta"));
    assert_ne!(lf, Fingerprint::markdown(b"alpha \nbeta\n"));
}

#[test]
fn fingerprint_uses_tagged_lowercase_sha256() {
    assert_eq!(
        Fingerprint::markdown(b"").to_string(),
        "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn task_plan_ignores_set_order_and_execution_state() {
    let first = tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: First\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Second\n      requirement_ids: ['1.2']\n    - id: '3'\n      kind: task\n      title: Test\n      requirement_ids: ['2.1', '1.1']\n      boundaries: ['src/z', 'src/a']\n      contracts: ['exports/z', 'exports/a']\n      depends_on: ['2', '1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n",
    );
    let second = tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: First\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Second\n      requirement_ids: ['1.2']\n    - id: '3'\n      kind: task\n      title: Test\n      requirement_ids: ['1.1', '2.1']\n      boundaries: ['src/a', 'src/z']\n      contracts: ['exports/a', 'exports/z']\n      depends_on: ['1', '2']\n",
    );

    assert_eq!(
        Fingerprint::task_plan(&first).expect("plan canonicalizes"),
        Fingerprint::task_plan(&second).expect("plan canonicalizes")
    );
}

#[test]
fn task_plan_preserves_meaningful_sequence_and_field_presence() {
    let ordered = tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Test\n      details: ['first', 'second']\n      requirement_ids: ['1.1']\n",
    );
    let reversed = tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Test\n      details: ['second', 'first']\n      requirement_ids: ['1.1']\n",
    );
    let omitted = tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Test\n      requirement_ids: ['1.1']\n",
    );

    let ordered = Fingerprint::task_plan(&ordered).expect("plan canonicalizes");
    assert_ne!(
        ordered,
        Fingerprint::task_plan(&reversed).expect("plan canonicalizes")
    );
    assert_ne!(
        ordered,
        Fingerprint::task_plan(&omitted).expect("plan canonicalizes")
    );
    assert_eq!(
        Fingerprint::task_plan(&omitted)
            .expect("plan canonicalizes")
            .to_string(),
        "sha256:b177798b7a93b175931345331f52b1ef8c7be0bc6a3da53743f436cad8eb24dd"
    );
}
