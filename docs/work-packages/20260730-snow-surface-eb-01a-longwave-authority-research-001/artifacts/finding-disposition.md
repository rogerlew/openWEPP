# Finding Disposition

Status: `all findings resolved`.

Evidence class: Ran + Static.

| Finding | Disposition | Resolution |
| --- | --- | --- |
| A-01 hybrid non-unity emissivity algebra | `accepted / corrected` | Fixed the selected canopy and snow effective emissivities at exactly one and excluded non-unity gray-surface exchange. |
| A-02 atmospheric equations not resolved | `accepted / corrected` | Selected Dilley-Unsworth and recorded exact equations, coefficients, units, clearness mapping, windows, error, and limits. |
| RB-01 inconsistent atmospheric route | `accepted / corrected` | Replaced the open Kimball/Unsworth/Crawford choice with one reproducible Dilley-Unsworth route. |
| RB-02 forcing compatibility overstated | `accepted / corrected` | Recorded daily dewpoint and daily repeated SIMIMPL28 cloud lineage; retained a cloud-mapping hold. |
| RB-03 unresolved emissivity and snow temperature | `accepted / corrected` | Fixed effective unity and retained runtime implementation hold until the canonical contract selects the active snow-temperature provider. |
| RB-04 sub-canopy errors attached to open air | `accepted / corrected` | Labeled `6.8/8.4 W m^-2` as measured sub-canopy evidence and denied transfer to openWEPP open-air forcing. |
| RB-05 delegation wording | `accepted / corrected` | Added the required explicit subagent spawning/delegation language to package and kickoff. |
| RB-06 source provenance | `accepted / corrected` | Added authoritative URLs, access date, locators, versions, and available byte hashes. |
| RB-07 catalog blocker inconsistency | `accepted / corrected` | Catalog now retains sky-view, cloud-mapping, and snow-temperature runtime holds. |
| DSV-RB-01 amendment exact-tree closure | `accepted / corrected` | Reopened the amended package, repeated dual review and terminal gates, and required two amendment-aware exact-tree verifications before restoring complete status. |
| DSV-RB-02 missing FSM2 equation-ledger row | `accepted / corrected` | Added the Equation 14 diffuse-transmission base, default extinction coefficient, candidate whole-canopy expression, units, decision, and EB-02 composition limit. |
| DSV-RB-03 structural-cover lineage | `accepted / corrected` | Added the static runner-authority lineage, downstream field limitation, consumer-binding obligation, and structural-floor double-count guard. |
| DSV-RB-04 roadmap hold and stop-loss | `accepted / corrected` | Made the EB-02 runtime hold explicit in the top roadmap and prohibited invented/site-fit mapping, new user coefficients, or required remote data as escape routes. |
| DSV-RB-05 governing authority and included scope | `accepted / corrected` | Added Essery et al. (2025) to governing authority and structural cover to the enumerated existing-state mapping inputs. |

Both reviewers rechecked the original corrections and the derived-sky-view
amendment corrections and issued `PASS`. No finding is deferred or
undispositioned.
