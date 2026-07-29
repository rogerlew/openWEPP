# CAL-07 Line-Count Governance

Evidence class: `Static`

No production Rust file changed. The only Rust source is the package-local
research executor at 164 lines, below the 800-line work-package ceiling.

Package-local Python tools total 1,529 lines:

| Tool | Lines |
|---|---:|
| `analyze.py` | 374 |
| `diagnose_forcing.py` | 111 |
| `execute.py` | 77 |
| `plot_hold.py` | 253 |
| `plot_results.py` | 229 |
| `prepare_inputs.py` | 216 |
| `validate.py` | 147 |
| `validate_hold.py` | 122 |

The larger analyzer and renderer remain separated by responsibility and are
below the production Rust file ceiling. The fail-closed disposition did not
require any kernel, runner, contract, fixture, or test edit.
