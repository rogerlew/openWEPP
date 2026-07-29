# Line-Count Governance

Status: `PASS`

Evidence class: `Ran + Static`

No `.rs` file changed. Therefore no new or modified Rust file reaches the
2,000-line warning or 3,000-line closure threshold.

Package-local Python tools are:

- `analyze.py`: 426 lines
- `execute.py`: 897 lines
- `plot_results.py`: 418 lines
- `rescore_harvard.py`: 149 lines
- `validate.py`: 459 lines

Python is outside the Rust line-count gate. The one-time Harvard rescore tool
retains the operator-adjudication trail; the other tools retain cohesive
execution, analysis, rendering, and independent-validation responsibilities.
