# Timing and Fidelity Evidence

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

## Binary Provenance

- Build command: `cargo build --release -p openwepp-runner --bins`.
- Build log: `artifacts/runner-build-prechange.log`.
- Binary: `target/release/openwepp-cli-hill`.
- SHA256: `57a5ffb0df6040d166d4d768439861dc1d4d138dfbb24af709bb785444cf62c8`.
- Git HEAD: `54c3815db9f53b05e0e4744840288e3ed7382228`.
- Dirty state at build: package docs/artifacts only.

## H2637 Runs

| Variant | Env | User | Wall | Exit | Manifest |
|---|---|---:|---:|---:|---|
| Default/off | no Lane-D active or implicit env | `2.29 s` | `0:02.31` | `0` | `artifacts/h2637-prechange-default-off/output/openwepp_hillslope_run_manifest.json` |
| Active plain | `OPENWEPP_LANED_ACTIVE=1`, implicit unset | `39.73 s` | `0:39.75` | `0` | `artifacts/h2637-prechange-active-plain/output/openwepp_hillslope_run_manifest.json` |
| Active explicit hybrid | `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1` | `33.45 s` | `0:33.47` | `0` | `artifacts/h2637-prechange-active-hybrid/output/openwepp_hillslope_run_manifest.json` |

Profile counters:

| Metric | Active plain | Active hybrid | Delta |
|---|---:|---:|---:|
| Solver runs | `11590` | `17898` | `+6308` |
| Solver steps | `10479200` | `7381405` | `-3097795` |
| Homogeneous steps | `3139424` | `381501` | `-2757923` |
| Source-free steps | `5817107` | `1739149` | `-4077958` |
| Implicit steps | `0` | `980804` | `+980804` |
| Equilibrium map evaluations | `0` | `0` | `0` |
| Branch evaluations | `0` | `20110873` | `+20110873` |
| Alpha evaluations | `173774272` | `119746445` | `-54027827` |

Timing result: current explicit hybrid is materially faster than active plain
on this host (`33.45 s` vs `39.73 s` user; `-15.8 %`).

## Closure Surfaces

| Manifest metric | Active plain | Active hybrid |
|---|---:|---:|
| `max_supply_reconstruction_rel` | `7.31201193525081e-16` | `7.31201193525081e-16` |
| `max_day_cascade_residual_rel` | `2.4765580376695655e-13` | `4.579976970630865e-13` |
| `max_day_seam_residual_rel` | `5.0415846159888125e-14` | `4.082921815102614e-14` |
| `max_day_identity_residual_rel` | `2.4507313136493173e-13` | `4.443101533649709e-13` |

Closure remains machine-scale under explicit hybrid.

## Output Hashes

- `H2637.loss.json`: byte-identical across default/off, active plain, and
  active hybrid:
  `725f57233fd60df097a824a2c20f26992a58b3a457594245a9ac91d2278f3cfb`.
- `H2637.hbp`: active plain
  `efd8c4255fbe976ecafb2bc89defb7bebd4e2054c9e65c89cd5353c4c31c3790`;
  active hybrid
  `bfb2b002f8b67cd3c4b42504ae9cbc02189c13651f658b0c035c51cd23f50621`.
- `H2637.pass.parquet`: active plain
  `21c54bf2b045c3fb2f79f39ca174e36a4d188b39f7064f2a75f1170be6bb1656`;
  active hybrid
  `44e3da28ed5a2c4b310507d8d2f03e65c3a902e2f01e59f08e11e732d80e1f34`.

## Plain-vs-Hybrid Deltas

Manifest aggregate deltas:

- `total_routed_outlet_m3`: `374463.0826831916` plain vs
  `372817.0547059383` hybrid; delta `-1646.0279772533 m3`
  (`-0.4395701615 %`).
- `total_end_window_storage_m3`: `3167.3224902980` plain vs
  `3261.4084499266` hybrid; delta `+94.0859596286 m3`
  (`+2.9705203659 %`).
- `total_tail_fold_m3`: `36426.0837554273` plain vs
  `36681.6501011700` hybrid; delta `+255.5663457427 m3`
  (`+0.7016025864 %`).
- `total_clamp_m3`: `3207.0525522191` plain vs `1655.1105346073`
  hybrid; delta `-1551.9420176118 m3` (`-48.3915368502 %`).

Pass parquet deltas:

- Shape `(731, 17)`, columns, and index are identical.
- Changed rows: `4`.
- `tdet` sum: `23.0500822964` plain vs `22.6148198098` hybrid;
  delta `-1.8883337637 %`.
- `sedcon_1..5` sums: all `-6.4742434128 %`.
- Row-level detail is recorded in
  `artifacts/prechange-pass-parquet-delta-rows.txt`.

## Promotion Decision

The timing/counter subgate passes and the closure subgate passes. Promotion
still holds because the active plain-vs-hybrid output deltas are material
publication deltas, not GAP-OFEHYB-002 branch-evaluator dust, and no current
contract ratifies tolerances for accepting them as the active-path default.
