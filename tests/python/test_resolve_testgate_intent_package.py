from __future__ import annotations

import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).parents[2]
SPEC = importlib.util.spec_from_file_location(
    "resolve_testgate_intent_package",
    ROOT / "tools/local_ci/resolve_testgate_intent_package.py",
)
assert SPEC is not None and SPEC.loader is not None
RESOLVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RESOLVER)

PACKAGE = "docs/work-packages/20260720-testgate-recovery-trust-001/package.md"


class IntentPackageFixture:
    def __init__(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.git("init", "-q")
        self.git("config", "user.email", "test@example.invalid")
        self.git("config", "user.name", "TESTGATE fixture")

    def close(self) -> None:
        self.temporary.cleanup()

    def git(self, *arguments: str) -> str:
        result = subprocess.run(
            ["git", *arguments],
            cwd=self.root,
            check=True,
            capture_output=True,
            text=True,
        )
        return result.stdout.strip()

    def commit(self, message: str) -> str:
        (self.root / "subject").write_text(message, encoding="utf-8")
        self.git("add", "subject")
        self.git("commit", "-q", "-m", message)
        return self.git("rev-parse", "HEAD")


class ResolveTestgateIntentPackageTests(unittest.TestCase):
    def setUp(self) -> None:
        self.fixture = IntentPackageFixture()

    def tearDown(self) -> None:
        self.fixture.close()

    def test_push_reads_exactly_one_head_trailer(self) -> None:
        head = self.fixture.commit(
            f"qualified increment\n\nTESTGATE-Intent-Package: {PACKAGE}"
        )
        self.assertEqual(
            RESOLVER.resolve(self.fixture.root, "push", head, ""),
            PACKAGE,
        )

    def test_push_rejects_missing_trailer(self) -> None:
        head = self.fixture.commit("missing declaration")
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "exactly one"
        ):
            RESOLVER.resolve(self.fixture.root, "push", head, "")

    def test_push_rejects_duplicate_trailers(self) -> None:
        head = self.fixture.commit(
            "duplicate\n\n"
            f"TESTGATE-Intent-Package: {PACKAGE}\n"
            f"TESTGATE-Intent-Package: {PACKAGE}"
        )
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "exactly one"
        ):
            RESOLVER.resolve(self.fixture.root, "push", head, "")

    def test_push_rejects_manual_input(self) -> None:
        head = self.fixture.commit(
            f"qualified increment\n\nTESTGATE-Intent-Package: {PACKAGE}"
        )
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "exact head commit"
        ):
            RESOLVER.resolve(self.fixture.root, "push", head, PACKAGE)

    def test_dispatch_requires_explicit_input(self) -> None:
        head = self.fixture.commit("manual")
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "requires an explicit"
        ):
            RESOLVER.resolve(
                self.fixture.root,
                "workflow_dispatch",
                head,
                "",
            )

    def test_dispatch_uses_explicit_input(self) -> None:
        head = self.fixture.commit("manual")
        self.assertEqual(
            RESOLVER.resolve(
                self.fixture.root,
                "workflow_dispatch",
                head,
                PACKAGE,
            ),
            PACKAGE,
        )

    def test_malformed_package_path_fails_closed(self) -> None:
        head = self.fixture.commit(
            "malformed\n\nTESTGATE-Intent-Package: ../package.md"
        )
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "invalid intent package"
        ):
            RESOLVER.resolve(self.fixture.root, "push", head, "")

    def test_dot_components_inside_package_prefix_fail_closed(self) -> None:
        head = self.fixture.commit("dot components")
        for package in [
            "docs/work-packages/../package.md",
            "docs/work-packages/./package.md",
        ]:
            with self.subTest(package=package), self.assertRaisesRegex(
                RESOLVER.IntentPackageError, "invalid intent package"
            ):
                RESOLVER.resolve(
                    self.fixture.root,
                    "workflow_dispatch",
                    head,
                    package,
                )

    def test_push_rejects_dot_component_inside_package_prefix(self) -> None:
        head = self.fixture.commit(
            "dot component\n\n"
            "TESTGATE-Intent-Package: docs/work-packages/../package.md"
        )
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "invalid intent package"
        ):
            RESOLVER.resolve(self.fixture.root, "push", head, "")

    def test_dispatch_rejects_output_line_injection(self) -> None:
        head = self.fixture.commit("output injection")
        for separator in ["\n", "\r"]:
            package = f"docs/work-packages/owner{separator}injected=value/package.md"
            with self.subTest(separator=repr(separator)), self.assertRaisesRegex(
                RESOLVER.IntentPackageError, "invalid intent package"
            ):
                RESOLVER.resolve(
                    self.fixture.root,
                    "workflow_dispatch",
                    head,
                    package,
                )

    def test_unsupported_event_fails_closed(self) -> None:
        head = self.fixture.commit("unsupported")
        with self.assertRaisesRegex(
            RESOLVER.IntentPackageError, "unsupported trusted event"
        ):
            RESOLVER.resolve(self.fixture.root, "pull_request", head, "")


if __name__ == "__main__":
    unittest.main()
