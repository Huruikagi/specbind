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
    assert_eq!(document.effective_dependencies, ["1.1", "1.2", "2"]);
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

#[test]
fn retained_completion_never_hides_pending_or_blocked_predecessors() {
    let plan = serde_json::json!({"schema_version":1,"plan":{"items":[
        {"id":"1","kind":"task","title":"New setup","requirement_ids":["1.1"]},
        {"id":"2","kind":"group","title":"Retained work","tasks":[
            {"id":"2.1","kind":"task","title":"Retained","requirement_ids":["1.1"]},
            {"id":"2.2","kind":"task","title":"Next child","requirement_ids":["1.1"]}
        ]},
        {"id":"3","kind":"task","title":"Next top","requirement_ids":["1.1"]}
    ]},"execution":{"tasks":{"2.1":{"status":"completed"},"2.2":{"status":"completed"}}}});
    for blocked in [false, true] {
        let mut document = plan.clone();
        if blocked {
            document["execution"]["tasks"]["1"] =
                serde_json::json!({"status":"blocked","blocked_reason":"Waiting"});
        }
        let model = TaskReadModel::derive(&load_tasks(&document.to_string()));
        assert!(!model.task("3").unwrap().actionable);
        assert_eq!(
            model.task("3").unwrap().effective_dependencies,
            ["1", "2.1", "2.2"]
        );
        assert_eq!(model.completed, 2);
        document["execution"]["tasks"]
            .as_object_mut()
            .unwrap()
            .remove("2.2");
        let model = TaskReadModel::derive(&load_tasks(&document.to_string()));
        assert!(!model.task("2.2").unwrap().actionable);
        document["execution"]["tasks"]["1"] = serde_json::json!({"status":"completed"});
        let model = TaskReadModel::derive(&load_tasks(&document.to_string()));
        assert_eq!(model.actionable_ids, ["2.2"]);
    }
}
