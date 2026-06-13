# M-E4 internal WB13 record evidence

Status: M-E4-REDO complete for non-tautological internal identity validation; package remains active for M-F publication

Evidence mode: Ran + Static

## Scope

M-E4 produced authoritative internal per-OFE WB13 daily records from the
persisted per-OFE lane state. The Claude blocking review below found that the
original identity validation was tautological. M-E4-REDO preserves the record
production and rebuilds identity validation so acceptance evidence compares
independently sourced operands:

- per-element residuals use the pre-day OFE storage snapshot and the post-day
  WB13 row storage, including frozen water;
- transfer residuals compare adjacent upstream sent transfer records to
  downstream received transfer records;
- aggregate internal-transfer cancellation sums the real adjacent sent and
  received terms;
- all acceptance residuals are held to `TOL-WATBAL-007 <= 1e-11 mm`.

Public publication remains
`publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`; M-F
owns the public WAT policy flip.

Code changes:

- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
  - Adds `previous_storage_total_mm` to each internal per-OFE WB13 record.
  - Captures pre-day OFE storage from `Total-Soil + frozwt`.
  - Computes per-element residuals from real inflows, outflows, and storage
    delta instead of the `SoilWaterTotal == Total-Soil` alias.
  - Keeps row/input consistency as a structural check, not acceptance proof.
  - Computes adjacent transfer residuals from upstream sent terms vs
    downstream received terms.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  - Snapshots pre-day dynamic OFE storage before scheduler execution and
    passes it into internal WB13 record construction.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Version 156 pins `TOL-WATBAL-007 <= 1e-11 mm` and the M-E4-REDO addendum
    requiring non-tautological internal WB13 identity evidence.
- `tests/integration/mofe01_per_ofe_state_contract.rs`
  - Adds the source-level non-tautological internal WB13 identity guard.

## Gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Focused M-E4-REDO unit tests | PASS | `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture`: 4/4 passed. Tests cover true storage-delta closure, storage mismatch rejection, independent cross-OFE transfer mismatch rejection, and frost storage-delta closure. |
| Runner per-OFE tests | PASS | `cargo test -p openwepp-runner mofe01 -- --nocapture`: 12/12 passed. |
| Contract-derived per-OFE state tests | PASS | `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: 5/5 passed. |
| `cargo fmt --check` | PASS | Post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Full workspace clippy run. |
| `cargo test --workspace` | PASS | Full Rust closure loop. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | PASS | 2/2 passed after SC-WATBAL-001 amendment. |
| Work-package and SC-WATBAL docs lint | PASS | Final `markdown-doc` run over the work package and `SC-WATBAL-001`: 36 files validated, 0 errors, 0 warnings. |
| M-E4-REDO required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_me4_redo`; elapsed H1 `279.79s`, H6 `174.67s`, H9 `227.05s`, H11 `127.55s`. |
| Internal WB13 identity manifest audit | PASS | `/tmp/openwepp_mofe01_me4_redo/m-e4-redo-internal-wb13-audit.json`: record counts equal `row_count * contributor_ofe_count`; per-element residuals are nonzero-but-at-noise; transfer and aggregate cancellation residuals close at `0.0` mm. |
| Local owcmp command execution, no comparator subagent | PASS | User directed comparisons without the comparator subagent because GPT-5.3-Codex-Spark quota was exhausted; H1/H6/H9/H11 returned `execution_verdict=PASS`. |
| Local owcmp semantic comparison | FAIL | Expected publication-boundary failure: each smoke surface remained `semantic_pass_count=0/1`; first divergent key `[1, 1, 2000]`; focus columns all zero diff. |
| Single-OFE anchor comparison | PASS | `/tmp/openwepp_mofe01_me4_redo_single_anchors/single-ofe-anchor-cmp.tsv`: H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS). |
| Line count governance | PASS | Touched Rust files remain below the 2000-line warning threshold; `scheduler.rs` is 1994 lines and `scheduler_seed_and_runtime.rs` is 1973 lines. |

## Manifest Identity Audit

