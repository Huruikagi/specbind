# Reconcile durable responsibility descriptions

This procedure owns the catalog entries `requirements-description`,
`design-description`, and `steering-description`. Use the new binary's migration
plan with the original pre-update version; treat its pending paths as a preview
inventory, never as write authority. Re-evaluate before each resumed batch.

1. Read the selected live artifacts through `artifact read` or `steering read`,
   and templates through `template read`. If ordinary loading fails, stop with
   the exact diagnostic; raw migration inspection is not authority to bypass it.
2. Read complete responsibilities and relevant current status. Propose one
   non-empty single-line description per missing field in the artifact language.
   Requirements describe the complete Spec behavior; Design describes technical
   decisions; Steering describes project-wide decisions. Template descriptions
   describe recurring responsibilities. Do not infer them from names alone.
3. Present the exact per-path diff and lifecycle effect. Obtain the separately
   confirmed reconciliation scope under templates-and-reconciliation guidance.
   Keep template and live changes separate; no template provenance is implied.
4. Apply only the confirmed targets through their owning authoring workflow.
   Preserve IDs, durable instructions, and body semantics. Never modify
   `spec.yaml`, `tasks.yaml`, Gate evidence, release history, or approval state to
   make the descriptions appear current. Metadata edits can stale fingerprints;
   any revalidation or approval is separately authorized ordinary lifecycle work.
5. Re-list templates and the relevant `spec list`, `artifact list <spec>`, or
   `steering list`. Inspect parser, traceability, and status diagnostics for the
   changed live artifacts. Re-run `migration plan --from <from> --json` with the
   same target version if explicitly supplied. Only missing fields remain
   pending; retain existing valid descriptions without rewriting them.

A complete probe proves metadata presence and validity, not semantic approval
or freshness. Report these separately. Recommended work may remain declined;
record its original version and remaining targets in the handoff so a later run
can resume without a persistent migration ledger.
