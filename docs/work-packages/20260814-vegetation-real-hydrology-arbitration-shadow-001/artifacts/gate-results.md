# Gate Results

Status: accumulating; failures and retries are retained.

## 2026-08-14 intake and architecture freeze

- `Ran: git status --short --branch` — PASS; clean `main`, local branch ahead
  of `origin/main` by the two intended local campaign commits.
- `Ran: tools/agents/find-agents --for ...` — PASS; applicable root, crate,
  test and work-package instructions recorded.
- `Static: production hydrology and vegetation-water traces` — PASS for design
  freeze; identified the seeded pre-hydrology day frame and the current missing
  OFE/source identity in Rust.
- No production Rust gate is claimed before implementation.
- `Ran: markdown-doc lint docs/...` — FAIL, invalid positional CLI syntax;
  preserved as a tooling invocation error, not a documentation failure.
- `Ran: markdown-doc lint --path docs/...` — PASS, 13 files, zero errors and
  zero warnings.
- `Ran: git diff --check` after architecture artifacts — PASS.
- `Ran: markdown-doc lint --path docs/...` after architecture artifacts —
  PASS, 17 files, zero errors and zero warnings.

## 2026-08-14 implementation loop

- `Ran: cargo test -p openwepp-hillslope-orchestrator
  vegetation_real_hydrology_shadow` — initial FAIL because two concurrently
  authored local test modules had the same name; retained and corrected.
- Same focused unit command — PASS, 8/8.
- `Ran: cargo test --test vegetation_real_hydrology_shadow_contract` — PASS,
  2/2; includes the actual V7 potential/authorization/fixed-cap public phase
  and production selector exclusion.
- `Ran: cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D
  warnings` — initial FAIL on one production line-count warning and test-only
  cast/float warnings; retained and corrected without changing physics.
- Same strict Clippy command — PASS.
- `Ran: cargo check` for kernel-contract, vegetation, biogeochemistry and
  hillslope-orchestrator — PASS for all four affected crates.
- `Ran: cargo clippy` strict affected-crate commands — PASS for all four
  affected crates.
- `Ran: cargo nextest run --test vegetation_real_hydrology_shadow_contract
  --profile quick` — PASS, 2/2.
- `Ran: cargo nextest run --test c3_vegetation_implementation_contract
  --profile quick` — PASS, 17/17.
- `Ran: cargo nextest run --test vegetation_boundary_authority_contract
  --profile quick` — PASS, 26/26.
- `Ran: cargo nextest run --test
  auth11_required_suite_obligation_guards_contract` — PASS, 3/3.
- `Ran: cargo nextest run -p openwepp-vegetation --profile quick` — PASS,
  225/225.
- `Ran: bash tools/release/check_authority_suite_antievasion.sh` — PASS.
- `Ran: check_science_contract_admission ... --worktree` — initial FAIL because
  the new science surface lacked an impact-map binding; retained and fixed.
- Same admission command — PASS, 45 contracts and 2 changed science surfaces.
- `Ran: cargo test -p openwepp-hillslope-orchestrator --quiet` — FAIL on an
  unrelated existing parallel global-state race; the named failing test passed
  alone. A serial retry was terminated after it stalled in long routing tests.
- `Ran: cargo nextest run -p openwepp-hillslope-orchestrator --profile quick`
  — PASS, 502/502; three known routing tests were slow, total 142.517 s.
- `Ran: cargo fmt --all -- --check` and `git diff --check` — PASS.

## 2026-08-14 independent review and remediation

- `Ran/Static: initial independent hydrology review` — NO-GO with findings
  `HYD-REV-001..007`; findings and observed evidence are retained in the
  review report and disposition artifact.
- `Ran/Static: initial Rust review intake` — FAIL on incomplete seeded-frame
  extraction, zero-demand transaction binding, interval/snapshot joins,
  full-depletion representation, error categories, impact binding and exact
  arbitration correspondence. All findings were accepted.
- `Ran: cargo test -p openwepp-hillslope-orchestrator
  vegetation_real_hydrology_shadow --quiet` after remediation — PASS, 11/11.
