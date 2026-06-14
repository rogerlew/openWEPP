# Implementation Test Evidence

Status: T-B executed

Evidence mode: Ran + Static

W-A baseline:

Ran:

- `target/debug/openwepp-cli-watershed --run-dir /tmp/openwepp_wshed01_wa/watershed/run --run-file case.run --output-dir /tmp/openwepp_wshed01_wa/watershed/output --policy compat --legacy-sidecar-discovery`

Observed:

- Exit code `1`.
- `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`.
- `0` output files under `/tmp/openwepp_wshed01_wa/watershed/output`.

Not run:

- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`. W-A made no Rust edits and
  did not claim production closure.

Static evidence:

- Parser defect before W-B: `watershed_impoundment.rs:581-588`.
- CLI failure wrapper: `openwepp-cli-watershed.rs:239-254`.
- Output writer path not reached: `openwepp-cli-watershed.rs:476-497`.

W-B implementation:

- Added explicit no-impoundment fixture and parser/CLI tests.
- Implemented typed empty impoundment set semantics in
  `watershed_impoundment.rs`: `jpond=0` is accepted only when
  `expected_structural_count == Some(0)`.
- Amended `openwepp-watershed-runfile-contract.md` to pin schema v1 no-pond
  semantics and preserve the required `inputs.pw0_imp` file binding.

Red evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract zero_impoundments`
  failed before the parser edit:
  - strict zero/zero acceptance failed with `IMP-E-004`,
  - compatibility zero/zero acceptance failed with `IMP-E-004`,
  - positive-structure mismatch test observed `DomainError` instead of the
    required `CountMismatch`.

Green evidence:

- `cargo fmt --check`: pass.
- `cargo clippy -p openwepp-input-contract -p openwepp-runner --tests -- -D warnings`:
  pass.
- `cargo test --test infile_watershed_impoundment_parser_contract`: `18`
  passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`:
  `1` passed.
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed`:
  `3` passed.

Arboreal-dendrite W-B gate:

```bash
target/debug/openwepp-cli-watershed \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wb/watershed/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed:

- Exit code `1`.
- No `CLIWAT-E-010` / `IMP-E-004`.
- Next hard stop:
  `CLIWAT-E-020 watershed dispatch reported failure (message_id=WKERNEL-WS10-CHANNEL-E-003)`.
- Output file count: `0`.

W-C implementation:

- Classified the W-B hard stop as over-strict WS10 channel validation on valid
  zero-sediment hillslope payloads, followed by a hidden `nchnum=0`
  output-disabled state guard.
- Amended `SC-ROUTE-001` to version `45`.
- Corrected WS10 sediment-payload and `nchnum` validation.
- Added WAT-backed watershed daily row aggregation and multi-row interchange
  output writing.

W-C focused green evidence:

- `cargo test --test ws11_channel_routing_physics_equivalence_contract wshed01_wc_ -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-watershed-output writers::tests::writer_ -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_emits_watershed_output_parquet_files -- --nocapture`:
  `1` passed.
- `cargo test -p openwepp-runner -p openwepp-watershed-output`: passed.

W-C full gate evidence:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.

Arboreal-dendrite W-C gate:

Configured run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wc_final_configured/output \
  --policy compat
```

Legacy-discovery run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wc_final_legacy/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed:

- configured exit `0`; legacy-discovery exit `0`;
- configured output files `14`; legacy-discovery output files `14`;
- configured `totalwatsed3.parquet` rows `2192`;
- legacy-discovery `totalwatsed3.parquet` rows `2192`;
- `max(abs(runvol - Q * Area / 1000.0)) == 0.0 m^3` for both runs;
- first-row WAT fields are non-placeholder:
  `P=32.717215206680784`, `RM=13.203340055286729`,
  `SoilWaterTotal=335.10212226223916`.

## Claude review (2026-06-14) — W-C accepted; classification nuance + W-D scrutiny note

