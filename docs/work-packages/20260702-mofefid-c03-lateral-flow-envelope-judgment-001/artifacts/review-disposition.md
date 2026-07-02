# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-02, hold-as-written). Codex ran
the scratch script, source-anchor checks, operand reconstruction, quickflow
sensitivity, and both authority guards (green). Four candidates **accepted**:

| # | Finding | Action |
|---|---|---|
| C03-CX-001 | Runner/input evidence only in scratch, not committed/manifested | `input-provenance.md` + the three analysis scripts (`c03_storm_decomp.py`, `c03_quickflow.py`, `c03_sensitivity.py`) committed into the package; run command, binary provenance (DC01 `91273392`), and fixture recorded. Parquet stays regenerable-not-vendored. |
| C03-CX-002 | Quickflow separation slope unpinned; could move ENV-E across the lower bound | **Substantive.** Separation-slope sweep run (`c03_sensitivity.py`): ENV-E stays **in-band [0.25, 0.80] across the full range (0.30–0.58)**; Hewlett–Hibbert canonical slope correctly converted (0.55 L s⁻¹ km⁻² h⁻¹ = 0.0475 mm/day) gives **0.40**; the **parameter-free surface-only anchor is 0.48**. Verdict does not hinge on the slope; headline figure corrected from the arbitrary-slope 0.46 to the H–H-canonical 0.40. Table in `verdict.md`. |
| C03-CX-003 | ENV-Y wording reads as internal-OFE latqcc sum; C03 uses outlet export | Contract `INV-SUBHYD-033` (rev 15) and the C01 derivation reworded: ENV-Y lateral term is `latqcc_outlet·A_outlet` (water leaving the toe), explicitly NOT an internal-OFE sum (which double-counts, since internal lateral is re-received downslope under INV-RUNOFFPART-031). |
| C03-CX-004 | ENV-T prose overstates monotonicity; 5 mm step-fit vs 10–20 mm binned transition unresolved | `verdict.md` corrected: median is **near-monotonic** (one minor reversal 0.095→0.092 inside noise; frequency ascends cleanly). The 5 mm mean-based step-fit is **rejected as a snowmelt artifact**; the reliable commencement is the ~10–20 mm median/frequency transition, which is what the verdict uses. |

Guards re-run first-hand at disposition: anti-evasion `PASS`; obligation
contract 2/2. No production code/tests changed; the CX-003 contract wording
is a clarification (same comparand C03 computed).

## Verdict after disposition (unchanged in direction)

H2637 post-DC01 **not-contradicted on all four tiers** — the ENV-E PASS is
now shown robust to the separation parameter, ENV-T's commencement is
honestly ~10–20 mm (wet-end note stands), and the ENV-Y comparand is
unambiguously the outlet-export convention. FARPOINT01 magnitude flag
resolved.