- `Ran: cargo nextest run --test
  vegetation_real_hydrology_shadow_contract --profile quick` after
  remediation — PASS, 3/3.
- `Ran: cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D
  warnings` after remediation — PASS.
- Remediation now uses the production `seed_day_frame()` state point, a
  production-owned withdrawal authorization endpoint backed by the one shared
  kernel proportional primitive, and the production-owned exact layer debit
  primitive. Partial frost is typed unsupported and cross-snapshot joins fail
  before the V7 solve.
- Fresh hydrology and Rust re-reviews are pending; these focused passes do not
  yet constitute terminal disposition.
- `Ran: check_science_contract_admission ... --worktree` after adding
  `SC-WATBAL-001` impact bindings — FAIL because that contract is still
  `draft/in_review`; the failed authority attempt is retained. Changed
  transaction surfaces were instead bound atomically to the admitted
  `SC-VEGETATIONTRANSACTION-001` without changing contract maturity.
- Same admission command after correction — PASS, 45 contracts and seven
  changed science surfaces.
- `Ran/Static: hydrology re-review` — executable findings closed; initial
  documentation-only NO-GO on one stale bounded-fingerprint statement. The
  statement was corrected and final re-review is pending.
- `Ran/Static: Rust re-review` — NO-GO on an independently reproduced
  binary64 aggregate authorization overdraw and inconsistent zero-supply
  reason classification. Both findings were accepted.
- `Ran: cargo test -p openwepp-kernel-contract proportional_ --quiet` after
  shared arithmetic correction — PASS, 4/4. The exact failing binary64 vector
  is now a regression and canonical allocation gives the final positive
  requester the bounded remainder without authorizing a zero request.
- `Ran: cargo test -p openwepp-vegetation water_phase --quiet` — PASS, 4/4.
- `Ran: cargo test -p openwepp-hillslope-orchestrator
  vegetation_real_hydrology_shadow --quiet` — PASS, 11/11.
- `Ran: cargo nextest run --test
  vegetation_real_hydrology_shadow_contract --profile quick` — PASS, 3/3.
- Final Rust re-review remains pending.
- `Ran: focused remediation sequence` — one intermediate FAIL, 10/11 shadow
  tests, exposed the zero-supply closing-remainder branch returning a typed
  operand failure. The failed attempt is retained; the shared allocator was
  corrected to skip closing adjustment when the canonical authorized sum is
  already zero.
- `Ran: cargo test -p openwepp-kernel-contract --lib --quiet` on the subsequent
  exact bytes — PASS, 55/55, including aggregate-overdraw, per-request
  overauthorization, derived overflow, zero supply and signed rounding
  poisons.
- `Ran: cargo test -p openwepp-vegetation water_phase --quiet` — PASS, 6/6,
  including eligible/excluded reason precedence and canonical ending
  reconstruction.
- `Ran: cargo test -p openwepp-hillslope-orchestrator
  vegetation_real_hydrology_shadow --quiet` — PASS, 13/13, including all-maxima
  finalization and signed-zero request/supply/frost vectors.
- `Ran: strict Clippy` for kernel-contract, vegetation and
  hillslope-orchestrator after the final numerical corrections — PASS.
- `Ran: check_science_contract_admission ... --worktree` — one intermediate
  FAIL identified missing exact binding for the newly centralized subsurface
  aggregate caller; binding was added. Final rerun PASS, 45 contracts and nine
  changed science surfaces.
- `Ran: markdown-doc lint --path ...` — PASS, 22 files, zero errors and zero
  warnings.
- The first comparator PASS preceded the final review-driven numerical diff;
  it is retained as historical evidence.
- `Ran: exact-terminal comparator rerun` — PASS with no retries: all four
  affected checks and strict Clippy gates, 17/17 implementation-contract,
  26/26 vegetation-authority, 3/3 AUTH11, 3/3 public Child-2 integration and
  507/507 orchestrator quick tests. Admission reported 45 admitted contracts
  and nine changed science surfaces. Anti-evasion, formatting, diff hygiene
  and package Markdown lint (22 files) also passed. Raw logs are retained at
  `/tmp/openwepp-child2-comparator-20260814-20260814-113603` and the full
  command record is appended to `comparator-results.md`.
