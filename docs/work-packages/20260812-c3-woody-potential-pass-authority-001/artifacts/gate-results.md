# Gate Results

Status: `focused, heavy, and terminal verification PASS`

Evidence mode: `Ran`

Append every attempt; do not replace failed evidence with only a later pass.

## Scaffold and Intake

| Command | Result |
| --- | --- |
| `git rev-parse HEAD` | PASS: `4f5bb1c599a683b63be56ecd9e7296f8faf01ed0`. |
| `git status --short --branch` | PASS: clean `main...origin/main` before scaffold. |
| `tools/agents/find-agents --for ...` | PASS: instruction chains recorded in `required-reading-map.md`. |
| `sha256sum` over frozen V1 and all three V2 definition copies | PASS: V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`; every V2 copy `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`. |
| `cmp -s` across all three V2 definition copies | PASS: byte-identical. |
| `markdown-doc lint docs/...` | FAIL: CLI rejects positional paths; retained as invocation-shape evidence. |
| `markdown-doc lint --path docs/work-packages/20260812-c3-woody-potential-pass-authority-001` | PASS: 24 files, 0 errors, 0 warnings. |
| `markdown-doc lint --path docs/work-packages/README.md` | PASS: 1 file, 0 errors, 0 warnings. |
| `git diff --check` | PASS. |

## V3 Draft Authority and Focused Tests

| Command | Result |
| --- | --- |
| `.venv/bin/python -m py_compile .../reference_calculator.py` | PASS. |
| `.venv/bin/python .../reference_calculator.py` repeated with digest comparison | PASS: deterministic 30,100-byte fixture, SHA-256 `bf0edfa96bef293fb2551895a40b0f17501dbf89ff525e7355d81e85877e0447`; all six oracle checks true. |
| `jq -e .` and `cmp` over both V3 definition copies | PASS: canonical JSON and byte-identical SHA-256 `6bee2a26fac1d6825a4ae7d1f3df4357cb1cf62d88a73263245f94c15379bae9`. |
| `cargo fmt --all -- --check && cargo nextest run --test vegetation_boundary_authority_contract --profile quick` | FAIL before tests: formatter reported the newly added V3 tests; retained. |
| `cargo fmt --all && ... vegetation_boundary_authority_contract` | FAIL: 15 passed, two historical assertions still expected V6 wording; retained. |
| focused authority test after V7 assertion correction | PASS: 17 tests. |
| `check_science_contract_admission.sh --base-ref 4f5bb1c... --worktree` | EXPECTED FAIL: changed contract remained `in_review/draft` before independent review. |
| `check_sc_unit_compliance.sh --path .../SC-VEGETATION-001.md` | PASS. |
| `check_authority_suite_antievasion.sh` | PASS. |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract --profile quick` | PASS: 3 tests. |
| `cargo clippy --test vegetation_boundary_authority_contract -- -D warnings` | FAIL: one 163-line test and two strict float comparisons; retained. |
| strict focused Clippy after helper extraction and bit-exact zero checks | PASS. |
| focused authority test after Clippy correction | PASS: 17 tests. |
| package Markdown lint | PASS: 30 files, 0 errors, 0 warnings. |
| contract Markdown lint | PASS: 1 file, 0 errors, 0 warnings. |
| `cargo fmt --all -- --check` and `git diff --check` | PASS. |

## Initial Independent Science Review

| Review | Result |
| --- | --- |
| Review A | NO-GO: `A-CRITICAL-001/002`, `A-HIGH-003/004/005`; radiation/wind/schema and protected V1/V2 bytes otherwise passed. |
| Review B | NO-GO: `B-CRITICAL-001/002`, `B-HIGH-003/004/005`; deterministic bytes and focused tests passed but were scientifically incomplete. |

All ten findings were accepted. The common-beta residual and reduced oracle
were rejected. V7 now specifies distinct class beta factors in a determined
six-unknown/six-residual system, exact numerical scales, and an immutable CTSM
Atkin source pin with corrected source units. Oracle/failure/poison regeneration
and fresh rereview remain required; draft lifecycle is unchanged.

## Accepted-Finding Remediation

