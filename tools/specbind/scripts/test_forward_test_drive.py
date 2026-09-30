"""Offline regression checks for the real-session harness; no model is called."""

import json
import os
from pathlib import Path
import shutil
import subprocess
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
                "pathlib.Path(args[args.index('-o') + 1]).write_text('Awaiting approval.\\n', encoding='utf-8')\n"
                "print(json.dumps({'type': 'thread.started', 'thread_id': 'test-thread'}))\n"
                "print(json.dumps({'type': 'item.completed', 'item': {'command': "
                + repr(command)
                + "}}))\n",
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
                self.assertEqual(args[args.index("-m") + 1], "gpt-5.6-terra")
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


if __name__ == "__main__":
    unittest.main()
