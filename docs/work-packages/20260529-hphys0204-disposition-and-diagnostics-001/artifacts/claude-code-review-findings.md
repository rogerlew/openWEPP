# HPHYS0204 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-30
Scope: integrated disposition + diagnostics package (docs-only) at commit
`c5913b3`, reviewed against the cited 39-hillslope run
`/tmp/hphys0207_20260530T042607Z/parity/` and the HPHYS0202–0207 implementation
lineage.

Evidence classes: **Static** (read source / contract / artifact) and **Ran**
(command executed). Reviewer-issued runs are marked; package "Ran" evidence is
recomputation over the HPHYS0207 run reports (no fresh rerun in this package).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — Residual table independently reproduces

Ran (reviewer, re-count over the 39 per-hillslope semantic reports under the
cited HPHYS0207 run). The fail-hillslope counts in `hphys0204_disposition.md`
and `hphys0204-physics-gap-matrix.md` reproduce exactly:

| Column | 0204 table | Reviewer re-count |
|---|---|---|
| Dp | 39/39 | 39/39 |
| latqcc | 39/39 | 39/39 |
| Total-Soil | 39/39 | 39/39 |
| SoilWaterTotal | 39/39 | 39/39 |
| ProfileFCStore | 27/39 | 27/39 |
| ProfileWPStore | 1/39 | 1/39 |
| ProfileDepth | 0/39 | 0/39 |
| ProfilePorosityCap | 0/39 | 0/39 |

## F-2 — Provenance is cited; evidence is recomputation, not a fresh run

Static. The gap-matrix, gate-results, implementation-and-test-evidence, and
verification artifacts all cite `/tmp/hphys0207_20260530T042607Z/parity/`. The
package "Ran" evidence is recomputation of counts/mean-abs summaries over the
HPHYS0207 run's semantic reports; this package did not execute a new
39-hillslope rerun. The diagnostics therefore reflect the post-HPHYS0207 state,
not a state including HPHYS0203's additions (HPHYS0203 added tests only — F-7 —
so this does not change the comparator columns).

## F-3 — GAP-001 and GAP-002 are classified as separate lanes despite shared FC/WP input lineage

Static. The gap-matrix classifies `HP204-GAP-001` (ProfileFCStore/ProfileWPStore)
as FC/WP storage-lineage and `HP204-GAP-002` (Dp, latqcc, Total-Soil,
SoilWaterTotal) as percolation/lateral/soil-water process-lineage "not resolved
by FC/WP depth-authority package." The GAP-002 columns are functions of the
field-capacity / wilting thresholds (percolation above FC; lateral flow and
soil-water bounds set by FC/WP/porosity). The kernel inputs for those thresholds
are the per-layer `thetfc_####`/`thetdr_####` symbols consumed by WB14
(`Wb14SoilThetaFieldCapacity`/`…Residual`) and the WB18 upper-limit derivation
(`wb11_seed`), which after HPHYS0207 carry the parser-grid remap
(HPHYS0207 review I-4), not the normalized-grid values WB13 publication now uses.
GAP-001 and GAP-002 thus share FC/WP input lineage on the kernel side; the
classification does not connect them.

## F-4 — GAP-001 groups a near-closed column with an open one

Static. `HP204-GAP-001` reports ProfileFCStore (27/39) and ProfileWPStore (1/39)
as one open gap. By the same disposition's closed/open split (ProfileDepth and
ProfilePorosityCap listed closed at 0/39), ProfileWPStore at 1/39 sits adjacent
to the closed families rather than to ProfileFCStore at 27/39. The grouping
presents WP's residual posture at FC's level.

## F-5 — The FC residual is reproducible in-process, not comparator-only

Static. The gap-matrix labels `HP204-GAP-001` an "investigation signal" under the
"higher-confidence comparator lane." Per the HPHYS0202–0207 arc, the FC residual
is present in openWEPP's own normalized-grid aggregation (the same aggregation
that yields exact porosity and 38/39 WP) — i.e., it is reproducible internally
and is not solely a legacy-comparison artifact. The matrix does not distinguish
this in-process-reproducible residual from a comparator-only flag.

## F-6 — "Process-authority-first promotability: pass" is corroborated only for the closed families

Static. `MEASURE-HP204-003` is marked pass and the disposition states comparator
residuals "do not by themselves negate upstream process-authoritative closure."
Independent corroboration of process-authority closure (zero parity on the
higher-confidence lane) is present only for ProfileDepth and ProfilePorosityCap.
For the open families (FC, Dp, latqcc, Total-Soil, SoilWaterTotal) the only
surface-level signal in the package is the nonzero comparator residual; there is
no positive in-package process-authority validation beyond declared-scope
completion of the upstream packages. The disposition retains `HOLD` and does not
claim closure for these families, so the promotability conclusion is not
overstated; the observation is that the "pass" is demonstrated for the closed
families and is a deferral for the open ones.

## F-7 — Package is docs-only; gates re-run at HEAD

Static. Commit `c5913b3` changes only package artifacts/contracts docs (no code
or test changes); the workspace test surface at HEAD reflects HPHYS0203's added
tests, not this package. Ran (reviewer, from `/home/workdir/openWEPP`, at HEAD
`c5913b3`): `cargo fmt --check` → exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0;
`cargo test --workspace` → exit 0. HPHYS0203's added tests pass directly
(`cargo test --test hphys0203_physics_robustness_contract` → 3 passed;
`cargo test -p openwepp-runner hphys0203_` → 4 passed).
