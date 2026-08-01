# Independent Authority Review B

Evidence mode: `Static`.

Review scope: package plan, authority manifest, frozen receipt, reconciliation
record, and exactly the four Phase A whitelisted authority inputs. No retained
result, observation, residual, score, attempt, or terminal-audit evidence was
read. Reviewer A's artifact was not read.

## Findings

### Moderate — scope the converted tolerance to the correct predicate

The dimensional decision is correct, but the version-6 amendment must not
generalize `1e-6 kg m^-2` to every mass-valued closure check. The pre-result
qualification protocol distinguishes hourly/daily vapor aggregation at
`1e-9 kg m^-2` from vapor-to-sublimation identity closure at
`1e-6 kg m^-2` (`prospective-qualification-protocol.md`, lines 29-34). The
contract amendment should bind `1e-6 kg m^-2` only to:

- the canonical `1e-9 m` SWE residual when that same water-equivalent residual
  is expressed as areal mass; and
- the vapor-to-sublimation transfer identity named by the frozen authority
  decision.

It should explicitly preserve other independently defined predicates,
including the represented-layer lifecycle boundary and any separately
specified hourly/daily aggregation tolerance.

Disposition: amendment requirement; not a blocker to freezing the authority
decision.

### Low — publish the normative decimal, not its binary rendering

The receipt's computed `1.0000000000000002e-06` is the ordinary binary `f64`
rendering of the exact dimensional product. Version 6 should state the
normative tolerance as `1e-6 kg m^-2`, with the identity
`1e-9 m * 1000 kg m^-3 = 1e-6 kg m^-2`. It should not imply that the rendered
floating-point tail is a distinct or relaxed threshold.

Disposition: editorial/numeric clarity amendment.

## Authority Assessment

### Dimensional authority

`SC-SNOWENERGY-001` version 5 establishes runtime mass closure at `1e-9 m`
SWE (`Tolerance and Numeric Notes`, lines 698-702). The unit-governance policy
requires named, directional, provenance-backed conversions
(`unit-governance.md`, lines 111-128). The named production helper defines
liquid-water density as `1000 kg m^-3` and evaluates areal mass as SWE depth
times that density (`openwepp-unit-boundary/src/lib.rs`, lines 85-97).
Therefore:

```text
1e-9 m * 1000 kg m^-3 = 1e-6 kg m^-2
```

The prospectively frozen qualification protocol independently selected
`1e-9 m` for snow-mass reconstruction and `1e-6 kg m^-2` for the
vapor-to-sublimation identity before result execution
(`prospective-qualification-protocol.md`, lines 20-39). The frozen receipt's
classification `CROSS_UNIT_PROTOCOL_TRANSCRIPTION_ERROR` is supported.

### Result-blindness

The manifest restricts Phase A to four pre-result inputs. Their current
SHA-256 identities exactly match all four hashes recorded in
`authority-freeze.json`. The reconciliation and receipt contain no result,
observation, residual-magnitude, or score dependency. The dimensional outcome
follows from the fixed `1e-9 m` authority, fixed density, named conversion, and
prospective protocol alone. Result-blindness passes for this static review.

### Distinction from represented-layer lifecycle

The lifecycle predicate is not a closure tolerance. Version 5 defines a layer
as represented only above `1e-9 kg m^-2`, equivalent to `1e-12 m` SWE, while
retaining `1e-9 m` for aggregate SWE/depth residual closure
(`SC-SNOWENERGY-001`, lines 54-59, 558, and 657-662). The frozen receipt
correctly keeps these predicates distinct. Substituting the lifecycle boundary
for the converted aggregate/transfer identity would be a thousand-fold unit
transcription error.

### Required version-6 amendment

Before Phase B, version 6 should:

1. increment `contract_version` and add a version-6 change-log entry;
2. state `1e-9 m SWE == 1e-6 kg m^-2` under the named liquid-water-density
   conversion;
3. bind `1e-6 kg m^-2` narrowly to the same SWE residual expressed as areal
   mass and to vapor-to-sublimation identity closure;
4. retain the distinct `1e-9 kg m^-2` represented-layer lifecycle boundary
   (`1e-12 m` SWE equivalent); and
5. state that separately governed mass predicates, especially hourly/daily
   vapor aggregation, retain their own tolerances.

## Decision

`GO_WITH_AMENDMENTS`

The authority freeze is dimensionally correct, independently supported by a
pre-result protocol, and adequately result-blind. Phase B may begin only after
the version-6 amendment satisfies the five requirements above and the amended
contract is verified against the frozen authority receipt.
