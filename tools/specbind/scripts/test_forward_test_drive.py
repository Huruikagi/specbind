"""Offline regression checks for the real-session harness; no model is called."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


HARNESS = Path(__file__).with_name("forward-test-drive.sh").resolve()
SHELL = shutil.which("sh") or "C:/Program Files/Git/bin/sh.exe"


class CodexDriveTest(unittest.TestCase):
    def test_start_resume_isolation_and_boundary(self):
        with tempfile.TemporaryDirectory(prefix="sb-drive-test-") as scratch:
            root = Path(scratch)
            fixture = root / "fixture with spaces"
            bin_dir = fixture / ".specbind" / "bin"
            bin_dir.mkdir(parents=True)
            mock_dir = root / "mock"
            mock_dir.mkdir()
            for path, body in (
                (bin_dir / "specbind", "#!/bin/sh\necho 'specbind test'\n"),
                (mock_dir / "codex", '#!/bin/sh\nexec python "$(dirname "$0")/codex.py" "$@"\n'),
            ):
                path.write_text(body, encoding="utf-8")
                path.chmod(0o755)
            command = "Get-Content " + "long-native-path/" * 12 + ".agents/skills/sb-plan/SKILL.md"
            (mock_dir / "codex.py").write_text(
                "import json, os, pathlib, sys\n"
                "args = sys.argv[1:]\n"
                "if args == ['--version']:\n"
                "    print('codex-cli test'); sys.exit(0)\n"
                "with open(os.environ['FORWARD_TEST_CALLS'], 'a', encoding='utf-8') as f:\n"
                "    f.write(json.dumps({'args': args, 'message': sys.stdin.read(),\n"
                "        'cwd': os.getcwd(), 'thread_env': os.getenv('CODEX_THREAD_ID')}) + '\\n')\n"
                "outcome = os.getenv('FORWARD_TEST_OUTCOME', 'completed')\n"
                "pathlib.Path(args[args.index('-o') + 1]).write_text(\n"
                "    '' if outcome == 'empty' else 'Awaiting approval.\\n', encoding='utf-8')\n"
                "print(json.dumps({'type': 'thread.started', 'thread_id': 'test-thread'}))\n"
                "print(json.dumps({'type': 'item.completed', 'item': {'command': "
                + repr(command)
                + "}}))\n"
                "if outcome == 'failed':\n"
                "    print(json.dumps({'type': 'turn.failed', 'error': {'message': 'provider failed'}}))\n"
                "elif outcome != 'truncated':\n"
                "    print(json.dumps({'type': 'turn.completed'}))\n"
                "sys.exit(7 if outcome == 'nonzero' else 0)\n",
                encoding="utf-8",
            )
            calls_path = root / "calls.jsonl"
            env = dict(os.environ)
            env.update(
                PATH=str(mock_dir) + os.pathsep + env["PATH"],
                FORWARD_TEST_CALLS=str(calls_path),
                CODEX_THREAD_ID="host-thread-must-not-leak",
            )

            def run(*args):
                return subprocess.run(
                    [SHELL, str(HARNESS), *args], env=env, cwd=root,
                    text=True, encoding="utf-8", capture_output=True,
                )

            start = run("start", "codex", str(fixture), "write the requirements.")
            self.assertEqual(start.returncode, 0, start.stdout + start.stderr)
            state = Path(str(fixture) + ".drive")
            self.assertEqual((state / "session").read_text().strip(), "test-thread")
            digest = (state / "turns/01-actions.txt").read_text(encoding="utf-8")
            self.assertIn(command, digest)

            refused = run("send", str(fixture), "must not be sent", "--expect", "false")
            self.assertEqual(refused.returncode, 3, refused.stdout + refused.stderr)
            self.assertFalse((state / "turns/02-message.txt").exists())
            self.assertEqual(len(calls_path.read_text().splitlines()), 1)

            resume = run("send", str(fixture), "I approve. Stop after Requirements.", "--expect", "true")
            self.assertEqual(resume.returncode, 0, resume.stdout + resume.stderr)
            calls = [json.loads(line) for line in calls_path.read_text().splitlines()]
            self.assertEqual(len(calls), 2)
            self.assertEqual(calls[0]["args"][0], "exec")
            self.assertEqual(calls[1]["args"][:3], ["exec", "resume", "test-thread"])
            self.assertEqual(calls[0]["message"].strip(), "write the requirements.")
            self.assertEqual(calls[1]["message"].strip(), "I approve. Stop after Requirements.")
            for call in calls:
                self.assertTrue(os.path.samefile(call["cwd"], fixture))
                self.assertIsNone(call["thread_env"])
                args = call["args"]
                self.assertEqual(args[args.index("-m") + 1], "gpt-6.1-sol")
                configs = [args[i + 1] for i, arg in enumerate(args) if arg == "-c"]
                for expected in (
                    'sandbox_mode="workspace-write"',
                    'model_reasoning_effort="medium"',
                    "allow_login_shell=false",
                    "shell_environment_policy.experimental_use_profile=false",
                    "memories.use_memories=false",
                    "memories.generate_memories=false",
                ):
                    self.assertIn(expected, configs)

            # A zero process status cannot hide provider failure or an incomplete
            # stream. Exercise the shell's propagation on resumed turns too.
            for outcome, expected in (("failed", 4), ("truncated", 5), ("empty", 5), ("nonzero", 4)):
                with self.subTest(outcome=outcome):
                    env["FORWARD_TEST_OUTCOME"] = outcome
                    result = run("send", str(fixture), "Continue.", "--expect", "true")
                    self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
                    self.assertEqual((state / "session").read_text().strip(), "test-thread")


class ReportTest(unittest.TestCase):
    """Exercise saved provider streams without calling either model."""

    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="sb-report-test-")
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.fixture = self.root / "fixture"
        self.fixture.mkdir()
        self.state = self.root / "fixture.drive"
        self.turns = self.state / "turns"
        self.turns.mkdir(parents=True)
        self.events = [
            {"type": "thread.started", "thread_id": "fixture-thread"},
            {"type": "turn.started"},
            {"type": "turn.completed"},
        ]

    def run_report(self, *, agent="codex", reply="Awaiting approval.\n", extra=""):
        if reply is not None:
            (self.turns / "01-reply.txt").write_text(reply, encoding="utf-8")
        (self.turns / "01-events.jsonl").write_text(
            "".join(json.dumps(event) + "\n" for event in self.events) + extra,
            encoding="utf-8",
        )
        return subprocess.run(
            [sys.executable, str(HARNESS.with_name("forward_test_report.py")),
             agent, str(self.fixture), str(self.state), "01"],
            text=True, encoding="utf-8", capture_output=True,
        )

    def assert_status(self, result, expected, text):
        self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
        self.assertIn(text, result.stdout)

    def test_codex_completed_turn(self):
        self.assert_status(self.run_report(), 0, "Awaiting approval.")
        self.assertEqual((self.state / "session").read_text().strip(), "fixture-thread")

    def test_codex_failure_is_visible_even_with_a_reply(self):
        self.events[-1] = {"type": "turn.failed", "error": {"message": "model unavailable"}}
        self.assert_status(self.run_report(), 4, "model unavailable")

    def test_codex_truncated_stream_is_invalid(self):
        self.events.pop()
        self.assert_status(self.run_report(), 5, "no completed turn")

    def test_codex_empty_reply_is_invalid(self):
        self.assert_status(self.run_report(reply=" \n"), 5, "no final message")

    def test_codex_missing_reply_is_invalid(self):
        self.assert_status(self.run_report(reply=None), 5, "no final message")

    def test_codex_missing_thread_is_invalid(self):
        self.events.pop(0)
        self.assert_status(self.run_report(), 5, "no thread ID")

    def test_malformed_event_is_invalid(self):
        self.assert_status(self.run_report(extra='{"type":'), 5, "invalid event JSON")

    def test_non_object_event_is_invalid(self):
        self.events.append(["unexpected"])
        self.assert_status(self.run_report(), 5, "event must be an object")

    def test_codex_tool_failure_does_not_replace_turn_result(self):
        self.events.insert(2, {"type": "item.completed", "item": {
            "type": "command_execution", "command": "test command", "exit_code": 1,
        }})
        self.events.insert(2, {"type": "item.completed", "item": {
            "type": "error", "message": "nonfatal model metadata warning",
        }})
        self.assert_status(self.run_report(), 0, "nonfatal model metadata warning")
        self.assertIn("test command", (self.turns / "01-actions.txt").read_text())

    def claude_events(self):
        (self.fixture / ".claude/skills/sb-plan").mkdir(parents=True)
        self.events = [
            {"type": "system", "subtype": "init", "cwd": str(self.fixture),
             "skills": ["sb-plan"], "tools": ["Agent"], "memory_paths": {}},
            {"type": "result", "subtype": "success", "is_error": False,
             "result": "Awaiting approval.", "num_turns": 1},
        ]

    def test_claude_completed_turn(self):
        self.claude_events()
        self.assert_status(self.run_report(agent="claude-code", reply=None), 0, "Awaiting approval.")

    def test_claude_result_error_is_not_success(self):
        self.claude_events()
        self.events[-1].update(is_error=True, subtype="error_max_turns")
        self.assert_status(self.run_report(agent="claude-code"), 4, "error_max_turns")

    def test_claude_permission_denial_blocks_measurement(self):
        self.claude_events()
        self.events[-1]["permission_denials"] = [{"tool_name": "Bash", "tool_input": {"command": "test"}}]
        self.assert_status(self.run_report(agent="claude-code"), 4, "permission denied")

    def test_claude_registry_mismatch_is_invalid(self):
        self.claude_events()
        self.events[0]["skills"] = []
        self.assert_status(self.run_report(agent="claude-code"), 5, "absent from the session registry")

    def test_claude_missing_result_is_invalid(self):
        self.claude_events()
        self.events.pop()
        self.assert_status(self.run_report(agent="claude-code"), 5, "no result")

    def test_claude_wrong_directory_is_invalid(self):
        self.claude_events()
        self.events[0]["cwd"] = str(self.root)
        self.assert_status(self.run_report(agent="claude-code"), 5, "not the fixture")

    def test_claude_auto_memory_is_invalid(self):
        self.claude_events()
        self.events[0]["memory_paths"] = {"auto": "host-memory"}
        self.assert_status(self.run_report(agent="claude-code"), 5, "auto memory is enabled")

    def test_claude_missing_dispatch_is_invalid(self):
        self.claude_events()
        self.events[0]["tools"] = ["Bash", "Skill"]
        self.assert_status(self.run_report(agent="claude-code"), 5, "no subagent dispatch tool")


if __name__ == "__main__":
    unittest.main()
