# PERFIDX03 Pre-Flip Registry Coverage

Status: PASS 2026-06-17
Evidence mode: **Ran** + **Static**

## Result

The diverse-management pre-flip registry gate passed after two reachable-set
fixes:

- Frost runtime fine-layer symbols now derive the terminal fine-layer count from
  `frost.options.fineTop`, `frost.options.fineBot`, and the terminal `dg` layer
  depth instead of assuming the default count.
- Irrigation depletion/fixed-date sidecar symbols are now parsed, projected, and
  included in the run-scoped registry/audit surface.

Initial CLI01 audit before the frost fix failed with 20 unknown frost symbols:
`fgfrst`, `slfsd_m`, `slsic_m`, `slsw_theta`, and `sltime_s` for fine suffixes
17 through 20. The final audit has no unknowns.

## Final Cohort

Command family:

```text
OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH=<audit.json> \
  target/release/openwepp-cli-hill \
  --run-dir <case-run-dir> --run-file case.run --output-dir <tmp-output> --policy compat
```

Final audit outputs live under `/tmp/perfidx03/preflip/audit_final/`.

| Case | Coverage purpose | Registry symbols | Constructed symbols | Unknown symbols |
| --- | --- | ---: | ---: | ---: |
| `cli01_cropland` | baseline cropland + frost topology | 2189 | 1714 | 0 |
| `perennial_cut` | perennial with multiple cut events | 2176 | 1715 | 0 |
| `perennial_grazing` | perennial grazing with multiple cycles | 2186 | 1715 | 0 |
| `irrigation_combo` | depletion + fixed-date irrigation sidecars | 2213 | 1729 | 0 |
| `rotation_two_year` | varied rotation/year schedule | 2251 | 1717 | 0 |

Timing log:

```text
PERFIDX03_PREFLIP_AUDIT_TIME case=cli01_cropland elapsed=0.31 user=0.13 sys=0.02 maxrss_kb=18800
PERFIDX03_PREFLIP_AUDIT_TIME case=perennial_cut elapsed=0.08 user=0.07 sys=0.01 maxrss_kb=18428
PERFIDX03_PREFLIP_AUDIT_TIME case=perennial_grazing elapsed=0.08 user=0.07 sys=0.01 maxrss_kb=18408
PERFIDX03_PREFLIP_AUDIT_TIME case=irrigation_combo elapsed=0.08 user=0.07 sys=0.01 maxrss_kb=18432
PERFIDX03_PREFLIP_AUDIT_TIME case=rotation_two_year elapsed=0.08 user=0.07 sys=0.01 maxrss_kb=18432
```

## Disposition

The pre-flip reachable-registry proof passed. This gate did not block the
authority flip attempt.
