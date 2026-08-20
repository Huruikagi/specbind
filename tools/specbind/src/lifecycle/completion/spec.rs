//! Spec completion lifecycle.

use std::path::Path;

use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    artifacts,
    freshness::{self, FreshnessStatus},
    schema::spec::v1::{CompletionGateEvidence, ImplementationRevision, PassedAt, WorkflowState},
    spec_status,
};

use super::{
    CompletionIssues, SpecAcceptOutcome, SpecInvalidateOutcome, SpecPreflightOutcome,
    candidate::{ValidatedCandidate, validate as validate_candidate},
    discovery_failure, finish_issues, from_discovery,
    guard::{clean_head, ensure_target_clean, spec_guard, valid_id},
    issue, one_issue, persist_regular, render_yaml, spec_path, validate_mutated_spec,
};

/// Begins validation for one explicit Spec at the converged milestone revision.
///
/// # Errors
///
/// Returns deterministic lifecycle, milestone, review, task, or Git diagnostics.
pub fn spec_preflight(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<SpecPreflightOutcome, CompletionIssues> {
    if let Some(revision) = accepted_revision(project_root, specbind_root, canonical_spec, None)? {
        return Ok(SpecPreflightOutcome::AlreadyAccepted {
            implementation_revision: revision,
        });
    }
    let revision = clean_head(project_root)?;
    spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &revision,
        false,
    )?;
    Ok(SpecPreflightOutcome::Ready {
        implementation_revision: revision,
    })
}

/// Accepts one strict completion candidate and performs `IMPLEMENTATION_VALIDATED`.
///
/// # Errors
///
/// Returns candidate, lifecycle, freshness, race, Git, serialization, or write
/// diagnostics without partially changing `spec.yaml`.
pub fn spec_accept(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    candidate_json: &str,
) -> Result<SpecAcceptOutcome, CompletionIssues> {
    let candidate = validate_candidate(candidate_json)?;
    if let Some(revision) = accepted_revision(
        project_root,
        specbind_root,
        canonical_spec,
        Some(&candidate),
    )? {
        return Ok(SpecAcceptOutcome::AlreadyAccepted {
            implementation_revision: revision,
        });
    }
    let initial = spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &candidate.implementation_revision,
        true,
    )?;
    let current = spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &candidate.implementation_revision,
        true,
    )?;
    if initial.source != current.source || initial.wire != current.wire {
        return Err(one_issue(
            "COMPLETION_INPUTS_CHANGED",
            Some(spec_path(canonical_spec)),
            "spec completion inputs changed during guarded acceptance",
        ));
    }

    let mut wire = current.wire;
    let active = wire.active_change.0.as_mut().ok_or_else(|| {
        one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion acceptance requires an active implementation change",
        )
    })?;
    let passed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_issue(
                "COMPLETION_TIMESTAMP_FAILED",
                Some(spec_path(canonical_spec)),
                error.to_string(),
            )
        })?;
    active.state = WorkflowState::ReleaseReady;
    let evidence = active.gate_evidence.as_mut().ok_or_else(|| {
        one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "implementation state requires cumulative gate evidence",
        )
    })?;
    evidence.completion = Some(CompletionGateEvidence {
        passed_at: PassedAt(passed_at),
        implementation_revision: ImplementationRevision(candidate.implementation_revision.clone()),
        mechanical_checks: candidate.mechanical_checks,
    });
    validate_mutated_spec(&wire, canonical_spec)?;
    let rendered = render_yaml(&wire, "COMPLETION_SPEC_SERIALIZE_FAILED")?;
    persist_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
        "COMPLETION_SPEC_WRITE_FAILED",
        &spec_path(canonical_spec),
    )?;
    Ok(SpecAcceptOutcome::Accepted {
        implementation_revision: candidate.implementation_revision,
    })
}

/// Explicitly clears stale completion evidence and rewinds to implementation.
///
/// # Errors
///
/// Returns prior-gate, target-path, serialization, or filesystem diagnostics.
pub fn spec_invalidate(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<SpecInvalidateOutcome, CompletionIssues> {
    if !valid_id(canonical_spec) {
        return Err(one_issue(
            "SPEC_COMPLETION_TARGET_INVALID",
            Some(format!("specs/{canonical_spec}")),
            "completion invalidation requires a canonical Spec ID",
        ));
    }
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(mut wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    let Some(active) = wire.active_change.0.as_ref() else {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation requires an active change",
        ));
    };
    let completion_present = active
        .gate_evidence
        .as_ref()
        .and_then(|evidence| evidence.completion.as_ref())
        .is_some();
    if active.state == WorkflowState::Implementation && !completion_present {
        return Ok(SpecInvalidateOutcome::NoChange);
    }
    if active.state != WorkflowState::ReleaseReady {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation requires release_ready or an already-clean implementation state",
        ));
    }

    let gate_inputs = artifacts::resolve_gate_inputs(specbind_root, canonical_spec);
    let freshness = freshness::evaluate_wire(&wire, &gate_inputs.inputs);
    let mut issues = gate_inputs
        .inventory
        .issues
        .iter()
        .cloned()
        .map(from_discovery)
        .collect::<Vec<_>>();
    for (name, gate) in [
        ("Requirements", &freshness.requirements),
        ("Design", &freshness.design),
        ("Tasks", &freshness.tasks),
    ] {
        if gate.status != FreshnessStatus::Fresh {
            issues.push(issue(
                "SPEC_COMPLETION_EARLIER_GATE_STALE",
                Some(spec_path(canonical_spec)),
                format!("{name} must remain fresh before completion-only invalidation"),
            ));
        }
    }
    ensure_target_clean(
        project_root,
        specbind_root,
        &spec_path(canonical_spec),
        &mut issues,
    );
    finish_issues(issues)?;

    let Some(active) = wire.active_change.0.as_mut() else {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation lost its active change",
        ));
    };
    active.state = WorkflowState::Implementation;
    if let Some(evidence) = active.gate_evidence.as_mut() {
        evidence.completion = None;
    }
    validate_mutated_spec(&wire, canonical_spec)?;
    let rendered = render_yaml(&wire, "COMPLETION_SPEC_SERIALIZE_FAILED")?;
    persist_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
        "COMPLETION_SPEC_WRITE_FAILED",
        &spec_path(canonical_spec),
    )?;
    Ok(SpecInvalidateOutcome::Invalidated)
}

fn accepted_revision(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    candidate: Option<&ValidatedCandidate>,
) -> Result<Option<String>, CompletionIssues> {
    if !valid_id(canonical_spec) {
        return Err(one_issue(
            "SPEC_COMPLETION_TARGET_INVALID",
            Some(format!("specs/{canonical_spec}")),
            "completion requires a canonical Spec ID",
        ));
    }
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    let Some(active) = wire.active_change.0.as_ref() else {
        return Ok(None);
    };
    if active.state != WorkflowState::ReleaseReady {
        return Ok(None);
    }
    let Some(completion) = active
        .gate_evidence
        .as_ref()
        .and_then(|evidence| evidence.completion.as_ref())
    else {
        return Ok(None);
    };
    let model = spec_status::resolve(project_root, specbind_root, canonical_spec)
        .map_err(|error| discovery_failure(error.issues))?;
    if model.freshness.completion.status != FreshnessStatus::Fresh {
        return Ok(None);
    }
    if let Some(candidate) = candidate
        && (completion.implementation_revision.0 != candidate.implementation_revision
            || completion.mechanical_checks != candidate.mechanical_checks)
    {
        return Ok(None);
    }
    Ok(Some(completion.implementation_revision.0.clone()))
}
