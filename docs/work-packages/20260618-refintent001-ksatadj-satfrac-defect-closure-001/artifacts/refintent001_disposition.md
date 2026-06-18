# REFINTENT001 Disposition

Evidence class: Static + Ran

Status: COMPLETE, 2026-06-18.

## Verdict

REFINTENT001 is complete. WB14 `ksatadj` now forms `sat_frac` per
`SC-SUBHYD-001#INV-SUBHYD-032`:

```text
sat_frac = avsat / (avpor * avcpm)
```

with top-two tillage weighting, residual-water numerator, rock-corrected
denominator, and both source-intent caps. The prior `theta_sum/ul_sum` surrogate
is removed from the active path.

## Files changed

- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`

## Acceptance

| Criterion | Status |
|---|---:|
| `INV-SUBHYD-032` source-intent formula | PASS |
| Source-intent operands required and typed | PASS |
| Non-aliased tests | PASS |
| Subhyd/WB14 contract tests | PASS |
| H2637 both UI variants | PASS |
| OFE1-OFE5 ladder | PASS |
| Full Rust gates | PASS |
| Line-count governance | PASS |

## FARPOINT01 flag

The H2637 `runvol / precip` value remained `71.003655003121%`. The FARPOINT01
71% flag is closed because the active algorithm is now the ratified
source-intent algorithm and conservation closure holds. This package did not and
should not chase legacy 55.5% parity.

## Open follow-ons

None for REFINTENT001. Any future disagreement with the absolute magnitude needs
a new authority package; it is not a defect in this package's implemented
source-intent closure.

## Post-review correction (Claude Code, 2026-06-18) — supersedes the FARPOINT01 claim above

Independent review found the FARPOINT01 flag-closure claim above is **UNSUPPORTED**. The
`ksatadj` `sat_frac` fix is **correct and retained**, but it is **byte-inert on H2637** —
verified two ways:

1. Post-fix `H2637.wat.parquet` SHA-256 `c70af52324b52c89…` is **identical** to the pre-fix
   STAGE2-LATQCC run (zero change across 235,961 rows; HBP identical too).
2. H2637 soil has **`ksatadj = 0`** (`p2637.sol` OFE blocks begin `0 'forest' …`; legacy
   `input.for:467` reads that token as `ksatadj(iplane)`). The `if ksatadj == 1` branch — which
   contains the fix — **never fires** for H2637.

Therefore the fix changes nothing about the H2637 71% magnitude, and **FARPOINT01 is NOT closed
by this package — it re-opens.** The H2637 lateral magnitude is driven by the **base soil
conductivity** (the `Ke`/`ssc` lineage), not `ksatadj` — consistent with STAGE2-LATQCC's own
observation ("the lateral conductivity is the soil conductivity exposed to WB19, *not* a ksatadj
override"). The `ksatadj` defect is real but **irrelevant to H2637** (it matters for `ksatadj = 1`
disturbed/burned forest soils).

**Actual open follow-on:** adjudicate the H2637 **base lateral/percolation conductivity** under
`SC-SUBHYD-001` / `SC-INFILE-SOIL-001` — that is where the 71% lives. See
`review-claude-independent.md`.
