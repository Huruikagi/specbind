# 0148: Route CLI feedback to structured public Issue Forms

Status: Accepted

## Context

SpecBind exposes stable diagnostics and a published user guide, but the CLI has
no discoverable route for reporting incorrect behavior or proposing an
improvement. The repository also has no Issue templates, so reports can omit
the version, command, environment, reproduction steps, or distinction between
the expected and actual result.

Adding a URL to every error would enlarge output consumed by agents and scripts
and would couple unrelated stable diagnostics to the feedback channel. Opening
a browser, collecting environment data, or submitting a report from the CLI
would introduce platform behavior, networking, consent, and private-project
data concerns that are unnecessary for an initial entry point.

The public command name becomes part of the same-major compatibility surface
under Decision 0144. Its boundary must therefore be explicit before release.

## Decision

### One discoverable CLI entry point

The public command is:

```text
specbind feedback
```

The root `specbind --help` footer points users to that command. Individual
success, no-change, and error results do not repeat the feedback route.

`specbind feedback`:

- succeeds without resolving a SpecBind project;
- returns stable result code `OK FEEDBACK_REPORTED`;
- prints separate direct links for a bug report and an improvement proposal;
- asks the user to include `specbind --version`, the command, and complete
  output;
- warns the user to remove secrets and private project content;
- explicitly states that it transmitted no information; and
- does not open a browser, access the network, inspect the project, collect
  environment data, write a file, or use the clipboard.

The command's English text follows the existing concise CLI output contract.
The public Japanese guide explains the same behavior and privacy boundary.

### Two bilingual Issue Forms

The GitHub repository provides two recommended forms:

- **Bug report / バグ報告** requires a summary, exact SpecBind version,
  environment, installation method, command, reproduction steps, expected
  behavior, actual behavior, and complete sanitized output.
- **Improvement proposal / 改善提案** requires the current problem, workflow
  area, and desired outcome. A proposed implementation and current workaround
  remain optional so users can report friction without designing the product.

Both forms accept Japanese or English and require confirmation that the user
searched existing Issues and removed secrets and private project content.
Blank Issues remain enabled so questions and reports that do not fit either
form are not silently excluded.

The CLI links directly to the form templates instead of the chooser. The
repository chooser remains useful for visitors arriving through GitHub.

## Consequences

- Users can discover a reporting route from the binary without a configured
  project.
- Bug reports receive the mechanical context needed to reproduce failures,
  while improvement proposals remain problem-first.
- CLI output stays deterministic and offline, and SpecBind never implies that
  it submitted a report.
- The repository URL and Issue template filenames embedded in the CLI are
  coordinated product URLs. Moving the repository or renaming a template
  requires updating the source, focused CLI tests, Decision, and user guide
  together.
- Automated diagnostic bundles, browser opening, private security reporting,
  and telemetry require separate decisions if introduced later.

## Implementation status

Implemented by the root help footer, `specbind feedback`, focused CLI tests,
bilingual GitHub Issue Forms, and the Japanese feedback guide.
