# Line-Count Governance

Status: `pass / no production source diff`

Evidence mode: `Static`

No `.rs` or other production source file is present in the package diff, so
production line-count thresholds are `NOT_APPLICABLE`.

Package-local evidence tools are:

- `run_adjudication.py`: 1,176 lines;
- `render_figures.py`: 523 lines;
- `verify_adjudication.py`: 334 lines; and
- `test_run_adjudication.py`: 120 lines.

These tools are nonproduction, single-purpose custody/reconstruction surfaces.
They are covered by syntax, unit, overwrite/refuse-or-identity-check,
independent reconstruction, hash, and exact-output gates. The sizes are
consistent with the accepted predecessor audit tool (1,108 lines) and do not
cross a governed production-host threshold.
