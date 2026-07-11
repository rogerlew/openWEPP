# Review Agent A — Numerical Meaning and Conservation

Status: `EXECUTED-PASS-WITH-FINDING-RECOMMENDATION`

Evidence mode: `Static + Ran`

Reviewed at UTC: `2026-07-11T06:44:53Z`

Role: independent WSHED-W11E numerical, conservation, authority, and
classification reviewer. Write access was restricted to this artifact.

## Recommendation

`PASS` for Review A's bounded scope, with the package verdict retained as
`SANITY-PASS-WITH-FINDING` because of `W11E-F001`. The current debug evidence
does not reproduce a W11D canonical defect, but it does retain material KW
timestep sensitivity that is too large to describe as an unqualified sanity
pass or as demonstrated timestep convergence.

The exact-release suite and required heavy gates were delegated and still
pending at this review boundary. This recommendation does not authorize final
package closure until those artifacts record green results on the final tree,
both reviews are dispositioned, and same-agent verification is complete.

## Finding summary

| Review ID | Severity | Result |
|---|---|---|
| `A-M1` / `W11E-F001` | Medium | accepted and corrected during review |
| `A-L1` | Low | accepted and corrected during review |

No High finding and no unresolved review finding remains.

## Findings

### A-M1 / W11E-F001 — Material KW timestep response was initially under-classified

Severity: `Medium` classification/evidence finding; not a demonstrated
production or canonical-contract defect.

Static + Ran evidence:

- The current two-channel KW route changes materially when the same hourly
  forcing is projected from 3,600-second to 600-second routing intervals:
  early-spike peak changes `0.999951840 -> 1.999993817 m3/s`, late-spike peak
  changes `0.992440232 -> 1.999993817 m3/s`, and late terminal storage changes
  `65.473952630 -> 110.260168180 m3`.
- These are approximately a 100% early-spike peak increase, a 101.5%
  late-spike peak increase, and a 68.4% late-storage increase. The broad-spread
  and uniform vectors are nearly grid-insensitive, so the response is tied to
  narrow-pulse resolution rather than a uniform scalar offset.
- Every result remains finite and nonnegative; terminal volume is bounded by
  available water, peak/input remains at or below one, sediment closes at
  roundoff, and the public ledger is consistent. Therefore the observation
  violates neither `SC-ROUTE-001#INV-ROUTE-021` nor
  `SC-ROUTE-001#INV-ROUTE-022`.
- The consumer test intentionally prints four `W11C_TIMESTEP` rows but its
  `W11C_FINDING` conditions cover only spread-versus-spike ordering,
  late-versus-early storage ordering, and sediment residual
  (`mt3_hbp_hourly_consumer_contract.rs:363-397,544-555`). Absence of a
  `W11C_FINDING` row therefore did not classify the magnitude of the timestep
  delta.
- The package defines `SANITY-PASS-WITH-FINDING` specifically for a bounded
  numeric observation without a violated canonical invariant
  (`package.md:92-107`). The initially recorded unqualified `SANITY-PASS` did
  not use that declared category.

Required response:

- Record the exact peak/storage deltas as a named finding.
- Use `SANITY-PASS-WITH-FINDING` if exact release and all remaining gates pass.
- Do not reopen W11D or call this a production defect without independent
  routing/timestep authority; equally, do not claim physical timestep
  convergence from the two grids.

Disposition observed during review:

- Accepted and corrected. `sanity-results.md` now records `W11E-F001` with the
  exact values, explicitly states that no current invariant is violated, and
  requires future validation claims to use independent timestep authority.
- `package.md` now records the material response in Surprises & Discoveries and
  selects `SANITY-PASS-WITH-FINDING` in the Decision Log when the remaining
  gates pass.

### A-L1 — Zero-control wording exceeded the asserted surfaces

Severity: `Low` evidence-truthfulness finding.

Static evidence:

- The printed KW and CREAMS zero rows assert peak, outlet volume, sediment,
  and storage are exact zero (`mt3_hbp_hourly_consumer_contract.rs:557-562`).
- The four MC zero controls assert only peak and outlet volume within `1e-12`
  (`mt3_hbp_hourly_consumer_contract.rs:270-296`). Their returned
  storage/sediment/balance fields are not asserted by that test.
- The initial phrase "all zero controls were exact zero" therefore exceeded
  the evidence available for the MC controls.

Required response:

- Distinguish exact printed KW/CREAMS zero rows from the two asserted MC
  surfaces; do not describe the other MC fields as absent from publication.

Disposition observed during review:

- Accepted and corrected. `package.md:94-102` and `sanity-results.md:30-34`
  now state that printed KW/CREAMS zero rows are exact, while the four MC
  controls execute with peak and outlet volume within `1e-12`; other MC fields
  are correctly described as unasserted.

## Authority and before/after legitimacy

Static:

- W11C commit `0c1ae324` is historical characterization, not an acceptance
  target. W11D commit `21f2844a` and canonical `SC-ROUTE-001` v56,
  `SC-SYSTEM-001` v90, and `SC-INFILE-CHANINP-001` v0.1.4 bind current
  classification.
- `INV-ROUTE-021` requires exactly `ntchr` terminals, branch-specific hydraulic
  storage, retained dry carry, and `chvol = volint + sinit - sfnl` without
  material negative storage or generated outlet water. `INV-ROUTE-022`
  requires convex/passive MC recurrence and typed rejection of inadmissible
  grids without clamp or fallback. `INV-SYSTEM-036` requires topology-terminal
  water/sediment publication.
