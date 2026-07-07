# Timing and Closure

Status: PARTIAL. Evidence mode: Ran.

## Binary Provenance

Ran:

```text
cargo build --release -p openwepp-runner --bins
stat -c '%y %s %n' target/release/openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill
```

Result:

```text
2026-07-07 10:05:51.291364237 -0700 9899560 target/release/openwepp-cli-hill
c6eb89c5a81769280cb5becb3b14ad80683f0726cbdf65365e34a4bba732f164  target/release/openwepp-cli-hill
```

## H2637 Timing

| Mode | Wall | User | Sys | `hybrid_implicit_stepping` |
|---|---:|---:|---:|---:|
| Active plain | `0:39.71` | `39.64` | `0.05` | `false` |
| Active hybrid | `0:33.37` | `33.33` | `0.02` | `true` |

Relative timing: H2637 active hybrid user time is `15.9%` lower than active
plain in this corrected package-local run.

## H2637 Closure

| Closure metric | Plain | Hybrid |
|---|---:|---:|
| `days_routed` | `610` | `610` |
| `days_seen` | `731` | `731` |
| `max_supply_reconstruction_rel` | `7.31201193525081e-16` | `7.31201193525081e-16` |
| `max_day_cascade_residual_rel` | `2.4765580376695655e-13` | `4.579976970630865e-13` |
| `max_day_identity_residual_rel` | `2.4507313136493173e-13` | `4.443101533649709e-13` |
| `max_day_seam_residual_rel` | `5.0415846159888125e-14` | `4.082921815102614e-14` |

The closure metrics remain at numerical closure scale for the completed H2637
pair. They do not close the selected-cohort suite because the first external
member fails active plain before output publication.
