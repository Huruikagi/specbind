---
name: sb-dev-forward-test
description: Run the behavioral forward tests for SpecBind's embedded skills against a fixture project, using subagents with no prior context. Use when a product-managed skill changed materially, before a release, or when a decision's effect on agent behavior needs checking.
---

# Run the skill forward tests

The [forward-test index](../../../docs/skill-forward-tests.md) routes to the
scenario contracts, measurement dashboard, run archive, and findings worklist.
The scenario documents are the contract; this skill is how to execute them
without rediscovering the operational traps each time.

The index also routes to the deliberately expensive end-to-end journey. Use
that journey only for cross-lifecycle changes or release-candidate confidence;
do not add it to the ordinary per-skill batch.

This tests **product-managed skills** — the ones under
`tools/specbind/assets/skills/` that get installed into consumer projects. It is
not about this repository's own workflow.

## Before anything

Record the commit under test:

```sh
git rev-parse --short HEAD
```

A run measures one build. If you fix something mid-run, every scenario still in
flight is measuring the old build, and its result is about that build only. Say
so rather than folding it into the same table.

## One fixture per scenario

```sh
sh tools/specbind/scripts/forward-test-fixture.sh /tmp/sb-<scenario> en
```

Never share a fixture between scenarios. Several depend on the starting state,
and a leftover milestone silently changes what is being tested. The builder
refuses to touch an existing directory, so delete and rebuild.

The builder prints an `export PATH=` line for the CLI it ships inside the
fixture. That line is not optional. The skills invoke `specbind` as a bare
command because a real installed project has it on PATH; without it the run stops
at "command not found" and produces no result about the skill at all.

Most scenarios have a recipe that builds their starting state and proves it:

```sh
sh tools/specbind/scripts/forward-test-scenario.sh <scenario> /tmp/sb-<scenario> en
```

It wraps the fixture builder, applies the scenario's precondition, and exits
nonzero with a message when the precondition did not take. Use it rather than
composing the state by hand; that is where a run silently becomes a different
run. Add a recipe when a scenario needs one, and give it a check that fails when
the setup is a no-op.

The end-to-end journey has its own prepare-and-judge harness because it starts
near the beginning of the lifecycle and its final state crosses several
scenario families:

```sh
sh tools/specbind/scripts/forward-test-journey.sh prepare hp1 /tmp/sb-hp1 en
# Drive the exact conversation in docs/skill-forward-tests/journey-scenarios.md.
sh tools/specbind/scripts/forward-test-journey.sh judge hp1 /tmp/sb-hp1
```

Do not run `judge` after repairing the fixture by hand. A failed judgment is the
measurement. Rebuild at a new target and rerun only when a fresh measurement is
actually worth its agent cost.

Scenarios with no recipe yet need a precondition set up first — an uncommitted edit, a
broken steering document. **Do that from the shell**, not from a helper script in
another language. On Windows the shell maps `/tmp` to a real directory, but a
native interpreter invoked from it does not, so a script that opens
`/tmp/sb-<scenario>/...` fails while every shell command beside it succeeds. A
precondition that silently did not apply turns a scenario into a different one.
Verify the precondition with a command before launching the run.

Add `--instrument-dispatch` as the final builder or recipe argument only when
the run is intended to prove fresh-context dispatch. It injects the ignored
`.forward-test/agents.log` instruction used to distinguish dispatch from the
main-context fallback. Ordinary workflow scenarios leave it off: they have no
dispatch claim to prove, and an unrelated instrumentation write must not block
the product workflow before it starts.

## Driving a run

Use a subagent with no prior context.

### Codex driver profile

When Codex drives a scenario through a subagent, use this profile:

- `fork_turns: "none"`
- `model: "gpt-5.6-terra"`
- `reasoning_effort: "medium"`

This is the default forward-test driver profile, not part of the product skill
under test. Override it only when the run deliberately compares models, and
record the override with the result.

A Codex-spawned subagent is a Codex result; a Claude Code Agent-tool subagent is
a Claude Code result. Record the actual driver because the scenario document
tracks them as separate columns.

The prompt gives three things and nothing else:

1. the working directory, given as the **native path** its file tools resolve,
   with the shell alias named separately if the two differ. On Windows a subagent
   writing to `/tmp/sb-<scenario>/...` with a file tool can land under the drive
   root instead of the shell's temp mapping, creating a file the CLI then reports
   as missing. One run caught that itself and moved the file; the next may not.
2. the `export PATH=` line, stated as an environment fact
3. the maintainer's request, phrased the way a maintainer would phrase it

Then ask only what it changed and what it ran.

The fixture's own `AGENTS.md` or `CLAUDE.md` is applicable project context. Say
that explicitly when the driver would otherwise treat a prompted working
directory as less authoritative than the host session. If it never reads the
fixture instructions and installed skill tree, the product skill was not
measured.

On Windows, also state the native Git Bash `sh.exe` location when a fixture's
canonical command uses `sh`. The printed `export PATH=` line still remains the
environment fact for the CLI. A run that cannot resolve either executable is an
environment failure, not a verdict about the product skill.

**Neutralize this repository's instructions.** A subagent is not a clean room: it
carries the host session's project instructions (`AGENTS.md` under Codex or
`CLAUDE.md` under Claude Code), not the fixture's. This repository's own
instructions tell an agent to answer in Japanese and to commit each finished unit
to `main` — and the checkpoint scenarios exist to measure whether the agent
commits. State in the prompt that the fixture is a standalone project and that
instructions from any other repository do not apply to it. That is environment
hygiene, not method.

A run reporting in Japanese against an `en` fixture is the visible symptom. Treat
it as a signal that the contamination is active, and re-read any checkpoint
result taken from the same batch.

