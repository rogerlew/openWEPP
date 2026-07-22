# Gate Results

| Gate | Status | Evidence |
| --- | --- | --- |
| Aggregate admission | PASS | Retained `aggregate-admission.json`; module scaffold `b14a90ba`. |
| `cargo fmt --all -- --check` | PASS | Ran at terminal source, exit 0. |
| Exact focused Nextest | PASS | Ran 1 passed, 152 skipped; run ID `82f91617-da31-4c62-8b91-93740f0b814f`. |
| Focused LLVM coverage | PASS | Target/helpers 100%; LCOV SHA `175f342a...`. |
| Focused CRAP | PASS | Target/helpers CRAP 4; JSON SHA `9e6fa09a...`. |
| Planner all-target warnings-denied Clippy | PASS | Ran at terminal source, exit 0. |
| `git diff --check` | PASS | Ran at terminal source, exit 0. |
| Line-count governance | PASS-WARN | 2,996 lines; warning and named split intent recorded. |
| Global TESTGATE | DEFERRED-BY-PLAN | Master ExecPlan assigns one changed-head qualification after both B02 modules complete. |

Ran: an attempted focused Nextest invocation placed `--exact` before the
harness separator and exited with a CLI usage error. It produced no test result
and was corrected by the successful exact command above. No HEAVY gate was run
by the parent.
