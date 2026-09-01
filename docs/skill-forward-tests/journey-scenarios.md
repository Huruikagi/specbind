# End-to-end skill forward-test journeys

[Back to the forward-test index](../skill-forward-tests.md). These journeys
measure whether the installed skills compose across lifecycle boundaries. They
are expensive release-smoke tests, not members of the ordinary per-skill batch.

## When to run one

Run HP1 only when a change can affect several lifecycle phases together, when a
release candidate needs one realistic vertical proof, or when a focused
scenario exposed a composition defect. Do not run it merely because one
authoring skill changed. Focused scenarios remain the cheaper diagnostic tool.

One HP1 measurement uses one fresh fixture and one continuous driver session.
If the product is fixed during the run, the old fixture remains evidence about
the old build. Prepare a new target for any rerun.

## HP1 — A small change reaches a verified local release

Prepare the deterministic project and release policy:

```sh
sh tools/specbind/scripts/forward-test-journey.sh prepare hp1 /tmp/sb-hp1 en
```

The harness installs dispatch instrumentation. Drive HP1 in a real session that
can start the product's planner, implementer, and reviewer subagents. A driver
that cannot dispatch may exercise the supported main-context fallback, but that
is not an orchestration measurement. Use the isolation and PATH rules from
[Running the tests](running.md), including the native fixture path on Windows.

Give the first request verbatim:

> Ask: Ship a cart quantity limit: adding an item must reject a quantity below 1 or any addition that would take one SKU above 99, leave the cart unchanged on rejection, and state the accepted bound in the error. Include automated coverage and release the finished change as v1.4.0.

Then continue the same session through these boundaries. Each line is sent only
after the preceding work has stopped and presented the state it owns.

1. At Discovery's scope confirmation: `I approve the Discovery scope you just presented for the cart change. Stop after Discovery.`
2. After Discovery: `Take the cart item through an approved plan in one go. Present the delegated gates first and wait for my confirmation.`
3. At the delegation boundary: `I authorize sb-plan to accept the requirements, design, and tasks gates for cart. Stop after Tasks approval.`
4. After Tasks approval: `Implement the approved cart work.`
5. After implementation: `Bind this milestone to v1.4.0, follow the project's local checkpoint policy for that binding, but do not publish or finalize. Stop when release preflight cannot proceed.`
6. After the expected not-ready preflight: `Is the cart work done?`
7. After completion is accepted: `Release this milestone.`
8. At Publish confirmation: `Create and verify the local annotated v1.4.0 tag exactly as proposed, then finalize the release.`

Do not compress these messages into broad advance permission. Discovery scope,
delegated gate approval, completion acceptance, and Publish are distinct
boundaries. The journey measures that the skills hand off between them without
silently widening authority.

Judge the resulting project mechanically:

```sh
sh tools/specbind/scripts/forward-test-journey.sh judge hp1 /tmp/sb-hp1
```

HP1 passes only when the judge reports every expectation as satisfied:

- the project test command passes and cart rejection preserves its input;
- the active milestone is gone, `cart` is idle, and its transient Brief and
  Tasks artifacts are gone;
- the canonical cart log and both `v1.4.0` release archives exist;
- `v1.4.0` is an annotated local tag pointing to the verified implementation
  commit before the finalization checkpoint, and no remote exists;
- the final worktree is clean; and
- dispatch instrumentation shows more than the driven context alone.

Record HP1 separately from focused scenarios. Include the driver profile, build,
pass or failure, the failed judge expectation, the final commit, tagged commit,
and dispatch-context count. A workflow pass with only one dispatch-log line is
a fallback-path pass and leaves orchestration unmeasured.
