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
MANIFEST = "docs/work-packages/aggregate/artifacts/batch-authority.json"
MASTER = "docs/work-packages/cqr-batch-execplan.md"


class Fixture:
    def __init__(
        self,
        *,
        aggregate_status: str = "ACTIVE",
        cover_tests: bool = True,
        manifest_master: bool = True,
        manifest_module: str = MODULE,
        module_paths: list[str] | None = None,
        module_suffix: str = "",
        scaffold_bindings: bool = True,
        use_canonical_template: bool = False,
        write_master: bool = True,
    ) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="cqr-aggregate-validator-")
        self.root = Path(self.temporary.name)
        self.git("init", "-q")
        self.git("config", "user.name", "Codex Test")
        self.git("config", "user.email", "codex@example.invalid")
        patterns = [
            "crates/example/src/lib.rs",
            "docs/work-packages/aggregate/**",
            MASTER,
            "docs/work-packages/module/**",
            "docs/work-packages/README.md",
        ]
        if cover_tests:
            patterns.append("tests/python/test_example.py")
        self.write(AGGREGATE, self.aggregate_text(aggregate_status, patterns))
        self.write(
            MANIFEST,
            self.manifest_text(cover_tests, manifest_master, manifest_module),
        )
        if write_master:
            self.write(MASTER, "# CQR Batch ExecPlan\n")
        self.commit("aggregate scaffold")
        self.aggregate_scaffold = self.git("rev-parse", "HEAD")
        module_text = (
            self.canonical_template_text(self.aggregate_scaffold)
            if use_canonical_template
            else self.module_text(
                self.aggregate_scaffold,
                paths=module_paths,
                suffix=module_suffix,
                include_bindings=scaffold_bindings,
            )
        )
        self.write(MODULE, module_text)
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
    def manifest_text(
        cover_tests: bool = True,
        include_master: bool = True,
        module_package: str = MODULE,
    ) -> str:
        required_paths = [
            "docs/work-packages/README.md",
            MODULE,
            MANIFEST,
            "docs/work-packages/module/**",
            "docs/work-packages/aggregate/**",
            "crates/example/src/lib.rs",
        ]
        if include_master:
            required_paths.insert(0, MASTER)
        if cover_tests:
            required_paths.append("tests/python/test_example.py")
        return json.dumps(
            {
                "schema_version": "openwepp-cqr-aggregate-batch-v1",
                "aggregate_package": AGGREGATE,
                "master_execplan": MASTER,
                "module_packages": [module_package],
                "required_paths": required_paths,
            },
            indent=2,
        ) + "\n"

    @staticmethod
    def module_text(
        scaffold: str,
        aggregate: str = AGGREGATE,
        *,
        paths: list[str] | None = None,
        suffix: str = "",
        include_bindings: bool = True,
    ) -> str:
        bindings = ""
        if include_bindings:
            bindings = f"""Aggregate admission package: `{aggregate}`
Aggregate scaffold commit: `{scaffold}`
Aggregate batch manifest: `{MANIFEST}`
Master ExecPlan: `{MASTER}`
"""
        planned = paths or [
            "crates/example/src/lib.rs",
            "tests/python/test_example.py",
            "docs/work-packages/module/**",
            "docs/work-packages/README.md",
        ]
        bullets = "\n".join(f"- `{path}`" for path in planned)
        return f"""# Module

Status: `ACTIVE`
{bindings}
## Intended Write Set

{bullets}
{suffix}"""

    @staticmethod
    def canonical_template_text(scaffold: str) -> str:
        text = (
            REPO / "docs/work-packages/templates/cqr-nightly-package.md"
        ).read_text(encoding="utf-8")
        replacements = {
            "{{aggregate_admission_package}}": AGGREGATE,
            "{{aggregate_scaffold_commit}}": scaffold,
            "{{aggregate_batch_manifest}}": MANIFEST,
            "{{master_execplan}}": MASTER,
            "{{target_module_path}}": "crates/example/src/lib.rs",
            "{{test_paths}}": "tests/python/test_example.py",
            "{{package_id}}": "module",
        }
        for placeholder, value in replacements.items():
            text = text.replace(placeholder, value)
        return text

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

    def test_accepts_canonical_template_shape_and_batch_manifest(self) -> None:
        result = self.fixture(use_canonical_template=True).run()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        payload = json.loads(result.stdout)
        self.assertEqual(payload["status"], "PASS")
        self.assertEqual(payload["aggregate_batch_manifest"], MANIFEST)
        self.assertEqual(payload["master_execplan"], MASTER)

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

    def test_rejects_late_aggregate_scaffold(self) -> None:
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
            Fixture.aggregate_text("ACTIVE", ["docs/work-packages/**"]),
        )
        fixture.commit("mutate aggregate authority")
        self.assert_failure(fixture.run(), "changed after scaffold")

    def test_rejects_mutated_batch_manifest(self) -> None:
        fixture = self.fixture()
        manifest = json.loads(Fixture.manifest_text())
        manifest["required_paths"] = [MANIFEST, MASTER, MODULE]
        fixture.write(MANIFEST, json.dumps(manifest, indent=2) + "\n")
        fixture.commit("mutate batch manifest")
        self.assert_failure(fixture.run(), "manifest changed after scaffold")

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
        self.assert_failure(fixture.run(), "binding changed after scaffold")

    def test_rejects_bindings_added_after_unique_module_scaffold(self) -> None:
        fixture = self.fixture(scaffold_bindings=False)
        fixture.write(MODULE, Fixture.module_text(fixture.aggregate_scaffold))
        fixture.commit("late aggregate binding")
        self.assert_failure(fixture.run(), "Aggregate admission package")

    def test_rejects_module_write_set_mutation(self) -> None:
        fixture = self.fixture()
        fixture.write(
            MODULE,
            Fixture.module_text(
                fixture.aggregate_scaffold,
                paths=[
                    "crates/example/src/lib.rs",
                    "tests/python/test_example.py",
                    "docs/work-packages/module/**",
                ],
            ),
        )
        fixture.commit("mutate module authority")
        self.assert_failure(fixture.run(), "intended write set changed")

    def test_rejects_deleted_and_readded_module_package(self) -> None:
        fixture = self.fixture()
        fixture.git("rm", "-q", MODULE)
        fixture.commit("delete module package")
        fixture.write(MODULE, Fixture.module_text(fixture.aggregate_scaffold))
        fixture.commit("readd module package")
        self.assert_failure(fixture.run(), "unique scaffold addition")

    def test_rejects_parent_traversal_path(self) -> None:
        self.assert_failure(
            self.fixture(module_paths=["../outside.rs"]).run(),
            "not repository-relative",
        )

    def test_rejects_duplicate_intended_write_set_heading(self) -> None:
        self.assert_failure(
            self.fixture(
                module_suffix="\n## Intended Write Set\n\n- `crates/example/src/lib.rs`\n"
            ).run(),
            "exactly one section",
        )

    def test_rejects_manifest_that_omits_mandatory_master(self) -> None:
        self.assert_failure(
            self.fixture(manifest_master=False).run(),
            "omits mandatory paths",
        )

    def test_rejects_invalid_manifest_module_package_path(self) -> None:
        self.assert_failure(
            self.fixture(manifest_module="docs/work-packages/not-a-package.md").run(),
            "invalid module package paths",
        )

    def test_rejects_missing_master_execplan_at_aggregate_scaffold(self) -> None:
        self.assert_failure(
            self.fixture(write_master=False).run(),
            "does not exist",
        )


if __name__ == "__main__":
    unittest.main()
