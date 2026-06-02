# H39 Dp/Pe Diagnosis

Status: completed

Evidence mode: Static + Ran

Static:
- Baseline `p39.sol` restrictive footer is `1 10000.0 0.01`.
- This maps to `slflag=1`, `ui_bdrkth=10 m`, and
  `kslast=0.01 mm/h`, so unrestricted bottom-layer `Ksi` must not control
  hourly H39 `Dp`/`Pe`.
- Baseline `perc.for` hourly bottom branch also sets `meblfc=1`, forcing
  `fx=1`; otherwise H39 day-1 `Dp` is about `10x` too low after only applying
  the thickness-weighted restrictive conductivity.

Ran:
- Final evidence root:
  `/tmp/hphys0248_20260602T114714Z_final`.
- H39 final trace:
  `/tmp/hphys0248_20260602T114714Z_final/hillslope_output/H39.hphys0248.trace.jsonl`.
- H39 final diagnosis:
  `/tmp/hphys0248_20260602T114714Z_final/reports/H39_hphys0248_diagnosis.md`.
- H39 first 10 days:
  - baseline `Dp=0.240000 mm/day`.
  - candidate `Dp=0.246960 mm/day`.
  - residual `+0.006960 mm/day`.
- HPHYS0247 prior H39 `Dp`: fail count `926`, mean abs `0.262718`,
  max abs `23.809497`.
- HPHYS0248 final H39 `Dp`: fail count `889`, mean abs `0.145745`,
  max abs `0.240000`.
