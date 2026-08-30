# 0162: Separate forward-test measurements from current findings

Status: Accepted

## Context

The forward-test ledger grew from one batch record into a document containing
three different kinds of information:

- historical measurements of one driver against one product build;
- a current summary of recorded scenario coverage;
- a mutable worklist of reproduced usability findings and environment limits.

Those records have different lifecycles. A failed measurement must remain
historical even after a retry passes. Current coverage must stay small enough to
scan. A usability observation may be discarded, fixed pending confirmation, or
resolved without rewriting the measurement that exposed it.

Keeping all three in one chronological document made `Latest run` an
ever-growing narrative and made routine updates increasingly likely to blur a
run verdict with later remediation.

## Decision

### One immutable record per tested build and driver

Each new batch is recorded under `docs/skill-forward-tests/runs/` as
`YYYY-MM-DD-<driver>-<short-build>[-N].md`. One record contains only scenarios
driven as the same agent against the same commit. A fix or build change starts a
new record; a retry never replaces the failed attempt.

Every record states the date, driver, model and profile when known, tested
build, fixture language, and scenarios. Each scenario row uses one of these
verdicts:

- `pass`: every scenario expectation held in the judged fixture;
- `product_failure`: the judged fixture violated a scenario expectation after
  confirming that the accepted Decisions require it;
- `scenario_invalid`: the recipe, precondition, expectation, or harness did not
  measure the intended product contract;
- `environment_invalid`: the driver environment did not exercise the installed
  product Skill;
- `environment_blocked`: the product Skill ran, but an external boundary
  prevented the scenario from reaching a verdict.

The row also records the failed expectation when applicable, the state left in
the fixture, and any linked finding. Concise mechanical evidence comes from the
fixture, not the driver's report. Raw transcripts and disposable fixtures are
not repository artifacts.

After a run record is committed it is historical. A later factual correction is
made explicitly; a later pass or changed interpretation does not erase its
original verdict.

### The dashboard is a projection, not another history

`docs/skill-forward-tests/results.md` is the current dashboard. It links the run
archive and findings worklist, summarizes normalized measurements, and shows
recorded passing coverage. It does not repeat the chronological narrative from
run records or imply that an old pass was measured against current `HEAD`.

The dashboard records the tested build and date for normalized measurements.
Historical measurements that predate this structure remain a labeled legacy
baseline rather than receiving inferred build metadata.

### Findings have their own lifecycle

`docs/skill-forward-tests/findings.md` is the triaged worklist. Reproduced
product findings receive stable `FT-NNNN` identifiers and move through:

- `open`;
- `fixed_pending`, after a focused fix but before the exact behavioral branch is
  confirmed in a fresh fixture;
- `resolved`, after behavioral confirmation.

Environment limitations receive separate `ENV-NNNN` identifiers and are
retained only while they affect interpretation. They are not product findings.
One-off non-defects, duplicates, fixture-only workarounds, and `none`
observations stay in the run record with their disposition and do not enter the
worklist.

A confirmed product defect may link to a GitHub Issue when external tracking is
useful. Creating an Issue is not required for every observation and does not
replace the repository measurement or finding record.

### Existing history is migrated without reconstruction

The ledger through 2026-08-30 is retained as
`runs/legacy-through-2026-08-30.md`. Its historical prose and verdicts are not
split retroactively into inferred batches. The current coverage and actionable
finding state are projected into the new dashboard and worklist; normalized run
records begin with the next measurement.

## Consequences

- A reader can answer current coverage, historical execution, and actionable
  findings from three bounded surfaces.
- A failed run stays visible without forcing every future dashboard reader
  through its full narrative.
- Finding remediation no longer changes the meaning of the measurement that
  exposed it.
- The format remains plain Markdown and Git-owned; no database, generated
  protocol, or mandatory Issue synchronization is introduced.

## Implementation status

Implemented. The legacy ledger is archived, the dashboard and findings worklist
are separated, and the running procedure and repository forward-test Skill use
the normalized run template.
