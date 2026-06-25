# SNOTEL Density Delta Ledger

Evidence mode: Ran.

Source: `target/snowfrost_fidelity_h/three_way_comparison.json`.

Command shape:

```text
jq -r '.sites[] | [...] | @tsv' target/snowfrost_fidelity_h/three_way_comparison.json
```

The committed JSON ledger pins the exact per-site values. Summary:

- Sites: `5`.
- Density fork route: `STRUCTURAL = 5`.
- Maximum absolute openWEPP-minus-legacy as-built mean-signed density residual:
  `4.351046738461008 kg m^-3`.
- Therefore openWEPP and pinned legacy are the same effective as-built density
  lineage for the current decision. The observation residuals are tens to
  hundreds of `kg m^-3`; the openWEPP-vs-legacy differences are negligible by
  comparison.
- The observed-density `ssd` arm did not improve depth MAE by 25% at any site.
- PySnobal is not an adoption target from this evidence: four sites ran with
  larger depth MAE than openWEPP as-built, and CSS Lab is unavailable from the
  known upstream thin-snow instability.

| Site | Climate | SSD arm | Density arm | Fork | openWEPP density bias | legacy density bias | openWEPP-legacy | openWEPP depth MAE | legacy depth MAE | density-arm depth MAE | PySnobal depth MAE |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|
| `snotel_mica_creek_st_joe_id` | northern_rockies_intermountain | 250 | 370 | STRUCTURAL | -109.531 | -105.681 | -3.851 | 0.264 | 0.270 | 0.512 | 0.866 |
| `snotel_paradise_wa` | cascades_maritime | 250 | 495 | STRUCTURAL | -298.888 | -298.515 | -0.373 | 0.744 | 0.747 | 1.785 | 2.474 |
| `snotel_css_lab_ca` | sierra_maritime | 250 | 380 | STRUCTURAL | -117.316 | -112.965 | -4.351 | 0.492 | 0.512 | 0.695 | n/a |
| `snotel_snowbird_ut` | wasatch_intermountain | 250 | 445 | STRUCTURAL | -121.205 | -120.614 | -0.591 | 0.626 | 0.631 | 1.133 | 1.417 |
| `snotel_niwot_co` | front_range_continental | 250 | 340 | STRUCTURAL | -57.354 | -56.399 | -0.955 | 0.244 | 0.240 | 0.374 | 0.658 |

Disposition:

- The next package should not tune `ssd`.
- The next package should not make PySnobal the runtime engine.
- The next package should author a contract/ADR for an opt-in physics candidate
  that can deliberately diverge from legacy only behind `snow_model =
  physics_bulk`.
