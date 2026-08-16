//! Derived current-milestone status over Roadmap, Spec, review, task, and Git state.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{
    artifacts::{self, DiscoveryIssue},
    cross_spec_review::{self, ReviewFreshnessStatus},
    freshness::{self, FreshnessStatus},
    repository,
    roadmap::{self, Dependency, DirectStatus, RoadmapDocument},
    schema::spec::v1::WorkflowState,
    spec_status::{self, ConsistencyHealth, SpecStatusModel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryStage {
    Requirements,
    Design,
    CrossSpecReview,
    Tasks,
    Implementation,
    Validation,
    ReleasePending,
    ReleaseReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneHealth {
    Consistent,
    Inconsistent,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MilestoneDiagnostic {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneAction {
    pub item: String,
    pub action: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneItemView {
    pub id: String,
    pub summary: String,
    pub status: String,
    pub waiting_for: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneStatusModel {
    pub milestone_id: String,
    pub target_release: Option<String>,
    pub stage: DeliveryStage,
    pub health: MilestoneHealth,
    pub review_status: ReviewFreshnessStatus,
    pub spec_state_counts: BTreeMap<String, usize>,
    pub direct_completed: usize,
    pub direct_total: usize,
    pub current_revision: Option<String>,
    pub items: Vec<MilestoneItemView>,
    pub actionable: Vec<MilestoneAction>,
    pub release_blockers: Vec<String>,
    pub diagnostics: Vec<MilestoneDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneStatusFailure {
    pub diagnostics: Vec<MilestoneDiagnostic>,
}

struct ItemFacts {
    id: String,
    summary: String,
    dependencies: Vec<String>,
    kind: ItemKind,
}

enum ItemKind {
    Spec { model: Option<Box<SpecStatusModel>> },
    Direct { completed: bool },
}

struct GitState {
    revision: Option<String>,
    clean: bool,
    diagnostic: Option<MilestoneDiagnostic>,
}

/// Resolves the one active milestone. Absence is a normal no-change result.
///
/// # Errors
///
/// Returns a fatal failure when the active Roadmap exists but no trustworthy
/// typed scope can be read.
pub fn resolve(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<Option<MilestoneStatusModel>, MilestoneStatusFailure> {
    let Some(roadmap) = read_roadmap(specbind_root)? else {
        return Ok(None);
    };
    let git = git_state(project_root);
    let review = cross_spec_review::evaluate_freshness(project_root, specbind_root);
    let mut diagnostics = BTreeSet::new();
    if let Some(diagnostic) = git.diagnostic.clone() {
        diagnostics.insert(diagnostic);
    }
    diagnostics.extend(review.issues.iter().map(|issue| MilestoneDiagnostic {
        code: issue.code,
        path: issue.source.clone(),
        message: issue.message.clone(),
    }));
    let mut facts = spec_facts(project_root, specbind_root, &roadmap, &mut diagnostics);
    facts.extend(direct_facts(&roadmap));
    diagnose_unscoped_active_specs(specbind_root, &roadmap, &mut diagnostics);
    if matches!(review.status, ReviewFreshnessStatus::Missing)
        && facts.iter().any(|item| match &item.kind {
            ItemKind::Spec { model, .. } => model
                .as_ref()
                .is_some_and(|model| model.task_model.is_some()),
            ItemKind::Direct { .. } => false,
        })
    {
        diagnostics.insert(MilestoneDiagnostic {
            code: "MILESTONE_TASKS_BEFORE_REVIEW",
            path: None,
            message: "current tasks.yaml exists before the required cross-spec review is accepted"
                .to_owned(),
        });
    }
    let validation_checkout_ready = git.clean
        || git.revision.as_deref().is_some_and(|revision| {
            freshness::assess_pending_completion_mutations(project_root, specbind_root, revision)
                .issues
                .is_empty()
        });
    let implementation_complete = implementation_completion(&facts, validation_checkout_ready);
    let all_items_implemented = implementation_complete.values().all(|complete| *complete);
    let all_specs_validated = spec_predicate(&facts, validated);
    let stage = derive_stage(&facts, review.status, &implementation_complete);
    let health = if diagnostics.is_empty() {
        MilestoneHealth::Consistent
    } else {
        MilestoneHealth::Inconsistent
    };
    let actionable = actionable_items(
        &facts,
        review.status,
        &implementation_complete,
        all_items_implemented,
        validation_checkout_ready,
        roadmap.target_release.is_some(),
    );
    let items = item_views(&facts, &implementation_complete);
    let release_blockers = release_blockers(
        &facts,
        &roadmap,
        review.status,
        git.clean,
        health,
        all_specs_validated,
    );
    let (stage, release_blockers) =
        derive_release_readiness(project_root, specbind_root, stage, release_blockers);
    let spec_state_counts = spec_state_counts(&facts);
    let direct_total = roadmap.direct_changes.len();
    let direct_completed = roadmap
        .direct_changes
        .iter()
        .filter(|item| item.status == Some(DirectStatus::Completed))
        .count();

    Ok(Some(MilestoneStatusModel {
        milestone_id: roadmap.milestone_id,
        target_release: roadmap.target_release,
        stage,
        health,
        review_status: review.status,
        spec_state_counts,
        direct_completed,
        direct_total,
        current_revision: validation_checkout_ready.then_some(git.revision).flatten(),
        items,
        actionable,
        release_blockers,
        diagnostics: diagnostics.into_iter().collect(),
    }))
}

fn read_roadmap(specbind_root: &Path) -> Result<Option<RoadmapDocument>, MilestoneStatusFailure> {
    let path = specbind_root.join("steering/roadmap.md");
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(failure("MILESTONE_ROADMAP_READ_FAILED", error.to_string())),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(failure(
            "MILESTONE_ROADMAP_NOT_REGULAR",
            "steering/roadmap.md must be a regular non-symlink file",
        ));
    }
    let content = fs::read_to_string(path)
        .map_err(|error| failure("MILESTONE_ROADMAP_READ_FAILED", error.to_string()))?;
    roadmap::parse(&content)
        .map(Some)
        .map_err(|error| MilestoneStatusFailure {
            diagnostics: error
                .issues
                .into_iter()
                .map(|issue| MilestoneDiagnostic {
                    code: issue.code,
                    path: Some("steering/roadmap.md".to_owned()),
                    message: issue.message,
                })
                .collect(),
        })
}

fn spec_facts(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) -> Vec<ItemFacts> {
    roadmap
        .new_specs
        .iter()
        .chain(&roadmap.spec_updates)
        .map(|item| {
            let id = format!("spec:{}", item.spec);
            let model = match spec_status::resolve(project_root, specbind_root, &item.spec) {
                Ok(model) => {
                    if model.milestone_id.as_deref() != Some(&roadmap.milestone_id) {
                        diagnostics.insert(MilestoneDiagnostic {
                            code: "MILESTONE_SPEC_IDENTITY_MISMATCH",
                            path: Some(format!("specs/{}/spec.yaml", item.spec)),
                            message: format!(
                                "{} active milestone does not match the Roadmap",
                                item.spec
                            ),
                        });
                    }
                    if model.health == ConsistencyHealth::Inconsistent {
                        diagnostics.insert(MilestoneDiagnostic {
                            code: "MILESTONE_SPEC_INCONSISTENT",
                            path: Some(format!("specs/{}/spec.yaml", item.spec)),
                            message: format!("{} has inconsistent status diagnostics", item.spec),
                        });
                    }
                    Some(model)
                }
                Err(error) => {
                    diagnostics.extend(error.issues.iter().map(from_discovery));
                    None
                }
            };
            ItemFacts {
                id,
                summary: item.summary.clone(),
                dependencies: item.depends_on.iter().map(dependency_key).collect(),
                kind: ItemKind::Spec {
                    model: model.map(Box::new),
                },
            }
        })
        .collect()
}

fn direct_facts(roadmap: &RoadmapDocument) -> Vec<ItemFacts> {
    roadmap
        .direct_changes
        .iter()
        .map(|item| ItemFacts {
            id: format!("direct:{}", item.id),
            summary: item.summary.clone(),
            dependencies: item.depends_on.iter().map(dependency_key).collect(),
            kind: ItemKind::Direct {
                completed: item.status == Some(DirectStatus::Completed),
            },
        })
        .collect()
}

fn requirements_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::Design))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
}

fn design_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::Tasks))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
        && model.freshness.design.status == FreshnessStatus::Fresh
}

fn tasks_approved(model: &SpecStatusModel) -> bool {
    state_rank(model.declared_state) >= state_rank(Some(WorkflowState::Implementation))
        && model.freshness.requirements.status == FreshnessStatus::Fresh
        && model.freshness.design.status == FreshnessStatus::Fresh
        && model.freshness.tasks.status == FreshnessStatus::Fresh
}

fn tasks_complete(model: &SpecStatusModel) -> bool {
    model.task_model.as_ref().is_some_and(|tasks| {
        tasks.completed == tasks.total() && tasks.pending == 0 && tasks.blocked == 0
    })
}

fn validated(model: &SpecStatusModel) -> bool {
    model.declared_state == Some(WorkflowState::ReleaseReady)
        && model.freshness.completion.status == FreshnessStatus::Fresh
}

fn implementation_completion(facts: &[ItemFacts], clean: bool) -> BTreeMap<String, bool> {
    facts
        .iter()
        .map(|item| {
            let complete = match &item.kind {
                ItemKind::Spec { model } => model.as_deref().is_some_and(tasks_complete) && clean,
                ItemKind::Direct { completed } => *completed,
            };
            (item.id.clone(), complete)
        })
        .collect()
}

fn derive_stage(
    facts: &[ItemFacts],
    review: ReviewFreshnessStatus,
    completion: &BTreeMap<String, bool>,
) -> DeliveryStage {
    let has_specs = has_specs(facts);
    if has_specs && !spec_predicate(facts, requirements_approved) {
        DeliveryStage::Requirements
    } else if has_specs && !spec_predicate(facts, design_approved) {
        DeliveryStage::Design
    } else if has_specs && review != ReviewFreshnessStatus::Fresh {
        DeliveryStage::CrossSpecReview
    } else if has_specs && !spec_predicate(facts, tasks_approved) {
        DeliveryStage::Tasks
    } else if !completion.values().all(|complete| *complete) {
        DeliveryStage::Implementation
    } else if has_specs && !spec_predicate(facts, validated) {
        DeliveryStage::Validation
    } else {
        DeliveryStage::ReleasePending
    }
}

fn actionable_items(
    facts: &[ItemFacts],
    review: ReviewFreshnessStatus,
    completion: &BTreeMap<String, bool>,
    all_implemented: bool,
    clean: bool,
    release_bound: bool,
) -> Vec<MilestoneAction> {
    let mut actions = Vec::new();
    for item in facts {
        match &item.kind {
            ItemKind::Spec { model } if !model.as_deref().is_some_and(requirements_approved) => {
                push_action(&mut actions, item, "requirements");
            }
            ItemKind::Spec { model }
                if !model.as_deref().is_some_and(design_approved)
                    && design_dependencies_ready(item, facts) =>
            {
                push_action(&mut actions, item, "design");
            }
            ItemKind::Spec { model }
                if review == ReviewFreshnessStatus::Fresh
                    && !model.as_deref().is_some_and(tasks_approved) =>
            {
                push_action(&mut actions, item, "tasks");
            }
            ItemKind::Spec { model }
                if model.as_deref().is_some_and(tasks_approved)
                    && !completion[&item.id]
                    && dependencies_ready(item, completion) =>
            {
                push_action(&mut actions, item, "implementation");
            }
            ItemKind::Spec { model }
                if all_implemented && clean && !model.as_deref().is_some_and(validated) =>
            {
                push_action(&mut actions, item, "validation");
            }
            ItemKind::Direct { completed }
                if !completed && dependencies_ready(item, completion) =>
            {
                push_action(&mut actions, item, "implementation");
            }
            _ => {}
        }
    }
    if has_specs(facts)
        && facts.iter().all(|item| match &item.kind {
            ItemKind::Spec { model } => model.as_deref().is_some_and(design_approved),
            ItemKind::Direct { .. } => true,
        })
        && review != ReviewFreshnessStatus::Fresh
    {
        actions.push(MilestoneAction {
            item: "milestone".to_owned(),
            action: "cross_spec_review",
        });
    }
    if all_implemented
        && facts.iter().all(|item| match &item.kind {
            ItemKind::Spec { model } => model.as_deref().is_some_and(validated),
            ItemKind::Direct { completed } => *completed,
        })
    {
        actions.push(MilestoneAction {
            item: "milestone".to_owned(),
            action: if release_bound {
                "release_preflight"
            } else {
                "bind_release"
            },
        });
    }
    actions
}

fn push_action(actions: &mut Vec<MilestoneAction>, item: &ItemFacts, action: &'static str) {
    actions.push(MilestoneAction {
        item: item.id.clone(),
        action,
    });
}

fn item_views(facts: &[ItemFacts], completion: &BTreeMap<String, bool>) -> Vec<MilestoneItemView> {
    facts
        .iter()
        .map(|item| {
            let status = match &item.kind {
                ItemKind::Spec { model } if model.as_deref().is_some_and(validated) => {
                    "validated".to_owned()
                }
                ItemKind::Spec { model } => model.as_deref().map_or_else(
                    || "unavailable".to_owned(),
                    |model| spec_status::state_name(model.declared_state).to_owned(),
                ),
                ItemKind::Direct { completed: true } => "completed".to_owned(),
                ItemKind::Direct { completed: false } => "pending".to_owned(),
            };
            let waiting_for = item
                .dependencies
                .iter()
                .filter(|dependency| !completion.get(*dependency).copied().unwrap_or(false))
                .cloned()
                .collect();
            MilestoneItemView {
                id: item.id.clone(),
                summary: item.summary.clone(),
                status,
                waiting_for,
            }
        })
        .collect()
}

fn release_blockers(
    facts: &[ItemFacts],
    roadmap: &RoadmapDocument,
    review: ReviewFreshnessStatus,
    clean: bool,
    health: MilestoneHealth,
    all_specs_validated: bool,
) -> Vec<String> {
    let mut blockers = Vec::new();
    if roadmap.target_release.is_none() {
        blockers.push("TARGET_RELEASE_UNBOUND".to_owned());
    }
    if !clean {
        blockers.push("WORKTREE_NOT_CLEAN".to_owned());
    }
    if has_specs(facts) && review != ReviewFreshnessStatus::Fresh {
        blockers.push("CROSS_SPEC_REVIEW_NOT_FRESH".to_owned());
    }
    if !all_specs_validated {
        blockers.push("SPEC_VALIDATION_INCOMPLETE".to_owned());
    }
    if facts
        .iter()
        .any(|item| matches!(&item.kind, ItemKind::Direct { completed: false }))
    {
        blockers.push("DIRECT_ITEMS_INCOMPLETE".to_owned());
    }
    if health == MilestoneHealth::Inconsistent {
        blockers.push("MILESTONE_INCONSISTENT".to_owned());
    }
    blockers
}

fn derive_release_readiness(
    project_root: &Path,
    specbind_root: &Path,
    stage: DeliveryStage,
    blockers: Vec<String>,
) -> (DeliveryStage, Vec<String>) {
    if stage != DeliveryStage::ReleasePending {
        return (stage, blockers);
    }
    match crate::release_readiness::resolve(project_root, specbind_root) {
        Ok(_) => (DeliveryStage::ReleaseReady, Vec::new()),
        Err(error) => (
            stage,
            error
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.code.to_owned())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        ),
    }
}

