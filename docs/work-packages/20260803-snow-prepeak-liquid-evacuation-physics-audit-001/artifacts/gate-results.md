# Gate Results

Status: `technical, dual-review, and dual-verification gates PASS / Stage-3 liquid closure BLOCKED / HOLD-EVIDENCE`

Evidence mode: `Static + Ran`

## Execution And Reconstruction

| Gate | Status | Command / evidence |
|---|---|---|
| Exact release build | `PASS` | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`; source `073dafe3`, binary `8fb77e17...c673`, 11,206,520 bytes. |
| Prospective v3 freeze | `PASS` | `run_audit.py --freeze`; freeze `1994e037...42f3` precedes accepted result and binds tool, helper, binary, fixture, observation, legacy, ERA/cloud, operator, event, censor, and tolerance identities. |
| Same-binary real consumer | `PASS` | `run_audit.py --execute`; baseline plus three operators by four sites, 16/16 return codes zero; receipt `52238c81...dfb0`. |
| Independent reconstruction | `PASS` | `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/tools/verify_reconstruction.py`; tool `1061f211...2347`, 61,364 daily / 1,472,736 hourly rows. |
| Primitive mass | `PASS` | Maximum all-row daily residual `9.9973e-13 m`; primary endpoint-window maximum `8.8396e-13 m`; tolerance `1e-12 m`. |
| CoE identities | `PASS` | Term, applied, and daily-raw maxima `2.02e-17`, `1.04e-17`, and `8.00e-17 m`. |
| Routed alias | `PASS` | `routed = loss + rain_released` maximum residual `1.56e-17 m`. |
| Bounded Stage-3 energy | `PASS` | Independent maximum `1.87e-8 J m^-2`; reconstruction-versus-trace maximum `7.28e-12 J m^-2`; tolerance `1e-6 J m^-2`. |
| Independent Stage-3 liquid | `BLOCKED` | JSONL omits incoming, routed, retained, and residual liquid operands owned by `DirectSnowStage3Diagnostics`; internal guard is not independent downstream evidence. This current exit criterion forces `HOLD-EVIDENCE`. |
| Focused CoE component/cap test | `PASS` | `cargo nextest run -p openwepp-hillslope-orchestrator simimpl29_melt_hour_covers_zero_wind_rain_and_cap_paths`; 1 passed, 434 skipped. |

## Integrity And Scope

| Gate | Status | Command / evidence |
|---|---|---|
| JSON syntax | `PASS` | `for f in .../artifacts/*.json; do jq empty "$f"; done`. |
| Python syntax without bytecode | `PASS` | `PYTHONDONTWRITEBYTECODE=1 .venv/bin/python` with `ast.parse` over both package tools; no `__pycache__` remains. |
| Overwrite refusal | `PASS` | Second `run_audit.py --execute` returned 1 with `refusing to overwrite ..._v3`. |
| Receipt integrity | `PASS` | `jq` confirms schema 3, 16 cells, all return codes zero, copied/source fixture manifests identical, and 16 runfile hashes. |
| Cohort/censor integrity | `PASS` | `jq` confirms 154 primary uncensored windows and 158 all-window sensitivity windows. |
| Protected tree identity | `PASS` | HEAD advanced after v3 from `073dafe3` to `06dc722c` through out-of-package docs only. Six frozen `git rev-parse HEAD:<path>` values still reproduce v3, including `references/50201000`; tracked and untracked diffs are empty for protected production Rust, contracts, tests, and frozen references. |
| Write-set reconciliation | `PASS with disclosed amendments` | Docs/package/catalog/roadmap, original/v2/v3 disposable targets, and the exact invalid-run `/tmp` recovery namespace are named in `owned-file-manifest.md`; all additions are explicitly retrospective, not prospective. |
| Rust line count | `NOT APPLICABLE` | No Rust diff. Package evidence tools are 1,108 and 146 lines and are governed by syntax, checksum, refusal, and reconstruction gates. |
| Markdown | `PASS` | `markdown-doc lint` reports package 27/0/0, catalog 1/0/0, and roadmap 1/0/0 files/errors/warnings. |
| Exact terminal diff | `PASS` | `git diff --check` passes; dirty paths are confined to the package, catalog, and roadmap, with no protected-path change. |
| Dual review | `PASS` | Both independent reviewers returned final exact-current `PASS` after every finding was accepted and remediated. |
| Dual verification | `PASS` | Both independent verifiers returned final exact-current `PASS` after all precheck findings were accepted, corrected, and rechecked. |

The package cannot be marked `complete`: one required scientific closure gate is
blocked by a source-confirmed missing downstream surface. All feasible direct
technical, review, and verification gates pass, so the terminal disposition is
`HOLD-EVIDENCE`.
