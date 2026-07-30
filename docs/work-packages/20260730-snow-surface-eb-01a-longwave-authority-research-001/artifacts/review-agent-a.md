# Review Agent A

Evidence class: `Static + primary-source verification`.

Final verdict after correction re-review: `PASS`.

The corrected package selects a coherent effective-black-surface exchange,
records one complete atmospheric-longwave route, and separates source-backed
equations from unresolved openWEPP providers. The stand-scale radiative
geometry, canopy-temperature regime, trunk exclusion, uncertainty, and claim
limits are now internally consistent and supported by the cited primary
sources.

## Final Amendment-Correction Re-review

Final amendment-aware verdict: `PASS`; no open Review A finding.

The DSV-RB-01 through DSV-RB-04 corrections introduce no science-authority
defect:

- The new equation-ledger row transcribes the FSM2 diffuse-transmission base
  correctly:
  `tau_d = exp(-1.6 k_ext VAI_eff)`, with the randomly oriented-element
  default `k_ext = 0.5` giving the bounded candidate
  `f_sky = exp(-0.8 VAI_eff)`. The row appropriately admits only this
  homogeneous-canopy mapping base; it does not claim that FSM2 defines the
  exact composition of openWEPP `VAI_eff`.
- The structural-cover lineage is now complete and truthful. Static
  structural canopy cover is available to the daily canopy realization,
  where it already acts as the persistent floor beneath foliar cover, but it
  is not exposed as an independent downstream daily-growth operand. The
  ledger's `CONSUMER_BINDING_NOT_READY` disposition and explicit prohibition
  on counting that cover both in the existing dynamic-cover floor and again
  in `VAI_eff` prevent an unsupported duplicate woody-obstruction term.
- The roadmap, package disposition, and final disposition now distinguish
  contract research from publication: EB-02 formulation work may proceed,
  while runtime implementation remains explicitly `HOLD` until the
  deterministic sky-view mapping, atmospheric-longwave mapping, effective
  unity-emissivity regime, and active snow-temperature prerequisites are
  contract-bound.
- The stop-loss is scientifically appropriate. If existing canopy state
  cannot support an authority-backed deterministic effective-vegetation-area
  mapping, work must stop rather than introduce a fitted cover/LAI blend, a
  new user coefficient, or a required remote-sensing data dependency.

The corrected tree also preserves the prior limits on canopy height and
observations: height is not inserted into the homogeneous Beer-law operator,
and observations remain optional validation evidence rather than runtime
authority. The resulting classification is sound:
`ADMIT_MAPPING_BASE` for the FSM2 operator, exact openWEPP `VAI_eff`
composition unresolved for EB-02, and runtime implementation held until that
canonical binding exists.

## Derived-Sky-View Amendment Re-review

Amendment verdict: `PASS`; no new Review A finding.

The user-directed decision changes the provider strategy, not the admitted
radiative meaning of `f_sky`. The amended package continues to define
`f_sky` as hemispherical, angle-weighted radiometric sky view and does not
equate it directly with plan-view canopy cover or LAI.

The FSM2 authority is used at the correct level. Essery et al. (2025),
Equation 14, derives diffuse transmission through canopy layer `n` as

`tau_d,n = exp(-1.6 k_ext Lambda_n)`.

For randomly oriented canopy elements, FSM2 gives default
`k_ext = 0.5`, yielding the stated whole-canopy candidate
`exp(-0.8 VAI_eff)`. Section 2.3 explicitly reuses Equation 14 for atmospheric
longwave transmission, and Equations 25 and 29 use that transmission as the
complementary sky/canopy weight. This makes FSM2 diffuse transmission an
authoritative base for a homogeneous stand-scale radiometric view operator.
For a multilayer canopy with a common extinction coefficient, multiplying the
layer transmissions produces the same exponential in total effective
vegetation area.

The amendment retains the necessary authority boundary:

- `canopy-to-sky-view-decision.md` defers the exact composition of
  `VAI_eff` to the EB-02 canonical-contract package. FSM2 defines effective
  vegetation area as leaves plus stems; it does not supply authority for an
  improvised blend of openWEPP canopy cover and LAI.
- Dynamic and structural canopy cover may enter only through a documented
  effective-area, gap, or clumping role. They are not renamed `f_sky`.
- Leaf-off obstruction is preserved as a required invariant, but the package
  does not invent the stem/woody-area conversion needed to satisfy it.
- Canopy height is explicitly excluded from the homogeneous Beer-law operator.
  It may enter only if EB-02 admits a source-defined finite-crown, gap, or
  other geometric correction.
