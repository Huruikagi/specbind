"""Summarize one saved agent turn; never infer a scenario pass from a reply."""

import json
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")

agent, fixture, state, turn = sys.argv[1:5]
turns = os.path.join(state, "turns")
events, problems = [], []
with open(os.path.join(turns, f"{turn}-events.jsonl"), encoding="utf-8") as stream:
    for line_number, line in enumerate(stream, 1):
        line = line.strip()
        if not line:
            continue
        try:
            event = json.loads(line)
        except ValueError:
            problems.append(f"invalid event JSON at line {line_number}")
            continue
        if not isinstance(event, dict):
            problems.append(f"event must be an object at line {line_number}")
            continue
        events.append(event)

def short(value, limit=160):
    text = " ".join(str(value).split())
    return text if len(text) <= limit else text[: limit - 1] + "…"

actions, facts, failures = [], [], []
reply_path = os.path.join(turns, f"{turn}-reply.txt")

if agent == "claude-code":
    init = next((e for e in events if e.get("type") == "system" and e.get("subtype") == "init"), None)
    result = next((e for e in reversed(events) if e.get("type") == "result"), None)
    if init is None:
        problems.append("no session init event was recorded")
    else:
        facts.append(f"model: {init.get('model')}")
        facts.append(f"permission mode: {init.get('permissionMode')}")
        same = lambda path: os.path.normcase(os.path.realpath(path))
        if same(init.get("cwd", "")) != same(fixture):
            problems.append(f"session cwd is {init.get('cwd')}, not the fixture")
        skills_dir = os.path.join(fixture, ".claude", "skills")
        installed = sorted(os.listdir(skills_dir)) if os.path.isdir(skills_dir) else []
        missing = [s for s in installed if s not in init.get("skills", [])]
        if not installed:
            problems.append("the fixture has no installed Claude Code Skills")
        elif missing:
            problems.append("installed Skills absent from the session registry: " + ", ".join(missing))
        if not {"Task", "Agent"} & set(init.get("tools", [])):
            problems.append("the session has no subagent dispatch tool")
        if (init.get("memory_paths") or {}).get("auto"):
            problems.append("auto memory is enabled for the session")
    for event in events:
        if event.get("type") != "assistant":
            continue
        for block in event.get("message", {}).get("content", []):
            if block.get("type") != "tool_use":
                continue
            name, arguments = block.get("name"), block.get("input") or {}
            detail = (arguments.get("command") or arguments.get("skill")
                      or arguments.get("file_path") or arguments.get("description")
                      or arguments.get("pattern") or "")
            actions.append(f"{name}: {short(detail)}")
    if result is None:
        problems.append("the run recorded no result")
        text = ""
    else:
        text = result.get("result") or ""
        facts.append(f"agent turns: {result.get('num_turns')}")
        if result.get("is_error") or str(result.get("subtype", "")).startswith("error"):
            failures.append(f"result error: {result.get('subtype')}")
            failures.extend(short(error, 1000) for error in result.get("errors") or [])
        for denial in result.get("permission_denials") or []:
            failures.append("permission denied: " + short(f"{denial.get('tool_name')} {json.dumps(denial.get('tool_input'))}"))
    with open(reply_path, "w", encoding="utf-8") as stream:
        stream.write(text + ("\n" if text and not text.endswith("\n") else ""))
else:
    # The reply comes from `-o`, but its existence alone cannot prove that the
    # turn finished. Preserve provider failures and reject truncated streams.
    for event in events:
        thread = event.get("thread_id") or event.get("session_id")
        if thread:
            facts.append(f"thread: {thread}")
            break
    completed = False
    for event in events:
        if event.get("type") == "turn.completed":
            completed = True
        elif event.get("type") == "turn.failed":
            failures.append("turn failed: " + short(event.get("error") or "unspecified error", 1000))
        elif event.get("type") == "error":
            facts.append("provider error: " + short(event.get("message"), 1000))
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("type") == "error":
            # Metadata warnings are not failed turns. Report without overriding
            # a later successful turn completion.
            facts.append("item error: " + short(item.get("message"), 1000))
        if event.get("type") == "item.completed" and item.get("command"):
            # Native Windows shell paths can consume the entire short() limit
            # before the Skill path or lifecycle command becomes visible.
            actions.append("command: " + " ".join(item["command"].split()))
    if not completed and not failures:
        problems.append("the run recorded no completed turn")

with open(os.path.join(turns, f"{turn}-actions.txt"), "w", encoding="utf-8") as stream:
    stream.writelines(a + "\n" for a in actions)
if agent == "codex" and turn == "01":
    thread = next((f.split(": ", 1)[1] for f in facts if f.startswith("thread: ")), "")
    if thread:
        with open(os.path.join(state, "session"), "w", encoding="utf-8") as stream:
            stream.write(thread + "\n")
    else:
        problems.append("the run reported no thread ID")

reply = ""
if os.path.exists(reply_path):
    with open(reply_path, encoding="utf-8") as stream:
        reply = stream.read()
if not reply.strip() and not failures:
    problems.append("the run wrote no final message")

print(f"== turn {turn} reply")
print(reply, end="" if reply.endswith("\n") or not reply else "\n")
print(f"== turn {turn} session")
for fact in facts:
    print(f"  {fact}")
print(f"  actions: {len(actions)} (see {turn}-actions.txt)")
for failure in failures:
    print(f"  AGENT RUN FAILED: {failure}")
for problem in problems:
    print(f"  ENVIRONMENT INVALID: {problem}")
# An explicit provider failure takes precedence over incomplete evidence.
sys.exit(4 if failures else 5 if problems else 0)