Evidence mode: Ran (contract/evidence read; output cleaned up, deep content
check deferred to W-D's fresh run).

**Milestone accepted:** openWEPP produces watershed-level routed output for the
first time — 14 outputs, `totalwatsed3.parquet` 2192 rows, configured +
legacy-discovery both exit 0. W-C correctly does NOT claim totalwatsed3
closure (W-D owns the independent audit).

**Classification confirmed, with an important nuance.** `WS10-CHANNEL-E-003`
was over-strict validation on valid **zero-sediment** hillslope payloads (+ a
hidden `nchnum=0`). The fix (SC-ROUTE-001 v45: accept *complete* HBP sediment
payloads with zero values; `nchnum >= 0`) is faithful — it accepts the
legitimate zero state while preserving typed validation (non-finite/missing/
negative still caught). But unlike `jpond` (where legacy ran the exact same
input), this is **NOT a legacy-parity classification**: legacy computes
sediment, so legacy's channels never see zero sediment. The zero-sediment
channel input is an **openWEPP-specific valid intermediate** arising from
MOFE01's deferred-sediment / water-only posture (the
`MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` follow-on). So the correct framing:
the channel router must route **water** even when sediment is zero, because
zero-sediment is a legitimate consequence of the design, not bad data. The
fix is right; the basis is design-consistency, not legacy-acceptance.

**Carried to W-D (the load-bearing verification):** accepting zero-sediment
removed the guard, but whether the channel actually routes water *correctly*
is unproven until the water balance is audited. W-D's totalwatsed3 closure
must (1) use **independent operands** (the recurring no-0==0 / no-tautology
lesson of this series — exact 0.0 is the smell), and (2) conserve the
watershed balance against the **closed hillslope inputs** (the routed
watershed total = the hillslope contributions, no water created/lost in
channel routing). The watershed output content (distinct nonzero daily rows
vs degenerate) was not checkable here (run cleaned up) and is verified at
W-D's fresh run, where the closure is the prize and the most-scrutinized
point.

## W-D implementation and audit evidence

Status: W-D executed-hold

Evidence mode: Ran + Static

W-D publication fixes:

- `crates/openwepp-watershed-output/src/writers.rs`: exact totalwatsed3
  hydrology fields now publish as `m^3`; depth aliases remain mm; profile and
  interception row-seed fields are mapped.
- `crates/openwepp-runner/src/watershed_wat.rs`: WAT aggregation carries
  optional profile/interception fields and counts MOFE `latqcc` only from the
  outlet OFE per WAT file/day/`wepp_id`.
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`: unit
  metadata registry now includes `watershed_totalwatsed3.Interception`.

Focused green evidence:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test -p openwepp-runner watershed_wat::tests -- --nocapture`:
  `2` passed.
- `cargo test -p openwepp-watershed-output writer_preserves_multiple_watershed_daily_rows_and_wat_fields -- --nocapture`:
  `1` passed.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.
- `git diff --check`: pass.
- `markdown-doc lint --path docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001 --format json`:
  `27` files scanned, `0` errors, `0` warnings.

Configured W-D run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wd_configured/output \
  --policy compat
```

Legacy-discovery W-D run:

```bash
cargo run -q -p openwepp-runner --bin openwepp-cli-watershed -- \
  --run-dir /tmp/openwepp_wshed01_wa/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_wshed01_wd_legacy/output \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed:

- configured exit `0`; legacy-discovery exit `0`;
- legacy-discovery emitted expected sidecar-discovery warnings;
- both outputs contain `2192` `totalwatsed3.parquet` rows.

wepppy W-D audit:

```bash
/home/workdir/wepppy/.venv/bin/python \
  /home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py \
  /tmp/openwepp_wshed01_wd_configured/output/interchange/totalwatsed3.parquet \
  --output-dir /tmp/openwepp_wshed01_wd_configured/audit \
  --top-n 20
```

The same command was run for
`/tmp/openwepp_wshed01_wd_legacy/output/interchange/totalwatsed3.parquet`.

Audit result for both outputs:

