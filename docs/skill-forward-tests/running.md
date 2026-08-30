# Running skill forward tests

[Back to the forward-test index](../skill-forward-tests.md).

## The rule that makes this tractable

**Check artifacts and machine state. Never check prose.**

A run is non-deterministic in its path: two correct runs will word things
differently, ask different clarifying questions, and order their steps
differently. None of that is a result. What the run leaves behind is.

Every expectation below is something you can read with a command or a file, and
that a correct run must produce regardless of how it got there. If you find
yourself judging whether an explanation was well phrased, you have left the test
and started reviewing the writing.

A scenario passes when every listed expectation holds. It fails when one does
not — not when the agent took an unexpected route to satisfy them all.

## Setup

```bash
sh tools/specbind/scripts/forward-test-fixture.sh /tmp/specbind-fixture en
```

The script builds the release binary, creates a small real codebase, installs
SpecBind for both agents with project instructions, and seeds two steering
documents plus one established `cart` Spec. It refuses to touch an existing
directory; delete and rebuild between scenarios rather than reusing a dirtied
one.

The fixture carries the ordinary Python bytecode ignores (`__pycache__/` and
`*.py[cod]`). Disposable interpreter caches are not product findings merely
because the synthetic project omitted a normal repository baseline. Scenarios
still fail on tracked or otherwise unignored generated output.

Pass `ja` as the second argument to exercise the localized surface.

Scenarios that start from something further along have a recipe that builds and
verifies that state:

```bash
sh tools/specbind/scripts/forward-test-scenario.sh r4 /tmp/sb-r4 en
```

The recipe owns starting state only. The request and the expectations stay here.

Use a target path the agent session can address directly. On Windows a `/tmp`
path is a shell alias that some tools cannot resolve, and an agent that has to
guess at the real location is being tested on something other than the skill.

The script ships the release binary inside the fixture and prints the `export
PATH=` line for it. **Run that line before starting the session.** The skills
invoke `specbind` as a bare command because a real installed project has it on
PATH; a fixture without it tests whether the agent can guess an install location,
which is not what is under test. Two of the first runs stopped there and produced
no result at all.

Then start an agent session **with no prior context** in that directory. Context
carried from developing the skill is the most common way a forward test passes
for the wrong reason: the agent already knows what you meant.

### The dispatch log

For a run intended to prove fresh-context dispatch, add
`--instrument-dispatch` as the final fixture-builder or scenario-recipe
argument. Only those fixtures ask **every** context — the session you drive and
every subagent dispatched below it — to append the task it was given to
`.forward-test/agents.log` as its first project action.

That file is how dispatch becomes checkable state instead of a claim in the
run's narration:

| The log holds | The run |
| --- | --- |
| One line | Never dispatched. Everything happened in the driven context |
| N+1 lines | Dispatched N times |
| A line whose task only makes sense to someone who watched the parent | Dispatched a brief that does not stand alone |

The third row is the one that could not be measured at all before. Decision 0109
requires a dispatched brief to be self-contained, and until now the only evidence
was the parent's account of what it sent.

The directory is git-ignored, so the log never dirties the worktree and never
reaches a commit. Read it, do not clean it mid-scenario, and discard it with the
fixture. Leave instrumentation off for an ordinary workflow scenario: it makes
no dispatch claim, and an unrelated log write must not stop it before the
product workflow begins.

## Driving a run

A subagent works, and lets you pin the model. For a Codex-driven run, use the
following default profile:

```yaml
fork_turns: none
model: gpt-5.6-terra
reasoning_effort: medium
```

`fork_turns: none` supplies the clean context the test requires. The model and
reasoning effort are properties of the test driver, not of the installed product
skill. Override them only for an intentional model comparison, and record the
override with the result. A few rules keep the run honest.

**Give the request, never the method.** State the working directory, state that
`specbind` is on PATH, and then give the maintainer's request as a maintainer
would phrase it. Naming a skill or a command teaches the answer.

Every scenario carries that request as a quoted line:

```text
> Ask: carts should reject adding more than 99 of one SKU.
```

**Use it verbatim.** It is not a summary of what to convey — it is the message,
written once and checked once against the rule above, so that it does not have to
be improvised and re-risked on every run. Improvising is where a request quietly
acquires the method: naming the phase, naming the artifact, or describing the
shape of the answer.

Anything in *(parentheses and italics)* is an instruction to you, not text to
send — usually how to answer the confirmation this scenario depends on.