fn spec_state_counts(facts: &[ItemFacts]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for item in facts {
        if let ItemKind::Spec { model } = &item.kind {
            let state = model.as_deref().map_or("unavailable", |model| {
                spec_status::state_name(model.declared_state)
            });
            *counts.entry(state.to_owned()).or_default() += 1;
        }
    }
    counts
}

fn spec_predicate(facts: &[ItemFacts], predicate: impl Fn(&SpecStatusModel) -> bool) -> bool {
    facts.iter().all(|item| match &item.kind {
        ItemKind::Spec { model } => model.as_deref().is_some_and(&predicate),
        ItemKind::Direct { .. } => true,
    })
}

fn has_specs(facts: &[ItemFacts]) -> bool {
    facts
        .iter()
        .any(|item| matches!(item.kind, ItemKind::Spec { .. }))
}

fn design_dependencies_ready(item: &ItemFacts, facts: &[ItemFacts]) -> bool {
    item.dependencies
        .iter()
        .filter(|dependency| dependency.starts_with("spec:"))
        .all(|dependency| {
            facts
                .iter()
                .find(|item| &item.id == dependency)
                .is_some_and(|item| {
                    matches!(&item.kind, ItemKind::Spec { model } if model.as_deref().is_some_and(design_approved))
                })
        })
}

