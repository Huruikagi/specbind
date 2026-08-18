---
name: specbind-forward-test
description: Run the behavioral forward tests for SpecBind's embedded skills against a fixture project, using subagents with no prior context. Use when a product-managed skill changed materially, before a release, or when a decision's effect on agent behavior needs checking.
---

# Run the skill forward tests

The scenarios and their expectations live in
[docs/skill-forward-tests.md](../../../docs/skill-forward-tests.md). That
document is the contract; this skill is how to execute it without rediscovering
the operational traps each time.

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

Several scenarios need a precondition set up first — an uncommitted edit, a
broken steering document. **Do that from the shell**, not from a helper script in
another language. On Windows the shell maps `/tmp` to a real directory, but a
native interpreter invoked from it does not, so a script that opens
`/tmp/sb-<scenario>/...` fails while every shell command beside it succeeds. A
precondition that silently did not apply turns a scenario into a different one.
Verify the precondition with a command before launching the run.

## Driving a run

Use a subagent with no prior context. Pin the model when comparing behavior
across models — the Agent tool's `model` accepts `sonnet`, `opus`, `haiku`, and
`fable`.

The prompt gives three things and nothing else:

1. the working directory, in a form the agent's file tools can address
2. the `export PATH=` line, stated as an environment fact
3. the maintainer's request, phrased the way a maintainer would phrase it

Then ask only what it changed and what it ran.

**Never name a skill or a command in the prompt.** Whether the agent finds and
uses the installed skill is the thing under test; telling it teaches the answer.

**Never ask it to justify its classification.** An expectation about what the
agent told the user cannot be measured from a report you asked for — you get the
sentence because you requested it. Read that from what the run said on its own.

## Expect a confirmation turn

Every scenario ending in a mutation has one. The skills confirm scope before
changing anything, so a first pass correctly stops with a proposal and an
unchanged repository. That is the skill working.

Answer as the maintainer would, then let it finish. Continue the same subagent
rather than starting a new one, so it keeps the state it gathered. The
confirm-then-mutate path is part of what the scenario tests, and it is where the
guarded CLI operations actually run.

**Confirm the phase, not the project.** Say what the scenario needs confirmed and
where to stop. "Proceed with that scope" reads as permission to build the whole
thing: one run answered that way went from discovery through requirements,
design, review, tasks, and implementation, which left every discovery expectation
unmeasurable because later phases had legitimately changed the same files.

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

## Finishing

Report per scenario: pass or fail, the expectation that did not hold, and the
state that was left behind. Then clean up the fixtures.

Record any defect the run exposed against the decision or skill it belongs to,
and update the scenario document when the run showed the procedure itself was
wrong. Both kinds of finding are normal; the runs that produce neither are the
ones worth being suspicious of.
