# Verification B

Status: `COMPLETE — HOLD`

Evidence mode: `Static + Ran`

## Independent scope and source identity

Verification covered package baseline
`792af753e7c936a66352ee69ef5c5c1a18447082`, review-correction commit
`6953a36b881e7167b47c76040208d1024818060a`, and the current bounded
worktree reconciliation. At verification time the only Rust change after
`6953a36b8` was the test-only `open_snow_tail_tests.rs` source binding; the
other worktree changes were package evidence and authority impact-map
generation 37. No production or canonical-contract file differed from the
review-correction commit.

The clean canonical run is attributed by the contemporaneous package record to
commit `6953a36b8`; the commit timestamp precedes the log, and the later
`open_snow_tail_tests.rs` correction is not compiled into the production
dependency of the `openwepp-runner --lib` fixture. The retained log itself does
not print a commit SHA, so this source binding depends on that execution record
rather than an embedded log manifest. I accept it for RB-004 but do not extend
it to the later test/impact-map worktree.

## Canonical authority and current bounded gates

Static inspection confirms `SC-SNOWENERGY-001` version 29 binds
`INV-SNOWENERGY-054` in the canonical branch table, invariant/guard map,
formal vectors 31--33, child obligation map, narrative, and change log.
Candidate density is copied bitwise and remains an exact convergence
coordinate; finalization restart and exactly-once stabilization retain
unchanged `TOL-SNOWENERGY-003`, the 96-iteration cap, exact 60-second floor,
refusal guards, and authentic-only publication.

Ran independently on the current worktree:

- `bash tools/release/check_science_contract_admission.sh --base-ref be40a9435 --worktree`
  — `PASS`, `A0_ADMITTED`, 49 contracts, four science surfaces, authority
  SHA-256
  `ce2befbdb7214be8194f01d3f8645663ce916a232ff476cc21692986034dad1a`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run` with the seven
  affected SnowEnergy integration binaries named by the package — `PASS`,
  47/47, run `d3ac04c8-571d-4240-924d-94e68c1ba92e`.
- `bash tools/release/check_authority_suite_antievasion.sh` — `PASS`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  — `PASS`, 3/3, run `24c33944-d773-493c-8602-eeaf9f6f7582`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(covered_convergence_policy_tests)'`
  — `PASS`, 19/19, run `b2b92fc9-aa04-4ab1-b9a7-73a8db3435c1`.
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(precomputed_terminal_accepted_executor_tests)'`
  — `PASS`, 5/5, run `5bddf65c-b2f9-4a52-97ca-d198b7d0aae3`.
- `git diff --check 792af753e --` and current `git diff --check` — `PASS`.

The current A0 digest supersedes the pre-source-binding v29 digest
`a8828192...` only for the terminal worktree identity; the earlier digest
remains valid historical evidence for clean commit `6953a36b8`.

## Canonical telemetry reconstruction

Retained log
`/tmp/stage3_fp_cold/one-day-review-correction.log` has SHA-256
`c6ba3bdb3a9bfd5d0bdd35e83fdb2f448dcd97dba67d70811d418e64cb856417`.
Its timing sidecar has SHA-256
`6464da227ef86a94fc8d5babe66658508eb6ebe2ad9d8076025fce427d5f21eb`.
Independent parsing of all 48 parent records gives:

- 696 direct trials = 491 accepted + 205 rejected;
- accepted widths `{60:49, 120:92, 180:320, 240:17, 300:3, 420:3,
  480:1, 900:3, 1800:3}` seconds, summing to 491 supports and exactly
  `86_400_000_000_000 ns` (one day);
- rejection classes: 160 phase, 45 other, zero event, zero both;
- 32 fixed-point cap signatures, including two finalization cold-content caps;
  the seven signature counts sum exactly to 32;
- 45 scaled comparison rejections and zero discrete comparison rejections;
  the four owner-path counts sum exactly to 45;
- mass residual `3.55271367880050093e-15` versus `1e-9`, energy residual
  `1.39698386192321777e-9` versus `1e-6`, receipt-energy residual
  `9.96351445792242885e-10` versus the binary64 `1e-9` bound, and receipt
  temperature residual `1.07434061646927148e-11` versus `1e-8`;
- body wall `339.10 s`; command elapsed/user/system `416.72/826.43/8.31 s`
  and maximum RSS `6,314,564 KiB`.

