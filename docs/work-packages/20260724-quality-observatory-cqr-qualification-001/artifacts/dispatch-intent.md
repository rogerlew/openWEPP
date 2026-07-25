# Dispatch Intent

Evidence class: Static.

- Qualified Order-6 subject:
  `955358449381ab38378d28dac93ba7b21b496d14`
- Order-6 TESTGATE run:
  [`30165527516`](https://github.com/rogerlew/openWEPP/actions/runs/30165527516)
- QA workflow: `.github/workflows/quality-observatory.yml`
- QA trigger: manual `workflow_dispatch` only
- QA concurrency: `openwepp-forest1-quality-observatory`
- TESTGATE concurrency: distinct `openwepp-forest1-testgate`

The preflight defect is corrected prospectively by requiring a successful
exact-repository/path/head TESTGATE run ID in addition to current-main source
admission. The correction requires a new committed and pushed head plus exact
successful TESTGATE qualification before one QA attempt.

No CQR collection or module implementation is authorized. CQR work in this
package is selection-only intake of the exact successful QA publication.
