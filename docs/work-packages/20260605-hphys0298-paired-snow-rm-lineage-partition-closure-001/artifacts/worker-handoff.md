# Worker Handoff

Status: complete

Evidence mode: static+ran

Static:

- HPHYS0298 remains `HOLD`.
- No production openWEPP physics file was changed.
- The next worker should not patch downstream WB13/WB17/WB18/WB19 consumers as
  compensation for this package's residuals.

Ran:

- Full runner:
  `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z`
- Review-fix regeneration:
  `.venv/bin/python docs/work-packages/20260605-hphys0298-paired-snow-rm-lineage-partition-closure-001/artifacts/hphys0298_paired_lineage_partition.py --run-root /tmp/hphys0298_full_20260605T000000Z --skip-full-suite --skip-targeted-traces`
- Validation gates are recorded in `artifacts/gate-results.md`.

## Result Summary

- Run root: `/tmp/hphys0298_full_20260605T000000Z`
- Candidate HEAD used by suite: `2e626969f7d0789ed80b2a3b4666fb6dc7689de8`
- Baseline authority:
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Baseline observe worktree: `/tmp/hphys0298_wepp_forest_obs`
- Baseline observe identity: pass for H1/H7/H39 with release=off and off=on
  bit identity.

## Partition Ledger

All nine target windows are `OPENWEPP-DEFECTIVE` at first divergent cut-point
`hourly-forcing`.

Interpretation: this is a porting-fidelity defect against the unimpeached
pinned-baseline precipitation-phase partition at
`/workdir/wepp-forest_260430_baseline/src/winter.for:410-412`. The paired
ledger proves openWEPP diverges at `hrsnow` or `hrrain`/`hrsnow` before
downstream WB13/WB17/WB18/WB19 consumers and before the corrected negative-melt
defect family.

| Hill | Window | First Symbols | Baseline RM | Candidate RM | RM Delta |
| --- | --- | --- | --- | --- | --- |
| H1 | first-2013 | `hrsnow` | 176.290000 | 161.617424 | 14.672576 |
| H1 | spring-2014 | `hrsnow` | 550.900000 | 487.337417 | 63.562583 |
| H1 | spring-2016 | `hrsnow` | 90.920000 | 75.643593 | 15.276407 |
| H7 | first-2013 | `hrsnow` | 194.240000 | 182.812732 | 11.427268 |
| H7 | spring-2014 | `hrsnow` | 577.160000 | 515.360976 | 61.799024 |
| H7 | spring-2016 | `hrsnow` | 152.610000 | 135.724574 | 16.885426 |
| H39 | first-2013 | `hrrain`, `hrsnow` | 52.280000 | 41.590702 | 10.689298 |
| H39 | spring-2014 | `hrsnow` | 549.600000 | 483.844778 | 65.755222 |
| H39 | spring-2016 | `hrsnow` | 99.230000 | 83.289837 | 15.940163 |

Full ledger: `artifacts/paired-lineage-summary.md` and
`artifacts/paired-lineage-ledger.json`.

## Full H1..H39 Metrics

Full metrics artifact: `artifacts/full-39-suite-metrics.md`.

| Column | Hillslope Fail Count | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| `Dp` | 38 | 10961 | 0.050444 | 0.244800 |
| `Ep` | 39 | 42688 | 0.633657 | 7.100844 |
| `Q` | 0 | 0 | 0.000000 | 0.000000 |
| `RM` | 39 | 7097 | 0.256086 | 27.960000 |
| `Snow-Water` | 39 | 10391 | 2.899431 | 65.506840 |
| `SoilWaterTotal` | 39 | 52185 | 56.010071 | 317.130129 |
| `Total-Soil` | 39 | 52185 | 56.010071 | 317.130129 |
| `latqcc` | 39 | 38462 | 0.285882 | 3.023092 |

## Review and Verification

- Review Agent A and Review Agent B findings are dispositioned in
  `artifacts/review-disposition.md`.
- Accepted review blockers were fixed:
  hourly forcing now precedes raw melt in classification, missing traces fail
  closed, per-symbol provenance is published, and observe identity includes
  release/off/on lanes.
- Dual verification artifacts are present:
  `artifacts/verification_agent_a.md` and
  `artifacts/verification_agent_b.md`.

## Cleanup State

- `/workdir/wepp-forest_260430_baseline` is the pinned baseline authority and
  remained clean after this package.
- `/tmp/hphys0298_wepp_forest_obs` is a detached diagnostic worktree and remains
  dirty by design with observe instrumentation and build outputs.
- Recovery command:
  `git -C /workdir/wepp-forest_260430_baseline worktree remove --force /tmp/hphys0298_wepp_forest_obs`

## Next Package

Recommended next package: baseline-authoritative migration of the winter hourly
snow/rain forcing partition consumed by snow/freeze logic. It should use the
HPHYS0298 harness as the continuation oracle and should keep the negative-melt
bug-fix authority from corrected openWEPP/wepp-forest history rather than
replicating the pinned negative-melt bug. The concrete porting target is
`/workdir/wepp-forest_260430_baseline/src/winter.for:410-412`, with paired
instrumented baseline observation through
`/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill` available as a
reference capability.
