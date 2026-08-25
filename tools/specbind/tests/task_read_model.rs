use specbind::{
    domain::tasks::Tasks,
    schema::runtime,
    task_read_model::{TaskPlanItemView, TaskReadModel, TaskStatus},
};

#[test]
fn derives_sparse_status_effective_dependencies_and_actionable_tasks() {
    let tasks = load_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: group\n      title: Build\n      tasks:\n        - id: '1.1'\n          kind: task\n          title: First\n          requirement_ids: ['1.1']\n        - id: '1.2'\n          kind: task\n          title: Second\n          requirement_ids: ['1.2']\n    - id: '2'\n      kind: task\n      title: Integrate\n      requirement_ids: ['1.3']\n    - id: '3'\n      kind: task\n      title: Document\n      requirement_ids: ['1.4']\n      boundaries: ['docs/']\n      depends_on: ['1.1']\nexecution:\n  tasks:\n    '1.1':\n      status: completed\n    '2':\n      status: blocked\n      blocked_reason: Waiting for an API decision\n",
    );

    let model = TaskReadModel::derive(&tasks);

    assert_eq!(model.total(), 4);
    assert_eq!((model.completed, model.pending, model.blocked), (1, 2, 1));
    assert_eq!(model.actionable_ids, ["1.2"]);
    let second = model.task("1.2").expect("second task");
    assert_eq!(second.status, TaskStatus::Pending);
    assert!(second.actionable);
    assert_eq!(second.effective_dependencies, ["1.1"]);
    let integrate = model.task("2").expect("integration task");
    assert_eq!(integrate.effective_dependencies, ["1.1", "1.2"]);
    assert_eq!(
        integrate.blocked_reason.as_deref(),
        Some("Waiting for an API decision")
    );
    let document = model.task("3").expect("documentation task");
    assert_eq!(document.explicit_dependencies, ["1.1"]);
    assert_eq!(document.effective_dependencies, ["1.1", "2"]);
    assert!(!document.actionable);
    let TaskPlanItemView::Group(group) = &model.items[0] else {
        panic!("first item should remain a group");
    };
    assert_eq!((group.completed, group.blocked), (1, 0));
}

fn load_tasks(input: &str) -> Tasks {
    runtime::load_tasks(input)
        .expect("schema-valid tasks")
        .try_into()
        .expect("semantically valid tasks")
}
