# worker handoff

Status: M-D handoff ready; M-E0 contract/test scaffold next

Evidence mode: Ran + Static

## Summary

M-D completed the design-only architecture increment:

- `mofe-per-ofe-state-architecture.md` defines the
  `PerOfeDailyWaterBalanceCollection` target shape, per-OFE record contents,
  lifecycle, and aggregate derivation rule.
- M-D selects per-OFE lane iteration over the existing scheduler phase graph,
  not `TopologyGraph` N-node encoding, because current topology nodes are
  hillslope/channel/impoundment system nodes.
- The design maps legacy `irs`/`rochek` continuation, WATBAL per-plane rows,
  and hourly carry copy-forward obligations to explicit per-OFE transfer state.
- M-E0 must amend `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001`
  and install failing contract tests before production code.
- No production code, science contracts, or tests were edited in M-D.

## M-C2 summary

M-C2 executed the scoping and comparison boundary and found a hard
runtime-state blocker:

- Existing MOFE hourly carry arrays are real hour-indexed transfer state, but
  not per-OFE daily WB output state.
- Current scheduler/writeback has one aggregate state surface and one aggregate
  flux surface; no OFE-keyed daily state collection exists.
- H1-H36 still complete with 36/36 exit code `0` under
  `/tmp/openwepp_mofe01_mc2`.
- Local `owcmp` was run without the comparator subagent under explicit operator
  direction because GPT-5.3-Codex-Spark weekly quota was exhausted:
  `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`,
  `structural_row_key_failures=350720`.
- Direct parquet audit shows all 29 multi-OFE surfaces still publish one
  `OFE=1` row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-B.
- M-C2 dual review/verification completed. Findings on write-set separation,
  exact gate taxonomy, and docs-lint counts were accepted and fixed.
- No production code was edited; the publication implementation path remains
  blocked until real per-OFE daily state exists.

## M-C summary

M-C executed the current boundary and found a hard publication-state blocker:

- H1-H36 still complete with 36/36 exit code `0`.
- Local `owcmp` was run without the comparator subagent under explicit operator
  direction because GPT-5.3-Codex-Spark weekly quota was exhausted:
  `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`,
  `structural_row_key_failures=350720`.
- Direct parquet audit shows all 29 multi-OFE surfaces still publish one
  `OFE=1` row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-B.
- No production code was edited; synthesizing per-OFE WAT rows from aggregate
  state would be surrogate physics.

## M-B summary

M-B retired the multi-OFE hydrology execution blocker:

- Current openWEPP completes H1-H36 with 36/36 exit code `0`.
- All 29 multi-OFE surfaces now complete the full 2192-day run.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-A outputs.
- Local owcmp execution passed, but semantic comparison remains failed due WAT row-key/per-OFE publication structure. That is M-C scope.
- No comparator subagent was used; comparisons were run locally per operator direction.
- Full three-identity acceptance is not proven; transfer and true per-element
  identities remain blocked until real per-OFE publication exists.

## M-A summary

M-A established the current execution boundary and the legacy routing calibration:

- Current openWEPP passes all seven 1-OFE H surfaces.
- Current openWEPP fails all 29 multi-OFE H surfaces before publication.
- Legacy H1-H36 WAT outputs preserve downstream `UpStrmQ`/`SubRIn` handoff to printed precision.
- No 15-OFE `pw0.wat.dat` exists for WAT closure.

## Local evidence left on disk

Local-only temp lane:

- `/tmp/openwepp_mofe01_mb/current_after_fix/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mb/current_after_fix/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_mb/current_after_fix/manifests/H*.json`
- `/tmp/openwepp_mofe01_mb/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/summary.json`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mc/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mc/manifests/H*.json`
- `/tmp/openwepp_mofe01_mc/owcmp/summary.json`
- `/tmp/openwepp_mofe01_mc/owcmp/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc/m-c-publication-audit.json`
- `/tmp/openwepp_mofe01_mc2/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mc2/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mc2/manifests/H*.json`
- `/tmp/openwepp_mofe01_mc2/owcmp/summary.json`
- `/tmp/openwepp_mofe01_mc2/owcmp/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_mc2/m-c2-publication-audit.json`
- `/tmp/openwepp_mofe01_ma/current/exit-codes.tsv`
- `/tmp/openwepp_mofe01_ma/current/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_ma/current/manifests/H*.json` for passing 1-OFE surfaces.
- `/tmp/openwepp_mofe01_ma/output/H*.wat.parquet` for passing 1-OFE surfaces.

These are not committed artifacts.

## Next worker focus

Execute M-E0 contract/test scaffolding for the M-D architecture. The current
single-row aggregate path and hour-indexed carry arrays are not enough to prove
`UpStrmQ`/`SubRIn` handoffs or non-aliased `QOFE`; do not manufacture those
fields from aggregate daily WAT rows.
