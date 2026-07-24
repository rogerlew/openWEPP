# Gate Evidence

Evidence mode: Ran.

## Focused Edit Loop

- Planner nextest: 174 passed, 14 skipped.
- TESTGATE alignment, executor, and campaign-currency contracts: 25 passed.
- Hardened schema/source alignment contract: 11 passed.
- Focused deletion guard test: passed.
- Planner all-target Clippy with warnings denied: passed.
- Rust format check and diff check: passed.
- Workflow YAML parse, shell syntax, and Python compile checks: passed.
- Targeted documentation lint: 9 files, 0 errors.

Full-repository documentation lint reported 17 pre-existing broken links outside
the package write set. Targeted changed-document lint passed and the terminal
plan selected that bounded documentation command.

## Exact Terminal Execution

- Executed head: `e1e26a150a949071045f88b2e6d9903732756060`
- Executor SHA-256:
  `678fcddf28804dbecaa3e88c64eadd5261fcc138e00037c5448eec349d20dce3`
- Audit: `READY`, 10/10 checks passed
- Audit ID:
  `acdcd9199f0e263c86f827e9aa7871189a7fa0d9e34139a0b3d190c6fd8f05a5`
- Receipt: `PASS`
- Receipt ID:
  `e98dbf5a88cac98b41fea03b5083be6deea77c89706e6f325d7cdbfe897825a1`
- Nodes: 12 passed, 0 failed, 0 blocked, 0 skipped, 0 retried
- Inventory: 2,288 planned, 2,288 executed, 0 unavailable
- Full nextest node: 2,262 tests across 193 binaries passed

Artifact SHA-256 values:

- terminal plan:
  `b6f039750d607adb7571a69af402d73841ef53b8f1e8028a4a4d1c92743c11ea`
- light receipt:
  `390a5efa76e8e590b84c8e5578ac19c43e666da6ed1f86c5a28900e44e26a429`
- pre-heavy audit:
  `3d2aaa5696d52f2a9312383d7e27359b8a7c94f0f3afcf2a31a9fdb47a6d34f9`
- final receipt:
  `fa839476521c70dacf57d1748fd99cea5c39365fb10d5118a4fc8035eb44aa49`

No live GitHub workflow was dispatched, as required by excluded scope.
Coverage and CRAP were not executed.
