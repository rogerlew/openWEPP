# CAL-07C Prospective Review B

Evidence class: `Static`

Review scope: CAL-07C `package.md`, `artifacts/intent-plan.md`,
`artifacts/required-reading-map.md`,
`artifacts/pre-execution-source-inventory.md`, retained CAL-07 and CAL-07B
package/disposition/science-summary evidence, `SC-PLANT-001`
OBL-PLANT-P-013, ADR-0028, ADR-0042, and the retained POWER source/method
objects. No CAL-07C result-bearing executor, analyzer, or validator output was
reviewed.

Recommendation: `GO FOR BOUNDED RESULT-BEARING EXECUTION AFTER THE DECLARED
PRE-EXECUTION ADMISSION GATES`

The amended protocol may proceed to CAL-07C result-bearing execution if, and
only if, the package first materializes the source/admission manifests and the
executor seam exactly as declared. I accept the amended daily-operand boundary:
the typed GSI kernel consumes a daily `VPD` operand, and the pre-execution
inventory reports all 1,666 admitted Alerce daily mean hourly-product VPD
values as finite and nonnegative. The 349 negative hourly paired-product rows
remain scientifically important negative evidence and a hard claim ceiling, but
they are not by themselves a kernel-input-domain failure for this bounded,
package-local daily-GSI execution.

This is not a production operator approval and does not replace
OBL-PLANT-P-013.

## Findings by severity

### Blocker

None for the amended protocol, provided the phase-3 source/admission and
executor-seam gates run before canopy output is minted.

### Major findings requiring disposition

#### CAL07C-PRB-001 — Executor path must prove Alerce consumes admitted `vpd_pa`

The original CAL-07 executor computed VPD internally from daily
`Tmax/Tmin/Tdew` and therefore correctly failed closed. CAL-07C can only lift
that immediate blocker if the package-local executor/schema makes VPD an
explicit forcing field and passes that value to
`GsiDailyForcing::vapor_pressure_deficit_pa`.

Required disposition before accepting results:

- Alerce rows carry explicit `vpd_pa` and `vpd_source` provenance from the
  hourly-product daily mean.
- Beza rows remain unchanged and continue to use the OBL-PLANT-P-013 daily
  summary operator.
- Validation proves the executor output VPD equals the admitted package-local
  operand, and on the three CAL-07 negative dates does not equal the rejected
  daily-summary value.
- A “what still reads the old path?” check names the producer table, executor
  input struct, `GsiDailyForcing` handoff, daily output surface, and negative
  proof that Alerce did not recompute VPD from CAL-07 daily extrema.

Rationale: accepted. This is already consistent with CAL-07C's intended
executor path, but it must be evidenced directly before result claims.

#### CAL07C-PRB-002 — Source/admission custody must be complete before execution

The retained full-period hourly source object and POWER method pages are
present, and `pre-execution-source-inventory.md` reports the required 39,984
`T2M` keys, 39,984 `T2MDEW` keys, 1,666 complete LST days, zero daily negative
VPD rows, and serialized-resolution compatibility with frozen CAL-07 daily
temperature/dew-point operands. Result-bearing execution should still wait for
the planned source/admission manifest.

Required disposition before accepting results:

- Manifest records path, SHA-256, byte count, retrieval timestamp, exact URL,
  API version, time standard, fill value, geometry/elevation, source list,
  units, and start/end metadata for each CAL-07C source object.
- The manifest links the frozen CAL-07 and CAL-07B predecessor evidence by
  path/digest rather than mutating or reacquiring it.
- The admission table records every Alerce day, 24-hour key inventory,
  hourly-negative count/minimum, daily admitted VPD, daily operand residuals,
  and pass/hold status.

Rationale: accepted. The protocol is sufficient if phase 3 produces this
custody before phase 4 execution; skipping it would be a hold.

#### CAL07C-PRB-003 — Claim wording must not say full-period hourly products are positive