fn dependencies_ready(item: &ItemFacts, completion: &BTreeMap<String, bool>) -> bool {
    item.dependencies
        .iter()
        .all(|dependency| completion.get(dependency).copied().unwrap_or(false))
}

fn dependency_key(dependency: &Dependency) -> String {
    match dependency {
        Dependency::Spec(value) => format!("spec:{}", value.spec),
        Dependency::Direct(value) => format!("direct:{}", value.direct),
    }
}

fn diagnose_unscoped_active_specs(
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    diagnostics: &mut BTreeSet<MilestoneDiagnostic>,
) {
    let scoped = roadmap.spec_ids().into_iter().collect::<BTreeSet<_>>();
    let Ok(entries) = fs::read_dir(specbind_root.join("specs")) else {
        return;
    };
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let Some(spec) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if scoped.contains(&spec) {
            continue;
        }
        let resolution = artifacts::resolve_spec(specbind_root, &spec);
        if resolution
            .wire
            .as_ref()
            .is_some_and(|wire| wire.active_change.0.is_some())
        {
            diagnostics.insert(MilestoneDiagnostic {
                code: "MILESTONE_UNSCOPED_ACTIVE_SPEC",
                path: Some(format!("specs/{spec}/spec.yaml")),
                message: format!("active spec {spec} is absent from Roadmap scope"),
            });
        }
    }
}

