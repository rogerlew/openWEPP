# Implementation Test Evidence

Status: W-C executed-hold

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
