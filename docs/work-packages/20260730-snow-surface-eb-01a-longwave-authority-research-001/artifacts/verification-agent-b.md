# Amendment-Aware Terminal Verification B

Evidence class: `Ran + Static + primary-source verification`.

Verdict: `PASS`.

Verification target: the exact current `SNOW-SURFACE-EB-01A` tree after the
derived-sky-view amendment, accepted review corrections, complete DSV finding
disposition, and corrected roadmap prerequisite wording.

## Authority And Derived Operator

- The official Essery et al. (2025) version of record confirms FSM2
  Equation 14:
  `tau_d,n = exp(-1.6 k_ext Lambda_n)`. Its published default
  `k_ext = 0.5` for randomly oriented canopy elements gives
  `exp(-0.8 VAI_eff)`. Section 2.3 and Equations 25 and 29 reuse this diffuse
  transmission as the complementary atmospheric/canopy longwave weight.
- The package admits only that homogeneous-canopy mapping base. FSM2 defines
  vegetation area as leaves plus stems, but it does not define the exact
  composition of openWEPP `VAI_eff`. The equation-ledger
  `ADMIT_MAPPING_BASE` and operand-ledger `DERIVED_OPERATOR_NOT_BOUND`
  dispositions preserve that canonical EB-02 decision.
- Dynamic canopy cover, structural cover, and LAI remain possible inputs to a
  future documented effective-area or clumping operator. None is directly
  relabeled as hemispherical radiometric sky view.
- The native-forest structural-cover lineage is truthful. Static structural
  cover reaches the daily canopy builder and already forms the floor under
  foliar dynamic cover, but it is not exposed as a separate downstream daily
  growth field. The operand ledger therefore requires explicit consumer
  binding and a guard against counting the same structural obstruction both
  through the dynamic-cover floor and again in `VAI_eff`.
- Canopy height is excluded from the homogeneous Beer-law operator unless a
  later finite-crown, gap, or geometric formulation gives it a defined role.
  The leaf-off requirement likewise preserves woody obstruction without
  inventing a stem-area conversion.
- The underlying exchange remains coherent:
  `L_sub_down = f_sky L_atm_down + (1-f_sky) sigma T_c^4`, with effective
  canopy and snow emissivity fixed at one, snow emission directed away from
  the snow, and net longwave positive toward snow in `W m^-2`.
- The corrected Dilley-Unsworth route, canopy-air proxy limits, stable-night
  limitation, stand-scale trunk exclusion, and polar-night caveat remain
  source-bounded and unchanged by the amendment.

## Product And Runtime Boundary

- The derived operator requires no new user-entered sky-view coefficient.
- Hemispherical photography, LiDAR, canopy-height products, sub-canopy
  radiation, and other remote observations are optional validation or
  uncertainty evidence, not required runtime data.
- Accessible primary sources support every load-bearing decision. Lawler and
  Link (2011) and the full Pomeroy et al. (2009) paper remain optional only if
  later work expands into discontinuous-canopy, gap-edge, or explicit-trunk
  physics. The acquisition ledger truthfully requests no user article or
  remote dataset for the present stand-scale decision.
- If an authority-backed deterministic `VAI_eff` composition cannot be formed
  from existing state, the amended stop-loss requires continued `HOLD` or
  explicit model-limitation adjudication. It prohibits an invented or
  site-fitted blend, a new user coefficient, or required remote data as escape
  routes.
- EB-02 canonical-contract research is
  `GO_WITH_PREREQUISITES`. Runtime implementation remains `HOLD` until the
  canopy-state-to-sky-view operator, Dilley-Unsworth cloud mapping,
  effective-unity exchange, and active snow-surface-temperature provider are
  canonically bound. No production-readiness, default, or executable-behavior
  claim is made.

## Reviews, Findings, And Lifecycle Records

- Both amendment-aware reviews record `PASS`.
- `finding-disposition.md` now explicitly dispositions A-01/A-02, RB-01
  through RB-07, and every DSV finding from DSV-RB-01 through DSV-RB-05.
  No finding is deferred, omitted, or left open.
- Amendment-aware Terminal Verification A independently records `PASS` against
  the 197-line package and current ledger dimensions.
- The package, final disposition, top-level roadmap, campaign roadmap,
  catalog, formulation decision, science summary, equation and operand
  ledgers, readiness figure, and sidecar consistently preserve the derived
  existing-state operator, no-direct-alias rule, no-new-user-coefficient rule,
  no-required-remote-data rule, and runtime `HOLD`.
- The top-level roadmap now refers to “those prerequisites” rather than an
  incorrect numeric count.

## Checks Run

| Check | Result | Evidence |
| --- | --- | --- |
| Deterministic generation | `PASS` | Ran `.venv/bin/python <temporary-copy>/tools/generate.py`; both generated SVG SHA-256 values are byte-identical to the exact tree. |
| CSV dimensions and content | `PASS` | Parsed rectangular nonblank retained records: source authority `10 x 6`, equation `13 x 7`, acquisition `4 x 5`, and operand readiness `15 x 6`, including headers. |
| SVG and sidecars | `PASS` | Both SVGs parse, have `role="img"`, exactly one title and description, and exact same-stem Markdown sidecars. |
| Visual inspection | `PASS` | Rasterized the amended readiness figure and inspected both figures. Titles, axes, units, legends, status labels, and curves are legible without clipping or overlap; sky view is correctly amber as “derive from canopy state.” |
| Markdown | `PASS` | `markdown-doc lint` and `markdown-doc validate` passed the 23-file package and all three changed roadmap/catalog files with zero errors or warnings. |
| Whitespace | `PASS` | `git diff --check` passed after the final DSV and roadmap corrections. |
| Package line count | `PASS` | `package.md` is `197` lines, matching line-count governance. No Rust threshold applies. |
| Exact write set | `PASS` | `git status --short` and the diff from base `8cba9424e7bce816e9ee2dc012a358bdefb1aaf6` contain only `docs/ROADMAP.md`, the snow surface-energy roadmap, the work-package catalog, and this package tree. |

No Rust, test, fixture, reference object, usersum, canonical contract,
selector, default, or public schema path changed. Rust, Nextest, Clippy,
comparator, calibration, and empirical gates are correctly `NOT APPLICABLE`
to this exact documentation-only research diff.

## Closure

No amendment-aware terminal finding remains. All required gates pass, every
review finding is dispositioned, and the contract-versus-runtime admission
split is truthful. `COMPLETE / PASS` is valid for this research package while
EB-02 runtime implementation remains explicitly held.
