# Line-Count Governance

Status: `pass / no production source diff`

Evidence mode: `Static`

No `.rs`, Cargo manifest, or other production source file is present in the
package diff. Production 2,000/3,000-line thresholds and host decomposition are
therefore `NOT_APPLICABLE`.

Package-local evidence tools are:

- `analyze.py`: 831 lines;
- `render_figures.py`: 211 lines;
- `verify_results.py`: 199 lines; and
- `test_analyze.py`: 117 lines.

All are nonproduction, single-purpose analysis surfaces below 2,000 lines.
They are covered by Python compilation, six focused tests, fail-closed source
identity checks, retained-output hashes, and an independent table reduction.
