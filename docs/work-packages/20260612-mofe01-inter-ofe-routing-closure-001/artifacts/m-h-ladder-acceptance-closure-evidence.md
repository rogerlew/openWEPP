# M-H ladder acceptance closure evidence

Status: executed; MOFE01 hillslope water-routing closure accepted

Evidence mode: Ran + Static

## Decision

M-H closes the MOFE01 hillslope-internal water-routing and public per-OFE WAT
publication rung on the full arboreal-dendrite 1-5-OFE ladder. The acceptance
authority is openWEPP conservation closure, not comparator value matching.

This closure does not claim watershed-output `totalwatsed3` closure and does
not claim sediment-coupled erosion `qin/qout` closure. Those are named
follow-ons below.

## Runtime lane

Fresh M-H evidence lives under `/tmp/openwepp_mofe01_mh_final`.

Ran:

- `cargo build -p openwepp-runner --bin openwepp-cli-hill --bin open_wepp_runner`:
  PASS.
- `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json`:
  PASS.
- Generated schema-v1 hillslope runfiles for H1-H36 from
  `/wc1/runs/ar/arboreal-dendrite/wepp/runs`.
- Ran all 36 hillslopes with `openwepp-cli-hill`, `--policy compat`, and
  `--legacy-sidecar-discovery`: 36/36 exit code `0`.
- Output inventory: 144 files, with 36 each of `.hbp`, `.loss.json`,
  `.plot.parquet`, and `.wat.parquet`.
- Manifest inventory: 36 run manifests.

## Ladder closure

`/tmp/openwepp_mofe01_mh_final/audits/m-h-ladder-audit.json` records:

| Metric | Result |
| --- | ---: |
| Hillslopes | 36 |
| Rows | 271808 |
| Expected rows | 271808 |
| Exit pass | 36/36 |
| Max transfer residual | `0.0 mm` |
| Max per-element residual | `5.968558980384842e-13 mm` |
| Max aggregate cancellation residual | `0.0 mm` |
| Max handoff residual | `5.684341886080802e-14 mm` |
| Downstream `QOFE == Q` alias rows | 0 |
| Hydrology clone active days | 0 |

Per OFE-count closure from
`/tmp/openwepp_mofe01_mh_final/audits/m-h-per-ofe-count.tsv`:

| OFE count | Hillslopes | Exit pass | Rows | Max transfer residual mm | Max per-element residual mm | Max handoff residual mm | Downstream alias rows | Hydrology clone active days |
| ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | H8,H15,H19,H20,H22,H23,H28 | 7/7 | 15344 | 0.0 | 0.0 | 0.0 | 0 | 0 |
| 2 | H11,H13,H16,H33,H36 | 5/5 | 21920 | 0.0 | `3.517186542012496e-13` | `3.552713678800501e-15` | 0 | 0 |
| 3 | H6,H12,H14,H29,H30 | 5/5 | 32880 | 0.0 | `2.984279490192421e-13` | `1.4210854715202004e-14` | 0 | 0 |
| 4 | H9,H25,H32 | 3/3 | 26304 | 0.0 | `4.973799150320701e-13` | `1.4210854715202004e-14` | 0 | 0 |
| 5 | H1,H2,H3,H4,H5,H7,H10,H17,H18,H21,H24,H26,H27,H31,H34,H35 | 16/16 | 175360 | 0.0 | `5.968558980384842e-13` | `5.684341886080802e-14` | 0 | 0 |

The single-OFE anchor is preserved: H8/H15/H19/H20/H22/H23/H28 compare
byte-identical against `/tmp/openwepp_mofe01_mfredo2_single_anchor` for
`.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).

## Local comparison

Ran local `owcmp` directly, without the comparator subagent:

```text
tools/owcmp/owcmp batch h1-h39-semantic \
  --baseline-dir /wc1/runs/ar/arboreal-dendrite/wepp/output \
  --baseline-pattern 'H{h}.wat.dat' \
  --candidate-dir /tmp/openwepp_mofe01_mh_final/output \
  --candidate-pattern 'H{h}.wat.parquet' \
  --candidate-year-offset 1999 \
  --output-root /tmp/openwepp_mofe01_mh_final/owcmp \
  --start 1 --end 36
