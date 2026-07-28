from __future__ import annotations

import contextlib
import hashlib
import importlib.util
import io
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "workplan_lint_under_test", TOOLS / "workplan_lint.py"
)
assert SPEC and SPEC.loader
LINT = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = LINT
SPEC.loader.exec_module(LINT)


@contextlib.contextmanager
def working_directory(path: Path):
    previous = Path.cwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(previous)


class RepositoryFixture:
    def __init__(self, root: Path) -> None:
        self.root = root
        subprocess.run(["git", "init", "-q"], cwd=root, check=True)
        subprocess.run(
            ["git", "config", "user.email", "lint@example.invalid"],
            cwd=root,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "workplan lint test"],
            cwd=root,
            check=True,
        )
        self.write("AGENTS.md", "# Agents\n")
        self.write("docs/work-packages/AGENTS.md", "# Work Packages\n")
        self.write(
            "docs/standards/testing-and-gate-strategy.md", "# Testing Strategy\n"
        )
        self.write(
            "docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md",
            "# Advisory Linter\n",
        )
        self.write("src/allowed.py", "VALUE = 1\n")
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "base"], cwd=root, check=True)
        self.base = self.git("rev-parse", "HEAD")
        self.package = (
            "docs/work-packages/20260727-example-advisory-package-001/package.md"
        )
        self.write_package(intent=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "package"], cwd=root, check=True)

    def git(self, *args: str) -> str:
        return subprocess.run(
            ["git", *args],
            cwd=self.root,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()

    def write(self, relative: str, value: str) -> None:
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value, encoding="utf-8")

    def write_package(self, *, intent: bool) -> None:
        intent_text = "\n## Implementation Intent\n\nTooling.\n" if intent else ""
        self.write(
            self.package,
            f"""# Example Package

Package ID:
`20260727-example-advisory-package-001`

Base commit: `{self.base}`
{intent_text}
## Declared Write Set

- `src/allowed.py`
- `docs/**`
- `tools/**`
- this package subtree
""",
        )


def snapshot(root: Path) -> dict[str, tuple[int, int, int, int, int, int, str]]:
    values: dict[str, tuple[int, int, int, int, int, int, str]] = {}
    for path in sorted(root.rglob("*")):
        metadata = path.lstat()
        values[path.relative_to(root).as_posix()] = (
            stat.S_IMODE(metadata.st_mode),
            metadata.st_size,
            metadata.st_mtime_ns,
            metadata.st_ctime_ns,
            metadata.st_ino,
            metadata.st_nlink,
            hashlib.sha256(path.read_bytes()).hexdigest()
            if stat.S_ISREG(metadata.st_mode)
            else "",
        )
    return values


def allowlisted_suffixes(fixture: RepositoryFixture) -> tuple[tuple[str, ...], ...]:
    head = fixture.git("rev-parse", "HEAD")
    return (
        ("rev-parse", "--show-toplevel"),
        ("rev-parse", "--verify", "--end-of-options", "HEAD^{commit}"),
        ("merge-base", "--is-ancestor", fixture.base, head),
        ("status", "--porcelain=v2", "-z", "--untracked-files=all"),
        (
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--name-status",
            "-z",
            fixture.base,
            head,
            "--",
            ".",
        ),
    )


