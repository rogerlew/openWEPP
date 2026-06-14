# M-I closure-completeness evidence

Status: executed; MOFE01 hillslope water-routing closure is done-done for the
1-5-OFE ladder.

Evidence mode: Ran + Static

## Decision

M-I closes the two operator-directed completeness gaps left after M-H review:

- independent in-runner hillslope-total water-balance identity, computed from
  internal per-OFE records and OFE areas rather than published WAT rows;
- retirement guard for the transitional multi-OFE aggregate double-execution
  path.

M-I does not claim watershed-output `totalwatsed3`, >10-OFE far-point closure,
or sediment-coupled erosion `qin/qout`; those remain named follow-ons.

## Contract and implementation

`SC-WATBAL-001` version 161 adds `INV-WATBAL-100` and `TOL-WATBAL-008`.
The runtime contract now requires the hillslope-total identity:

```text
sum_i (external_in_i - external_out_i - delta_storage_i) * area_i / sum_i area_i
```

Internal `UpStrmQ`, `SubRIn`, non-outlet routed surface runoff, and non-outlet
lateral handoff are excluded from external outputs. Only outlet surface and
lateral exports leave the hillslope. Downstream transfer input depths are
scaled by upstream-area/current-area using the same OFE areas used by the
residual.

Production changes:

- `per_ofe_internal_wb13.rs` records OFE area, computes
  `hillslope_total_identity_max_abs_mm`, fails above `1.0e-9 mm`, and reports
  term-level residual detail.
- `00_runner_intake_and_lane_setup.rs` passes upstream/current OFE area ratios
  into persistent transfer state.
- `scheduler_publication.rs` and `openwepp-cli-watershed.rs` publish and
  validate `hillslope_total_identity_max_abs_mm` in run manifests.
- Source-level tests now require the independent identity tokens and the
  mutually exclusive multi-OFE/single-OFE scheduler lifecycle branch.

The double-execution seam was static-audited during M-I. Current code already
uses the persistent multi-OFE lifecycle and aggregate single-OFE lifecycle as
mutually exclusive `if/else` branches, so no production removal edit was
required for M-I-b. M-I adds the regression guard that prevents reintroducing
a multi-OFE aggregate compute-and-discard path.

## Red/green

Ran the M-I red target before production implementation:

```text
cargo test -p openwepp-runner mofe01_mi -- --nocapture
```

Result: FAIL as intended. The target could not compile because
`InternalPerOfeWb13Record.area_m2` and
`PerOfeInternalWb13RunSummary.hillslope_total_identity_max_abs_mm` did not
exist yet.

After implementation:

- `cargo test -p openwepp-runner mofe01_mi -- --nocapture`: PASS.
- `cargo test -p openwepp-runner per_ofe_state -- --nocapture`: PASS.
- `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`: PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows -- --nocapture`: PASS.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture`: PASS.

## Full ladder

Fresh M-I evidence lives under `/tmp/openwepp_mofe01_mi_final`.

The H1-H36 batch reused the M-H schema-v1 runfiles. Those runfiles carry
relative output paths rooted at the runfile directory, so the first M-I batch
wrote current outputs under `/tmp/openwepp_mofe01_mh_final/output`. The current
M-I outputs were copied into `/tmp/openwepp_mofe01_mi_final/output` after the
run, and M-H/M-I comparisons below use manifest checksums rather than the
overwritten M-H output files.

Runtime:

- H1-H36 execution: 36/36 zero exits.
- Runtime: `1583` seconds.
- Manifest inventory: 36.
- Output inventory after copy: 144 files.
- Error signature scan: no `CLIHILL-E`, `HKERNEL-*-E`, runtime surface
  failure, or identity-tolerance failure in `/tmp/openwepp_mofe01_mi_final/logs`.

`/tmp/openwepp_mofe01_mi_final/audits/m-i-manifest-residual-summary.json`
records:

| Metric | Result |
| --- | ---: |
| Max hillslope-total identity residual | `3.306423012547295e-13 mm` |
| Hillslope with max hillslope-total residual | H26 |
| Tolerance | `1.0e-9 mm` |
| All hillslope-total residuals within tolerance | `true` |
| All multi-OFE hillslope-total residuals nonzero-at-noise | `true` |
| Max per-element residual | `5.968558980384842e-13 mm` |
| Max transfer residual | `0.0 mm` |
| Max aggregate transfer cancellation residual | `0.0 mm` |

Per OFE-count hillslope-total maxima:

| OFE count | Hillslopes | Max hillslope-total residual mm | Nonzero multi-OFE residual count |
| ---: | ---: | ---: | ---: |
| 1 | 7 | `0.0` | 0 |
| 2 | 5 | `3.130398145553943e-13` | 5 |
| 3 | 5 | `2.0834011898441139e-13` | 5 |
| 4 | 3 | `2.752836865209086e-13` | 3 |
| 5 | 16 | `3.306423012547295e-13` | 16 |

## Comparisons

Ran local `owcmp` directly, without the comparator subagent:

```text
tools/owcmp/owcmp batch h1-h39-semantic \
  --baseline-dir /wc1/runs/ar/arboreal-dendrite/wepp/output \
  --baseline-pattern 'H{h}.wat.dat' \
  --candidate-dir /tmp/openwepp_mofe01_mi_final/output \
  --candidate-pattern 'H{h}.wat.parquet' \
  --candidate-year-offset 1999 \
  --output-root /tmp/openwepp_mofe01_mi_final/owcmp \
  --start 1 --end 36
```

Result:

- Execution verdict: PASS.
- Structural row-key failures: 0.
- Semantic value verdict: FAIL / investigation signal.
- Semantic pass count: 0/36.
- First divergent key: H1 `[4, 3, 2002]`.

M-H to M-I WAT checksum comparison used manifest checksums because the M-I
runfile-relative output paths overwrote the M-H output directory:

- `/tmp/openwepp_mofe01_mi_final/audits/m-i-vs-mh-wat-checksum.tsv`: 36/36
  WAT checksums unchanged.

Single-OFE anchor:

- `/tmp/openwepp_mofe01_mi_final/audits/m-i-single-ofe-anchor-cmp.tsv`: 28/28
  byte-identical comparisons against
  `/tmp/openwepp_mofe01_mfredo2_single_anchor/output` for
  H8/H15/H19/H20/H22/H23/H28 `.hbp`, `.loss.json`, `.plot.parquet`, and
  `.wat.parquet`.

## Final gates

Ran after M-I implementation and evidence updates:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS (`advisories ok, bans ok, licenses ok, sources ok`).
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract`: PASS.
- `markdown-doc lint --path ... --format json`: PASS, 42 files scanned, 0
  errors, 0 warnings.
- `git diff --check`: PASS.

## Residual follow-ons

M-I leaves the same bounded follow-ons named by M-H:

- `WATERSHED-OUTPUT-TOTALWATSED3-MOFE01`.
- `MOFE-GT10-FARPOINT-CLOSURE`.
- `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF`.
- Comparator value-family parity adjudication remains an ADR-0017
  investigation signal, not a MOFE01 acceptance blocker.
- Line-count splits remain due before further growth in warning-size files.
