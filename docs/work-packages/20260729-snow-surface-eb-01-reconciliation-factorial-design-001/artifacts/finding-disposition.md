# Finding Disposition

Status: `complete`; every review and terminal finding is accepted and resolved.

Evidence class: Static + Ran.

| Finding | Disposition | Correction |
| --- | --- | --- |
| A-01 observation reconciliation/counts | `accepted` | Generator now derives normalized per-stratum total/non-null counts, including Harvard `821` total rows per stratum; the ledger adds custody, location, period, resolution, units, forcing uncertainty, fixture, binding, and operator. |
| A-02 readiness schema | `accepted` | Matrix now carries all three canonical status fields and all ten science-contract-spec obligations with `PASS`/`NOT_APPLICABLE`, evidence, and rationale. |
| A-03 control-volume closure | `accepted` | Mass uses total ice-plus-liquid storage before/after; energy uses an explicit pre-routing surface/phase control volume, duration, area basis, thermal storage, phase mass, and source lineage. |
| A-04 operational decision rules | `accepted` | Added machine-readable hard gates, operators, threshold/window owners, failure outcomes, protected lanes, and explicit EB-04 holds where values require successor authority. |
| A-05 hard-coded evidence | `accepted` | Candidate results and plots derive from retained JSON; observation counts derive from normalized CSVs; normalized files are hashed; schemas, units, binding, periods, and manifest hashes fail closed. |
| B-01 exact-one coupling | `accepted` | Signed vapor exchange and latent heat are positive toward snow; the phase-appropriate latent heat and exact algebraic equivalence are frozen. |
| B-02 reproducible evidence | `accepted` | Same correction as A-05. |
| B-03 carrier default | `accepted` | Ledgers now separate production default, available opt-in, and factorial requirement; production default explicitly has Stage 3 disabled. |
| B-04 readiness governance | `accepted` | Same correction as A-02. |
| B-05 machine-readable stop-loss | `accepted` | Added `stop-loss.csv`, `decision-rules.csv`, and `successor-admission-decision.csv`; unresolved numeric authority blocks EB-04 before results. |
| B-06 premature completion | `accepted` | Package, campaign roadmap, canonical roadmap, and catalog remain executing/review correction until terminal verification. |
| T-A-01 latent implementation label | `accepted` | Current implementation now records latent heat as `missing_runtime`; Stage A/B is correctly described as mass-only with no produced/debited latent-energy flux. |
| T-A-02 vapor response sign | `accepted` | Response ledger now freezes signed `vapor_mass_exchange` as positive deposition / negative sublimation; a loss-positive view is derived only. |
| VB-01 step-integrated mass units | `accepted` | All exact-step mass amounts now use `kg m^-2`; their time basis states that they are integrated over the exact step. Only W m^-2 fluxes are multiplied by duration. |
| VB-03 CSV inventory count | `accepted` | Exact-diff reconciliation now records 14 generated CSV artifacts. |
| VB-04 authority-route count | `accepted` | Stop-loss prose now names the two EB-02 authority routes and explicitly separates priority 3 transfer data. |
| VA/VB recheck response units | `accepted` | Response units are physical `W m^-2` or `kg m^-2`; operators separately state hourly step-mean flux or exact-step-integrated mass. |

No finding was rejected, deferred, or converted to an unsupported successor
claim. The correction retains the asymmetric science outcome: EB-03 is
provisionally admitted contract-first; EB-02 and EB-04 remain held.
