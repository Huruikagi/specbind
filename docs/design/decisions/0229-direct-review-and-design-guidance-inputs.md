# 0229: Align Direct recovery and independent review inputs

Status: Accepted

## Context

A product-Skill audit found Spec-backed replanning instructions in Direct-only
diagnosis routing, and a shared Task review protocol requiring Spec artifacts
even when the caller supplied Direct work. Design validation also omitted
Steering while Contract Review explicitly assigned project-guidance alignment
to that validator. Adoption's plural selector placeholder had already caused
an invalid multi-selector CLI invocation (FT-0063).

## Decision

- Direct diagnosis that reveals a need for canonical artifacts or an unsettled
  product/architecture decision returns to Discovery. A Drive caller receives
  `REROUTABLE` plus `HUMAN_DECISION`, the exact item and finding, and attributable
  partial paths/diff. It receives no invented Task progress. Drive replan does
  not authorize Direct reclassification or a Plan dispatch for that item.
- Direct review reads the exact Roadmap summary, observable done condition,
  applicable checks and results, every current Steering document, and applicable
  shared Contract. It requires no Spec artifacts. Missing required inputs yield
  `CANNOT_REVIEW`; an explicit empty Steering collection or absent shared
  Contract is valid. The ordinary correctness standard and Direct retry bounds
  remain unchanged for both inline and independent review.
- Independent Design validation lists and reads every current Steering document,
  in addition to its existing Rules and Spec inputs. A conflict with readable
  project guidance is a finding. Failed Steering reads stop as `BLOCKED / INPUT`
  without a Design verdict, repair, deferred write, or Design revision charge.
  Plan preserves that stop just as it preserves Decision 0224's environment
  stop. Neither stop weakens `NOT_READY` for insufficient Design meaning.
- Adoption reads each listed Steering selector in its own CLI invocation.
  The CLI's single-selector interface is unchanged.

These clarifications retain the phase owners, confirmation boundaries, current
scope, independent judgments, and existing recovery budgets. No schema,
generated role, installed path, or lifecycle transition changes.

## Verification

Rendered-Skill and protocol tests cover Direct routing and complete governing
inputs. Fresh behavioral scenarios cover independent Direct review (I3r), a
Direct artifact-diagnosis continuation with replan authority (I14), Design versus
Steering (VD7), a failed Steering read (VD8), and per-selector adoption reads
(A2). Results and any environment limitations belong in the run archive.
