# 0211: Plan project migrations across executable version boundaries

Status: Accepted

## Context

Issue #54 needs cumulative project migration instructions with mechanical
applicability and completion checks, separate from installation-client updates
and the historical `migrate cc-sdd` command.

## Decision

`migration plan --from <semver> [--to <semver>] [--json]` is read-only. The default
target is the running binary's version. Strict SemVer precedence selects entries
whose introduction boundary is in `(from, to]`. Build metadata does not affect
precedence; prereleases sort before their corresponding stable release. Equal
precedence yields an empty plan; downgrade requests fail. A target beyond the
catalog's known coverage fails rather than claiming future migration completeness.

Entries have stable IDs, source version ranges, introduction boundaries,
required/recommended/optional classification, deterministic-CLI/agent-procedure/
manual-stop handlers, stable procedure selectors, preview, verification, and
idempotency instructions. Catalog validation rejects duplicate identities,
invalid version ranges, missing procedure metadata, and unsafe same-major
required rewrites. Entries are ordered by boundary then ID; targets by path.
Source ranges use the non-empty form `>=minimum, <boundary`, with full SemVer
versions. Source eligibility uses the same precedence comparison (including
prereleases), not a package manager's implicit prerelease exclusion.

The first boundary is the planned additive release `1.5.0`: recommended
Requirements, Design, and Steering description reconciliation, including owned
templates. This boundary must match the release that first ships these changes.
Explicit `--to 1.5.0` permits prerelease development verification; this does not
publish or change the executable version. Later releases retain cumulative entries.

Dedicated probes run before ordinary configuration/artifact loading. They inspect
only a guarded regular `.specbind.json` for a safe relative `specDir`, then guarded
Markdown under the relevant live/template roots. They do not require the current
configuration schema, language, Spec state, or Gate model. Missing descriptions
are pending targets; invalid or ambiguous inputs block a trustworthy probe.
Supported documents with descriptions are complete; an absent applicable artifact
family is not applicable. No ledger records completion: every run rechecks current
files, so interruption resumes from remaining targets and later regressions reopen.

Same-major compatibility follows Decision 0144. Cross-major entries can describe
old inputs through dedicated probes without the current loader; absent explicit
catalog coverage for a major boundary is an error, never an empty success.
Synthetic catalog fixtures exercise required cross-major manual and CLI routes;
they are not invented production migrations.

`sb-configure` records the original version before updating through the installation
client, obtains the plan with the new binary, refreshes managed assets, reloads its
new package, and executes only authorized procedures. Persist the original `from`
version in the handoff for interrupted runs; never infer it from the new binary.
Recommended reconciliation requires its own confirmed preview. Requirements and
Design changes retain authoring, approval, and freshness boundaries; Steering stays
under configuration ownership. Re-run the same plan before completion and report
pending/blocked required entries separately from declined recommendations.

## Consequences

Planning never mutates files, installs binaries, or approves content. A required
pending entry is a successful plan result but prevents reporting migration work
complete. Probe uncertainty returns an error with available plan detail. Only
changes needing concrete project work belong in the catalog.
