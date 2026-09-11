# Shared Contract scenarios

[Back to the forward-test index](../skill-forward-tests.md).

These scenarios exercise [Decision 0209](../design/decisions/0209-project-shared-contract.md).
Prepare each with `sh tools/specbind/scripts/forward-test-shared.sh sh1|sh2|sh3 <target>`.
Use the isolated driver and confirmation procedure in [Running the tests](./running.md).

## SH1 — Prepare a shared agreement without a dedicated Spec

The fixture has matching English/Japanese catalog keys, no shared Contract,
and a Direct-only milestone assigning `translations` to `catalog-agreement`.
The Direct summary states the paths, contribution policy, and invariants.

Maintainer request: “Plan the milestone's shared translation resources using
the scoped agreement. Stop after planning and its review; do not implement
catalog changes.”

Approve only the concrete Contract Review assessment once it is presented.
Expect the installed Plan shared procedure and Contract Review owner to run,
`<specDir>/specs/shared-contract.yaml` to declare `translations` with both catalog paths and
the scoped rules, and `contract owners locales/en.json` to return the shared
selector. No new Spec or Tasks may appear. Catalogs remain unchanged, the
Direct remains pending, review is fresh, and the authorized planning/review
checkpoints leave a clean tree.

## SH2 — Implement ordinary copy maintenance under an existing agreement

The fixture already has a shared translation agreement at the baseline and
a Direct-only milestone assigning a single English typo correction. It has
no `sharedContractChanges` and requires no Contract Review.

Maintainer request: “Implement the scoped catalog typo correction and finish
that Direct item.”

Expect the installed Direct implementation procedure to correct only
`common.title` in `locales/en.json` from `Bookshp` to `Bookshop`, verify the
catalog invariants, checkpoint, and complete `catalog-typo`. The Japanese
catalog and shared agreement remain byte-identical, no new Spec or Tasks
appear, review remains `not_applicable`, and the tree is clean. A task review
or its prerequisite failure must be measured, never bypassed by the driver.

## SH3 — Retain a shared agreement through reverse adoption

The A4 fixture is at Contract Review with cart/order Designs approved and no
implementation changes. Its confirmed reverse proposal additionally records
`project-overview`, managing the existing `README.md`, in the temporary
adoption record and project shared Contract.

Maintainer request: “Resume the existing reverse establishment, including its
already-proposed shared project documentation agreement, through finalization.
Please use English.”

Expect the installed adoption continuation to perform milestone Contract
Review before baseline finalization, preserve the shared agreement, archive
the review containing its fingerprint, and retain the existing README and
implementation unchanged. No dedicated shared Spec, Tasks, Direct item, or
product release is created. The CLI must preserve `sb-adopt` reverse handlers,
and the completed checkpoint must leave a clean tree.
