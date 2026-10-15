# 0223: Review individual Designs before the complete Contract graph

Status: Accepted

## Context

Decision 0222 identified a whole-graph prerequisite that prevents reviewing one
Design while another new delivery Spec still has no Contract. The maintainer
chose to settle each Spec locally first, accepting that later Design work or
Contract Review can send it back for revision.

## Decision

Add the read-only `specbind check contracts --for-design <spec>` projection.
It requires a valid Contract for the selected Spec and each Spec named by its
direct `Consumes` entries. Every directly consumed target entry must resolve;
shared resources are loaded and checked when the selected Spec consumes them.
Self-consumption remains an error. Required provider manifests must be valid,
but their own outgoing references are not traversed or checked in this scope.
Unrelated Spec discovery and shared resources not consumed by this Spec are
outside this projection. The ordinary whole-project check is unchanged.

The scoped result uses `DESIGN_CONTRACTS_VERIFIED` / `DESIGN_CONTRACTS_FAILED`,
lists the loaded Spec Contracts and direct dependencies, and explicitly reports
that the whole graph was not evaluated. Ownership overlap warnings are limited
to the selected Spec against the loaded providers. Global ownership overlap,
cycles, incoming consumers and unconsumed exports remain full-graph concerns.

Design authors and independent validators use this projection in delivery and
reverse establishment. This supersedes Decision 0186's manual exception for
waiting reverse participants, not its dependency ordering. Missing or invalid
direct provider evidence still prevents readiness. Semantic review must catch
dependencies expressed in Design prose but omitted from the Contract; omitting
a `Consumes` entry cannot make a dependency optional.

Existing-seam authors retain their incoming-impact investigation. Full-graph
neighbor queries remain strict. When unfinished unrelated Specs prevent those
queries, authors inspect the available Contracts through inventories, report
incomplete incoming-impact knowledge, and do not fabricate missing Contracts.
Known conflicts affecting the proposed change still need resolution or an
owned scope/recovery handoff.

A local `READY` or Design approval is a checkpoint against current inputs.
Later Design work or Contract Review may find a conflict and require the
ordinary confirmed Design recovery and independent revalidation. No automatic
rewind, new approval authority or weakened freshness rule is introduced.
The complete project graph is still mandatory at Contract Review and its
acceptance guard; a scoped success never substitutes for global acceptance.

## Verification

Test unrelated missing/invalid Contracts, strict current and direct-provider
inputs, target entries, self-consumption and shared resources. Verify that
provider outgoing references are deferred and the global check remains strict.
Behaviorally review a complete Design beside an unfinished independent Spec,
and reject an otherwise identical Design with an unavailable direct provider.
Keep lifecycle records unchanged in both read-only reviews.
