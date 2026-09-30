#!/usr/bin/env sh
# Drives a forward-test scenario through a real top-level agent session.
#
# An Agent-tool subagent is not a real session: it does not see the fixture's
# installed Skills (ENV-0001), cannot dispatch the product's own subagents, and
# correctly refuses an approval relayed by the session that spawned it
# (ENV-0003). This harness instead starts the agent's own non-interactive CLI in
# the fixture directory and resumes the same session for every later turn, so
# each message arrives on the user channel of a top-level session whose project
# is the fixture.
#
# Usage:
#   forward-test-drive.sh start <claude-code|codex> <fixture> <message|->
#                               [--model <model>] [--effort <level>]
#   forward-test-drive.sh send  <fixture> <message|-> [--expect <command>]...
#
# `-` reads the message from standard input. Every `--expect` command runs in
# the fixture with the fixture CLI first on PATH before the message is sent; if
# any fails, nothing is sent. Use it to prove the run stopped at the boundary a
# confirmation is about to answer.
#
# Turn records are written beside the fixture, in `<fixture>.drive/`, so they
# never appear in the fixture's own Git status.
#
# Exit status: 0 turn completed; 1 usage or harness error; 3 an `--expect`
# precondition failed and nothing was sent; 4 the agent run failed; 5 the
# session environment did not measure the installed product (environment
# invalid).
#
# See docs/skill-forward-tests/running.md for how to use the result.

set -eu

usage() {
    sed -n '/^# Usage:/,/^# `-` reads/p' "$0" | sed '$d; s/^# \{0,1\}//' >&2
    exit 1
}

fail() {
    echo "forward-test-drive: $1" >&2
    exit 1
}

python_runner() {
    for candidate in python py python3; do
        if "$candidate" -c "import sys; sys.exit(0)" >/dev/null 2>&1; then
            echo "$candidate"
            return 0
        fi
    done
    fail "no working Python interpreter found"
}

# The native path of a directory. On Windows a native interpreter does not
# resolve the shell's `/tmp` mapping, so Python must be handed `pwd -W`.
native_dir() {
    ( cd "$1" && { pwd -W 2>/dev/null || pwd; } )
}

read_message() {
    if [ "$1" = - ]; then
        cat
    else
        printf '%s\n' "$1"
    fi
}

# Host-session variables leak identity, nesting limits, and extra instruction
# directories into a child agent. The first probe of this harness, run from a
# Claude Code session, resumed the host's own session ID. Strip every agent
# variable except those that only select credentials, a provider, or a
# platform executable.
scrubbed_env_args() {
    env | grep -E '^(CLAUDECODE|CLAUDE_[A-Z0-9_]*|CCR_[A-Z0-9_]*|CODEX_[A-Z0-9_]*)=' | cut -d= -f1 |
        grep -v -x -e CLAUDE_CONFIG_DIR -e 'CLAUDE_CODE_USE_[A-Z0-9_]*' \
            -e CLAUDE_CODE_OAUTH_TOKEN -e 'CLAUDE_CODE_SKIP_[A-Z0-9_]*_AUTH' \
            -e CLAUDE_CODE_GIT_BASH_PATH -e CODEX_HOME |
        sort -u || true
}

next_turn() {
    count=$(find "$state/turns" -name '*-message.txt' | wc -l | tr -d ' ')
    printf '%02d' $((count + 1))
}