- Static diff review from W11C to W11D confirms that the KW/CREAMS sanity
  scenarios retain the same zero, early spike, early spread, uniform, and late
  spike hourly arrays and the same serial two-channel topology. W11D adds
  blocking corrected assertions, removes inadmissible MC from the executable
  sanity matrix, and exercises those MC grids in a separate typed-rejection
  test. Thus the W11C/current KW and CREAMS rows are legitimate diagnostic
  before/after comparisons rather than different forcing cohorts.
- The two-channel fixture writes topology-correct `chan.inp` IDs `3 4`, validates
  them against `{3,4}`, requires `ParsedBranch` with no warning, and checks the
  requested normalized timestep (`mt3_hbp_hourly_consumer_contract.rs:813-865`).
  This preserves the W11C review correction and prevents a default-grid alias.
- Both HBP files are reparsed from serialized bytes before external runoff and
  sediment totals are assembled (`mt3_hbp_hourly_consumer_contract.rs:894-950`).
  The current source operands therefore do not come only from the in-memory
  arrays used by the fixture writer.

Conclusion: the before/after table is legitimate for the named defect families
and does not treat comparator agreement as authority.

## Water, storage, and sediment anti-tautology

Static:

- W11E correctly labels public `chanwb.Balance` as supporting
  self-consistency, not independent storage authority
  (`package.md:79-86`, `test-design.md:19-23`).
- The real CLI matrix separately checks serialized external HBP totals against
  terminal Parquet runoff, sediment, element identity, storage, and balance.
  These checks establish the public consumer path but do not by themselves
  prove the Manning storage operand.
- Independent authority remains the W11D 101-segment Manning reconstruction,
  fresh-day independent `sinit`/`sfnl`/`chvol` reconstruction, dry carried-
  storage vector, and dual-timestep final-slot vector. W11E neither replaces
  those gates with producer algebra nor relabels the unrestricted flux residual
  as hydraulic storage.
- Uniform KW's printed raw residual of approximately `-10.168594800 m3` is
  correctly explained as omitted initial storage in that diagnostic. The
  authorized ledger includes equal initial/final steady hydraulic storage and
  closes at roundoff; it is not generated water.
- Terminal CREAMS publication independently rejects the historical serial
  aliases: current event element is channel 2, volume is `7,200 m3`, and
  sediment mass is `240 kg`, rather than element 1, `14,400 m3`, or a
  rate-as-mass value.
- Current KW sediment egress equals the reparsed 240 kg source to roundoff
  (maximum printed residual approximately `4.83e-13 kg`). The matrix does not
  infer this mass from `chanwb` water balance.

Ran:

- `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d
  --no-fail-fast`: PASS, 10/10 independent W11D vectors.

Conclusion: W11E uses the public ledger at the right evidence grade and retains
the independent W11D operands as the conservation gate.

## MC admission and typed rejection

Static:

- The four W11C active branch/grid combinations (`ipeak=4/5` at 3,600/600
  seconds) are contract-inadmissible. The current test requires all 16 active
  scenario executions to fail before publication with typed
  `WKERNEL-WS10-CHANNEL-E-003`; four zero controls execute without entering the
  unstable recurrence (`mt3_hbp_hourly_consumer_contract.rs:270-296`).
- A separate 60-second fixture uses admitted geometry and executes both static
  and dynamically refreshed MC with finite positive peaks, a passive
  `<=1.1 m3/s` bound, finite storage, and closed public balance
  (`mt3_hbp_hourly_consumer_contract.rs:299-321,693-750`).
- W11D's full-route kernel vector additionally proves 1,440 finite nonnegative
  terminals, convex coefficients, water closure, and static/dynamic coefficient
  and hydrograph divergence. Thus typed rejection cannot pass vacuously, and
  dynamic MC is not a static alias.

Conclusion: rejection is a successful canonical outcome only for the named
inadmissible grids; a valid production MC route remains executable.

## Executed review evidence

Ran:

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract --no-capture` | PASS, 7/7; 15 result rows, 4 timestep rows, 0 `W11C_FINDING` rows; nextest 13.736 s |
| `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d --no-fail-fast` | PASS, 10/10 |
| `git diff --check` | PASS |
| `git diff --name-only -- '*.rs'` | PASS, no Rust paths modified by W11E |
| `wc -l crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | 1,541 lines; below the 2,000-line advisory threshold |

The focused rerun independently reproduced the package's current debug values,
including maximum printed absolute balance residual approximately
`1.779e-12 m3`, maximum sediment residual approximately `4.83e-13 kg`, and no
emitted `W11C_FINDING` row. The absence of that row does not erase
`W11E-F001`; it reflects the existing test's narrower finding predicates.

## Heavy-gate boundary and final conclusion

At the review snapshot, `gate-results.md` and
`release-binary-provenance.md` remained queued under the explicitly authorized
`comparator_suite_runner`. Review A did not run or substitute the exact release,
erosion, full-workspace, clippy, deny, or documentation gates. No further Cargo
command was run after the heavy runner reported a shared-binary relink race.

Pending heavy work is not silently classified as pass. Final disposition must
remain open until the delegated runner records exact release provenance and
all required gates on a non-racing stabilized execution. Any substantive
release mismatch or real gate failure would supersede this bounded review
recommendation.

Within the completed scope, the real consumer reproduces W11D's corrected
terminal water/storage/sediment behavior; the before/after comparator is
legitimate; conservation evidence is not resting on writer algebra; and MC
admission/rejection is non-vacuous and contract-aligned. With `W11E-F001`
retained as a Medium non-gate observation and A-L1 corrected, Review A
recommends `PASS` and an eventual `SANITY-PASS-WITH-FINDING` verdict if every
remaining required gate passes.