Everything else the run needs — the working directory, `specbind` on PATH, and
that the fixture stands alone — is setup you state around the quoted line, and is
the same for every scenario.

**Say the fixture stands alone.** A subagent inherits the host session's project
instructions rather than the fixture's, so this repository's rules about
answering in Japanese and committing to `main` travel with it. The checkpoint
scenarios measure whether the agent commits, so that inheritance can produce a
failure the skill did not cause.

**Expect a confirmation turn.** Every scenario that crosses a guarded transition
needs one. Discovery confirms scope before mutation; authoring phases may write a
draft first and then confirm approval. A stop at the applicable boundary is the
skill working, not failing. Answer by referring to what was just presented and
name the stopping point: "I approve the plan you just presented for Discovery
only. Stop after Discovery." or "I approve the Requirements and active Requirement
ID selection you just presented. Stop after Requirements." If the run did not
present every value its approval accepts, do not infer the missing value for it;
the scenario has not reached an approvable boundary. A bare "go ahead" can authorize the whole feature,
after which later phases legitimately rewrite the files an earlier scenario is
checking.

**Do not ask the agent to justify its classification.** Ask what it changed and
what it ran. An expectation about what the agent told the user cannot be measured
from a report you prompted for — you get the sentence because you asked, not
because the skill produced it. Read those expectations from the run's own output
instead.

Rebuild the fixture between scenarios. Several scenarios depend on the starting
state, and a leftover milestone from the previous one silently changes what is
being tested.

**A run can change your machine, not only the fixture.** The fixture bounds what
SpecBind touches; it does not bound the agent. One T2 run installed two Python
packages while diagnosing a YAML parse failure, which landed in the host
environment rather than under the target directory.

That is worth knowing in both directions. Treat it as ordinary agent behavior to
account for — check afterwards if it matters to you — and read it as a signal:
an agent reaching outside the project usually means a diagnostic inside the
project was not good enough, which is a finding about the skill or the CLI.

### Which agent is being driven

The fixture installs for both agents already — `install --agent claude-code
--agent codex` — so `.claude/skills/` and `.agents/skills/` are both present and
nothing in the setup changes between them. A Codex-spawned subagent measures
Codex; a Claude Code Agent-tool subagent measures Claude Code. Start the driver
for the agent you are measuring and leave the other tree alone.

Both agents read the **same skill body**. Rendering maps the declared metadata
onto each platform's Front Matter schema and never edits the document, so a run
under Codex and a run under Claude Code are given identical instructions.

The driving rules above apply unchanged, and the second one applies harder under
Codex. It inherits the host session's `AGENTS.md` rather than its `CLAUDE.md`,
which is this repository's own instruction file: the same rules about answering
in Japanese and committing to `main` travel with it by a different route. Say
the fixture stands alone regardless of which agent you drive.

When the driver is a Claude Code Agent-tool subagent, pick scenarios that do not
cross an approval. Such a subagent hears the driving session, not the user, and
refuses a relayed approval on the correct ground that another agent's message is
not the user's consent — so DS1, DS4, T1, T4 and every other authoring phase stop
with a correct draft and an unapproved gate. Measure those from a real session
started in the fixture directory. The same driver has no dispatch tool and does
not see the fixture's installed skills in its Skill registry; it reads
`SKILL.md` from disk, which is faithful to the document but leaves dispatch on
the main-context fallback.

Name the inherited rules when you say the fixture stands alone. "Instructions
from any other repository do not apply" was not enough in the 2026-08-29 batch:
two of six drivers still answered in Japanese against an `en` fixture. Adding
that this covers any rule about response language and about committing or
pushing stopped it.

### Driving a nested-dispatch run

`specbind-implement` dispatches subagents of its own, and `specbind-plan-design`
dispatches parallel investigation. Driving those with a subagent would nest one
inside another.

Prefer a real session started in the fixture directory when the driver cannot
nest subagents. A subagent driver is valid only when its platform supports the
product's nested dispatch and leaves enough capacity for it. In either case,
the dispatch log below is what proves the path actually ran.

**A run that could not dispatch still produces the right artifacts.** Decision
0109 gives dispatch a main-context fallback, which is correct for compatibility
and dangerous here: if dispatch silently fails, the run takes the fallback, the
files come out right, and every expectation passes without the dispatch path
ever executing. Their artifacts are identical, so no artifact separates them.

