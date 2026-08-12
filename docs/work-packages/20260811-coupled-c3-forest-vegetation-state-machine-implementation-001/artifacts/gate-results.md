# Gate Results

Status: `EXECUTING / V2 Milestone 0 PASS / focused remediation active`

Evidence mode: `Ran`

| Gate | Result | Evidence |
|---|---|---|
| Milestone 0 A0/model/oracle | PASS | pre-implementation artifact and comparator logs |
| Focused compile | PASS | `cargo check` for the three affected crates |
| Focused clippy | PASS | affected crates with `-D warnings` |
| Boundary scaffold | PASS | 7 tests in `vegetation_boundary_authority_contract` |
| E01--E22 public transaction | FAIL | science review B-CRITICAL-001--005 |
| independent five-ledger closure/rollback | FAIL | science review B-HIGH-006 |
| exact numerical/guard posture | FAIL | science review B-CRITICAL-002/003 and B-HIGH-007 |
| Critical/full workspace/benchmarks | NOT RUN | illegitimate while material focused gates fail |
| dual terminal verification | NOT RUN | terminal bytes are not closure-eligible |

## Remediation Continuation

The rows above preserve checkpoint results, including failures. New command
results append below; no retry replaces historical evidence.

| Gate | Result | Evidence |
|---|---|---|
| checkpoint identity and clean tree | PASS | `git rev-parse HEAD` returned `c064206883bd26848a93bd4b9b104b7f5b647344`; `git status --short --branch` reported clean `main...origin/main` |
| instruction discovery for full continuation write set | PASS | `tools/agents/find-agents --for ...` returned root plus `crates/AGENTS.md`, `tests/AGENTS.md`, and `docs/work-packages/AGENTS.md` as applicable |
| restored A0 suite | PASS | `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`: 12 passed |
| implementation/transaction suite | PASS | `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`: 10 passed, including the real public transaction and 13 injected rollback phases |
| vegetation crate | PASS | `cargo nextest run -p openwepp-vegetation --profile quick`: 2 passed, including exact four-node/cap hydraulic oracle vectors |
| BGC crate | PASS | `cargo nextest run -p openwepp-biogeochemistry --profile quick`: 3 passed for proportional competition, species/layer separation, unused authorization, and duplicate receipt rejection |
| hillslope orchestrator crate | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick`: 490 passed in 149.178 s; three existing slow OFE-routing tests |
| focused checks | PASS | all four required package-scoped `cargo check` commands passed |
| focused strict Clippy | PASS after correction | all four required package-scoped commands passed; an earlier BGC test-only float-comparison failure was corrected rather than erased |
| science admission | PASS after correction | first retry rejected missing impact binding for `vegetation_diagnostic.rs`; bounded catalog bindings were added for it and the typed resource DTO; retry returned `A0_ADMITTED contracts=44 science_surfaces=2` |
| authority anti-evasion | PASS | `check_authority_suite_antievasion.sh` passed |
| AUTH11 obligation guards | PASS | 3 passed |
| SC unit compliance | PASS | both `SC-VEGETATION-001.md` and `SC-BIOGEOCHEM-001.md` returned no findings |
| independent oracle regeneration | PASS | 11 selected values matched at relative/absolute tolerance `1e-12`; fixture SHA-256 `339537846d2cad3b5f03f55cd1946b545a35a49aca9d92f6cd952e9bde83e964` |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check` and `git diff --check` passed |

## Hold-boundary continuation

| Gate | Result | Evidence |
|---|---|---|
| coverage-corrected radiation | PASS | vegetation crate 7/7 after exact tile-local `area/C_s`, `f_t` aggregation, and multirank boundary correction |
| implementation transaction suite | PASS after containment update | 10/10; ambiguous partial-cover E04 now returns typed unsupported before candidate execution |
| repeat science review | HOLD | independent targeted adjudication confirms a canonical E04 heterogeneous-tile state/aggregation omission |
| repeat Rust review | FAIL / accepted findings remain | numerical diagnostics, multirank final-liquid routing, independent owner reconstruction, all-owner commit API, duplicate request identity, and line-count work remain open |
| heavy/full/benchmark gates | NOT RUN | prohibited because focused science review cannot pass without constitutive authority |
| terminal verification and prompt archive | NOT RUN | no terminal-complete bytes exist |
| Markdown package lint | PASS after invocation correction | initial invocation omitted `--path` and failed argument parsing; `markdown-doc lint --path <package>` then validated 46 files with 0 errors and 0 warnings |
| final HOLD admission/lint reconciliation | PASS | admission returned `A0_ADMITTED contracts=44 science_surfaces=3`; both SC unit lints passed; package Markdown lint validated 47 files; `git diff --check` passed |

## Accepted review-remediation rerun

The first fresh science and Rust reviews returned FAIL and remain preserved in
their remediation review artifacts. All material findings were accepted and
corrected before this rerun.

| Gate | Result | Evidence |
|---|---|---|
| restored A0 suite | PASS | 12 passed |
| implementation/transaction suite | PASS | 10 passed, including strict digest and serialized 13-phase rollback evidence |
| vegetation crate | PASS | 6 passed, including Brent/singularity, condensation capacity, two-rank boundary routing, and stem-only optics poisons |
| BGC crate | PASS | 3 passed, including deterministic order, species/layer separation, and exact receipt rejection |
| hillslope orchestrator crate | PASS | 490 passed in 148.030 s; three known slow routing tests |
| focused strict Clippy | PASS | all four affected crates passed `--all-targets -- -D warnings` |
| science admission | PASS after binding correction | first run rejected the new kernel-contract manifest surface; impact-map generation 20 added its SC/A1 binding; rerun returned `A0_ADMITTED contracts=44 science_surfaces=3` |
| authority anti-evasion | PASS | authority suite anti-evasion script passed |
| AUTH11 obligation guards | PASS | 3 passed |
| SC unit compliance | PASS | both vegetation and BGC contracts returned no findings |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check` and `git diff --check` passed |

## 2026-08-12 Stage-B V2 Intake

The historical failures and HOLD evidence above remain preserved. The separate
contract-first authority package released the missing topology authority at
commit `817b082d01d194cde61b1cf284bd85e40e44afc9`.

| Gate | Result | Evidence |
|---|---|---|
| V2 definition identity | PASS | both canonical copies SHA-256 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3` |
| independent V2 oracle regeneration | PASS | regenerated fixture byte-identical; SHA-256 `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1` |
| shared transaction contract identity | PASS | SHA-256 `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8` |
| science admission | PASS | 45 contracts, zero science surfaces, receipt `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628` |
| contract unit checks | PASS | vegetation and vegetation-transaction contracts found no findings |
| authority anti-evasion | PASS | source-level suite guard passed |
| AUTH11 | PASS | 3/3 |
| A0 vegetation authority | PASS | 14/14 |
| formatting, Markdown, diff hygiene | PASS | 49 package files and catalog had zero Markdown findings; format and diff checks passed |
