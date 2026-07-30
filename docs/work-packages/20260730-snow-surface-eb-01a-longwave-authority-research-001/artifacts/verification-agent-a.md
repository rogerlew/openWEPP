# Amendment-Aware Terminal Verification A

Evidence class: `Ran + Static + primary-source verification`.

Verdict: `PASS`.

Verification target: the exact current `SNOW-SURFACE-EB-01A` tree after the
derived-sky-view amendment and accepted review corrections.

## Science And Authority

- Essery et al. (2025), FSM2 Equation 14, gives
  `tau_d,n = exp(-1.6 k_ext Lambda_n)`. Its published default
  `k_ext = 0.5` for randomly oriented canopy elements yields
  `exp(-0.8 VAI_eff)`. Section 2.3 and Equations 25 and 29 explicitly reuse
  diffuse transmission as the complementary atmospheric/canopy longwave
  weight. The package therefore admits a valid homogeneous-canopy mapping
  base.
- The package does not overstate that base. FSM2 defines vegetation area as
  leaf plus stem area but does not define the exact composition of openWEPP
  `VAI_eff`. That composition remains a canonical EB-02 decision, so the
  equation-ledger disposition `ADMIT_MAPPING_BASE` and operand status
  `DERIVED_OPERATOR_NOT_BOUND` are truthful.
- The underlying radiative meaning remains hemispherical and angle weighted.
  Dynamic canopy cover, structural cover, and LAI are inputs to a future
  documented effective-area or clumping mapping; none is directly relabeled
  as sky-view fraction.
- Static structural canopy cover is source-authorized and reaches the native
  forest daily canopy builder. The builder already publishes dynamic canopy
  cover as the maximum of structural cover and foliar cover, while the
  downstream daily-growth state exposes dynamic cover rather than a separate
  structural field. The operand ledger correctly records
  `CONSUMER_BINDING_NOT_READY` and requires a double-count guard.
- Canopy height is not inserted into the homogeneous Beer-law expression. It
  may enter only under separately admitted finite-crown, gap, or geometric
  authority.
- The leaf-off invariant is preserved without invented stem area: the future
  mapping must retain woody obstruction, but this package does not synthesize
  an unsupported cover/LAI blend.
- The prior longwave formulation remains dimensionally and physically
  coherent: canopy emission displaces obscured atmospheric emission, effective
  canopy and snow emissivity are fixed at one, snow emission is upward, and
  net longwave is positive toward snow in `W m^-2`.
- The Dilley-Unsworth atmospheric route, air-temperature proxy limits, stable
  nighttime caveat, stand-scale trunk exclusion, and polar-night limitation
  remain unchanged and source bounded.

## Product And Runtime Boundary

- No new user-entered sky-view coefficient is authorized.
- Hemispherical photographs, LiDAR, canopy-height models, and sub-canopy
  radiation observations remain optional validation or uncertainty evidence,
  not required runtime inputs.
- If existing canopy state cannot support an authority-backed deterministic
  effective-vegetation-area mapping, the stop-loss requires `HOLD` or explicit
  model-limitation adjudication. It prohibits an invented or site-fitted blend
  and prohibits escaping through a new user coefficient or required remote
  dataset.
- EB-02 canonical-contract research is
  `GO_WITH_PREREQUISITES`; EB-02 runtime implementation remains `HOLD` until
  the canopy-state mapping, cloud mapping, effective-unity exchange, and active
  snow-temperature provider are canonically bound. The package makes no
  production-readiness, default-activation, or executable-behavior claim.

## Reviews, Findings, And Lifecycle Records

- Both amendment-aware reviews record `PASS`.
- `finding-disposition.md` explicitly dispositions A-01/A-02, RB-01 through
  RB-07, and DSV-RB-01 through DSV-RB-05. No finding is deferred or omitted.
- The package, top-level roadmap, campaign roadmap, catalog, formulation
  decision, science summary, ledgers, figure sidecar, and final disposition all
  preserve the same split: contract research admitted, runtime implementation
  held, derived existing-state sky view required, and no new user or remote
  input.
- The exact diff matches the prospective write set: `docs/ROADMAP.md`, the
  snow surface-energy roadmap, `docs/work-packages/README.md`, and this package
  tree. No Rust, test, fixture, reference object, usersum, canonical contract,
  selector, default, or public schema is changed.

## Checks Run

- Ran the package generator in a temporary package copy: `PASS`; both generated
  SVGs are byte-identical to the exact tree.
- Parsed the four current ledgers with all retained fields nonblank:
  source authority `10 x 6`, equation `13 x 7`, acquisition `4 x 5`, and
  operand readiness `15 x 6`, including headers.
- Parsed both SVGs: each has `role="img"`, exactly one title and description,
  and an exact same-stem Markdown sidecar.
- Rasterized and visually inspected both figures. Titles, axes, units, status
  labels, legends, and curves are legible without clipping or overlap. The
  readiness figure correctly marks derived sky view as an amber contract step.
- Ran `markdown-doc lint` and `markdown-doc validate` over all 23 package
  Markdown files and each changed roadmap/catalog file: zero errors or
  warnings.
- Checked every local Markdown link in the package and changed
  roadmap/catalog files: all resolve.
- Ran `git diff --check`: `PASS`.
- Confirmed `package.md` is 197 lines. No Rust line-count threshold applies.

Rust, Nextest, Clippy, comparator, calibration, and empirical gates are
correctly `NOT APPLICABLE` to this exact documentation-only research diff. No
required current-scope validation is deferred, and Verification A finds no
remaining closure blocker.
