# Authority Lineage Map

Status: complete

Evidence mode: Static + Ran

## Exact Source Surfaces

| Surface | Identity / location | Authority role |
| --- | --- | --- |
| WEPP Chapter 3 | `references/50201000/chap3.pdf`, SHA-256 `ffd628a5...db43`, Section 3.6 | 1995 handbook equation and stated assumptions |
| Pinned post-2007 legacy | `/workdir/wepp-forest_260430_baseline`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, blob SHA-256 `475e6f89...b7aa` | normative migration baseline |
| Current Rust | `infiltration_reconciliation.rs:831-904`, frozen SHA-256 `e0f8ccae...6d2` | current producer arithmetic |
| Current caller | `infiltration_reconciliation.rs:1046-1050,1461-1490,1560-1635` | daily/hourly gate, state chronology, and shortwave dispatch |
| Canonical contracts | `SC-SNOWFREEZE-001.md` and `SC-SNOWENERGY-001.md`, frozen hashes in `audit-freeze.json` | current production obligations and admitted seam |
| Independent physics | Marks et al. 1998, Marks et al. 1999, Ohmura 2001, Walter et al. 2005 | correctness authority for physical-state requirements, not replacement constants |

The clean convenience checkout is not the pinned authority: its `HEAD` is
`2f65506d...` and its `melt.for` hash is `e5864334...`. All legacy findings
below use `git show dac3c950...:src/melt.for`, never the worktree file.

## Formula Lineage

| Element | Chapter 3, 1995 | Pinned legacy | Current Rust | Adjudication |
| --- | --- | --- | --- | --- |
| total | `0.0254(A-B+C+D)` | `0.0254(A+B+C+D)` with signed `B` | same signed sum, then positive pack cap | Rust matches pinned operation and sign ownership |
| `A` | `0.0607 R_h(1-C)` plus below-freezing attenuation | same base; attenuation is commented out | same base for canonical CoE; absorption factor is exactly `1` | pinned fidelity; handbook cold-hour chronology diverged |
| `B` | unsigned clear-sky/cloud term subtracted from total | adds `0.025 T_F/24` and stores clear-sky loss with negative sign | exact pinned signed form | pinned fidelity; temperature energy was reallocated from the handbook `D` family |
| `C` | wind/turbulent proxy with air/dewpoint temperatures, canopy, canopy height, displacement, roughness, assumed 2 m wind | 10 m one-sixth wind adjustment; removes dynamic roughness/displacement; adds canopy air-temperature branch and calm branch | exact pinned form and legacy `1609 m/mile` conversion | pinned fidelity; major post-handbook structural change |
| `D` | hourly air-temperature background plus rain heat | rain heat only; dewpoint temperature if positive, otherwise air temperature | exact pinned form and legacy `39.37 in/m` conversion | pinned fidelity; not the handbook `D` equation |
| caller gate | melt while snow exists; no melt when daily `Tmax < -3 C`; density gates exported liquid | no formula when interval-start pack is absent or daily midpoint temperature is below `0 C` | same audited midpoint/pack ordering with a typed `1e-12 m` inactive threshold; snow added to an inactive pack does not melt that hour | material post-handbook gate change; not whole-caller identity |
| thermal state | assumes surface soil temperature is `0 C` during melt; no prognostic cold-content operand in the melt equation | no snow-surface temperature or cold-content input | no snow-surface temperature or cold-content input to CoE | state required by full energy-balance formulations is not represented by the melt producer |

Pinned locations are `melt.for:126-147,156-180,188-229,232-301` and
`snowd.for:74-116,177-255`. Current Rust locations are
`infiltration_reconciliation.rs:831-904,907-968,1046-1050,1461-1490` and
`03_kernel_support_00_support_helpers.rs:897-904`.

## Contract And Consumer Map

- `SC-SNOWFREEZE-001:243` explicitly classifies the four trace values as
  exact CoE empirical melt-depth contributions, not measured fluxes.
- `SC-SNOWFREEZE-001:305` fixes the signed-`B` current runtime identity and
  preserves the Corps/Chapter term family for modernization.
- `SC-SNOWFREEZE-001:338-339` and `SC-SNOWENERGY-001:201-202,547,557`
  preserve CoE melt ownership and prohibit Stage 3 positive energy excess
  from becoming melt.
- The runtime applies CoE melt before the downstream Stage 3 thermal/liquid
  path. Therefore existing surface temperature and cold content do not gate
  the melt generated at `infiltration_reconciliation.rs:1582-1603`.
- Density and liquid-routing consumers act after raw CoE generation. A raw
  positive `coe_applied_m` with interval-start density below `350 kg m^-3` is
  not proof that the same density persists through same-hour mixing or that the
  same amount was routed out of the snowpack.

## Lineage Verdict

No same-input Rust transcription defect was found in the audited term generator
and daily-midpoint/interval-start-pack ordering. Rust's typed zero threshold,
inactive drift, and contract-authorized redistribution/routing mean this is not
a claim of whole-caller behavioral identity. The post-2007 pinned baseline,
not the 1995 handbook formula, is the immediate arithmetic parent.

The authority issue is narrower than rejecting empirical melt models: current
contracts deliberately preserve CoE ownership, and Ohmura 2001 explains why
air temperature can proxy multiple melt-energy terms. But the material
2007/2008 departures from the handbook have no cited independent validation or
bounded transferability authority, while full energy-balance formulations use
surface temperature, cold content, radiation, and turbulent-exchange state
that remains downstream. Contract authorization fixes current ownership; it
does not by itself supply that missing validation.