| Command or audit | Result |
| --- | --- |
| parent audit of coupled trial-step normalization | FAIL: the first remediated oracle reused the current-iterate water tolerance for a trial state; retained as pre-freeze evidence. |
| corrected oracle regeneration, repeated digest comparison, `py_compile`, JSON validation, and all fixture self-checks | PASS: trial states recompute their own canonical shared water-flux scale; deterministic 40,887-byte fixture SHA-256 `cccc02b0ba835ae4e9788acfb674fa055d28fbf4f7106ee46243bb0c113931b4`; all eight checks and 34 executed poisons pass. |
| `sha256sum reference_calculator.py` | PASS: generator SHA-256 `50a6366ec72383f94ff7c806cf4d08ad5f5564ac345a825fa2bdf2550ac0645e`. |
| `jq -e .`, `cmp`, and `sha256sum` over both final V3 definition copies | PASS: canonical JSON and byte-identical SHA-256 `563d6f0758e5a16c19acba68ef29fe5771fe9d2ba1f80ebf8471a2c2a763d7a3`; the definition binds both fixture and generator digests. |
| strict focused Clippy after binding the generator and immutable source | FAIL: the V3 identity test reached 101 lines against the 100-line limit; retained before helper extraction. |
| `cargo fmt --all -- --check`, strict focused Clippy, and V3 authority nextest after helper extraction | PASS: 17/17 focused authority tests. |
| unit compliance, authority anti-evasion, AUTH11, package/contract/catalog Markdown lint, and `git diff --check` | PASS: AUTH11 3/3; 30 package files, one contract, and one catalog validated with no findings. |

## First Stable-Byte Rereview

| Review | Result |
| --- | --- |
| Review A rereview | NO-GO: `A-CRITICAL-001` and `A-HIGH-005` resolved; `A-CRITICAL-002`, `A-HIGH-003`, and `A-HIGH-004` remain unresolved because the oracle still substitutes E07--E10/root-path/aerodynamic kernels and lacks genuine poison/precedence execution. |
| Review B rereview | NO-GO: `B-CRITICAL-001` and `B-HIGH-005` resolved; `B-CRITICAL-002`, `B-HIGH-003`, and `B-HIGH-004` remain unresolved for the independently confirmed biochemical, precedence, and poison gaps. |

All remaining findings are accepted for a second remediation cycle. Lifecycle
remains draft and no implementation authority is released.

## Second Accepted-Finding Remediation

| Command or audit | Result |
| --- | --- |
| strengthened Rust reconstruction of the canonical initial `ci` bracket | FAIL: the first revised fixture exposed only Brent's final narrowed bracket, not the initial `[Gamma*(T),ca]` bracket; retained before adding both operands. |
| strict focused Clippy after expanded E07--E15/precedence/poison assertions | FAIL: combined evidence helper reached 127 lines; retained before splitting diagnostics and inventory checks. |
| final independent oracle compilation and repeated regeneration | PASS: deterministic 49,960-byte fixture SHA-256 `ee98dd49b0054e1488aead34ee4eceb49905f0f7978afb6554c7f61f16b894ed`; generator SHA-256 `fb08da650b92b58cf34fb609eab6284d45be9a88d9e82b32ba5f3e4cfcb8b905`. |
| final fixture structural assertions | PASS: eight checks, 40/40 executed poisons, all ten typed poisons executed by owning validators, full five-edge precedence order, exact temperature-dependent `ci` initial brackets, and no candidate publication. |
| final V3 definition JSON/copy/digest | PASS: byte-identical definition copies SHA-256 `fa6b7fa7c86a059b9d0a46065a23a7e35c2ce749d494e04e7842c0341bd901f0`; fixture and generator digests bound. |
| warnings-denied focused Clippy and V3 authority nextest after helper extraction | PASS: 17/17 tests; Rust independently reconstructs neutral `rah/raw`, E07--E10 intermediates/identities, E14 `kr/ks/k3/RAI`, all 40 poisons, finite diagnostic payloads, and precedence. |
| frozen V1/V2 SHA-256 and final `git diff --check` | PASS: V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`; V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`; diff clean. |

## Second Stable-Byte Rereview

| Review | Result |
| --- | --- |
| Review B second rereview | NO-GO solely under `B-CRITICAL-002`: all other findings resolved, but the fixture used `cp_air=1005.0` and `latent_heat=2450000.0` instead of immutable imported V1 constants `1004.64` and `2501000.0`; accepted for immediate correction. |

## Final Review and Admission

| Command or review | Result |
| --- | --- |
| Review A final exact-byte addendum | GO: all findings resolved, no new material finding. |
| Review B final exact-byte addendum | GO: `B-CRITICAL-002` fixed with immutable `cp_air=1004.64` and `latent_heat=2501000.0`; all findings resolved, no new material finding. |
| `check_science_contract_admission.sh --base-ref 4f5bb1c... --worktree` after lifecycle promotion | PASS: `A0_ADMITTED contracts=45 science_surfaces=0`, authority SHA-256 `f4e3e5280f46fbc881fb5f766b67e007d08d72f485c4abe0144da4fa8a46a97b`. |
| first focused authority run after active promotion | FAIL: registry assertion still expected the historical `in_review/draft` row; retained before lifecycle-only assertion correction. |
| focused active-lifecycle retry, strict Clippy, anti-evasion, AUTH11, formatting, and diff hygiene | PASS: authority 17/17; AUTH11 3/3; all other commands clean. |

