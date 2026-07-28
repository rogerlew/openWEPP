# Exact Diff Reconciliation

Evidence class: Ran + Static.

Declared base:
`72e433d16b4f9c35f2bb05cee8c7d92b1e16108d`.

The terminal package diff contains only:

- the three declared catalog/roadmap paths;
- `tools/validation/workplan-lint`;
- `tools/validation/workplan_lint.py`;
- `tools/validation/test_workplan_lint.py`;
- `tools/validation/README.md`; and
- this package subtree.

There are no fixture files and no undeclared package-owned paths. The unrelated
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` remains
untracked, unstaged, unchanged, and explicitly excluded.

Production line count is 1,011, below the 3,000-line ceiling. The product has
one structurally confined subprocess surface and no legacy planner import, CI,
receipt, ledger, lifecycle, publication, recovery, calibration, custody,
daemon, or database surface.
