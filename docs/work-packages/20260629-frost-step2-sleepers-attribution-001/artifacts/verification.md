# Verification

Evidence mode: Ran.

## Commands

| Gate | Command | Result |
| --- | --- | --- |
| Attribution analyzer | `.venv/bin/python artifacts/attribute_sleepers.py ...` | PASS |
| Analyzer syntax | `.venv/bin/python -m py_compile artifacts/attribute_sleepers.py` | PASS |
| Scoped Markdown lint | `markdown-doc lint --path ...` | PASS |
| Scoped Markdown validate | `markdown-doc validate --path ...` | PASS |
| Whitespace check | `git diff --check` | PASS |

## Gate Results

- Only Step 1 `FORCING-LIMITED` Sleepers sites analyzed: PASS.
- Per-water-year onset/thaw/frozen-duration tables emitted: PASS.
- Magnitude residuals tagged forcing-limited/non-verdict-bearing: PASS.
- Candidate frost-model timing defects point to Step 3 families only: PASS.
- No runtime, fixture, schema, default, selector, or contract-physics file
  changed: PASS.

## Residual Risk

The attribution depends on the provisional `+/-14 day` timing tolerance in
`TOL-SNOWFREEZE-008/011`. Step 4 ratification must decide whether the Step 1
systematic-timing-fraction cutoff should become contract authority.