After the amendment, CAL-07C correctly records 349 negative hourly
paired-product rows as retained signed components. One authority-boundary
sentence still says CAL-07B proved “paired hourly POWER products are
positive.” That was true for CAL-07B's three failure-date cases, not for the
full CAL-07C period.

Required disposition before final claims:

- Narrow that sentence to the three CAL-07 failure dates, or replace it with
  the amended full-period inventory statement.
- Final science summary, figure sidecars, roadmap, and catalog text must not
  imply POWER hourly-product VPD is everywhere nonnegative or physically
  admissible.

Rationale: accepted as claim-calibration. This does not block result execution
once the amended boundary is preserved, but it blocks a clean final disposition
if left uncorrected.

#### CAL07C-PRB-004 — Figures and sidecars must expose the negative hourly evidence

The package now says negative hourly products are counted, plotted, and carried
into source/claim disposition. That obligation is necessary because the daily
mean admission otherwise risks visually hiding subdaily source incompatibility.

Required disposition before final claims:

- At least one source/admission figure makes the 0 Pa line, negative hourly
  rows, minimum hourly VPD, and daily admitted VPD visible without relying only
  on color.
- Result figures separate Alerce's hourly-derived daily VPD from Beza's
  unchanged daily-summary VPD and label both source operators.
- Each Markdown sidecar binds exact source/admission table digests, equations,
  units, LST basis, POWER gridded/reanalysis limitation, negative-hourly claim
  ceiling, no-clipping rule, and no-OBL-replacement boundary.

Rationale: accepted. Figure evidence is not merely decorative here; it is part
of the scientific negative-evidence record.

### Minor observations

- The package's 37-member, two-site, 1,666-day output expectation implies
  123,284 daily member/site rows if execution proceeds. Validation should
  assert that exact inventory before any score or verdict table is accepted.
- The final roadmap/catalog update should distinguish three statuses if they
  occur: Alerce forcing blocker lifted for CAL-07C only, CAL-07C bounded
  observational evaluation completed, and Order 7 advancement withheld or
  advanced. These are not interchangeable.

## Accepted rationale

- `SC-PLANT-001` binds the kernel to a finite nonnegative daily `VPD` operand;
  OBL-PLANT-P-013 remains the production derivation and still hard-fails
  negative daily-summary VPD. CAL-07C does not amend that obligation.
- ADR-0042 allows data/source limitations to constrain claims without
  preventing bounded evaluation when the science implementation surface and
  evidence limits are explicit.
- ADR-0028 is not being used to promote a new production mechanism. CAL-07C is
  only a package-local source-product admission for a bounded evaluation, with
  production change deferred to a separate contract-first package if ever
  pursued.
- The amended negative-hour boundary is acceptable because negative hourly
  components are not clipped, deleted, or normalized; they are retained in the
  arithmetic mean and reported as a claim ceiling.
- Keeping Beza unchanged preserves the original CAL-07 comparison lane and
  prevents a broad silent forcing rewrite.

## Rejected claims

The protocol does not support any of the following:

- replacing OBL-PLANT-P-013 in production;
- clipping or canonicalizing negative daily contract VPD;
- claiming POWER hourly-product VPD is everywhere physically valid;
- claiming POWER gridded forcing is on-site meteorology;
- using CAL-07C alone to calibrate absolute LAI, biomass, canopy cover, or
  evergreen floor;
- claiming Order 7 completion unless every non-forcing CAL-07 gate also passes
  with its evidence ceiling preserved.

## Final GO/HOLD

`GO FOR BOUNDED RESULT-BEARING EXECUTION AFTER THE DECLARED PRE-EXECUTION
ADMISSION GATES`.

If source/admission custody, full-period daily nonnegative VPD admission, Beza
unchanged forcing, or the explicit executor VPD seam cannot be proven before
canopy output is generated, the package must switch to `HOLD` with that exact
blocker named.
