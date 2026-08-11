# Review Agent B

Status: `complete — PASS; all findings closed`

Evidence mode: `Static + Ran`

This is a fresh post-correction hydrology/science review of the actual WAT5
runtime, output, runner wiring, focused tests, canonical contracts, and
refreshed terminal evidence. The verdict was derived independently from those
surfaces rather than another review artifact.

## Independent execution

- Focused orchestrator WAT5 tests passed 17/17, nextest run
  `f5bcc98a-08b3-4d45-a541-ff52c688732c`.
- The five named WAT5 contract, property, typed round-trip, HBP/routing
  exclusion, and protected-peak targets passed 13/13, nextest run
  `245c2f80-e58d-46da-bcf5-66cb7a9ebf0c`.
- Independently read
  `/home/workdir/openwepp-wat5-terminal/on/output/H61.wat-subhourly.parquet`.
  It has 26 columns, 24 rows, schema metadata matching version 1.0, global bins
  `0..23`, exact 300-second duration/start relationships, and no
  `hour_index = floor(subinterval_index/12)` violation.
- Independently reconstructed its two authoritative hours. Both closing-depth
  residuals are exact zero; producer-recorded residuals are
  `-3.469446951953614e-15 mm` and exact zero. All 24 exponent, power-rate, and
  power-duration values are null.
- Independently recomputed the WAT5 SHA-256 as
  `71f943f9ff30f74846f74d521c66ecee8dce64f7ddcd5fe2c64e4d12008ed938`.
  The manifest binds that hash to release binary SHA-256
  `f264661135cde810ff4914df80f5aba1e176349af89537794f18187e49bbc85a`.
- Independently compared enabled/disabled protected outputs under
  `/home/workdir/openwepp-wat5-terminal/{on,off}`. HBP, PASS, WAT, and loss
  JSON are byte-identical in each pair.
- Read the p102 terminal failure. It exits through
  `WAT5-E-001 positive additional supply lacks 300-second timing`, publishes no
  WAT5 target, and leaves no WAT5 temporary file.

## Prior finding disposition

| Finding | Disposition | Verified correction |
|---|---|---|
| `WAT5-B-001` — no boundary-split Green-Ampt advance | `closed` | `compute_wb14_subhourly_profile` clones and splits every hyetograph interval at exact 300-second boundaries before chronological Green-Ampt advancement. Cumulative infiltration is continuous across pieces. The delayed-ponding vector proves exact-zero first-bin excess and positive second-bin excess; refreshed real rows now show the expected evolving infiltration/generation shape rather than proportional copies. |
| `WAT5-B-002` — tolerance used as a zero/source classifier | `closed` | Missing-producer, additional-supply, raw-support, authoritative-depth, and sparse-activity decisions use exact positive/zero predicates after dimensional validation. Tests retain positive rainfall below the closure tolerance and reject positive untimed supply below it. The tolerance is confined to residual adjudication. |
| `WAT5-B-003` — hourly-reset subinterval index | `closed` | Runtime publishes the day-relative bin directly. The hour-crossing vector emits bins 11 and 12, and the refreshed Parquet emits bins `0..23` with the canonical hour relationship and day-relative start time. |
| `WAT5-B-004` — no composed hour/day closure validation | `closed` | WAT5 now validates raw event closure including depression-storage change, each composed `closed WB14 + saturation` hour, and the accumulated day through `WAT5-E-004`. The published hourly residual is the composed closing-surface residual. Rain-plus-saturation, raw-storage, and explicit hour/day failure vectors exercise the guards. |

## Science and claim-integrity assessment

- Dimensional custody is coherent: runtime depths remain meters, runtime rates
  remain meters per second, and named publication helpers perform `m -> mm`
  and `m s^-1 -> mm h^-1` once. Parquet metadata identifies depth, rate,
  sparse-zero, isolated-raw, hourly-closed, and saturation-hold semantics.
- Raw replay now has the required temporal meaning. The p61 sequence begins
  with full infiltration and zero generation, then evolves with cumulative
  Green-Ampt state; it is no longer compatible with uniform post-solve
  redistribution.
- Hourly mass authority remains the unchanged WB14/WB19 ledger. WAT5 scales
  raw shape within each hour, labels saturation as
  `hourly_zero_order_hold`, and neither claims nor supplies finer saturation
  timing.
- Source completeness is bounded truthfully. Every positive hourly-only
  runon/melt supply is rejected rather than spread or rainfall-shaped. The
  accepted domain is local rain-timed diagnostic water plus labeled hourly
  saturation return; multi-OFE runon, melt, frost-release timing, HBP, and
  routed watershed behavior remain outside adoption.
