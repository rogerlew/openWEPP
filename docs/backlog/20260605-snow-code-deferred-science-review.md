# Snow Code Deferred Science Review

## Status

- `state`: backlog
- `maturity`: concept / planning only
- `default_path`: not eligible
- `date`: 2026-06-05
- `relates`: [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)

## Purpose

Capture a deferred science-review need for legacy snow routines before
openWEPP treats their behavior as trusted process authority.

The HPHYS0298-0313 arc showed that the snow/`RM` lineage is not just a porting
problem. The legacy snow routines contain in-source uncertainty markers,
undocumented terms, and explicit comments that some implemented equations
diverge from published WEPP documentation. Those markers are load-bearing for
the same surfaces now used in openWEPP semantic-parity diagnostics:
`snodpt`, `densgt`, `hrsnow`, `driftg`, `wmelt`, `RM`, and `Snow-Water`.

This backlog item proposes a documented science review of `snowd.for` and the
`winter.for` forcing/state surfaces it consumes. The output should be a
contract-grade evidence package that either reconciles the legacy implementation
against external authority or records owned scientific questions before any
production snow-kernel correction is authorized.

## Legacy Evidence

All source citations refer to pinned baseline
`/workdir/wepp-forest_260430_baseline/src/snowd.for` unless stated otherwise.

| Evidence | Location | Why it matters |
|---|---:|---|
| Implemented equation diverges from documentation | `:295` — `XXX -- Note: This equation differs from the on[e] in the User Doc.` for Eq. 3.7.5 | The source records an unreconciled code-versus-documentation discrepancy. openWEPP needs a documented authority decision before treating either side as target behavior. |
| Density-mixing input is questioned in two branches | `:169`, `:183` — `XXX -- Shouldn't "snodpt" be replaced by the snow depth *yesterday* ?` | This directly affects `densgt` and carried snow depth in cold-snowing and melt-density branches. |
| `driftf` / `driftg` definitions are blank | `:18-19` | HPHYS0313 showed these terms can be misattributed. Their physical meaning, units, and active/dead status need explicit evidence. |
| Daily quantity controls an hourly branch | `:112-116` — Dun 2007 changed `hrtemp .lt. -4.0` to `(tmax+tmin)/2 .lt. 0`, with comment `a daily modle in a hourly way` | A daily mean temperature and shifted threshold control an hourly snow/melt regime. The change is documented in source but not reconciled to a physics specification here. |
| Density threshold/cap was edited in place | `:128-129` — prior `if(densgy.gt.250) setf = 1` commented and replaced with `if(densgy.gt.ssd) setf = 1` | The source preserves provenance of a threshold change, but not a contract-grade rationale or authority citation. |
| External snow-equation authority is cited | `:124`, `:137`, `:168`, `:182`, `:294` — CRM Eq. 3.7.1 / 3.7.2 / 3.7.3 / 3.7.5 | The source points to reviewable external authority. The missing work is reconciliation, not discovery. |

These are not style findings. They are unresolved process questions in the
legacy code path that controls snowpack state, density mixing, melt routing,
and water-balance publication surfaces.

## Governance Interpretation

This is an institutional maintenance gap, not a contributor-blame finding.
The dated S. Dun 2007 edits are useful because they preserve authorship,
intent, old behavior, and unresolved uncertainty in source comments. The
deferred work is the custodian-side obligation to convert those uncertainty
markers into documented decisions.

ADR-0017 makes this especially important: comparator agreement is a flag, not a
target. A legacy source comment such as "differs from the User Doc" is not
itself proof that openWEPP should replicate or reject the implementation. It is
a trigger for contract-first adjudication using source provenance, dimensional
checks, external snow equations, and conservation constraints.

## Proposed Review Scope

A future work package should produce a snow science-review dossier covering:

1. **Evidence ledger**
   - Enumerate every snow `XXX`, dated edit, blank definition, and
     documentation-divergence marker with file, line, symbol, branch, and
     affected openWEPP surface.
2. **Dimensional audit**
   - Declare units for `snodpt`, `densg/densgt/densgy`, `hrsnow`, `hrmlt`,
     `driftf`, `driftg`, `wmelt`, and density-settling terms.
   - Separate daily, hourly, state, and publication surfaces.
3. **Regime audit**
   - Identify branch predicates and time-base assumptions, especially the
     daily-mean-temperature condition used inside hourly snow processing.
4. **External-authority reconciliation**
   - Compare implementation lines against CRM Chapter 3.7 equations and WEPP
     User Documentation.
   - Record whether each discrepancy is documented correction, legacy defect,
     ambiguous scientific question, or intentionally replicated legacy wobble.
5. **Contract-grade disposition**
   - Produce proposed `SC-SNOWFREEZE-001` amendments or an explicit `HOLD`
     artifact for unresolved questions.

## Non-Goals

- No production snow-kernel edits from this backlog note.
- No silent adoption of legacy behavior as correct without external authority.
- No empirical compensation in WB13/WB17/WB18/WB19/WB12 for unresolved snow
  defects.
- No personal blame assignment for historical comments or edits.

## Governing Constraints

- Correctness authority remains ADR-0011 + canonical `SC-*` contracts +
  conservation and external physics authority.
- Legacy source is evidence of implementation and intent, not a correctness
  oracle.
- Any resulting physics change must be contract-first:
  1. canonical `SC-*` amendment,
  2. contract-derived tests,
  3. pre-implementation gate evidence,
  4. production implementation.
- Comparator deltas caused by reviewed snow behavior must be classified under
  ADR-0017: `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`,
  `OPENWEPP-DEFECTIVE`, or `UNRESOLVED`.
- Any accepted legacy wobble must be explicitly named, justified, and owned so
  it cannot silently become target physics.

## Promotion Criteria

This backlog item becomes eligible for a work package only when the package can
name its authority set and validation gates. Minimum gates:

1. **Pinned source ledger:** all relevant `snowd.for` / `winter.for` lines
   enumerated with symbols, units, and affected surfaces.
2. **External references:** CRM Chapter 3.7 and WEPP User Documentation
   citations available with exact equation/section references.
3. **Contract target:** specific `SC-SNOWFREEZE-001` invariants/obligations to
   amend or create.
4. **No production-edit shortcut:** package starts with review/specification;
   implementation follows only after contract authority is accepted.
5. **Owned unresolveds:** each unresolved scientific question carries owner,
   next evidence gate, and follow-on package trigger.

## Open Questions

- Is the Eq. 3.7.5 / User Documentation divergence at `snowd.for:295` a legacy
  implementation defect, an undocumented correction, or a documentation error?
- Does density mixing in the flagged branches use current-hour snow depth or
  prior-day snow depth, and what does CRM Eq. 3.7.3 require?
- What are `driftf` and `driftg` physically and dimensionally? Are they active
  in the forest configuration or dead inputs?
- Is the daily-mean-temperature condition inside hourly processing defensible
  as a regime proxy, or does it require re-derivation?
- Which reviewed behavior, if any, should be preserved as documented legacy
  wobble rather than corrected physics?