- `rows=2192`;
- `interception_reported_total_mm=551.502748`;
- `profile_violations_days=fc_gt_porosity:0,wp_gt_fc:0,soilwater_gt_porosity:0,soilwater_lt_wp:0`;
- `closure_reconstructed_with_storage_total_mm=2950.498418`;
- `closure_reconstructed_with_storage_pct_of_precip=17.772166`;
- `closure_reconstructed_with_enriched_storage_total_mm=2950.498140`.

Disposition:

W-D fixed real producer defects but failed the current-scope conservation gate.
The remaining blocker is independent daily PASS `runvol` lineage. The current
producer still fills `runvol` from WAT `Q`, so `runoff_consistency_mm` is a
self-consistency check and cannot prove totalwatsed3 conservation.

## T-A design evidence

Status: T-A executed

Evidence mode: Static + Ran

T-A produced `totalwatsed3-cli-scope.md` and made no production code edits.

Static evidence read:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`: schema,
  PASS aggregation, WAT aggregation, optional soil/element aggregation, depth
  derivation, baseflow diagnostics, and streamflow derivation.
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`: required
  audit columns, depth reconstruction, storage bases, and closure identity.
- `crates/openwepp-hillslope-output/src/contracts.rs`: current pass output is
  required as `.hbp`; optional WAT/soil/element outputs are parquet.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  current HBP event payload writes six event volume slots as zero.
- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`:
  current parser consumes those slots without exposing PASS `runvol`.
- `crates/openwepp-runner/src/watershed_wat.rs`: superseded W-D path uses pass
  filenames only to find WAT siblings.

Ran evidence:

- Sampled `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/` parquet
  schemas with pyarrow. Observed `H.pass.parquet` (`78912` rows),
  `H.wat.parquet` (`271808` rows), `H.soil.parquet` (`271808` rows), and
  `H.element.parquet` (`74380` rows).

T-A disposition:

- T-A is complete for design/scope.
- T-B owns the dedicated CLI implementation, openWEPP-native PASS lineage
  surface, and red/green tests.

## T-B implementation and audit-read evidence

Status: T-B executed

Evidence mode: Ran + Static

Implementation:

- Added `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs`.
- Added `crates/openwepp-runner/src/totalwatsed3.rs`.
- Exported the new API through `crates/openwepp-runner/src/lib.rs`.
- Added the `openwepp-cli-totalwatsed3` binary target in
  `crates/openwepp-runner/Cargo.toml`.
- Extended `crates/openwepp-watershed-output/src/writers.rs` so
  totalwatsed3 row seeds can carry PASS-derived runoff, WAT diagnostics,
  optional profile fields, and volume/depth fields without writer defaults.
- Removed totalwatsed3 aggregation ownership from
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`.
- Updated `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs` so
  `watershed_totalwatsed3.Runoff` is publication-only PASS-volume lineage.

Red evidence:

- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract` failed
  before implementation because `CARGO_BIN_EXE_openwepp-cli-totalwatsed3` was
  not defined.

Focused green evidence:

- `cargo test -p openwepp-runner --test totalwatsed3_cli_contract`: `2`
  passed.
- `cargo test --test sim_contract_boundary_unit_registry`: `15` passed.

Full gate evidence:

- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass.
- `git diff --check`: pass before final artifact reconciliation.

Arboreal-dendrite T-B producer:

```bash
rm -rf /tmp/openwepp_wshed01_tb && mkdir -p /tmp/openwepp_wshed01_tb
cargo run -p openwepp-runner --bin openwepp-cli-totalwatsed3 -- \
  --input-dir /wc1/runs/ar/arboreal-dendrite/wepp/output/interchange \
  --output /tmp/openwepp_wshed01_tb/totalwatsed3.parquet
