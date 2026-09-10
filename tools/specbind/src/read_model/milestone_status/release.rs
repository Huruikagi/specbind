use super::{
    BTreeMap, BTreeSet, DeliveryStage, ItemFacts, ItemKind, MilestoneHealth, Path,
    ReviewFreshnessStatus, RoadmapDocument, SpecStatusModel, spec_status,
};

pub(super) fn release_blockers(
    facts: &[ItemFacts],
    roadmap: &RoadmapDocument,
    review: ReviewFreshnessStatus,
    health: MilestoneHealth,
    all_specs_validated: bool,
) -> Vec<String> {
    let mut blockers = Vec::new();
    if roadmap.target_release.is_none() {
        blockers.push("TARGET_RELEASE_UNBOUND".to_owned());
    }
    if !matches!(
        review,
        ReviewFreshnessStatus::Fresh | ReviewFreshnessStatus::NotRequired
    ) {
        blockers.push("CONTRACT_REVIEW_NOT_FRESH".to_owned());
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

pub(super) fn derive_release_readiness(
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

pub(super) fn spec_state_counts(facts: &[ItemFacts]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for item in facts {
        if let ItemKind::Spec { model, .. } = &item.kind {
            let state = model.as_deref().map_or("unavailable", |model| {
                spec_status::state_name(model.declared_state)
            });
            *counts.entry(state.to_owned()).or_default() += 1;
        }
    }
    counts
}

pub(super) fn spec_predicate(
    facts: &[ItemFacts],
    predicate: impl Fn(&SpecStatusModel) -> bool,
) -> bool {
    facts.iter().all(|item| match &item.kind {
        ItemKind::Spec { model, .. } => model.as_deref().is_some_and(&predicate),
        ItemKind::Direct { .. } => true,
    })
}

pub(super) fn has_specs(facts: &[ItemFacts]) -> bool {
    facts
        .iter()
        .any(|item| matches!(item.kind, ItemKind::Spec { .. }))
}