Read `.forward-test/agents.log` instead. One line means the run never
dispatched, however confidently it said otherwise; the count is how many fresh
contexts existed, and what each line says is whether the brief it received stood
on its own.

Record **which path the run took** alongside pass or fail. A pass by way of the
fallback is a pass for the workflow and **not** a measurement of dispatch, and
recording it as an unqualified pass makes the matrix claim coverage it does not
have.

## Recording a run

These are samples, not proofs. Copy the
[run template](./run-template.md) to
`runs/YYYY-MM-DD-<driver>-<short-build>[-N].md`. One file records one driver
against one tested build. A fix, rebuild, or driver change starts a new file.

Use one verdict for every scenario:

- `pass` when every expectation held in the judged fixture;
- `product_failure` when the judged fixture violated an expectation after
  confirming that the accepted Decisions require it;
- `scenario_invalid` when the recipe, precondition, expectation, or harness did
  not measure the intended product contract;
- `environment_invalid` when the driver environment did not exercise the
  installed product Skill;
- `environment_blocked` when the Skill ran but an external boundary prevented a
  verdict.

Record the expectation that did not hold, the fixture state left behind, and
concise mechanical evidence. The driver's report is not evidence. A retry gets
its own row or run record and never replaces the failed attempt.

After the run record is complete, update the
[measurement dashboard](./results.md) with only its current projection. Put a
reproduced product finding in the [findings worklist](./findings.md) under a
stable `FT-NNNN` identifier. Environment limitations use `ENV-NNNN` and do not
enter the product finding lifecycle.

A scenario that fails once and passes on retry is a finding, not a flake. The
skill is ambiguous enough that the agent can go either way, and the ambiguity is
the defect.

**A divergence between agents is a finding about the skill, not about the
agent.** It is the same rule one step out: both agents were handed the identical
body, so a scenario that passes under one and fails under the other proves the
document admits two readings. Fix the skill. "Codex does it differently" is a
restatement of the defect, not an explanation of it.

A scenario with no result for an agent has not been measured under it. There is
no blank row to fill in and no expectation that every scenario is eventually run
twice. The dashboard is a projection of observed evidence, not a checklist or a
claim about current `HEAD`.

### Post-run usability debrief

Pass or fail is established from the fixture, never from the driven agent's
report. After that judgment is recorded, continue the same agent once more for a
read-only usability debrief. Record `git status --short` before and after, and
discard the debrief if it changed the fixture.

Ask what made the agent hesitate, infer an unstated fact, take an unnecessary
extra step, or risk choosing the wrong action. Name CLI, Skill, Template,
Protocol, Adapter, diagnostics, and Other only as possible surfaces, not as a
request to manufacture one complaint per category. The agent runs no commands
and changes no files during this turn. Each observation uses this shape:

```text
- Surface: CLI | Skill | Template | Protocol | Adapter | Other
- Friction: what was difficult or ambiguous
- Evidence: the exact command, output, wording, or path encountered
- Workaround: what the agent inferred or did to continue
- Impact: cosmetic | extra-step | ambiguity | wrong-action-risk
```

These are qualitative observations, not scenario evidence and not accepted
product defects. Reproduce an observation against the owning Decision, CLI, or
asset before changing it. Repeated observations reveal systematic friction; a
single `wrong-action-risk` is enough to investigate before starting another
batch. Keep these observations out of the pass and no-passing-measurement
tables.

The findings worklist is triaged state, not an append-only debrief archive. The
run record retains the concise observation and its disposition:

- keep reproduced, unresolved product findings in the open table;
- move a fixed finding to the compact resolved table with its fixing build;
- retain an environment limitation only while it still affects interpretation;
- remove duplicates, one-off non-defects, fixture-only workarounds, and `none`
  observations after the run itself is recorded.

The current worklist should answer what remains actionable without requiring
readers to re-evaluate every past debrief.

### How much to re-run per agent

Run the complete set once under a newly supported agent, to find out where it
diverges. After that, a skill change needs re-running only the scenarios its
change can reach, under each agent that has a result to keep honest.

When only one agent can be run, prefer the scenarios where the two plausibly
differ: the ones that measure **stopping and confirmation** (D9, R3, R4, R5, DS3,
DS4, DS6), **whole-set reading** (D11, D12), and **checkpoint behavior** (C1, C2,
C3). Scenarios that only check the artifacts an authoring phase produced (R1, R2,
DS1, DS2) are the least agent-sensitive and a single sample covers them.
