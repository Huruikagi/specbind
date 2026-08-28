# Skill forward tests

Mechanical conformance runs in CI. Behavioral verification checks whether an agent given an embedded skill leaves the intended artifacts and machine state in a fixture project. Run it when a skill changes materially and before a release.

[Decision 0096](./design/decisions/0096-skill-asset-layout.md) establishes this
manual behavioral-verification boundary.

The suite is split by concern so measurement history does not obscure the procedure or scenario contracts:

| Document | Purpose |
| --- | --- |
| [Running the tests](./skill-forward-tests/running.md) | Fixture setup, driver isolation, confirmation turns, judging state, usability debriefs, and rerun policy |
| [Measurement ledger](./skill-forward-tests/results.md) | Passing measurements, stopped runs, environment failures, and post-run usability observations |
| [Planning scenarios](./skill-forward-tests/planning-scenarios.md) | Discovery, Requirements, Design, Contract review, and Tasks |
| [Delivery scenarios](./skill-forward-tests/delivery-scenarios.md) | Implementation, release, validation, task review, and debugging |
| [Orchestration scenarios](./skill-forward-tests/orchestration-scenarios.md) | Configuration, quick-plan scope modes, checkpoints, gap analysis, steering, and failure handling |
| [End-to-end journeys](./skill-forward-tests/journey-scenarios.md) | High-cost release-smoke scenarios that compose the whole workflow |

Scenario definitions are the behavioral contract. The ledger records measurements against specific builds; it never rewrites a scenario merely to make a run pass.
