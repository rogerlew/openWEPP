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

## V5 Capped-Pass Authority Continuation

Status: `COMPLETE / V5 focused, heavy, review, and terminal gates passed`

Evidence mode: `Static; Ran evidence will be appended`

All V3 failures, retries, passes, reviews, and terminal verification above are
preserved. V5 command results must be appended here; a successful retry will
not replace a failed or incomplete attempt.

| Gate | Result |
| --- | --- |
| reopen base | PASS / Static: `2685a1ea9fcfd51fef426eeb4c3685b419b2f768` |
| V3 checkpoint preservation | PASS / Static: completed V3 disposition, archived prompt, review, gate, and verifier artifacts retained unchanged. |
| parent cap-authority audit | ACCEPTED / Static: missing fully coupled cap-active vector, equality convention, independent law/cap operands, generalized residual/Jacobian, and capped diagnostic ordering. |
| initial V5 package Markdown lint | PASS / Ran: `markdown-doc lint --path docs/work-packages/20260812-c3-woody-potential-pass-authority-001`; 36 files, 0 errors, 0 warnings. |
| initial V5 diff hygiene | PASS / Ran: `git diff --check`. |
| V4 predecessor freeze | PASS / Ran: exact digest `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`. |
| initial V9/V5 canonical authority and definition candidate | SUPERSEDED / Static+Ran: V9 imported exact V4 and the byte-identical V5 definition copies hashed to `c9609f5659770da42dd6fa73f2830b6bf6173eba3eef83024f3aec56b3dc31bf`; later review corrections changed the candidate bytes. |
| initial independent fixture regeneration and digest identity | SUPERSEDED / Ran: `.venv/bin/python .../reference_calculator_v5.py` regenerated vectors `50bb74575aa4596c89b986f84bd25a415c24a88c7232a418bc7b0fbd5e85e73d`, generator `104be4a2f96ed8c78a92e500e5be25bd155aa345b2878382438a68deb71bdde7`, and definition `c9609f56...`; these were not the final frozen bytes. |
| mixed active set and equality/near-tie vectors | FAIL / superseded evidence: the initial accepted mixed vector obtained a law-active layer only by using `A>D`. A deterministic valid-space audit is required before deciding whether a positive law-active branch is reachable under `A<=D`, monotone uptake, and redistribution rejection. |
| fully authorized reduction and unavailable-layer vectors | FAIL / superseded evidence: the initial oracle used `A>D` to create a nonbinding branch, which violated the transaction bound. Accepted review correction requires `A=D`, equality-active branches, exact Stage-A value reduction, and explicit unavailable-layer vectors. |
| cap-active failure diagnostics and rollback | PASS / Ran: singular, iteration-limit, 20-halving backtracking-exhaustion, and authorization-domain paths were executed; candidates are null and beginning-state digests are byte-identical. |
| V4-to-V5 identity rebind and stale-V4 poison | PASS / Ran: payload bytes are identical, V5 model/configuration/state identities are distinct, and stale V4 identity rejects. |
| named poison discrimination | PASS / Ran: all named amount/rate, area/time, sequential-clamp, branch, ordering, debit, borrowing, and typed identity poisons executed and discriminate. |
| first independent V5 verifier run | FAIL / Ran: stale candidate hashes after review remediation. |
| second independent V5 verifier run | FAIL / Ran: verifier lacked its canonical-byte helper. |
| third independent V5 verifier run | FAIL / Ran: verifier used a path-digest helper for in-memory bytes. |
| fourth independent V5 verifier run | FAIL / Ran: complete V4 preimage includes source identities, while migration payload correctly excludes and rebinds model/configuration identities. |
| fifth independent V5 verifier run | FAIL / Ran: `sequential_clamp_potential_q` was non-discriminating because a post-hoc minimum aliases the capped layer flux; accepted review correction requires a distinct coupled endpoint poison. |
| first frozen-candidate independent V5 reviews | NO-GO / Ran: candidate definition `1b5d05ec43b13ce28a1007940ffc774b83c05eca09c23aa541d14eee7c4464e5`, vectors `b243658fd5f8230b6d7b03f64fa74472a5ac72dcbfa39faee278a7fae62ab3b9`, and generator `77f19fe5cf6660d91dfa072ff99ac07ec4bba735d69a317a0dab9331031d738f` reused accepted operands in failure diagnostics, omitted a distinct accessible zero-root vector, exposed anonymous failure residuals, and omitted required poison alternatives. The candidate is superseded; the NO-GO remains historical evidence. |
| remediated V5 two-regeneration freeze | PASS / Ran: two consecutive generator executions produced byte-identical definition `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`, vectors `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`, and generator `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`. |
| remediated independent V5 verifier | PASS / Ran: exact 27-poison inventory, actual failed-iterate operands, tie-derived active lists, typed residual records, distinct accessible zero-root, V4-to-V5 identity rebind, hydrology debit, and owner rollback reconstructed independently. Derived V5 configuration `ab3ede8d493859d1b70f8ae0014f6cdbaafac38f9bea83f0470c9dc5275ace26`; state `fa7348db0e99eae3dbe843500e51d0b3cf01f732827fc93d3117a9ca2ab79c9c`. |
| remediated focused verifier/unit/anti-evasion/AUTH11 gates | PASS / Ran: independent verifier, SC unit compliance, authority anti-evasion, and 3/3 AUTH11 tests passed on frozen bytes. |
| pre-promotion vegetation authority suite | FAIL / Ran: 16/17 passed; `canonical_schema_and_registry_entry_are_bound` correctly rejected the still-`in_review/draft` registry row because the suite requires `approved/active`. This lifecycle-order failure is preserved and must pass after dual-review promotion. |
| frozen-byte independent Review B | GO / Ran: exact definition `0ee6a50d...`, vectors `6f5e9554...`, and generator `4c3a1cfc...`; all prior findings resolved, with no unresolved science, closure, diagnostic, poison, rollback, migration, or protected-byte finding. |
| frozen-byte independent Review A | GO / Ran: exact definition `0ee6a50d...`, vectors `6f5e9554...`, and generator `4c3a1cfc...`; all prior findings resolved, with no unresolved material science, numerical, migration, or closure finding. |
| admission after dual-review promotion | PASS / Ran: science-contract admission reported `A0_ADMITTED`, 45 contracts, authority digest `393d3c06aa1dafacb4a174230d5609bda3b0d2ec1e22d127dd8f2a49a5a1e2de`; unit, anti-evasion, and 3/3 AUTH11 gates also passed. |
| first post-promotion vegetation authority retry | FAIL / Ran: 16/17 passed; registry compatibility requires its historical evidence/date columns to remain `static` and `2026-08-12`. The V5 status promotion was correct; only those two registry metadata fields were restored. |
| second post-promotion vegetation authority retry | FAIL / Ran: 16/17 passed; the registry compatibility test also requires the predecessor token `OPENWEPP_C3_WOODY_V3` literally. The row's compressed `V1--V4` wording was expanded without changing authority. |
| final focused vegetation authority suite | PASS / Ran: 17/17 tests passed after lifecycle-compatible registry metadata was restored. |
| first Markdown-lint invocation | FAIL / tooling: `.venv/bin/markdown-doc` does not exist in this checkout; no document validation ran in that invocation. |
| focused Markdown, formatting, and diff hygiene | PASS / Ran: system `markdown-doc` validated the contract, index, and 38 package files with zero findings; `cargo fmt --all -- --check` and `git diff --check` passed. |
| first V5 heavy comparator batch | FAIL / Ran: workspace Clippy, doctests, dependency policy, formatting, diff hygiene, admission, V5 regeneration/verifier, unit, anti-evasion, AUTH11, and vegetation authority gates passed. Full workspace nextest ran 2,582 tests: 2,581 passed, 33 skipped, and one infrastructure-sensitive source-contract test failed because the timestamped TMPDIR made its Unix socket path exceed `SUN_LEN`. Two Markdown invocations also failed to execute because the wrapper placed `lint` before the command. Exact logs are retained in `artifacts/v5-heavy-rerun-20260813T120528Z/`; bounded short-TMPDIR and corrected-command retries are required. |
| bounded heavy remediation, first filter | FAIL / Ran: the fully qualified nextest expression matched zero tests. This was a filter-shape failure; no test ran. |
| bounded heavy remediation, exact failed test | PASS / Ran: with short TMPDIR `/home/roger/v5t`, `test(=paths_symlinks_and_special_entries_fail_closed)` ran the intended source-contract test and passed in 24.027 s; 2,614 other tests were skipped by the exact filter. Correct package/index Markdown and SC-unit commands also passed. Logs are retained in `artifacts/v5-short-remed-20260813T130609Z/`. |
| Critical heavy closure and exact terminal reconciliation | PASS / Ran: workspace Clippy, 2,581 unaffected full-profile tests, exact short-path retry of the sole infrastructure-sensitive test, doctests, dependency policy, formatting, diff hygiene, admission, oracle/verifier, unit, anti-evasion, AUTH11, authority tests, and Markdown all pass. Demonstrable exclusion is the Unix-socket path length only; no production or science byte changed between full and bounded runs. Terminal reconciliation is recorded in `terminal-diff-reconciliation.md`. |
| terminal-verifier focused rerun and current admission digest | PASS / Ran: both verifiers independently reran focused gates; current exact authority digest is `b9241219ac2d01cdc80a6c7787d814cc8462382edc08f1db215c889cfa37132a`, replacing neither the earlier `393d3c06...` evidence nor its then-current bytes. |
| two fresh terminal verifiers | PASS / Ran: both independent verifiers returned PASS with no unresolved material finding on exact V5 bytes and reconciled heavy evidence. |
| focused authority/admission/unit/anti-evasion/AUTH11 gates | PASS / Ran: see exact command rows above; no focused finding remains. Workspace-wide Clippy is assigned to the heavy comparator. |
| two fresh independent V5 science reviews | GO / Ran: both reviewers approved exact `0ee6a50d...` / `6f5e9554...` / `4c3a1cfc...` bytes and confirmed GO after lifecycle-only cleanup. |
| two fresh terminal verifiers and V5 prompt archive | PASS / Ran: both verifiers returned PASS with no unresolved material finding; the active prompt was archived byte-for-byte at SHA-256 `959670289be1dc0c89f5f4acddcebbde03eeda938f6b4e9ab9022bc83326bcf5`. |

No V5 row may borrow V3 or V4 evidence where capped-pass authority bytes,
fixture bytes, or tests changed.
