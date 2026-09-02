# 0186: Make reverse Design Contract checks phase-relative

Status: Accepted

## Context

Decision [0181](./0181-reverse-spec-establishment.md) reuses Requirements,
Design, independent Design validation, and milestone Contract Review while
forbidding Tasks. The Plan scheduler keeps Requirements parallel, orders Design
by Roadmap dependencies, and places one global Contract Review barrier after
every participating Design is approved.

The Design author and validator both run the project-wide `check contracts` as
a structural preflight. In a multi-Spec reverse milestone, a downstream Spec
legitimately has no Contract until its Design becomes actionable. The command
therefore reports `CONTRACT_GRAPH_CONTRACT_UNAVAILABLE` before the first Design
can be validated or approved, while the downstream Design cannot start until
that approval. Repeating either side cannot change the state.

## Decision

- `check contracts` remains project-wide and unchanged. Its complete successful
  verdict remains mandatory at milestone Contract Review.
- During Design authoring or independent validation only, a failed graph check
  is a provisional pass when every error is
  `CONTRACT_GRAPH_CONTRACT_UNAVAILABLE` for another participant in the same
  reverse milestone and current milestone status reports that participant as
  waiting for an earlier Design dependency.
- The current Spec must have a readable Contract. An unavailable current
  Contract, an unavailable Contract outside that exact waiting set, any target,
  entry, schema, inventory, or project error, and any inability to prove the
  reverse waiting state still stops the Design phase or returns `NOT_READY`.
- Warnings retain their existing Design judgment. The provisional exception is
  reported explicitly and never described as a clean whole graph.
- The scheduler reruns the same check for each Design wave. After every
  participating Design exists and is approved, Contract Review requires the
  normal complete graph and accepts no provisional exception.

## Consequences

- A valid dependency-ordered reverse milestone can advance one Design wave at
  a time without authoring downstream Contracts prematurely.
- Ordinary milestones and inconsistent reverse graphs remain fail closed.
- The CLI keeps one deterministic whole-project graph contract; lifecycle-aware
  interpretation remains in the owning Skills.

## Verification

Focused Skill tests require the exact reverse-only predicate in both Design
authoring and independent validation and retain the global barrier wording in
Plan. A fresh multi-Spec reverse fixture verifies that the first Design reaches
semantic validation while the downstream Contract is absent and that no other
graph error is waived.
