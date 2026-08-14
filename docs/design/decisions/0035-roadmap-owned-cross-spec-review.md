# 0035: Keep cross-spec review evidence in the active roadmap

Status: Accepted

## Context

Contract-impact classification and downstream review describe relationships among milestone changes and persistent spec contracts. Persisting the same classification, affected contract entries, and reviewed consumers in each participating spec's completion evidence would duplicate milestone-wide facts and require reconciliation when the review scope changes.

The active `roadmap.md` already owns milestone scope, dependency ordering, and cross-spec evidence. Each active spec identifies its roadmap scope through `active_change.milestone_id` and its canonical spec identity.

## Decision

- The active roadmap is the canonical owner of current contract-impact classification, affected contract-entry references, downstream review scope, and accepted downstream review outcome.
- A cross-spec review record is associated with the relevant milestone item or `(milestone_id, canonical spec identity)` pair. One shared record may cover several affected specs; it is not copied into every `spec.yaml`.
- Per-spec completion evidence contains no `contract_impact`, `downstream_review`, affected-spec list, or duplicated cross-spec pass flag.
- Under Decision 0041, the CLI resolves the relevant roadmap record through the active change's `milestone_id` and canonical spec identity. No additional roadmap-evidence reference field is added to `spec.yaml` v1.
- Completion acceptance requires the resolved roadmap classification to be present and current. A justified local-only classification satisfies this guard without a downstream-spec list; a contract-affecting classification additionally requires the applicable downstream review scope and accepted outcome.
- If downstream review requires another spec to change or be revalidated, discovery and roadmap maintenance add or update that work in milestone scope. Merely reviewing a consumer does not create completion evidence in the consumer's `spec.yaml`.
- Roadmap archival preserves the accepted cross-spec record as milestone release evidence. Per-spec changelogs may summarize the final classification and changed contract entries without becoming the canonical active review store.
- This decision fixes storage ownership and lookup identity. The exact parseable `roadmap.md` grammar and cross-spec review record syntax remain a separate schema decision.

## Consequences

- Cross-spec evidence is recorded once at the level where its scope and routing effects are meaningful.
- `spec.yaml` completion evidence remains spec-local and does not become a partial copy of roadmap state.
- Changing contract classification or downstream scope invalidates the affected lifecycle gates through roadmap consistency checks rather than by comparing duplicated fields.
- Release finalization can archive one coherent milestone-wide review record alongside the roadmap.
