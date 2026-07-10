# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Focused watershed CLI behavior suite | PASS | `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract --profile quick`; 29/29 passed. |
| Focused bin clippy/check/fmt | PASS | Attempted source passed each before rollback. |
| `git diff --check` | PASS | Attempted source passed before rollback. |
| Isolated target coverage / CRAP | HOLD | 74.378% lines, 36.451% regions, 33 functions below 75%, one CRAP row above 30. |
| Workspace closure gates | Not run | Correctly withheld: local CQR closure failed before final gates. |

No authority-suite, cohort-fixture, or required-case binding changed.