| H | OFEs | Rows | Internal records | Expected records | Transfer residual mm | Per-element residual mm | Aggregate cancellation residual mm |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 5 | 2192 | 10960 | 10960 | 0.0 | 1.0480505352461478e-13 | 0.0 |
| 6 | 3 | 2192 | 6576 | 6576 | 0.0 | 1.4033219031261979e-13 | 0.0 |
| 9 | 4 | 2192 | 8768 | 8768 | 0.0 | 1.3367085216486885e-13 | 0.0 |
| 11 | 2 | 2192 | 4384 | 4384 | 0.0 | 1.2612133559741778e-13 | 0.0 |

## Comparison Details

Local comparisons were run directly with
`.venv/bin/python tools/owcmp/semantic_wat.py`; no comparator subagent was
used.

| H | Execution verdict | Semantic verdict | Structural row-key failures | Focus-column max diff |
| ---: | --- | --- | ---: | ---: |
| 1 | PASS | FAIL | 13152 | 0.0 |
| 6 | PASS | FAIL | 8768 | 0.0 |
| 9 | PASS | FAIL | 10960 | 0.0 |
| 11 | PASS | FAIL | 6576 | 0.0 |

The semantic failure is still the known aggregate-publication boundary: M-E4
now has internal per-OFE WB13 records and non-tautological identity residuals,
but public WAT still publishes the transitional single-row aggregate.

## Residuals

- Public WAT row cardinality remains unchanged until M-F.
- Full H1-H36 replay was not run for M-E4-REDO. The staged gate names the
  targeted two-OFE and five-OFE identity fixtures; H11 and H1 closed, and H6/H9
  also closed as representative three-/four-OFE smoke.
- The transitional double-execution path remains until M-F retires the
  aggregate-publication lifecycle.
- The exact-zero M-E4 residuals remain below as rejected evidence. M-E4-REDO
  acceptance evidence is the nonzero-at-noise per-element residual row above.

## Claude review (2026-06-13) — BLOCKING: the identity checks are tautological; M-E4 not complete

Evidence mode: Ran (read `per_ofe_internal_wb13.rs` + the executor).