class WorkplanLintTest(unittest.TestCase):
    maxDiff = None

    def fixture(self):
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name).resolve()
        return temporary, RepositoryFixture(root)

    def test_all_modes_are_complete_deterministic_and_non_mutating(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        fixture.write("src/allowed.py", "VALUE = 2\n")
        fixture.write("notes.txt", "outside\n")
        with working_directory(fixture.root):
            for mode in LINT.MODES:
                before = snapshot(fixture.root)
                first = LINT.analyze(fixture.package, mode)
                second = LINT.analyze(fixture.package, mode)
                after = snapshot(fixture.root)
                self.assertEqual(first, second)
                self.assertEqual(before, after)
                self.assertEqual(first["analysis_status"], "complete")
                self.assertEqual(first["mode"], mode)
                self.assertRegex(first["base_sha"], r"^[0-9a-f]{40}$")
                self.assertRegex(first["head_sha"], r"^[0-9a-f]{40}$")
                expected_scope = mode != "pre-edit"
                self.assertEqual(first["observed_scope"]["index"], expected_scope)
                self.assertEqual(first["observed_scope"]["worktree"], expected_scope)
                self.assertEqual(first["observed_scope"]["untracked"], expected_scope)
            working = LINT.analyze(fixture.package, "working-tree")
        self.assertTrue(
            any(
                item["category"] == "scope-mismatch"
                and "notes.txt" in item["message"]
                for item in working["findings"]
            )
        )

    def test_terminal_observes_base_to_head_and_dirty_paths(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        fixture.write("src/allowed.py", "VALUE = 3\n")
        fixture.write("outside.rs", "fn main() {}\n")
        subprocess.run(["git", "add", "."], cwd=fixture.root, check=True)
        subprocess.run(["git", "commit", "-qm", "terminal changes"], cwd=fixture.root, check=True)
        fixture.write("dirty.md", "# Dirty\n")
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "terminal")
        messages = [item["message"] for item in result["findings"]]
        self.assertTrue(any("outside.rs" in message for message in messages))
        self.assertTrue(any("dirty.md" in message for message in messages))
        self.assertIn("OBL-RUST-001", {item["rule_id"] for item in result["findings"]})
        self.assertIn("OBL-DOC-001", {item["rule_id"] for item in result["findings"]})

    def test_findings_do_not_change_success_exit(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        fixture.write("outside.py", "print('advice only')\n")
        output = io.StringIO()
        with working_directory(fixture.root), contextlib.redirect_stdout(output):
            code = LINT.main(
                ["--package", fixture.package, "--mode", "working-tree", "--format", "json"]
            )
        value = json.loads(output.getvalue())
        self.assertEqual(code, 0)
        self.assertEqual(value["analysis_status"], "complete")
        self.assertTrue(value["findings"])
        self.assertNotIn("HOLD", output.getvalue())
        self.assertNotIn("BLOCKED", output.getvalue())

    def test_schema_and_human_output_are_exact_and_advisory(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "pre-edit")
        self.assertEqual(
            set(result),
            {
                "schema_version",
                "analysis_status",
                "mode",
                "repository_root",
                "package",
                "base_sha",
                "head_sha",
                "observed_scope",
                "policy_inputs",
                "unavailable_analyses",
                "findings",
                "error",
            },
        )
        self.assertEqual(result["schema_version"], "1.0.0")
        text = LINT.human_output(result)
        self.assertIn("advisory analysis", text)
        self.assertIn("grants no permission or lifecycle status", text)
        for item in result["findings"]:
            self.assertEqual(
                set(item),
                {
                    "rule_id",
                    "category",
                    "confidence",
                    "impact",
                    "action",
                    "message",
                    "observed_location",
                    "governing_source",
                    "applicability",
                    "reasoning",
                    "suggested_command",
                },
            )

    def test_missing_intent_is_advice(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        fixture.write_package(intent=False)
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "working-tree")
        self.assertIn("WP-INTENT-001", {item["rule_id"] for item in result["findings"]})
        self.assertEqual(result["analysis_status"], "complete")

    def test_injected_git_failure_is_partial_and_manual_route_survives(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)

        def fail(_root: Path, _suffix):
            raise LINT.AnalysisUnavailable(
                "git", "INJECTED_FAILURE", "test-only read failure"
            )

        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "terminal", runner=fail)
        self.assertEqual(result["analysis_status"], "partial")
        self.assertEqual(result["unavailable_analyses"][0]["reason_code"], "INJECTED_FAILURE")
        self.assertTrue(result["package"])
        self.assertEqual(len(result["policy_inputs"]), 4)

    def test_partial_cli_exits_three_and_misuse_exits_two(self) -> None:
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            code = LINT.main(["--format", "json"])
        value = json.loads(output.getvalue())
        self.assertEqual(code, 2)
        self.assertIsNone(value["analysis_status"])
        self.assertEqual(value["error"]["code"], "INVOCATION_MISUSE")
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            code = LINT.main(["--mode", "terminal", "--format", "json"])
        value = json.loads(output.getvalue())
        self.assertEqual(code, 2)
        self.assertEqual(value["mode"], "terminal")
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            code = LINT.main(["--format=json", "--package=x", "--mode=invalid"])
        value = json.loads(output.getvalue())
        self.assertEqual(code, 2)
        self.assertEqual(value["error"]["code"], "INVOCATION_MISUSE")

    def test_nonrepository_is_unavailable_with_one_json_envelope(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = io.StringIO()
            with working_directory(Path(temporary)), contextlib.redirect_stdout(output):
                code = LINT.main(
                    [
                        "--package",
                        "docs/work-packages/example/package.md",
                        "--mode",
                        "pre-edit",
                        "--format",
                        "json",
                    ]
                )
        lines = output.getvalue().splitlines()
        self.assertEqual(code, 3)
        self.assertEqual(len(lines), 1)
        value = json.loads(lines[0])
        self.assertEqual(value["analysis_status"], "unavailable")
        self.assertEqual(value["error"]["code"], "NOT_A_REPOSITORY")

    def test_unexpected_internal_failure_is_unavailable_not_a_crash(self) -> None:
        output = io.StringIO()
        with patch.object(LINT, "analyze", side_effect=RuntimeError("injected")), (
            contextlib.redirect_stdout(output)
        ):
            code = LINT.main(
                [
                    "--package",
                    "docs/work-packages/example/package.md",
                    "--mode",
                    "terminal",
                    "--format",
                    "json",
                ]
            )
        value = json.loads(output.getvalue())
        self.assertEqual(code, 3)
        self.assertEqual(value["analysis_status"], "unavailable")
        self.assertEqual(value["error"]["code"], "INTERNAL_ERROR")
        self.assertNotIn("injected", output.getvalue())

    def test_package_symlink_and_oversize_fail_without_following(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        package = fixture.root / fixture.package
        target = package.with_name("real.md")
        package.rename(target)
        package.symlink_to(target)
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "pre-edit")
        self.assertEqual(result["analysis_status"], "partial")
        self.assertEqual(
            result["unavailable_analyses"][0]["reason_code"], "OPEN_FAILED"
        )

    def test_revision_and_path_operands_are_conservative(self) -> None:
        for value in ("", "-HEAD", "../HEAD", "HEAD $(touch x)", "a" * 201):
            with self.assertRaises(LINT.InvocationError):
                LINT.validated_revision(value)
        for value in ("", "-path", "../path", "/absolute", "a\\b"):
            with self.assertRaises(LINT.InvocationError):
                LINT.normalize_relative(value, label="test")

    def test_porcelain_rename_and_unmerged_records_preserve_exact_paths(self) -> None:
        rename = (
            b"2 R. N... 100644 100644 100644 "
            + b"0" * 40
            + b" "
            + b"1" * 40
            + b" R100 new/path.py\0old/outside.py\0"
        )
        self.assertEqual(
            LINT.parse_status(rename), ("new/path.py", "old/outside.py")
        )
        unmerged = (
            b"u UU N... 100644 100644 100644 100644 "
            + b"0" * 40
            + b" "
            + b"1" * 40
            + b" "
            + b"2" * 40
            + b" conflict/file.py\0"
        )
        self.assertEqual(LINT.parse_status(unmerged), ("conflict/file.py",))

    def test_missing_ambiguous_and_mismatched_identity_are_findings(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        package = fixture.root / fixture.package
        original = package.read_text(encoding="utf-8")
        package.write_text(
            original.replace(
                "Package ID:\n`20260727-example-advisory-package-001`\n", ""
            ).replace(f"Base commit: `{fixture.base}`", ""),
            encoding="utf-8",
        )
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "pre-edit")
        rules = {item["rule_id"] for item in result["findings"]}
        self.assertIn("WP-IDENTITY-PACKAGE_ID_MISSING", rules)
        self.assertIn("WP-IDENTITY-BASE_MISSING", rules)
        self.assertEqual(result["analysis_status"], "partial")
        package.write_text(
            original.replace(
                "`20260727-example-advisory-package-001`", "`wrong-package-id`", 1
            )
            + f"\nBase commit: `{fixture.base}`\n",
            encoding="utf-8",
        )
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "pre-edit")
        rules = {item["rule_id"] for item in result["findings"]}
        self.assertIn("WP-IDENTITY-PACKAGE_ID_PATH_MISMATCH", rules)
        self.assertIn("WP-IDENTITY-BASE_AMBIGUOUS", rules)

    def test_detached_head_is_reported(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        fixture.git("checkout", "--detach", "-q")
        with working_directory(fixture.root):
            result = LINT.analyze(fixture.package, "pre-edit")
        self.assertIn(
            "WP-IDENTITY-DETACHED-HEAD",
            {item["rule_id"] for item in result["findings"]},
        )

    def test_exact_git_argv_environment_and_closed_stdin(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        invocations = (
            (("rev-parse", "--show-toplevel"), f"{fixture.root}\n".encode()),
            (
                ("rev-parse", "--verify", "--end-of-options", "HEAD^{commit}"),
                (fixture.git("rev-parse", "HEAD") + "\n").encode(),
            ),
            (
                (
                    "merge-base",
                    "--is-ancestor",
                    fixture.base,
                    fixture.git("rev-parse", "HEAD"),
                ),
                b"",
            ),
            (
                ("status", "--porcelain=v2", "-z", "--untracked-files=all"),
                b"",
            ),
            (
                (
                    "diff",
                    "--no-ext-diff",
                    "--no-textconv",
                    "--name-status",
                    "-z",
                    fixture.base,
                    fixture.git("rev-parse", "HEAD"),
                    "--",
                    ".",
                ),
                b"",
            ),
        )

        class Process:
            def __init__(self, expected, output, argv, **kwargs):
                self.returncode = 0
                self.expected = expected
                self.output = output
                self.assertion = (argv, kwargs)

        for suffix, output in invocations:
            holder = {}

            def factory(argv, **kwargs):
                process = Process(suffix, output, argv, **kwargs)
                holder["process"] = process
                return process

            with patch.object(LINT.subprocess, "Popen", side_effect=factory), patch.object(
                LINT, "bounded_communicate", return_value=(output, b"")
            ):
                observed = LINT.default_runner(fixture.root, suffix)
            process = holder["process"]
            argv, kwargs = process.assertion
            self.assertEqual(tuple(argv), (*LINT.GIT_PREFIX, *suffix))
            self.assertEqual(kwargs["env"], LINT.GIT_ENV)
            self.assertIs(kwargs["stdin"], subprocess.DEVNULL)
            self.assertEqual(observed, output)

    def test_nonallowlisted_git_argv_never_launches(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        for suffix in (
            ("status", "--short"),
            ("log", "--oneline"),
            ("diff", "--name-only"),
            ("rev-parse", "--verify", "--end-of-options", "-bad^{commit}"),
        ):
            with self.subTest(suffix=suffix), patch.object(
                LINT.subprocess, "Popen"
            ) as process:
                with self.assertRaises(
                    (LINT.AnalysisUnavailable, LINT.InvocationError)
                ):
                    LINT.default_runner(fixture.root, suffix)
                process.assert_not_called()

    def test_git_timeout_and_output_limit_are_unavailable(self) -> None:
        process = subprocess.Popen(
            [sys.executable, "-c", "import time; time.sleep(2)"],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        with patch.object(LINT, "GIT_TIMEOUT_SECONDS", 0.05):
            with self.assertRaises(LINT.AnalysisUnavailable) as raised:
                LINT.bounded_communicate(process)
        self.assertEqual(raised.exception.reason_code, "GIT_TIMEOUT")
        self.assertIsNotNone(process.returncode)

        process = subprocess.Popen(
            [sys.executable, "-c", "import sys; sys.stdout.write('x' * 4096)"],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        with patch.object(LINT, "MAX_GIT_BYTES", 128):
            with self.assertRaises(LINT.AnalysisUnavailable) as raised:
                LINT.bounded_communicate(process)
        self.assertEqual(raised.exception.reason_code, "GIT_OUTPUT_LIMIT")

    def test_executable_entrypoint_outputs_one_json_document(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        result = subprocess.run(
            [
                str(TOOLS / "workplan-lint"),
                "--package",
                fixture.package,
                "--mode",
                "pre-edit",
                "--format",
                "json",
            ],
            cwd=fixture.root,
            env={"PATH": "/usr/bin:/bin"},
            capture_output=True,
            text=True,
            check=True,
        )
        value = json.loads(result.stdout)
        self.assertEqual(value["analysis_status"], "complete")
        self.assertEqual(result.stderr, "")

    def test_every_prohibited_config_class_refuses_before_process_launch(self) -> None:
        cases = (
            '[include]\npath = /tmp/other\n',
            '[includeIf "gitdir:/tmp"]\npath = /tmp/other\n',
            '[core]\nfsmonitor = /tmp/helper\n',
            '[core]\nhooksPath = /tmp/hooks\n',
            '[core]\npager = /tmp/helper\n',
            '[core]\nattributesFile = /tmp/attributes\n',
            '[core] # accepted trailing comment\npager = /tmp/helper\n',
            '[include] ; accepted trailing comment\npath = /tmp/other\n',
            '[includeIf\t"gitdir:/tmp"]\npath = /tmp/other\n',
            '[alias\t"x"]\nfoo = !/tmp/helper\n',
            '[pager\t"x"]\nstatus = /tmp/helper\n',
            '[maintenance "daily"]\ncommand = /tmp/helper\n',
            '[alias]\nstatus = !/tmp/helper\n',
            '[pager]\nstatus = /tmp/helper\n',
            '[credential]\nhelper = /tmp/helper\n',
            '[url "ssh://example/"]\ninsteadOf = local:\n',
            '[diff]\nexternal = /tmp/helper\n',
            '[diff "x"]\ntextconv = /tmp/helper\n',
            '[filter "x"]\nclean = /tmp/helper\nsmudge = /tmp/helper\nprocess = /tmp/helper\n',
        )
        for addition in cases:
            with self.subTest(addition=addition):
                temporary, fixture = self.fixture()
                try:
                    suffixes = allowlisted_suffixes(fixture)
                    with (fixture.root / ".git/config").open("a", encoding="utf-8") as stream:
                        stream.write(addition)
                    with patch.object(LINT.subprocess, "Popen") as process:
                        for suffix in suffixes:
                            with self.assertRaises(LINT.AnalysisUnavailable) as raised:
                                LINT.default_runner(fixture.root, suffix)
                    process.assert_not_called()
                    self.assertEqual(
                        raised.exception.reason_code,
                        "PROHIBITED_GIT_CONFIGURATION",
                    )
                finally:
                    temporary.cleanup()

    def test_every_prohibited_attribute_class_refuses_before_launch(self) -> None:
        cases = (
            "*.txt diff=driver\n",
            "*.txt filter=driver\n",
            "*.txt textconv=helper\n",
            "*.txt clean=helper\n",
            "*.txt smudge=helper\n",
            "*.txt process=helper\n",
        )
        for value in cases:
            with self.subTest(value=value):
                temporary, fixture = self.fixture()
                try:
                    suffixes = allowlisted_suffixes(fixture)
                    fixture.write(".gitattributes", value)
                    with patch.object(LINT.subprocess, "Popen") as process:
                        for suffix in suffixes:
                            with self.assertRaises(LINT.AnalysisUnavailable):
                                LINT.default_runner(fixture.root, suffix)
                    process.assert_not_called()
                finally:
                    temporary.cleanup()

    def test_git_info_attributes_refuses_every_allowlisted_invocation(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        info_attributes = fixture.root / ".git/info/attributes"
        info_attributes.parent.mkdir(parents=True, exist_ok=True)
        info_attributes.write_text("*.txt filter=driver\n", encoding="utf-8")
        suffixes = allowlisted_suffixes(fixture)
        with patch.object(LINT.subprocess, "Popen") as process:
            for suffix in suffixes:
                with self.assertRaises(LINT.AnalysisUnavailable):
                    LINT.default_runner(fixture.root, suffix)
        process.assert_not_called()

    def test_nested_attributes_are_never_pruned_before_launch(self) -> None:
        for directory in (
            ".venv",
            ".mypy_cache",
            ".pytest_cache",
            "__pycache__",
            "node_modules",
            "target",
            "build",
            "dist",
        ):
            with self.subTest(directory=directory):
                temporary, fixture = self.fixture()
                try:
                    fixture.write(f"{directory}/.gitattributes", "*.txt filter=driver\n")
                    with patch.object(LINT.subprocess, "Popen") as process:
                        with self.assertRaises(LINT.AnalysisUnavailable):
                            LINT.default_runner(
                                fixture.root, ("rev-parse", "--show-toplevel")
                            )
                    process.assert_not_called()
                finally:
                    temporary.cleanup()

    def test_helper_and_network_canaries_are_never_executed(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        canary = fixture.root / "helper-ran"
        helper = fixture.root / "helper.sh"
        helper.write_text(f"#!/bin/sh\nprintf ran > {canary}\n")
        helper.chmod(0o755)
        with (fixture.root / ".git/config").open("a", encoding="utf-8") as stream:
            stream.write(
                f'\n[credential]\nhelper = {helper}\n'
                f'[url "https://127.0.0.1:1/"]\ninsteadOf = local:\n'
            )
        with patch.object(LINT.subprocess, "Popen") as process:
            with self.assertRaises(LINT.AnalysisUnavailable):
                LINT.default_runner(fixture.root, ("rev-parse", "--show-toplevel"))
        process.assert_not_called()
        self.assertFalse(canary.exists())

    @unittest.skipUnless(shutil.which("strace"), "strace is required for network proof")
    def test_all_modes_make_no_network_system_call(self) -> None:
        temporary, fixture = self.fixture()
        self.addCleanup(temporary.cleanup)
        with tempfile.TemporaryDirectory() as trace_directory:
            for mode in LINT.MODES:
                trace = Path(trace_directory) / f"{mode}.trace"
                result = subprocess.run(
                    [
                        shutil.which("strace") or "/usr/bin/strace",
                        "-f",
                        "-e",
                        "trace=network",
                        "-o",
                        str(trace),
                        str(TOOLS / "workplan-lint"),
                        "--package",
                        fixture.package,
                        "--mode",
                        mode,
                        "--format",
                        "json",
                    ],
                    cwd=fixture.root,
                    env={"PATH": "/usr/bin:/bin"},
                    capture_output=True,
                    text=True,
                    check=True,
                )
                self.assertEqual(json.loads(result.stdout)["analysis_status"], "complete")
                network_trace = trace.read_text(encoding="utf-8")
                self.assertNotRegex(network_trace, r"socket\(AF_INET6?\b")
                self.assertNotRegex(
                    network_trace,
                    r"\b(sendto|recvfrom)\(.*(AF_INET|sin_addr|sin6_addr)",
                )

    def test_product_source_has_one_subprocess_surface_and_no_legacy_import(self) -> None:
        source = (TOOLS / "workplan_lint.py").read_text(encoding="utf-8")
        entrypoint = (TOOLS / "workplan-lint").read_text(encoding="utf-8")
        self.assertEqual(source.count("subprocess.Popen("), 1)
        self.assertIn("sys.dont_write_bytecode = True", entrypoint)
        for forbidden in (
            "openwepp_gate_planner",
            "openwepp-gate-planner",
            "testgate",
            "receipt",
            "ledger",
            "attestation",
            "requests.",
            "urllib.",
            "shell=True",
            "os.system",
        ):
            self.assertNotIn(forbidden, source.lower())


if __name__ == "__main__":
    unittest.main()