The optimization and closure arithmetic in the package are therefore
supported. This verification did not rerun the expensive one-day fixture.

## Full-profile and Clippy failure census

The retained full-profile log SHA-256 is
`dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`.
It starts 3,628 tests across 244 binaries under profile `full` and terminates
with exit 100 after `5,022.73 s`. Deduplicating the status records by nextest
ordinal gives exactly 96 `FAIL` and 29 `TIMEOUT`; hence 3,503 passed and all
3,628 started tests received an outcome. Nextest separately reports 48 tests
and five binaries skipped before the started inventory, so “zero not-run” is
truthful only for the 3,628 started tests.

The unique failure/timeout census is:

| Surface family | FAIL | TIMEOUT | Total |
|---|---:|---:|---:|
| `openwepp::assurance*` integration binaries | 75 | 0 | 75 |
| `openwepp-assurance` crate tests | 7 | 0 | 7 |
| `openwepp::cli*` integration binaries | 1 | 8 | 9 |
| other `openwepp::*` integration binaries | 2 | 8 | 10 |
| `openwepp-hillslope-orchestrator` | 5 | 2 | 7 |
| `openwepp-runner` library tests | 3 | 7 | 10 |
| runner watershed behavior binary | 3 | 2 | 5 |
| runner CLI-hill binary | 0 | 1 | 1 |
| runner CHANINP-default binary | 0 | 1 | 1 |
| **Total** | **96** | **29** | **125** |

Three orchestrator failures are the stale accepted-endpoint source scan fixed
by the current test-only binding; the independent current 5/5 run closes that
narrow defect. It does not transform the retained full-profile result into a
pass: the other 122 failures/timeouts remain undispositioned, including two
orchestrator snow-free assertions and two orchestrator long-fixture timeouts.
Their location outside the declared write set is not proof that this critical
kernel change could not affect them.

The retained workspace Clippy log SHA-256 is
`aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`
and exits 101 after `7.27 s`. It contains two unique root denied-lint
diagnostics: `filter_map_bool_then` in `openwepp-coupled-time` and
`similar_names` in `openwepp-biogeochemistry`. Cargo stops after those
compilation failures, so this is a complete command failure record but not an
exhaustive census of every possible later workspace lint. No passing
warnings-denied result or governing exception is present.

## Write set, line count, and diagnostic persistence

The actual baseline-to-worktree diff is contained by the amended intended
write set: five covered orchestrator files, the canonical contract and index,
seven compiler-discovered integration consumers, package/catalog files, and
the authority impact map. The current map is generation 37 and now has an exact
critical `SC-SNOWENERGY-001` binding for `open_snow_tail_tests.rs`.

`open_snow.rs` is 2,721 lines and the accepted-endpoint include is 529 lines.
Baseline lines 342--869 and include lines 1--528 both hash to
`97aec7cad748caac7a2b3c6fbf2c1023074495f6b4ce233c95893bb9bd10bdd5`.
The 2,000-line `WARN`, rationale, owner, and next-touch split intent are
recorded; neither file reaches the mandatory 3,000-line threshold.

No added production field, serialization/restart member, receipt member,
public output, environment-controlled trace, or print path exists. The new
stabilization state is a private stack-local boolean and the finalization
iterate is unpublished. Temporary iteration-history tracing is absent from the
terminal diff, so the no-persisted-microstepping-diagnostic claim passes static
verification.

## Finding-closure audit and verdict

- `RA-001`, `RA-003`, `RB-003`, `RB-004`, and `RB-005`: `CLOSED` by v29,
  stateful seam coverage, clean canonical evidence, and corrected line-count
  evidence.
- `RB-006`: `CLOSED` by intended-write-set reconciliation, current helper and
  digest text, corrected line counts, generation-37 impact-map binding, and
  exact current A0 admission.
- `RA-002`, `RB-001`, and `RB-002`: `OPEN` and closure-blocking. The required
  critical full profile and warnings-denied workspace gate both fail.

Verification verdict: `HOLD`. The optimization-specific result, v29 authority,
telemetry arithmetic, closure, source-order correction, write-set bounds, and
absence of diagnostic persistence verify successfully. Package `COMPLETE`
would nevertheless violate the gate non-deferral rule while the full profile
and warnings-denied Clippy remain `FAIL`. The package/review language that
retains `NO-GO`/`HOLD` and does not claim those failures are waived is
truthful.