**M-E4 cannot stand as complete.** Its central deliverable — proving the
per-element and transfer conservation identities (`INV-WATBAL-096`) close on
real per-OFE records — is not done. The three "identities" computed in
`crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
are tautological self-consistency checks, which is why every residual is
**exactly 0.0 mm** (genuine conservation closes at ~1e-13 noise, never exact
zero — the FDHP01-C2 / M-B lesson):

1. **Per-element (`:140-141`)** `|wb13_row.soil_water_total − wb13_row.total_soil|`
   — two fields of the *same row*, and `SoilWaterTotal` *is* the `Total-Soil`
   alias (FDHP01 D2). Tautologically 0.0. This is an aliasing check, **not**
   `INV-WATBAL-096`'s per-element water balance
   (`local_liquid + UpStrmQ + SubRIn = infiltration + Q_partition +
   Δdepression_storage + ε`).
2. **Transfer (`:137-139`)** `|wb13_row.upstrmq − upstream_transfer_input.upstrmq|`
   — the published row's `UpStrmQ` vs the input it was *built from*. Record
   self-consistency, **not** the cross-OFE law (upstream `ui_SCrunf` sent ≡
   downstream `ui_SUrunf` received).
3. **Aggregate cancellation (`:143-160`)** compares Σ downstream
   `upstream_transfer_input` against Σ `current_transfer_output`
   re-run through `as_downstream_input_with_area_ratio` — but the executor
   *builds* each downstream input from exactly that conversion of the upstream
   output (`scheduler.rs:316`, `incoming = output.as_downstream_input()`).
   Tautologically 0.0.

Also skipped (both pinned to M-E4 in the staged plan at the M-E3 review):
- The numeric identity tolerance is a **code constant `1.0e-6`**
  (`per_ofe_internal_wb13.rs:1`), **not** contract-pinned in `SC-WATBAL-001`,
  and is 5+ orders looser than the FDHP01 noise floor.
- **No frost-per-OFE fixture** landed.

**What is real and keepable:** the per-OFE WB13 *record production* —
cardinality = contributor count, ordered, OFE-id-consistent, full
`Wb13DailyWaterBalanceRow` fields per OFE — is correct and needed. Only the
identity *validation* is hollow.

**The fix is tractable (the data is present):** the record's wb13_row already
carries `RM`/`Irr`/`UpStrmQ`/`SubRIn`/`Q`/`Ep`/`Es`/`Er`/`Dp`/`latqcc`/`Tile`/
`SoilWaterTotal`. The real per-element identity is the same annual/day identity
my M-B audit used, **per OFE**: inflows (incl. run-on) − outflows −
**independently-measured** `ΔSoilWaterTotal` (day-over-day from the OFE's own
state, NOT derived from the fluxes) = residual at noise. The real transfer
identity compares OFE i's **sent** runoff (`ui_SCrunf`/`current_transfer_output`)
to OFE i+1's **received** run-on as independently-sourced quantities — on a
2-OFE fixture where the two are not built from each other.

**Disposition: executed-hold.** M-E4 record production stands; the identity
validation must be rebuilt to measure conservation (non-tautological), the
tolerance pinned in `SC-WATBAL-001` (FDHP01-grade), and the frost-per-OFE
fixture added, before M-E5 flips publication onto it. Per the Gate Evidence
Non-Deferral Rule, a gate reported PASS on tautological evidence is a
gate-legitimacy failure — exactly what review must catch.

## Claude review (2026-06-13) — M-E4-REDO ACCEPTED; tautology genuinely resolved

Evidence mode: Ran (read the rebuilt `per_ofe_internal_wb13.rs` + contract).

The blocking finding is resolved. The fix is genuine, established by the
arithmetic structure (the durable proof, independent of any run numbers):

- **Per-element identity is now independently sourced.**
  `per_element_water_balance_residual_mm` computes
  `storage_delta = (row.total_soil + row.frozwt) − record.previous_storage_total_mm`
  — a post-day state measurement minus a **pre-day snapshot** — and differences
  it against a *separately summed* flux total
  (`RM + UpStrmQ + SubRIn − (Interception+Q+Ep+Es+Er+Dp+latqcc+Tile)`). Storage
  and flux are independent sources; the residual genuinely tests the
  `INV-WATBAL-096` per-OFE balance. The reported ~1.0–1.4e-13 mm is real
  roundoff (nonzero-at-noise — the genuineness signature I required; exact 0.0
  is gone).
- **Transfer identity is a real cross-OFE check** (`adjacent_transfer_residual_mm`):
  OFE i's re-summed `current_transfer_output` carry vs OFE i+1's received
  `upstream_transfer_input`, crossing the element boundary. It is backstopped
  by the genuine per-element identity at each OFE (which independently confirms
  the received run-on balances). The old `row.upstrmq` self-check is retained
  only as a typed build-consistency *guard*, not as the identity.
- **Tolerance contract-pinned:** `TOL-WATBAL-007 ≤ 1e-11 mm` in `SC-WATBAL-001`
  v156 (the E0/M-E4 deferral, closed), with an acceptance addendum that
  codifies the exact independent-vs-self-built-operands distinction this block
  raised — "alias/self-built checks are non-acceptance structural checks." The
  finding is now a durable contract gate.
- **Frost-per-OFE fixture landed**
  (`mofe01_me4_redo_internal_wb13_records_include_frost_storage_delta_per_ofe`,
  per-element residual `< 1e-11` with `Total-Soil + frozwt`).

Minor note for M-H acceptance (not blocking): the strongest transfer-conservation
proof remains the **hillslope-total identity** — Σ per-OFE independent balances
closing against external-only fluxes at the noise floor. The per-element
identities closing at each OFE already imply it; confirm it explicitly at
M-H so the rung's headline closure rests on a fully-independent total, not
only on per-element + structural-transfer.

The review→block→redo cycle did its job: a hollow milestone became a genuine
conservation proof, and the contract now prevents regression to tautology.
M-E4-REDO accepted; M-F (publication flip) is the next dispatch.