# Runs one turn. Arguments: turn number, message file.
run_turn() {
    turn=$1
    message_file=$2
    events="$state/turns/$turn-events.jsonl"
    stderr_file="$state/turns/$turn-stderr.txt"
    reply="$state/turns/$turn-reply.txt"
    agent=$(cat "$state/agent")
    session=$(cat "$state/session" 2>/dev/null || true)
    model=$(cat "$state/model")
    effort=$(cat "$state/effort")

    set --
    for name in $(scrubbed_env_args); do
        set -- "$@" -u "$name"
    done
    set -- env "$@" PATH="$fixture/.specbind/bin:$PATH"

    case "$agent" in
    claude-code)
        # Project and local settings only: user-level CLAUDE.md, hooks, and
        # settings belong to the maintainer's machine, not the fixture. Auto
        # memory would carry one run into the next at the same fixture path.
        set -- "$@" claude -p --output-format stream-json --verbose \
            --setting-sources project,local \
            --settings '{"autoMemoryEnabled":false}' \
            --permission-mode acceptEdits --allowedTools "Bash,Skill"
        [ -n "$model" ] && set -- "$@" --model "$model"
        [ -n "$effort" ] && set -- "$@" --effort "$effort"
        if [ "$turn" = 01 ]; then
            set -- "$@" --session-id "$session"
        else
            set -- "$@" --resume "$session"
        fi
        ;;
    codex)
        set -- "$@" codex exec
        [ "$turn" = 01 ] || set -- "$@" resume "$session"
        set -- "$@" --json -o "$reply" -c 'sandbox_mode="workspace-write"'
        [ -n "$model" ] && set -- "$@" -m "$model"
        [ -n "$effort" ] && set -- "$@" -c "model_reasoning_effort=\"$effort\""
        set -- "$@" -
        ;;
    esac

    status=0
    ( cd "$fixture" && "$@" <"$message_file" >"$events" 2>"$stderr_file" ) || status=$?

    runner=$(python_runner)
    report_status=0
    "$runner" - "$agent" "$(native_dir "$fixture")" "$(native_dir "$state")" "$turn" <<'PY' || report_status=$?
import json, os, sys

agent, fixture, state, turn = sys.argv[1:5]
turns = os.path.join(state, "turns")
events = []
with open(os.path.join(turns, f"{turn}-events.jsonl"), encoding="utf-8") as stream:
    for line in stream:
        line = line.strip()
        if line.startswith("{"):
            try:
                events.append(json.loads(line))
            except ValueError:
                pass

def short(value, limit=160):
    text = " ".join(str(value).split())
    return text if len(text) <= limit else text[: limit - 1] + "…"

actions, problems, facts = [], [], []
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
        if result.get("is_error"):
            facts.append(f"result error: {result.get('subtype')}")
        for denial in result.get("permission_denials") or []:
            facts.append("permission denied: " + short(f"{denial.get('tool_name')} {json.dumps(denial.get('tool_input'))}"))
    with open(reply_path, "w", encoding="utf-8") as stream:
        stream.write(text + ("\n" if text and not text.endswith("\n") else ""))
else:
    # The Codex JSONL event shape is read defensively: only the thread ID and
    # executed commands are extracted, and the reply comes from `-o`.
    for event in events:
        thread = event.get("thread_id") or event.get("session_id")
        if thread:
            facts.append(f"thread: {thread}")
            break
    for event in events:
        item = event.get("item") or {}
        if event.get("type") == "item.completed" and item.get("command"):
            actions.append(f"command: {short(item.get('command'))}")
    if not os.path.exists(reply_path):
        open(reply_path, "w", encoding="utf-8").close()
        problems.append("the run wrote no final message")

with open(os.path.join(turns, f"{turn}-actions.txt"), "w", encoding="utf-8") as stream:
    stream.writelines(a + "\n" for a in actions)
if agent == "codex" and turn == "01":
    thread = next((f.split(": ", 1)[1] for f in facts if f.startswith("thread: ")), "")
    with open(os.path.join(state, "session"), "w", encoding="utf-8") as stream:
        stream.write(thread + "\n")

print(f"== turn {turn} reply")
with open(reply_path, encoding="utf-8") as stream:
    sys.stdout.write(stream.read())
print(f"== turn {turn} session")
for fact in facts:
    print(f"  {fact}")
print(f"  actions: {len(actions)} (see {turn}-actions.txt)")
for problem in problems:
    print(f"  ENVIRONMENT INVALID: {problem}")
sys.exit(5 if problems else 0)
PY

    if [ "$status" -ne 0 ]; then
        echo "forward-test-drive: the agent exited $status; see $stderr_file" >&2
        exit 4
    fi
    [ "$report_status" -eq 0 ] || exit "$report_status"
    [ "$agent" != codex ] || [ -n "$(cat "$state/session")" ] ||
        fail "the Codex run reported no thread ID; the session cannot be resumed"
}

