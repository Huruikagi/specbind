# 0190: Resolve concrete paths against Contract File Ownership

Status: Accepted

## Context

Discovery enters the SpecBind workflow when a request modifies a path declared
under a persistent Spec's Contract File Ownership. That condition is intended
to be a mechanical check, but the Discovery Skill currently reads candidate
Contracts and interprets exact-path and terminal-`/**` declarations itself.

The Contract graph read model already loads every valid persistent Contract,
normalizes File Ownership paths for portable ASCII-case-insensitive comparison,
and detects overlaps. Leaving lookup in prose duplicates that deterministic
model and lets different Agents disagree about whether an owned path enters the
workflow.

The CLI must not turn this lookup into a semantic ownership verdict. A path may
be unowned because the Contract is incomplete, and a request may change an
existing Spec's behavior without naming any concrete path. New responsibility,
behavioral impact, scope decomposition, and ambiguous ownership remain Agent
judgments.

## Decision

### Read-only command

The Contract inspection family adds:

```text
specbind contract owners <path>
```

`<path>` is one concrete project-relative portable path. It uses `/`
separators, contains no root, drive prefix, `.` or `..` segment, and accepts no
wildcard. A terminal `/**` remains declaration-only Contract syntax and is
invalid as a query.

The command resolves the complete current persistent Contract graph and fails
closed with the existing `CONTRACT_GRAPH_READ_FAILED` result when that graph is
incomplete. An invalid query fails with
`CONTRACT_OWNERS_PATH_INVALID`. It does not require an active Milestone and
never inspects whether the queried filesystem path exists or is changed.

Every File Ownership declaration whose exact path or terminal-`/**` subtree
contains the query is returned with its canonical Spec, entry ID, and declared
path. Matching is ASCII-case-insensitive, like the accepted portable Contract
path comparison. Results use deterministic Contract graph order.

Zero matches is successful and explicitly reports `Owners: none`. The result
reports `Ambiguous across Specs: yes` exactly when declarations from more than
one canonical Spec match. Several matching declarations in one Spec retain
their distinct entry evidence but are not cross-Spec ambiguity.

Success uses `CONTRACT_OWNERS_REPORTED` and remains text-first. This demonstrated
consumer does not add JSON or a general response format.

### Discovery consumption

When a change request supplies a concrete project-relative path, project
routing and `sb-discovery` run `contract owners` rather than reimplementing File
Ownership matching. Any returned owner mechanically establishes that the path
is inside a managed boundary and the request enters the workflow. Cross-Spec
ambiguity makes every returned Contract a candidate input and prevents silent
single-owner selection.

When no pending Roadmap item already matches the request, that entry result
routes to Discovery and its scope-confirmation boundary before any source or
lifecycle mutation. Imperative wording and an explicitly named file authorize
the lookup and classification; they do not independently authorize
implementation or create a Direct item.

No match proves only that current Contracts declare no owner for that path. It
does not prove that the request is ordinary work: Discovery still judges
observable behavior, existing responsibility, new durable responsibility,
active-Milestone framing, and other request evidence. When the request supplies
no concrete path, the Agent does not infer one merely to call the command.

The CLI never classifies the work as Direct, Existing Spec update, or New Spec,
and never claims semantic impact. Those decisions still require Steering,
Requirements, Contract meaning, and maintainer confirmation.

## Consequences

- Exact and subtree File Ownership matching has one product implementation.
- Discovery receives precise candidate Specs without scanning every Contract
  or treating a partial graph as complete.
- A named-file implementation request cannot bypass Discovery merely because
  its requested edit is precise.
- An unowned path remains visible without being misrepresented as permission to
  bypass Discovery.
- Contract declarations remain the only persistent ownership source; the query
  creates no index or lifecycle state.

## Verification

CLI tests cover exact, subtree, ASCII-case-insensitive, zero-match, cross-Spec
ambiguous, invalid-path, and incomplete-graph results. Skill and project-
instruction tests require the command for explicit concrete paths and retain
semantic classification in Discovery. A fresh Discovery forward test verifies
that a natural request naming an owned path reaches the existing Spec through
the projection without teaching the command in the prompt.