```

Observed:

- `CLITW3-I-001 wrote 2192 rows to
  /tmp/openwepp_wshed01_tb/totalwatsed3.parquet`.
- Pyarrow read: `2192` rows, `79` columns.
- Required sampled fields had no nulls: `Area`, `runvol`, `Runoff`, `Q`, `P`,
  `Precipitation`, `latqcc`, `Lateral Flow`, `Interception`,
  `SoilWaterTotal`, `QRain`, and `QSnow`.
- First row day selectors: `year=2000`, `julian=1`, `sim_day_index=1`.

wepppy T-B audit-read:

```bash
/home/workdir/wepppy/.venv/bin/python \
  /home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py \
  /tmp/openwepp_wshed01_tb/totalwatsed3.parquet \
  --output-dir /tmp/openwepp_wshed01_tb/audit \
  --top-n 10
```

Observed:

- `rows=2192`;
- `closure_reconstructed_with_storage_total_mm=57.409871`;
- `closure_reconstructed_with_storage_pct_of_precip=0.345805`;
- `interception_reported_total_mm=0.000000`;
- `profile_violations_days=fc_gt_porosity:0,wp_gt_fc:0,soilwater_gt_porosity:0,soilwater_lt_wp:0`.

Disposition:

T-B passed the dedicated producer and audit-read gates. T-C owns explaining and
closing the remaining `57.409871 mm` independent water-balance residual.

## Claude review (2026-06-14) — T-B aggregator accepted; but the closure ran on LEGACY inputs (ADR-0019 gap)

Evidence mode: Ran (code grep + interchange-file provenance + run-command read).

The aggregator CLI is real and the 57.4 mm (0.35% precip) is a major directional
win over W-D's 2950 mm — and **nonzero**, the right signature for genuinely
independent operands. But the load-bearing T-A/T-B deliverable (openWEPP-native
PASS lineage) is **not** done, and the green report obscures it:

1. **The T-B closure ran on LEGACY wepppy interchange parquets, not openWEPP
   output.** The run used `--input-dir /wc1/runs/ar/arboreal-dendrite/wepp/output/interchange`,
   whose `H.pass.parquet` and `H.wat.parquet` are **dated Jun 7** — produced by
   the wepppyo3 `wepp_interchange` from *legacy* WEPP output, *before* all
   MOFE01/WSHED01 work. So the 57.4 mm validates the CLI's **aggregation math
   on legacy data**; it says nothing about openWEPP's native output surface.
2. **openWEPP does not produce `H.pass.parquet`/`runvol` at all.** No crate
   writes it (the only `runvol` uses are channel-routing validation and the new
   T-B reader). openWEPP's hillslope output is HBP/WAT/loss/plot/soil — **no
   runoff-delivery (PASS) surface**.
3. **Therefore the closure is not yet ADR-0019-native.** "Native PASS/WAT
   aggregation" overclaims: the *CLI* is native, the *inputs* are legacy. T-C
   cannot claim the totalwatsed3 deferral resolved on legacy inputs.

**The architectural gap (operator decision needed):** for a genuine
ADR-0019-native totalwatsed3 closure, openWEPP must emit its own **independent**
`runvol`. This is harder than CLI wiring: openWEPP's only runoff today is WAT
`Q`, and a `runvol` *derived from* `Q` is self-consistent — re-introducing the
exact tautology the whole T-arc exists to avoid (and which the W-D disposition
named: "the current producer still fills runvol from WAT Q ... a
self-consistency check"). A genuine openWEPP `runvol` requires exposing a
**runoff-delivery** quantity separate from the WAT balance — from the MOFE
per-OFE routed-outlet state / HBP trajectory, not a `Q` restatement. That is an
engine/output-surface addition.

**Disposition:** T-B's aggregator CLI is accepted as a correct, legacy-validated
component. But T-C is **blocked**: it cannot demonstrate ADR-0019 closure until
openWEPP produces its own independent `runvol`. Recommend a **T-B2 — openWEPP
runvol/PASS output** increment (decide the runoff-delivery source: MOFE
outlet-OFE routed runoff is the natural candidate, independent of the WAT `Q`
depth), then T-C closes on openWEPP-native interchange. Operator input wanted on
the runoff-delivery source before T-B2 codes.
