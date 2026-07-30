#!/usr/bin/env python3
"""Focused tests for tracked assurance review rendering."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("render_assurance_review_drafts.py")
SPEC = importlib.util.spec_from_file_location("review_renderer", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {SCRIPT}")
RENDERER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RENDERER)


class ReviewRendererTests(unittest.TestCase):
    def test_inventory_and_comparison_detect_missing_extra_and_drift(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            expected = root / "expected"
            observed = root / "observed"
            expected.mkdir()
            observed.mkdir()
            (expected / "same").write_text("same", encoding="utf-8")
            (expected / "drift").write_text("one", encoding="utf-8")
            (expected / "missing").write_text("missing", encoding="utf-8")
            (observed / "same").write_text("same", encoding="utf-8")
            (observed / "drift").write_text("two", encoding="utf-8")
            (observed / "extra").write_text("extra", encoding="utf-8")
            findings = RENDERER.compare_inventories(
                RENDERER.regular_inventory(expected),
                RENDERER.regular_inventory(observed),
            )
            self.assertEqual(
                findings,
                ["missing: missing", "extra: extra", "drift: drift"],
            )

    def test_inventory_rejects_symlink(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "source").write_text("source", encoding="utf-8")
            try:
                (root / "alias").symlink_to("source")
            except OSError as error:
                self.skipTest(f"symlink unavailable: {error}")
            with self.assertRaisesRegex(
                RENDERER.ReviewRenderError, "contains a symlink"
            ):
                RENDERER.regular_inventory(root)

    def test_review_index_is_deterministic_and_explicitly_nonapproved(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            reports = Path(temporary)
            version = reports / "example-report" / "1.0.0"
            version.mkdir(parents=True)
            (version / "index.md").write_text(
                "# Example Assurance Report\n", encoding="utf-8"
            )
            (version / "build-manifest.json").write_text(
                '{"source_root_sha256":"' + ("a" * 64) + '"}\n',
                encoding="utf-8",
            )
            first = RENDERER.render_index(reports)
            second = RENDERER.render_index(reports)
            self.assertEqual(first, second)
            text = first.decode("utf-8")
            self.assertIn("rendered **DRAFT** reports", text)
            self.assertIn("independently approved", text)
            self.assertIn(
                "[Example Assurance Report](example-report/1.0.0/index.md)",
                text,
            )

    def test_install_is_idempotent_and_replaces_complete_tree(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            destination = root / "review-drafts"
            candidate = root / "candidate"
            destination.mkdir()
            candidate.mkdir()
            (destination / "old").write_text("old", encoding="utf-8")
            (candidate / "new").write_text("new", encoding="utf-8")
            self.assertEqual(
                RENDERER._install_candidate(candidate, destination), "applied"
            )
            self.assertFalse((destination / "old").exists())
            self.assertEqual(
                (destination / "new").read_text(encoding="utf-8"), "new"
            )
            repeated = root / "repeated"
            repeated.mkdir()
            (repeated / "new").write_text("new", encoding="utf-8")
            self.assertEqual(
                RENDERER._install_candidate(repeated, destination), "no-op"
            )


if __name__ == "__main__":
    unittest.main()
