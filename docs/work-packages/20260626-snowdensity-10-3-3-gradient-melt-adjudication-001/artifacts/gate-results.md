# Gate Results

Evidence mode: Ran.

| Gate | Result | Notes |
|---|---|---|
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py` | pass | Python syntax check. |
| `cargo build -q -p openwepp-runner --bin openwepp-snowbench` | pass | Built diagnostic snowbench binary. |
| `.venv/bin/python tools/snowfreeze_observed/cancov_gradient_melt_adjudication.py --output-dir target/snowdensity10_3_3_gradient_melt_adjudication --snowbench-binary target/debug/openwepp-snowbench` | pass | Ran 14 CoE melt replays: seven comparison surfaces for each model. |
| `find target/snowdensity10_3_3_gradient_melt_adjudication -maxdepth 5 -name openwepp-snowbench.stderr -size +0c -print` | pass | No non-empty stderr files. |
| `cargo test --test snowdensity10_3_3_gradient_melt_adjudication` | pass | 3 passed, 0 failed. |
| `cargo clippy --test snowdensity10_3_3_gradient_melt_adjudication -- -D warnings` | pass | No warnings. |
| `cargo fmt --check` | pass | Formatting clean after applying `cargo fmt`. |
| `git diff --check` | pass | No whitespace errors. |

## Adjudication Result

Report artifacts:

- `artifacts/gradient_melt_adjudication.json`
- `artifacts/gradient_melt_adjudication.md`

Disposition: `LOW-CANOPY-NON-PROMOTION`.

The shortwave/albedo CoE modernization does not earn low-canopy value on the
current verdict-bearing evidence. Low-canopy exact-bound robust failures worsen
from `6` to `7`; robust ordinal score remains `70`.

Promotion/default activation remains unauthorized.
