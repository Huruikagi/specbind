# Report bugs and suggest improvements

Use the GitHub Issue Forms when you find a problem in the CLI, product-managed
Skills, Agent integration, installed files, or documentation, or when you want
to improve a workflow. The CLI can show the same entry points from inside or
outside a project:

```console
specbind feedback
```

The command prints the bug-report and improvement-form URLs, the information
to include, and publication cautions. It does not open a browser, access the
network, collect environment information, or transmit project content.

## Report a bug

Use [Bug report / バグ報告](https://github.com/Huruikagi/specbind/issues/new?template=bug-report.yml)
and include:

- a concise summary and the complete output of `specbind --version`;
- the affected area, operating environment, installation method, and relevant
  Agent or client versions;
- the relevant command, Skill, or installed file;
- minimal reproduction steps, expected behavior, and actual behavior; and
- observable evidence such as diagnostic codes, CLI output, generated
  artifacts, or a Git diff.

For a Skill problem, include the original request after removing confidential
information. Use `unknown` if you cannot identify the relevant command or
Skill. Review every line before publishing and remove tokens, user names,
private repository names, local paths, and private Spec or artifact content.

## Suggest an improvement

Use [Improvement proposal / 改善提案](https://github.com/Huruikagi/specbind/issues/new?template=improvement.yml).
Describe the affected area, current problem, workflow, and desired outcome.
You do not need to propose a command or implementation. Explain the problem
and usage context first, and add proposed behavior or workarounds only when
useful.

If neither form fits, you may open a blank Issue. Prefer the structured forms
for reproducible defects and product or workflow changes.