- Hemispherical photography, LiDAR, and sub-canopy radiation observations are
  optional independent validation and uncertainty evidence, not runtime
  operands or prerequisites. Their absence does not weaken the equation-level
  authority already supplied by FSM2, Essery et al. (2008), and Rutter et al.
  (2023).
- Runtime implementation remains held until EB-02 binds the exact
  `VAI_eff` mapping, so `AUTHORITY_BASE_ADMITTED` /
  `DERIVED_OPERATOR_NOT_BOUND` is a truthful readiness classification.

The product constraint against a new user sky-view coefficient is therefore
scientifically compatible with the selected homogeneous-stand formulation.
It does not authorize canopy-cover aliasing, an unproven height term, or a
site-fitted surrogate. If EB-02 cannot define effective leaf-plus-stem area
from existing state with source-backed constants, the present stop-loss still
requires a hold rather than invented algebra.

## Correction Re-review

| Original finding | Resolution | Re-review evidence |
| --- | --- | --- |
| A-01 non-unity emissivity algebra | `RESOLVED` | `equation-ledger.csv`, `formulation-decision.md`, `science-summary.md`, and `uncertainty-and-scope.md` now fix effective canopy and snow emissivity at exactly one. The displayed equations match Rutter Equation 2 and the Sicart full-emitter simplification. They explicitly exclude non-unity material emissivity unless a future formulation includes reflected longwave and gray-surface exchange. |
| A-02 atmospheric equation authority | `RESOLVED` | `atmospheric-longwave-formulation.md` and `equation-ledger.csv` record `w = 4650 e_0/T_0`, the corrected Dilley flux equation, back-calculated clear-sky emissivity, the selected Unsworth-Monteith correction, `k_cld = 0.15`, `k_clr = 0.80`, the bounded clearness-to-cloud mapping, units, signs, coefficients, source locations, 24-hour-window behavior, RMSD, and extreme-latitude limitation. |

Primary-source rechecks confirm:

- Flerchinger Table 1 supports the Dilley coefficients and precipitable-water
  conversion as written when `w` is expressed in `kg m^-2`/mm and normalized
  by `25`; the artifact also records the equivalent centimeter form.
- Flerchinger Table 2 gives
  `epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c`.
- Flerchinger Table 9 gives the selected Dilley-Unsworth pair
  `k_cld = 0.15` and `k_clr = 0.80`; Table 10 gives `24.5 W m^-2` subdaily
  and `14.9 W m^-2` daily RMSD for that pair.
- Flerchinger sections 4.5 and 5 support the centered 24-hour window for
  midnight, the weak diurnal skill, and the extreme-latitude winter warning.
- Rutter Equation 2 supports the effective-unity two-component canopy
  equation, while Musselman and Pomeroy Equation 17 remains correctly excluded
  as the fuller non-unity/tree-scale alternative.

The corrected operand ledger no longer calls the legacy repeated daily cloud
fraction source-equivalent, no longer transfers sub-canopy error statistics
to openWEPP's open-air temperature, and leaves the snow-temperature provider
unresolved. The split decision is therefore truthful:
`GO_WITH_PREREQUISITES` for contract research/amendment and continued
`HOLD` for runtime implementation.

No open Review A finding remains.

## Original Findings (Pre-correction Record)

### A-01 — High — The displayed non-unity emissivity formulation omits reflected longwave

`equation-ledger.csv`, `formulation-decision.md`, and `science-summary.md`
display

`L_sub_down = f_sky L_atm_down + (1-f_sky) epsilon_c sigma T_c^4`

and

`L_net = L_sub_down - epsilon_s sigma T_s^4`.

Those equations are authoritative when canopy and snow are treated as
effective black surfaces (`epsilon_c = epsilon_s = 1`). They are not the
source-backed generalization for arbitrary emissivities less than one.

Rutter et al. (2023), section 4 and Equation 2, explicitly set effective canopy
emissivity to one because multiple longwave scattering drives apparent
emissivity toward unity. Sicart et al. (2004), section 2a, likewise state that
their radiation equation assumes snow and vegetation are full emitters and
therefore neglects reflection. In contrast, Musselman and Pomeroy (2017),
Equation 17, write gray-surface exitance as
`J = epsilon sigma T^4 + (1-epsilon) L_in`; their Equations 18–19 then exchange
the complete exitances. The current package retains the emission term but
omits the reflected term.

The snow expression has the analogous problem if interpreted as absorbed net
longwave for `epsilon_s < 1`: an opaque gray snow surface reflects the
unabsorbed part of downwelling radiation. Subtracting only
`epsilon_s sigma T_s^4` from an otherwise fully absorbed `L_sub_down` does not
preserve that exchange.

Action: choose and state one coherent authority path:

1. bind the selected stand-scale candidate to effective
   `epsilon_c = epsilon_s = 1`, preserving the exact Rutter/Sicart
   simplification; or
2. retain non-unity emissivities and record the full reflected-plus-emitted
   exitance/exchange equations, including how multiple reflections and the
   snow absorptivity are handled.

Do not call emissivity “explicit” while displaying a hybrid equation. Update
the sensitivity figure/sidecar if the selected path changes its assumed
emissivity.

### A-02 — High — The atmospheric-longwave formulation is named, not resolved at equation level

The package purpose and included scope require exact equations, units,
coefficients, assumptions, and source corrections. The `ATM_CLEAR` and
`ATM_CLOUD` rows instead contain prose placeholders:

- “Dilley-O'Brien clear-sky longwave using air temperature and precipitable
  water”; and
- “Kimball cloud correction with supplied cloud fraction.”

No artifact records the corrected Dilley equation, the precipitable-water
equation and unit conversion, or the Kimball cloud-emission equations and
coefficients. `formulation-decision.md` therefore cannot tell a later contract
author which exact source expression to bind, even though it declares that
step a prerequisite. This is especially important because the package itself
distinguishes the corrected version-of-record form and because Kimball's
correction requires more than merely multiplying by a cloud fraction.

Flerchinger et al. (2009) does support Dilley clear-sky radiation combined with
Kimball, Unsworth-Monteith, or Crawford cloud corrections. That comparative
selection is adequate. The missing equation record is not.

Action: transcribe the selected corrected Dilley and Kimball equations into
the equation ledger or a dedicated atmospheric-formulation artifact. Define
every symbol, temperature and vapor-pressure unit, coefficient, cloud-base
temperature rule, cloud-fraction range, day/night behavior, and polar-night
limitation, with table/equation/page provenance. If EB-02 is allowed to choose
among multiple cloud corrections, state that explicitly and give each
candidate's complete expression; otherwise select one.

## Positive evidence

- The sky-view integral is correctly angle weighted and normalized by
  `1/pi`. The package correctly rejects plan-view canopy cover as a silent
  alias for hemispherical radiometric sky view.
- The complementary sky/canopy partition is correct: canopy emission
  displaces the obscured sky contribution rather than being added over the
  full hemisphere.
- The positive-toward-snow sign convention is clear. Under the effective
  black-surface assumption, `L_net = L_sub_down - sigma T_s^4` is internally
  consistent.
- The canopy-temperature interpretation is appropriately conditional.
  Rutter et al. (2023) support sub-canopy air temperature as an effective
  proxy in homogeneous stands and show why open/above-canopy air is weaker on
  calm, clear nights. The package does not disguise that stable-night
  limitation as calibration.
- The trunk exclusion is defensible for the initial stand-average model.
  Rutter et al. (2023) found little gain from a shortwave heating predictor in
  continuous stands, while Musselman and Pomeroy (2017) found an approximately
  `0.05` trunk view factor at one meter and a highly localized trunk influence.
  The retained gap/edge and individual-tree exclusion is therefore necessary.
- The leaf-off claim is bounded to the Rutter et al. boreal birch evidence; it
  does not claim that dynamic canopy cover already supplies woody-plus-foliage
  radiometric geometry.

## Evidence inspected

Repository artifacts:

- `package.md`
- `source-authority-ledger.csv`
- `equation-ledger.csv`
- `operand-readiness-ledger.csv`
- `formulation-decision.md`
- `uncertainty-and-scope.md`
- `science-summary.md`
- `source-acquisition-needed.csv`
- `tools/generate.py`
- `SC-SNOWFREEZE-001` and the EB-01 authority-gap record

Primary source bytes:

- Essery et al. (2008), DOI `10.1002/hyp.6930`, Equations 1–6.
- Sicart et al. (2004), DOI
  `10.1175/1525-7541(2004)005<0774:ASSODN>2.0.CO;2`, sections 2a–2b.
- Flerchinger et al. (2009), DOI `10.1029/2008WR007394`, algorithm tables and
  conclusions.
- Musselman and Pomeroy (2017), DOI `10.1175/JHM-D-16-0111.1`, Equations
  9–20 and discussion.
- Rutter et al. (2023), DOI `10.1029/2022JD037980`, Equation 2, Table 3, and
  discussion.

Ran read-only repository searches with `sed`, `find`, and `rg` to reconcile
the package equations and prior EB-01 authority gaps. No production code,
contract, fixture, roadmap, package decision, or other agent-owned artifact
was modified.

At this review boundary, A-01 and A-02 are closure blockers. After correction,
Review A should re-check the exact emissivity path, atmospheric coefficients,
units, and the resulting `GO_WITH_PREREQUISITES` claim.
