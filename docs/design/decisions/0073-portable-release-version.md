# 0073: Use an opaque portable release-version label

Status: Accepted

## Context

Projects use SemVer, calendar versions, sequential release numbers, and other naming systems. SpecBind needs one release identity for roadmap binding, per-spec log labels, archive filenames, and links, but it should not assign project-specific version semantics. Because the value becomes part of a filename, an unrestricted string would create path traversal and cross-platform portability problems.

## Decision

- A non-null roadmap `target_release` is an opaque, case-sensitive ASCII string matching this exact grammar:

  ```regex
  ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$
  ```

- The value is therefore 1 through 64 ASCII characters. It begins with an ASCII letter or digit; subsequent characters may additionally contain `.`, `_`, `+`, or `-`.
- Valid examples include `v1.4.0`, `1.4.0`, `1.4.0-rc.1`, `1.4.0+build.7`, `2026-08-15`, and `release_42`.
- Values containing path separators, whitespace, `..` traversal as a complete segment, colons, shell metacharacters outside the grammar, non-ASCII text, or more than 64 characters are invalid. The grammar, rather than a separate blacklist, is authoritative.
- SpecBind does not parse or validate SemVer meaning, infer version ordering, add or remove a leading `v`, change case, trim a supplied value into validity, or otherwise normalize the label.
- The exact accepted value is used consistently as:
  - roadmap `target_release`
  - the release label in canonical per-spec `log.md` entries
  - `<version>` in `releases/<version>-roadmap.md`
  - `<version>` in `releases/<version>-cross-spec-review.md`
- Distinct exact strings such as `v1.4.0` and `1.4.0` are distinct release identities.
- Before binding, rebinding, preflight, or finalization, the CLI validates the grammar and returns `ERROR INVALID_RELEASE_VERSION` without mutation when it fails.
- Archive collision checks compare derived archive filenames using ASCII case-insensitive equality even on a case-sensitive filesystem. This prevents histories such as `v1.4.0` and `V1.4.0` from becoming non-portable to Windows. The stored release identities themselves remain case-sensitive and are not rewritten.
- The CLI continues to enforce path containment, symbolic-link safety, and actual filesystem collision checks independently of this lexical grammar.

## Consequences

- SpecBind supports common semantic, calendar, and project-specific release labels without taking ownership of their ordering rules.
- One exact label connects active binding, human-readable logs, archived artifacts, and links.
- Archive filenames remain safe and portable across supported development environments.
- Projects that need display names containing spaces or localized text keep those names in Markdown prose, not in `target_release`.