action=${1:-}
[ -n "$action" ] || usage
shift

case "$action" in
start)
    [ $# -ge 3 ] || usage
    agent=$1
    fixture_arg=$2
    message=$3
    shift 3
    model=
    effort=
    while [ $# -gt 0 ]; do
        case "$1" in
        --model) [ $# -ge 2 ] || usage; model=$2; shift 2 ;;
        --effort) [ $# -ge 2 ] || usage; effort=$2; shift 2 ;;
        *) usage ;;
        esac
    done
    case "$agent" in
    claude-code)
        command -v claude >/dev/null 2>&1 || fail "claude is not on PATH" ;;
    codex)
        command -v codex >/dev/null 2>&1 || fail "codex is not on PATH"
        # The default forward-test driver profile for Codex.
        [ -n "$model" ] || model=gpt-5.6-terra
        [ -n "$effort" ] || effort=medium ;;
    *) fail "unknown agent: $agent (expected claude-code or codex)" ;;
    esac
    [ -d "$fixture_arg" ] || fail "$fixture_arg is not a fixture directory"
    fixture=$(CDPATH= cd -- "$fixture_arg" && pwd)
    [ -x "$fixture/.specbind/bin/specbind" ] || [ -x "$fixture/.specbind/bin/specbind.exe" ] ||
        fail "$fixture has no fixture CLI under .specbind/bin"
    state="$fixture.drive"
    [ ! -e "$state" ] || fail "$state already exists; one session per fixture, rebuild the fixture"

    mkdir -p "$state/turns"
    printf '%s\n' "$agent" >"$state/agent"
    printf '%s\n' "$fixture" >"$state/fixture"
    printf '%s\n' "$model" >"$state/model"
    printf '%s\n' "$effort" >"$state/effort"
    if [ "$agent" = claude-code ]; then
        "$(python_runner)" -c "import uuid; print(uuid.uuid4())" >"$state/session"
        agent_version=$(claude --version 2>/dev/null || echo unknown)
    else
        : >"$state/session"
        agent_version=$(codex --version 2>/dev/null || echo unknown)
    fi
    specbind_version=$(PATH="$fixture/.specbind/bin:$PATH" specbind --version 2>/dev/null || echo unknown)
    {
        echo "agent: $agent ($agent_version)"
        echo "requested model: ${model:-agent default}"
        echo "requested effort: ${effort:-agent default}"
        echo "fixture CLI: $specbind_version"
        echo "started: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    } >"$state/meta.txt"

    turn=01
    read_message "$message" >"$state/turns/$turn-message.txt"
    run_turn "$turn" "$state/turns/$turn-message.txt"
    ;;
send)
    [ $# -ge 2 ] || usage
    fixture_arg=$1
    message=$2
    shift 2
    [ -d "$fixture_arg" ] || fail "$fixture_arg is not a fixture directory"
    fixture=$(CDPATH= cd -- "$fixture_arg" && pwd)
    state="$fixture.drive"
    [ -s "$state/session" ] || fail "no started session for $fixture; use start first"

    message_file="$state/pending-message.txt"
    read_message "$message" >"$message_file"
    while [ $# -gt 0 ]; do
        [ "$1" = --expect ] && [ $# -ge 2 ] || usage
        if ( cd "$fixture" && PATH="$fixture/.specbind/bin:$PATH" && export PATH &&
            eval "$2" ) >/dev/null 2>&1; then
            echo "  PASS $2"
        else
            echo "  FAIL $2" >&2
            rm -f "$message_file"
            echo "forward-test-drive: the boundary precondition did not hold; nothing was sent" >&2
            exit 3
        fi
        shift 2
    done

    turn=$(next_turn)
    mv "$message_file" "$state/turns/$turn-message.txt"
    run_turn "$turn" "$state/turns/$turn-message.txt"
    ;;
*)
    usage
    ;;
esac