fn git_state(project_root: &Path) -> GitState {
    let revision = repository::output(project_root, &["rev-parse", "HEAD"]);
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    );
    match (revision, status) {
        (Ok(revision), Ok(status)) => GitState {
            revision: Some(revision.trim().to_owned()),
            clean: status.is_empty(),
            diagnostic: None,
        },
        (Err(error), _) | (_, Err(error)) => git_failure(error.to_string()),
    }
}

fn git_failure(message: String) -> GitState {
    GitState {
        revision: None,
        clean: false,
        diagnostic: Some(MilestoneDiagnostic {
            code: "MILESTONE_GIT_STATE_FAILED",
            path: None,
            message,
        }),
    }
}

fn state_rank(state: Option<WorkflowState>) -> u8 {
    match state {
        None => 0,
        Some(WorkflowState::Requirements) => 1,
        Some(WorkflowState::Design) => 2,
        Some(WorkflowState::Tasks) => 3,
        Some(WorkflowState::Implementation) => 4,
        Some(WorkflowState::ReleaseReady) => 5,
    }
}

fn from_discovery(issue: &DiscoveryIssue) -> MilestoneDiagnostic {
    MilestoneDiagnostic {
        code: issue.code,
        path: issue.path.as_ref().map(|path| path.as_str().to_owned()),
        message: issue.message.clone(),
    }
}

fn failure(code: &'static str, message: impl Into<String>) -> MilestoneStatusFailure {
    MilestoneStatusFailure {
        diagnostics: vec![MilestoneDiagnostic {
            code,
            path: Some("steering/roadmap.md".to_owned()),
            message: message.into(),
        }],
    }
}

#[must_use]
pub fn stage_name(stage: DeliveryStage) -> &'static str {
    match stage {
        DeliveryStage::Requirements => "requirements",
        DeliveryStage::Design => "design",
        DeliveryStage::CrossSpecReview => "cross_spec_review",
        DeliveryStage::Tasks => "tasks",
        DeliveryStage::Implementation => "implementation",
        DeliveryStage::Validation => "validation",
        DeliveryStage::ReleasePending => "release_pending",
        DeliveryStage::ReleaseReady => "release_ready",
    }
}

#[must_use]
pub fn review_name(status: ReviewFreshnessStatus) -> &'static str {
    match status {
        ReviewFreshnessStatus::NotRequired => "not_applicable",
        ReviewFreshnessStatus::Missing => "absent",
        ReviewFreshnessStatus::Fresh => "fresh",
        ReviewFreshnessStatus::Stale => "stale",
        ReviewFreshnessStatus::Invalid => "invalid",
    }
}
