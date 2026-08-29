# 0156: Expose derived Contract graph reads without persisting another graph

Status: Accepted

## Context

[Decision 0155](./0155-versioned-yaml-contract-artifact.md) puts the complete
authoritative local boundary declaration in each persistent Spec's
`contract.yaml`. The existing Contract graph read model combines those files,
resolves every `consumes` target, detects graph diagnostics, and already serves
`check contracts` and milestone Contract Review.

`check contracts` is intentionally a pass-or-fail consistency gate. It reports
counts and warnings, but does not expose the resolved edges an author needs to
answer narrower questions: which managed providers one Spec directly consumes,
and which managed Specs directly consume a provider. Agents must otherwise read
every Contract or reconstruct the graph with ad hoc search.

Persisting a second graph artifact would duplicate authoritative Contract data
and create a synchronization obligation. Calling a mechanically reachable Spec
an "impact" would also overstate what topology proves: semantic compatibility,
external consumers, and whether a particular edit matters remain review
judgment.

## Decision

### Commands

The CLI exposes three read-only projections:

```text
specbind contract graph
specbind contract dependencies <spec>
specbind contract consumers <spec>
```

- `contract graph` reports every resolved direct Contract dependency reference.
- `contract dependencies <spec>` reports the direct provider references from
  that Spec's `consumes` entries.
- `contract consumers <spec>` performs the reverse lookup and reports every
  direct managed consumer reference targeting that Spec.
- `<spec>` is a canonical Spec identity, never a path.
- Results retain entry-level selectors on both sides of each edge so multiple
  seams between the same two Specs remain visible and auditable.
- Ordering is deterministic and follows the Contract graph read model.
- These commands persist nothing, require no active milestone, and are never
  mutation authority.

The family deliberately has no `impact` command. A direct or transitive graph
relationship identifies a review candidate, not semantic impact. It also has no
transitive mode in v1: callers can inspect the complete direct graph without the
CLI implying that every reachable Spec is affected.

### Validation and output

All three commands resolve the complete current persistent Contract set. They
fail closed with `ERROR CONTRACT_GRAPH_READ_FAILED` if project discovery, any
Contract, or any dependency reference is structurally invalid; a partial graph
is never presented as complete. A valid graph may still carry the review
warnings accepted by Decisions 0011 and 0090.

`contract graph` returns `OK CONTRACT_GRAPH_REPORTED`, lists the resolved edges,
and reports the project-wide warning count. The focused commands return
`OK CONTRACT_DEPENDENCIES_REPORTED` or `OK CONTRACT_CONSUMERS_REPORTED` and list
their matching edges. An empty result is successful and explicitly reports
`none`. An unknown or unresolved Spec returns `ERROR CONTRACT_SPEC_NOT_FOUND`.

Output remains text-first under Decision 0067. This decision does not add JSON,
DOT, Mermaid, or another serialization format. Those would establish a broader
machine or visualization contract and require a separate demonstrated use case.

## Consequences

- Contract files remain the only persistent source of Contract graph truth.
- Design and review workflows can query reverse dependencies without scanning
  every Spec.
- `check contracts` remains the CI-oriented verdict, while `contract ...`
  commands are inspection projections over the same read model.
- The CLI does not claim semantic or unmanaged-consumer impact from topology.

## Implementation status

Implemented. The three commands share the existing Contract graph resolver,
reject incomplete graphs, and render deterministic entry-level direct edges.
`specbind-design` uses the two focused projections before editing a possible
existing seam, then reads each named neighboring Contract while retaining
ordinary investigation for new or unmanaged relationships.
