# Prepare a project shared Contract

Use this procedure for explicit `--shared` scope, an explicit request to plan
the milestone's shared resources, or a Drive handler with mode `shared`.
It authorizes only the shared changes already assigned to Direct items in the
Roadmap. It does not create a Spec or grant Spec gate/rewind authority.

Read these complete inputs before authoring:

```sh
specbind milestone status
specbind milestone scope --include-body
specbind schema read shared-contract/v1
specbind contract shared read
specbind contract shared consumers
specbind adapter read git --for consume
```

Resolve `specDir` from project configuration. The optional canonical output is
`<specDir>/shared-contract.yaml`, outside `specs/`. Absence is valid; invalid
presence stops preparation for diagnosis. Never classify a read failure as an
empty manifest. Read the corresponding baseline through Git, distinguishing
an absent path from an unreadable baseline. The CLI owns the baseline.

Each resource has a stable ID, description, exact/subtree project paths,
change_policy and invariants. Describe each feature's permitted contribution
and the guarantees it must preserve. Rules are natural language, not commands
to execute. Do not invent verification commands, feature requirements, or
JSON-key ownership enforced by the CLI. Do not copy Steering policy into a
second independently maintained source: shared change conditions belong here;
Steering may explain the rationale.

Only author the resources listed in `sharedContractChanges` in the scoped
Direct items. `*` represents creating/removing an empty manifest, not wildcard
permission to change resources. If an actual change is unassigned, return to
Discovery to repair scope. Maintain resource IDs across path moves. A removal
must resolve every incoming reference first. Never delete Spec artifacts to
make a resource shared.

Reuse an existing proposal on re-entry. Compare it with the scoped intent and
any review findings; do not rewrite an already suitable proposal. Present the
concrete shared agreement and its affected consumer candidates. Check the
complete graph with `specbind check contracts`. A shared proposal may expose
pending consumer changes: route those to their owning Spec Design rather than
editing Spec artifacts here. Apply the Git adapter's authorized checkpoint to
this preparation before handing control back. Do not complete the Direct item
or edit the resource's implementation files during preparation.

If invoked from an all-Spec planning run, prepare the shared agreement before
consumer Design and return to that run. It still owns its selected gates and
the single milestone Contract Review. This internal preparation does not
recursively invoke Plan or accept a premature review.

For standalone `--shared` or Drive mode `shared`, if participating Specs still
need Requirements/Design work, name the exact next owning phase and return.
Do not wait for a Direct implementation to complete before consumer Design:
the proposed agreement is a planning input, not an implementation dependency.
If every participating Design is ready (including zero Specs), invoke the
installed `sb-contract-review` owner and follow its confirmation boundary.
Do not replace its semantic review with `check contracts`. A fresh review is
the terminal success of this mode; resource implementation and Direct
completion belong to `sb-implement` afterward.

No separate shared approval state, shared Tasks, or shared completion record
exists. An unchanged actionable entry after a handoff is a waiting/attention
result, never permission to regenerate the proposal indefinitely.
