# CAL-07 Independent Terminal Scientific and Method Review B

Evidence class: `Static + Ran`

Recommendation: `PASS FOR PACKAGE CLOSURE AS HOLD`

No closure-blocking scientific, custody, execution, claim-calibration, or
figure defect was found. This recommendation closes only the documented
`HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT` disposition. It
does not pass CAL-07, advance roadmap Order 7, or authorize a forcing repair.

## Ranked findings

### P2 — The gate-evidence receipt slightly overstates validator coverage

`gate-evidence.md` says `validate_hold.py` checked “source and custody hashes.”
The validator checks every retained source object's manifest size and SHA-256,
but it checks the custody table only for 37 rows and 37 unique candidate IDs.
It does not compare the two recorded predecessor hashes with the current
CAL-04B files or compare the seven execution columns with `inputs/ensemble.csv`.

This is not closure-blocking because this review ran those missing checks:
the predecessor hashes are exactly
`83e749a3961604e4592f2a2217db30965c8bbb59f4752d0ff6d85fbac61fd986`
and
`a56a07ef1df713a1555afb4098bd4f5e0fbe4b9f324ded0bd370c785fea3190e`,
and the 37 custody rows match the execution ensemble after CSV newline
normalization. Correct the receipt wording or extend `validate_hold.py` in a
future maintenance pass so the named validator and its claimed coverage are
identical.

Disposition: `accepted, non-blocking documentation/validator precision`.

### P2 — The VPD compatibility plot is boundary-readable but not
quantitatively self-contained

`cal07-forcing-vpd-compatibility.svg` clearly exposes all three negative
Alerce points and the zero rule. Its vertical axis has no numeric ticks, and
the zero rule is not labeled in the plot, although the sidecar says the
panel-specific ranges are “shown explicitly.” Exact failed values remain
available in the companion operands plot and both sidecars, so no scientific
claim is lost and the diagnostic set is usable. If the figure is revised,
add sparse numeric ticks and an in-plot `0 Pa contract boundary` label.

Disposition: `accepted, non-blocking figure-usability improvement`.

## Source custody and observational authority

Static:

- The package retains 13 exact source objects, full acquisition endpoints,
  retrieval date, byte counts, and SHA-256 digests. The files include both
  raw daily camera products, transition products, site metadata, ROI records,
  the live fair-use statement, and both raw POWER responses.
- The authority record correctly limits the PhenoCam lanes to provisional,
  shared-method relative-greenness evidence. It does not convert GCC90 or an
  evergreen vegetation label into LAI, foliar mass, canopy amplitude, or a
  quantitative evergreen floor.
- POWER is correctly described as gridded/reanalysis forcing, and the material
  Alerce elevation mismatch is retained. The source record does not claim
  on-site meteorological representativeness.

Ran:

- `validate_hold.py` matched every retained source object's size and digest.
- Independent predecessor hashing matched every digest repeated in the
  37-row custody table; all candidate IDs are unique and the parameter rows
  match `inputs/ensemble.csv`.
- The live retained fair-use page explicitly states CC BY 4.0, while the
  package truthfully preserves and explains the stale generated-metadata
  wording instead of silently discarding the conflict.

## Fail-closed execution and negative-VPD diagnosis

Static:

- The Rust executor uses the contract equation
  `1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`, rejects negative or
  non-finite VPD, and performs no clip, dew-point edit, deletion,
  interpolation, or site/member omission.
- Output is accumulated in memory and written only after all members, sites,
  and days complete. Therefore the first invalid day cannot publish a partial
  daily result.
- `execute.py` runs the two focused phase/consumer tests only after successful
  kernel execution. The package correctly reports that they did not run and
  makes no passage claim.

Ran:

- A fresh package-local executor invocation with an isolated Cargo target and
  temporary output failed at `SH-EN-ALERCE / 2022-07-22`, returned status 1,
  and created no output file.
- `validate_hold.py` passed with 3,332 diagnosed forcing rows and exactly
  three negative Alerce days: 2022-07-22, 2022-09-15, and 2025-09-09.
- The independent values are negative and finite; the 2025 event lies inside
  a prespecified scoring year, so the incompatibility cannot be dismissed as
  warm-up-only evidence.
- None of `daily-kernel-output.csv`, `gate-results.csv`,
  `ensemble-daily.csv`, `shape-scores.csv`, `transition-residuals.csv`, or
  `verdict-matrix.csv` exists. No partial or result-bearing canopy artifact
  leaked into the terminal tree.

## Prospective integrity, figures, and claims

Static:

- Both prospective review records end in explicit bounded-execution `GO`
  decisions and state that no result-bearing artifact informed those
  decisions. Their frozen operators and evidence ceilings remain intact in
  the terminal package. The recorded filesystem ordering also places both
  final prospective reviews before the execution incident, although file
  timestamps are supporting rather than immutable provenance.
- All four SVGs are diagnostic source/hold figures, not canopy-result
  substitutes. The observation plot leaves camera gaps disconnected; the
  operands plot prints each negative VPD value; the boundary plot labels every
  state in text as well as color.
- Every figure has the required Markdown sidecar sections and exact source
  binding. The diagnostic manifest matches the current CSVs, SVGs, and
  sidecars byte-for-byte. Visual rendering confirmed legible labels and no
  missing or misleading model series.
- The roadmap and work-package catalog truthfully say Order 7 is open, no
  canopy result exists, and advancement did not pass. They preserve the
  unevaluated amplitude/floor and phase-transformed-real-consumer boundaries.

Ran:

- `validate_hold.py` passed.
- All diagnostic-manifest size and SHA-256 rows matched independently.
- All four SVGs rendered successfully and were inspected together with their
  sidecars.

## Resume and closure decision

The hold boundary is scientifically legitimate: the current contract rejects
the frozen input, and the package has no authority to manufacture a repaired
forcing series. Resume requires either continuous, provenance-complete,
contract-admissible forcing for the frozen Alerce lane or a prior
science-contract amendment that defines bounded canonicalization, including
threshold, units, rationale, provenance, and tests.

Either route is necessary but does not itself pass roadmap Order 7. A resumed
run must still honor the frozen ensemble and operators, re-run all required
gates, and retain the already named quantitative amplitude/evergreen-floor
and phase-transformed-real-consumer limitations until independent evidence
closes them.

Terminal recommendation: `PASS FOR PACKAGE CLOSURE AS HOLD`.

## Correction re-review

Evidence class: `Static + Ran`

Superseding recommendation: `PASS FOR PACKAGE CLOSURE AS HOLD`

Both nonblocking P2 findings are corrected. `validate_hold.py` now hashes the
exact CAL-04B accepted-ensemble and candidate-configuration tables and
requires the custody table to contain those two digests. The corrected
forcing-VPD figure now provides numeric ticks for both panel-specific scales
and labels the `0 Pa contract boundary` directly in each panel.

Ran: the corrected hold validator passed; every current diagnostic-manifest
size and SHA-256 row matched; and the revised SVG rendered with legible ticks,
boundary labels, and all three negative-Alerce markers. The finding
disposition accurately records both corrections. No open Review B finding
remains, and the scientific hold disposition is unchanged.