## Heavy Closure

| Comparator command | Result |
| --- | --- |
| workspace warnings-denied Clippy | PASS. |
| workspace doctests | PASS. |
| `cargo deny check` | PASS. |
| formatting and diff hygiene | PASS. |
| active admission, anti-evasion, AUTH11, unit compliance, focused authority suite | PASS. |
| first `cargo nextest run --workspace --profile full` | INCOMPLETE / `rc=100`: comparator manually interrupted after 1,364,557 ms (~22m45s) while slow assurance-v2 tests continued; no scientific test failure was reported. Full log retained under `artifacts/heavy-closure-20260812-170911/`. A long-allowance comparator retry is required. |
| long-allowance full-workspace retry with `/tmp` override | ENVIRONMENT FAIL / `rc=100`: after 2,213 s (~36m53s), nested assurance builds exhausted the 439 GB root filesystem and failed with `ENOSPC`; no test summary or scientific failure. Log retained under `artifacts/retry-full-20260812-173248/`. Retry must use the 24 TB free `/home` filesystem, not `/tmp`. |
| first `/home`-backed comparator execution | INCOMPLETE: the run advanced without a scientific failure, but its execution wrapper ended before a terminal summary. Log retained under `artifacts/v3-nextest-20260812-181033-914552/`. |
| second `/home`-backed comparator execution | ORCHESTRATION FAIL: manually interrupted at 826.267 s while two tests still ran; nextest consequently reported 206 passed and two SIGINT failures, with 2,273 not run. Log retained under `artifacts/v3-nextest-20260812-184149-987294/`. |
| third `/home`-backed comparator execution | ORCHESTRATION FAIL: manually interrupted at 2,809.367 s while one test still ran; nextest consequently reported 238 passed and one SIGINT failure, with 2,242 not run. Log retained under `artifacts/v3-nextest-20260812-185549-1020833/`. |
| uninterrupted detached `/home`-backed `cargo nextest run --workspace --profile full` | PASS / `rc=0`: 2,481/2,481 tests passed, 51 slow, 33 canonically skipped; nextest elapsed 3,318.773 s and wrapper duration was 3,329 s. The unique `/home/workdir/openwepp-v3-nextest.7LzluH` scratch directory was removed after completion. Log and summary retained under `artifacts/v3-nextest-20260812-194314-1131755/`. This proves the earlier apparent stalls were premature orchestration interrupts, not test failures. |

The comparator's remaining ten required commands passed in the first heavy
run. The uninterrupted full-workspace result closes the sole missing heavy
gate without replacing or erasing any earlier failed or incomplete evidence.

## Terminal Hygiene Delta

| Command or audit | Result |
| --- | --- |
| `git diff --check` after intent-to-add exposed all new files | FAIL: one whitespace-only oracle line and one extra blank line at the end of the captured doctest log; earlier checks had not inspected untracked files. Retained as the reason for the byte-only correction. |
| oracle whitespace correction and deterministic regeneration | PASS: fixture remains byte-identical at `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109`; generator is now `7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a`. |
| V3 definition rebinding | PASS: both definition copies remain byte-identical at new SHA-256 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`. No equation, value, fixture, contract section, or executable production byte changed. |
| focused exact-byte rerun | PASS: strict authority-test Clippy, 17/17 vegetation authority tests, active admission, unit compliance, anti-evasion, AUTH11 3/3, formatting, package/contract/index/catalog Markdown, and diff hygiene. |

The earlier workspace Clippy, doctest, dependency, and 2,481-test full-profile
evidence is reused under the canonical evidence-reuse rule: the only executable
source delta after that run is the expected digest literal in the focused
authority test, which the exact-byte focused Clippy and 17-test run exercise.
The oracle delta is whitespace-only, its output is byte-identical, and no
production source or other workspace test changed. Both science reviewers
confirmed this final digest-only delta before terminal verification.

## Terminal Verification and Archive

| Gate | Result |
| --- | --- |
| independent terminal verifier A | PASS: no unresolved material finding. |
| independent terminal verifier B | PASS: no unresolved material finding. |
| kickoff prompt archival | PASS: byte-for-byte archived SHA-256 `9252d2ce3fb553d598099762319cb502ae2a06ad2520b0da9ce31c09485f1c5c`. |

The package is complete and releases V3 implementation authority only.
