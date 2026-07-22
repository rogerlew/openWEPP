from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO = Path(__file__).resolve().parents[2]
VALIDATOR = REPO / "tools/local_ci/check_cqr_aggregate_admission.py"
AGGREGATE = "docs/work-packages/aggregate/package.md"
MODULE = "docs/work-packages/module/package.md"


class Fixture:
    def __init__(self, aggregate_status: str = "ACTIVE", cover_tests: bool = True) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="cqr-aggregate-validator-")
        self.root = Path(self.temporary.name)
        self.git("init", "-q")
        self.git("config", "user.name", "Codex Test")
        self.git("config", "user.email", "codex@example.invalid")
        patterns = [
            "crates/example/src/lib.rs",
            "docs/work-packages/module/**",
            "docs/work-packages/README.md",
        ]
        if cover_tests:
            patterns.append("tests/python/test_example.py")
        self.write(AGGREGATE, self.aggregate_text(aggregate_status, patterns))
        self.commit("aggregate scaffold")
        self.aggregate_scaffold = self.git("rev-parse", "HEAD")
        self.write(
            MODULE,
            self.module_text(self.aggregate_scaffold),
        )
        self.commit("module scaffold")
        self.module_scaffold = self.git("rev-parse", "HEAD")

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

    def write(self, relative: str, text: str) -> None:
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")

    def commit(self, message: str) -> None:
        self.git("add", ".")
        self.git("commit", "-q", "-m", message)

    @staticmethod
    def aggregate_text(status: str, patterns: list[str]) -> str:
        bullets = "\n".join(f"- `{pattern}`" for pattern in patterns)
        return f"# Aggregate\n\nStatus: `{status}`\n\n## Declared Write Set\n\n{bullets}\n"

    @staticmethod
    def module_text(scaffold: str, aggregate: str = AGGREGATE) -> str:
        return f"""# Module

Status: `ACTIVE`
Aggregate admission package: `{aggregate}`
Aggregate scaffold commit: `{scaffold}`

## Declared Write Set

- `crates/example/src/lib.rs`
- `tests/python/test_example.py`
- `docs/work-packages/module/**`
- `docs/work-packages/README.md`
"""

    def run(
        self,
        *,
        scaffold: str | None = None,
        aggregate: str = AGGREGATE,
    ) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [
                str(VALIDATOR),
                "--repo",
                str(self.root),
                "--aggregate-package",
                aggregate,
                "--aggregate-scaffold",
                scaffold or self.aggregate_scaffold,
                "--module-package",
                MODULE,
            ],
            check=False,
            capture_output=True,
            text=True,
        )


class AggregateAdmissionTests(unittest.TestCase):
    def fixture(self, **arguments: object) -> Fixture:
        fixture = Fixture(**arguments)
        self.addCleanup(fixture.close)
        return fixture

    def assert_failure(self, result: subprocess.CompletedProcess[str], phrase: str) -> None:
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        payload = json.loads(result.stdout)
        self.assertEqual(payload["status"], "FAIL")
        self.assertIn(phrase, payload["error"])

    def test_accepts_earlier_immutable_covering_authority(self) -> None:
        result = self.fixture().run()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(json.loads(result.stdout)["status"], "PASS")

    def test_rejects_non_active_scaffold_status(self) -> None:
        self.assert_failure(
            self.fixture(aggregate_status="COMPLETE").run(),
            "ACTIVE or READY",
        )

    def test_rejects_missing_aggregate_package(self) -> None:
        self.assert_failure(
            self.fixture().run(aggregate="docs/work-packages/missing/package.md"),
            "does not exist",
        )

    def test_rejects_insufficient_scaffold_write_set(self) -> None:
        self.assert_failure(
            self.fixture(cover_tests=False).run(),
            "does not cover",
        )

    def test_rejects_aggregate_scaffold_that_does_not_predate_module(self) -> None:
        fixture = self.fixture()
        fixture.write("late-marker.txt", "late\n")
        fixture.commit("late aggregate marker")
        late_scaffold = fixture.git("rev-parse", "HEAD")
        fixture.write(MODULE, Fixture.module_text(late_scaffold))
        fixture.commit("bind late aggregate scaffold")
        self.assert_failure(fixture.run(scaffold=late_scaffold), "predate")

    def test_rejects_mutated_aggregate_write_set(self) -> None:
        fixture = self.fixture()
        fixture.write(
            AGGREGATE,
            Fixture.aggregate_text(
                "ACTIVE",
                ["crates/example/src/lib.rs", "docs/work-packages/module/**"],
            ),
        )
        fixture.commit("mutate aggregate authority")
        self.assert_failure(fixture.run(), "changed after scaffold")

    def test_rejects_mismatched_module_binding(self) -> None:
        fixture = self.fixture()
        fixture.write(
            MODULE,
            Fixture.module_text(
                fixture.aggregate_scaffold,
                "docs/work-packages/different/package.md",
            ),
        )
        fixture.commit("mismatch module binding")
        self.assert_failure(fixture.run(), "binding does not match")


if __name__ == "__main__":
    unittest.main()
