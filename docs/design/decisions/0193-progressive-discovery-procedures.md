# 0193: Load Discovery procedures progressively

Status: Accepted

## Context

`sb-discovery` is the common entry point before a request has a confirmed
SpecBind boundary. Its entrypoint had accumulated the complete ordinary
Discovery procedure alongside routing to reverse establishment and two Source
Collection providers. Selecting Discovery therefore loaded more than five
hundred lines before the request's mode was known, even though reverse runs do
not use the ordinary procedure and ordinary conversational requests use no
provider.

The procedural detail remains necessary. Compressing its confirmation,
ownership, mutation, Brief, or checkpoint rules would reopen behavioral
ambiguity already covered by forward tests. The issue is when that detail is
loaded, not whether the contract exists.

## Decision

The `sb-discovery` package adds `references/ordinary.md`. The entrypoint retains
only language-style consumption, mode and provider selection, and the shared
authority boundary.

It routes references as follows:

- an explicit request to establish Specs from existing code or resume an active
  reverse establishment reads `references/reverse.md` and does not read the
  ordinary procedure;
- every other request reads `references/ordinary.md`;
- an ordinary request with an explicit local file or directory additionally
  reads `references/local-files.md`;
- an ordinary request with explicit GitHub repository and Milestone identities
  additionally reads `references/github-milestone.md`.

An ordinary conversational request loads no provider reference. An invalid or
unavailable explicit provider selector stops rather than falling back to a
different provider or conversational Discovery.

The ordinary reference retains the existing entry, ownership, classification,
confirmation, mutation, Brief, checkpoint, and reporting contract. The Source
Collection references point to that procedure for shared classification rules.
This decision changes package loading and ownership, not lifecycle semantics or
authorization.

## Consequences

- Automatic Discovery selection loads a small router before any substantial
  mode-specific procedure.
- Reverse establishment no longer loads ordinary mutation instructions it must
  not execute.
- Ordinary conversational Discovery no longer loads acquisition procedures for
  absent Source Collections.
- The package gains one managed file while keeping each workflow obligation in
  one place.

## Verification

Catalog tests require all four references, require the entrypoint to route each
one directly, and keep ordinary behavioral invariants against
`references/ordinary.md`. Installation tests cover the additional managed file
for Codex and Claude Code. A fresh ordinary Discovery forward test confirms that
the installed router selects and completes the ordinary procedure without
loading an unrelated mode.
