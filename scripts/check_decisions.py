"""Validate Decision identifiers, headings, and the repository-map index."""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DECISIONS = ROOT / "docs" / "design" / "decisions"
REPOSITORY_MAP = ROOT / "docs" / "repository-map.md"

FILENAME = re.compile(r"^(?P<id>\d{4})-[a-z0-9][a-z0-9.-]*\.md$")
HEADING = re.compile(r"^# (?P<id>\d{4}): .+$")
INDEX_ROW = re.compile(
    r"^\| \[(?P<id>\d{4})\]\(\./design/decisions/(?P<name>[^)]+)\) \|"
)


def duplicates(values: dict[str, list[str]], label: str) -> list[str]:
    return [
        f"duplicate {label} {value}: {', '.join(paths)}"
        for value, paths in sorted(values.items())
        if len(paths) > 1
    ]


def main() -> int:
    errors: list[str] = []
    files_by_id: dict[str, list[str]] = defaultdict(list)
    file_ids: dict[str, str] = {}

    for path in sorted(DECISIONS.glob("*.md")):
        match = FILENAME.fullmatch(path.name)
        if match is None:
            errors.append(f"invalid Decision filename: {path.name}")
            continue
        decision_id = match.group("id")
        files_by_id[decision_id].append(path.name)
        file_ids[path.name] = decision_id

        lines = path.read_text(encoding="utf-8").splitlines()
        first_line = lines[0] if lines else ""
        heading = HEADING.fullmatch(first_line)
        if heading is None:
            errors.append(f"invalid Decision heading in {path.name}: {first_line!r}")
        elif heading.group("id") != decision_id:
            errors.append(
                f"Decision heading mismatch in {path.name}: {heading.group('id')}"
            )

    errors.extend(duplicates(files_by_id, "Decision id"))

    index_by_id: dict[str, list[str]] = defaultdict(list)
    index_by_name: dict[str, list[str]] = defaultdict(list)
    for line in REPOSITORY_MAP.read_text(encoding="utf-8").splitlines():
        match = INDEX_ROW.match(line)
        if match is None:
            continue
        decision_id = match.group("id")
        name = match.group("name")
        index_by_id[decision_id].append(name)
        index_by_name[name].append(decision_id)
        if file_ids.get(name) != decision_id:
            errors.append(f"repository map id/path mismatch: {decision_id} -> {name}")

    errors.extend(duplicates(index_by_id, "repository-map Decision id"))
    errors.extend(duplicates(index_by_name, "repository-map Decision path"))

    indexed = set(index_by_name)
    files = set(file_ids)
    for name in sorted(files - indexed):
        errors.append(f"Decision missing from repository map: {name}")
    for name in sorted(indexed - files):
        errors.append(f"repository map references a missing Decision: {name}")

    if errors:
        print("Decision validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(f"Decision validation passed: {len(files)} unique Decisions are indexed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