```

Result:

- Execution verdict: PASS.
- Structural row-key failures: 0.
- Semantic value verdict: FAIL / investigation signal.
- Semantic pass count: 0/36.
- First divergent key: H1 `[4, 3, 2002]`.

Focus-column maxima from `/tmp/openwepp_mofe01_mh_final/owcmp/summary.json`:

| Column | Max abs diff |
| --- | ---: |
| RM | `55.30726311125751` |
| Snow-Water | `292.64` |
| SoilWaterTotal / Total-Soil | `552.9365291849313` |
| Ep | `7.159873861649016` |
| Es | `0.0397788702956757` |
| Dp | `0.24479936398816834` |
| Q | `134.76278723317412` |
| latqcc | `52.31990291280066` |

This is not an M-H acceptance failure. ADR-0017 and this package make
comparator values a flag only; row keys now align, public per-OFE row
cardinality is correct, anti-clone gates close, and the three conservation
identities close at roundoff on the full ladder.

## totalwatsed3 boundary

M-H attempted the watershed-output path using the fresh H1-H36 `.hbp` files:

```text
target/debug/openwepp-cli-watershed \
  --run-dir /tmp/openwepp_mofe01_mh_final/watershed/run \
  --run-file case.run \
  --output-dir /tmp/openwepp_mofe01_mh_final/watershed/output \
  --policy compat \
  --legacy-sidecar-discovery
```

The command failed closed before output writing:

```text
CLIWAT-E-010 failed parsing watershed impoundment
/wc1/runs/ar/arboreal-dendrite/wepp/runs/pw0.imp:
IMP-E-004: line 2 invalid domain value '0' for jpond; expected >= 1
```

No `totalwatsed3.parquet` was produced. The WBVAL06/6a end-to-end
`totalwatsed3` deferral is therefore explicitly re-stated as the next
watershed-output mechanism, not claimed closed by MOFE01 M-H.

## Follow-ons

- `WATERSHED-OUTPUT-TOTALWATSED3-MOFE01`: make the watershed-output lane accept
  or explicitly model the arboreal-dendrite no-impoundment `pw0.imp` state,
  consume the M-H per-hillslope pass files, produce `totalwatsed3.parquet`, and
  run the totalwatsed3 water-balance audit.
- `MOFE-GT10-FARPOINT-CLOSURE`: run a high-OFE substrate that reaches the
  known legacy-defect domain above 10 OFEs and prove openWEPP's three
  identities still close.
- `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF`: implement true sediment-coupled
  downstream `erod14_qin` from prior-OFE erosion `qout` plus particle/class
  fraction handoff lineage before flipping `erod14_qin_sediment_coupled`.


## Claude review (2026-06-13) — closure SUBSTANCE accepted; 5 completeness items recorded

Evidence mode: Ran (duckdb on the M-H cohort, source/ROADMAP inspection).

**The water-routing closure substance is genuine and accepted.** Unlike the
M-E4 tautology and M-F clone (which were hollow), M-H rests on real evidence:
the per-element identity is genuine (5.97e-13 mm on the now-distinct,
routed per-OFE hydrology), row cardinality is exact (271808), `QOFE != Q`,
single-OFE anchor 28/28 byte-identical, and the closure narrative is honest —
"hillslope water-routing closure," **not** "MOFE complete," with sediment and
watershed/totalwatsed3 named as follow-ons. ROADMAP/README hygiene is clean
(MOFE removed; watershed/totalwatsed3 next with the `pw0.imp` blocker named).

**But the closure is substantively-accepted, not fully-verified — five
completeness items, two of them owed soon:**

1. **Hillslope-total identity not computed (the flagged final-acceptance
   anchor).** Acceptance rests on the genuine per-element + the *structural*
   transfer/aggregate checks (0.0 by construction — built-from-each-other, as
   characterized at M-E4-REDO). The independent hillslope-total (Σ per-OFE
   balances vs external-only fluxes) is algebraically *implied* by per-element
   + sent==received, but never independently proven. It must be computed
   **in-runner** with the internal volume operands (like M-E4-REDO does the
   per-element) — a from-published-WAT attempt is not reliable (my own attempt
   gave a 3061 mm artifact from area/normalization handling, which is exactly
   why it belongs in-runner). **Owed before the rung is "done-done."**
2. **Transitional double-execution NOT retired** (`00_runner_intake_and_lane_setup.rs:1251`
   + `:1313` both run per multi-OFE day). My M-E5 pin required retiring the
   aggregate lifecycle when publication flipped; M-F flipped publication but
   left the aggregate path computing-and-discarding — 2× cost and a dual
   source of truth. **Owed soon** (perf + latent-bug hygiene).
3. **>10-OFE far-point demonstration** (openWEPP exceeding the legacy ceiling)
   — a named follow-on per package.md, not carried into M-H's follow-on list.
4. **±10–25% magnitude-parity adjudication** (the M-F-REDO2 posture: scattered,
   no systematic bias, expected ADR-0017 divergence) — record as the standing
   comparator-flag posture, not silently dropped.
5. **Line-count splits** — `scheduler.rs`, `scheduler_seed_and_runtime.rs`,
   `openwepp-cli-watershed.rs` all crossed 2000 (WARN, under 3000); due at the
   next touch.

Recommendation: MOFE01 closes as **water-routing closure accepted** with items
1–2 as a short closure-completeness pass (M-H2) before the watershed rung
consumes these outputs, and 3–5 carried as named follow-ons. The core
routing-conservation claim is real; these make it fully independent and clean.