**Never name a skill or a command in the prompt.** Whether the agent finds and
uses the installed skill is the thing under test; telling it teaches the answer.

**Never ask it to justify its classification.** An expectation about what the
agent told the user cannot be measured from a report you asked for — you get the
sentence because you requested it. Read that from what the run said on its own.

## Expect a confirmation turn

Every scenario that crosses a guarded transition has one. Discovery confirms
scope before its mutation; authoring phases may write a draft first and then
confirm its approval. A first pass that stops at the applicable boundary is the
skill working. Judge the draft and guarded state appropriate to that phase
instead of assuming the whole repository must still be unchanged.

Answer as the maintainer would, then let it finish. Continue the same subagent
rather than starting a new one, so it keeps the state it gathered. The
confirm-then-mutate path is part of what the scenario tests, and it is where the
guarded CLI operations actually run.

**Confirm the phase, not the project.** Refer explicitly to what the agent just
presented and where to stop: "I approve the plan you just presented for Discovery
only. Stop after Discovery." or "I approve the Requirements and active Requirement
ID selection you just presented. Stop after Requirements." If an authoring phase
did not present every value its approval accepts, do not supply or infer the missing
value for it; the scenario has not reached an approvable boundary. A bare "Proceed
with that scope" can read as permission
to build the whole thing: one run answered that way went from discovery through
requirements, design, review, tasks, and implementation, which left every
discovery expectation unmeasurable because later phases had legitimately changed
the same files.

## Judge from the fixture, never from the report

Read the result with commands against the fixture:

```sh
specbind milestone status
specbind milestone scope
specbind spec list
specbind spec status <spec>
git status --short
```

A subagent's summary is a claim about what it did. It is usually accurate and it
is not evidence. Every expectation in the scenario document is readable from the
fixture; read it there.

## Ask for a usability debrief after judgment

Only after the fixture has been judged and the pass or failure recorded, ask the
same driver for a qualitative usability debrief. Asking earlier contaminates the
scenario: it teaches the agent which surfaces the maintainer cares about and can
turn a naturally discovered ambiguity into a prompted answer.

Record `git status --short` before the debrief. Then continue the same driver
with this read-only follow-up; do not start a fresh agent, because the useful
evidence is what this agent remembers hesitating over:

```text
The scenario result has already been judged. Do not run commands or change any
files. Reflect only on the work you just completed.

What made you hesitate, infer an unstated fact, take an unnecessary extra step,
or risk choosing the wrong action? Consider the CLI, skill, template, protocol,
adapter, diagnostics, and any other product surface that actually affected the
work. Do not invent an issue for every category. If there was no friction, say
"none".

For each observation, report only:
- Surface: CLI | Skill | Template | Protocol | Adapter | Other
- Friction: what was difficult or ambiguous
- Evidence: the exact command, output, wording, or path you encountered
- Workaround: what you inferred or did to continue
- Impact: cosmetic | extra-step | ambiguity | wrong-action-risk
```

After the answer, run `git status --short` again. A changed fixture means the
debrief violated its read-only boundary; discard the observation, classify the
mutation before cleanup, and do not fold it into the scenario result.

The debrief is not pass/fail evidence and does not overturn a result already
measured from the fixture. It is an exploratory observation. Record it
separately, reproduce it against the owning contract before changing the
product, and look for recurrence across scenarios or agents. One
`wrong-action-risk` is enough to investigate before the next batch; lower-impact
observations normally need either concrete reproduction or a repeated pattern.

Treat `docs/skill-forward-tests/findings.md` as a triaged worklist, not an
append-only transcript. Keep reproduced unresolved product findings, compact
resolved findings to their behavior change and fixing build, and retain
environment limitations only while they affect interpretation. Remove
duplicates, one-off non-defects, fixture-only workarounds, and `none`
observations from the worklist after the run itself records their disposition.

## When something fails

Decide which of these it is before changing anything:

- **The skill is wrong.** Fix the skill.
- **The decision is wrong.** Fix the decision first, then the skill. This is the
  common case when a scenario and an accepted decision disagree, and it is why
  the scenario document says to check the decision before editing a row.
- **The scenario is wrong.** Fix it, but only after confirming the decisions do
  not require what it asked for.
- **The harness is wrong.** Fix the fixture builder or this skill, and note that
  the scenario never actually ran.

Never adjust an expectation until it passes. A forward test tuned to pass has
stopped measuring anything.

A scenario that fails once and passes on retry is a finding, not a flake: the
skill is ambiguous enough for the agent to go either way, and that ambiguity is
the defect.

The same applies across agents. Both are handed the identical skill body —
rendering only rewrites Front Matter — so a scenario that passes under one and
fails under the other proves the document admits two readings. That is a skill
defect, and "the other agent behaves differently" restates it rather than
explains it.

## Finishing

Copy `docs/skill-forward-tests/run-template.md` to
`docs/skill-forward-tests/runs/YYYY-MM-DD-<driver>-<short-build>[-N].md`. One
record contains one driver and one tested build. Use `pass`,
`product_failure`, `scenario_invalid`, `environment_invalid`, or
`environment_blocked` per scenario, and record the expectation that did not
hold, the state left behind, and fixture evidence. A retry never replaces its
failed attempt.

Update `docs/skill-forward-tests/results.md` only with the current projection.
Give reproduced product findings stable `FT-NNNN` identities in
`docs/skill-forward-tests/findings.md`; use separate `ENV-NNNN` identities for
active environment limitations. Record non-findings and discarded observations
only in the run record. Then clean up the fixtures.

Record any defect the run exposed against the decision or skill it belongs to,
and update the scenario document when the run showed the procedure itself was
wrong. Both kinds of finding are normal; the runs that produce neither are the
ones worth being suspicious of.
