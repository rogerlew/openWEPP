from __future__ import annotations

import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).parents[2]
SPEC = importlib.util.spec_from_file_location(
    "resolve_testgate_comparison_base",
    ROOT / "tools/local_ci/resolve_testgate_comparison_base.py",
)
assert SPEC is not None and SPEC.loader is not None
RESOLVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RESOLVER)


class ComparisonBaseFixture:
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
        subject = self.root / "subject"
        subject.write_text(f"{message}\n", encoding="utf-8")
        self.git("add", "subject")
        self.git("commit", "-q", "-m", message)
        return self.git("rev-parse", "HEAD")


class ResolveTestgateComparisonBaseTests(unittest.TestCase):
    def setUp(self) -> None:
        self.fixture = ComparisonBaseFixture()
        self.anchor = self.fixture.commit("anchor")
        self.before = self.fixture.commit("before")

    def tearDown(self) -> None:
        self.fixture.close()

    def resolve(
        self,
        head: str,
        *,
        event_name: str = "push",
        event_before: str | None = None,
        input_base: str = "",
    ) -> str:
        return RESOLVER.resolve(
            self.fixture.root,
            event_name,
            head,
            self.before if event_before is None else event_before,
            input_base,
        )

    def test_push_defaults_to_exact_event_before(self) -> None:
        head = self.fixture.commit("head")
        self.assertEqual(self.resolve(head), self.before)

    def test_push_override_may_expand_to_ancestor(self) -> None:
        head = self.fixture.commit(
            "recovery\n\nTESTGATE-Comparison-Base: " + self.anchor
        )
        self.assertEqual(self.resolve(head), self.anchor)

    def test_push_rejects_duplicate_and_malformed_overrides(self) -> None:
        duplicate = self.fixture.commit(
            "duplicate\n\n"
            f"TESTGATE-Comparison-Base: {self.anchor}\n"
            f"TESTGATE-Comparison-Base: {self.anchor}"
        )
        with self.assertRaisesRegex(
            RESOLVER.ComparisonBaseError, "at most one"
        ):
            self.resolve(duplicate)
        malformed = self.fixture.commit(
            "malformed\n\nTESTGATE-Comparison-Base: HEAD~2"
        )
        with self.assertRaisesRegex(
            RESOLVER.ComparisonBaseError, "lowercase 40-character"
        ):
            self.resolve(malformed)

    def test_push_rejects_forward_or_unrelated_override(self) -> None:
        head = self.fixture.commit(
            "forward\n\nTESTGATE-Comparison-Base: " + self.before
        )
        with self.assertRaisesRegex(
            RESOLVER.ComparisonBaseError, "only expand"
        ):
            self.resolve(head, event_before=self.anchor)

        unrelated = ComparisonBaseFixture()
        self.addCleanup(unrelated.close)
        unrelated_head = unrelated.commit("unrelated")
        injected = self.fixture.commit(
            "unrelated\n\nTESTGATE-Comparison-Base: " + unrelated_head
        )
        with self.assertRaises(RESOLVER.ComparisonBaseError):
            self.resolve(injected)

    def test_push_rejects_input_and_dispatch_rejects_trailer(self) -> None:
        head = self.fixture.commit("head")
        with self.assertRaisesRegex(
            RESOLVER.ComparisonBaseError, "exact head"
        ):
            self.resolve(head, input_base=self.anchor)
        trailer = self.fixture.commit(
            "dispatch\n\nTESTGATE-Comparison-Base: " + self.anchor
        )
        with self.assertRaisesRegex(
            RESOLVER.ComparisonBaseError, "rejects"
        ):
            self.resolve(
                trailer,
                event_name="workflow_dispatch",
                input_base=self.anchor,
            )

    def test_dispatch_uses_input_or_head_parent(self) -> None:
        head = self.fixture.commit("dispatch")
        self.assertEqual(
            self.resolve(
                head,
                event_name="workflow_dispatch",
                input_base=self.anchor,
            ),
            self.anchor,
        )
        self.assertEqual(
            self.resolve(
                head,
                event_name="workflow_dispatch",
                input_base="",
            ),
            self.before,
        )


if __name__ == "__main__":
    unittest.main()
