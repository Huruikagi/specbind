# Debug protocol

This protocol is the shared baseline for diagnosing a failure that stopped work.
It applies to every supported agent and cannot be waived by a project template
or shared rule.

It owns how a cause is established and what a diagnosis must contain. When
diagnosis is triggered, how many rounds are allowed, and who applies the fix
belong to the owning skills.

## You are diagnosing, not repairing

The diagnosis is the deliverable. Someone else applies the fix, from a context
that did not watch this failure happen.

Change nothing in the repository. A diagnosis that has already edited the code
leaves the next agent unable to tell which state it is reasoning about, and
destroys the evidence the next round would need.

## Fresh context is the point

You were given the failure and the inputs, and deliberately not the history of
attempts that failed.

That omission is the mechanism, not an oversight. A retry that inherits the
reasoning which just failed reliably reproduces it, and the most common shape of
a stuck loop is an agent re-deriving the same wrong model with growing
confidence. Reason from the evidence in front of you.

If the evidence is genuinely insufficient to reach a cause, say so and name what
would be sufficient. Do not fill the gap with an assumption about what was
probably tried.

## Establish the cause, not a plausible story

The first explanation that fits the symptom is usually a symptom of something
else.

- Reproduce or locate the failure precisely before explaining it. An error
  message names where something surfaced, not where it went wrong.
- Distinguish what the evidence shows from what you infer from it, and say which
  is which.
- Follow the failure to the point where the system's actual behavior first
  diverges from what the approved artifacts require. That divergence is the
  cause; everything after it is consequence.
- When two causes remain possible, state both and state what would distinguish
  them. A confident single answer that is wrong costs more than an honest fork.

A cause you cannot point at in the code, the configuration, the data, or the
artifacts is not yet a cause.

## The category changes who fixes it

Say which of these the failure is, because each has a different owner:

- **Implementation defect.** The code does not do what the Design requires. The
  fix belongs to the task.
- **Plan defect.** The task, its ordering, or its prerequisites are wrong. The
  fix belongs to the task plan.
- **Design or requirements defect.** The approved artifacts specify something
  that cannot work, or contradict each other. The fix belongs to that phase, and
  no amount of implementation effort substitutes.
- **Environment or dependency.** The system under test is not in the state the
  work assumes. The fix is usually outside the change entirely.

Misrouting here is expensive: an artifact defect handed back as an
implementation defect produces repeated attempts at work that cannot succeed.

## The diagnosis must be actionable without you

The report is read by an agent with no memory of this analysis. It contains:

- the failure, stated precisely enough to recognize
- the cause, with the specific location that supports it
- the category above
- a concrete next action for whoever owns that category
- what remains uncertain, when anything does

Where the next action is a code change, describe what must become true rather
than dictating a diff, unless the exact edit is itself the finding. The
implementer has context you do not.

## Say when it cannot be diagnosed

A failure you cannot explain from the available evidence is reported as such,
with what you ruled out and what evidence would be needed.

That is a useful result. A confident guess presented as a cause sends the next
round in a direction chosen by nothing, and it is indistinguishable from a real
finding by the time it fails.

Always end with this block. The caller parses the category, never the prose:

```text
## Diagnosis
- CATEGORY: IMPLEMENTATION | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED
- CAUSE: <what diverges, and where>
- NEXT_ACTION: <for whoever owns that category>
- UNCERTAIN: <what remains open, or none>
```

Use `UNDETERMINED` when the available evidence cannot establish a category. Put
the leading possibilities in `UNCERTAIN` and make `NEXT_ACTION` the
evidence-gathering step that distinguishes them. Do not guess a category merely
to fill the block.