- Diagnostics-off/on byte identity and static consumer exclusion support the
  noninterference claim: WAT5 does not feed peak, HBP, transfer, routing,
  erosion, or persistent water state.
- Erosion `NO_ADOPTION` remains intact. No exponent, candidate selector,
  candidate solve, Topanga outcome, or production cutover exists. All
  power-equivalent fields are null, the method is
  `water_only_no_erosion_adoption`, and production erosion retains its hourly
  mean.

## Targeted terminal A0 re-review

Ran and independently inspected after the terminal A0 correction:

- `check_science_contract_admission.sh --base-ref c9f28a7d... --worktree`
  admitted 43 contracts and all 17 WAT5 science paths, with worktree
  fingerprint
  `6f95845b5065e9134cded858e69ed359b2e42bd32318f800f87801d4088d1298`.
- A broader focused authority/contract set passed 34/34, nextest run
  `03cf40e1-d099-42b4-96e9-6e97dc399940`.
- The final subhourly/advisory/auth11 set independently passed 12/12, nextest
  run `3a63cdad-31e5-41b9-8cf4-623204765075`.
- `check_authority_suite_antievasion.sh`, scoped SC unit compliance, Binding
  Exposure validation, JSON parsing, contract Markdown, and diff whitespace
  checks passed.
- The terminal post-A0 full workspace passed 2,380/2,380 with 33 skipped,
  nextest run `b920db77-070f-4686-a7bf-2e2727094374`; post-A0 workspace
  doctests passed with zero failures.
- Final write-set reconciliation is exact: 87 declared owned paths and 87
  observed package paths, with no owned/unowned difference.

Static authority disposition is otherwise correct:

- `SC-WATBAL-001` is unchanged and retains its `in_review`/`draft` lifecycle;
  it contains no WAT5 invariant or tolerance amendment.
- Approved, active `SC-OUTPUT-WAT5-001` locally owns `INV-WAT5-001..007` and
  `TOL-WAT5-001`; its registry row matches its front matter.
- The contract's same-directory atomic no-replace hard-link publication text
  matches the writer implementation.
- Worktree admission unions the tracked diff with untracked, non-ignored
  paths, so it sees the untracked WAT5 modules and tests rather than producing
  the rejected base-equals-head zero-surface receipt.
- The impact map has exactly 17 atomic WAT5 bindings. The four shared core
  frame, executor, runner-test, and runner-execution paths retain separate
  `SC-PLANT-001` and `SC-OUTPUT-WAT5-001` entries and their respective
  blocking exact-inventory A1 definitions.
- The checker rejects missing bindings, multi-contract entries, duplicate
  contract bindings, provisional/unknown contracts, missing or non-executable
  A1 coverage, and mismatched A3 bindings.

### Closed — `WAT5-B-A0-001`: complete worktree authority fingerprint

The accepted correction adds `str(input_registry)` directly to the worktree
`authority_paths` set. The same sorted loop now hashes the registry path, a
separator, its exact bytes, and a trailing separator alongside the science
paths, contracts, external registry, impact map, A1 definitions, and admission
checker. Therefore any parser-registry content mutation changes the digest
input even when it preserves the derived contract-file set.

The source-level regression
`worktree_admission_fingerprint_includes_every_registry_input` binds
`--worktree`, `str(input_registry)`, the science-contract index, external
registry, impact map, and gate-definition inputs. Direct inspection confirms
that these markers occur in the fingerprint construction rather than an
unrelated code path. Combined with the deterministic byte-hashing loop, this
is sufficient sensitivity evidence for the current implementation; an
additional mutation harness is not required to close this finding.

Fresh exact-worktree admission produced `6f95845b...`, superseding the
incomplete `66477f70...` receipt. The final 12/12 focused set, JSON validation,
and diff check pass. `WAT5-B-A0-001` is closed with no residual authority-input
identity gap.

## Gate-legitimacy check

`PASS` for WAT5 hydrology/science, the 17-path authority disposition, and the
terminal A0 receipt identity. `WAT5-B-A0-001` is closed; no review finding is
open, deferred, or assigned to follow-up.

The post-A0 full-workspace and doctest receipts are current for the final
executable diff, and the owned manifest reconciles exactly at 87/87. No pending
Critical-validation or exact-diff residual remains in this review.

## Recommendation

Approve the corrected WAT5 water product as `DIAGNOSTIC_ONLY` for its explicitly
source-complete domain and preserve erosion `NO_ADOPTION` plus all stated
exclusions. The final A0 fingerprint and focused authority gates pass; no
additional WAT5 process, output-science, authority, Critical-validation, or
write-set correction is required by this review.
