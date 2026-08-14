# Review Agent A — Contract/Profile Review

Evidence class: `Static + Ran`

Initial verdict: `HOLD`.

The initial review found incompatible LSE resource identity, missing mandatory
kernel-profile/obligation/guard surfaces, incomplete routed topology and
configuration/transaction identity, lexical-only vectors, incomplete unit
governance, under-specified calibration posture, and missing proportional
enthalpy splitting. Exact details remain preserved in the agent transcript for
the review turn.

Contract versions 2 and 3 dispositioned every finding. The final v3 re-review
independently ran the authority suite (9/9), boundary-unit registry (21/21), SC
unit compliance, and diff hygiene.

Final verdict: `PASS / no material authority or profile finding`.

The final review confirmed exact LSE identity, embedded per-OFE continuations,
source/tile-local retention, `basis_ofe_id` re-keying, machine-readable units,
truthful runtime-parity deferral, no raw-rain/canopy duplication,
retained-energy basis, and unequal-area routing.

Bounded v4 amendment review: `PASS`. The strict, digest-bound
`ground_ingress_mode` closes caller-driven branch inference without a new
material ambiguity or unit-registry seam.

## Rust implementation review at `a92cd5db5`

Evidence class: `Static exact-byte`.

Verdict: `HOLD / implementation remediation required`.

The reviewer accepted seven material findings:

1. The unified finalization discarded actual LSE, soil-thermal, retained
   enthalpy, infiltration, and rollback candidates.
2. Public mutable resource-candidate fields made the ingress closure
   self-referential and forgeable.
3. Derived restart JSON and separately named digest views did not implement one
   canonical persisted representation; restart combinations were under-checked.
4. Unified identity omitted exact production OFE/lane/area/day binding and
   hashed only the legacy 64-bit soil fingerprint rather than canonical bytes.
5. `runoff.rs` crossed the mandatory 3000-line threshold; two other touched
   files require WARN rationale.
6. Runtime errors lacked canonical codes, precedence, identity, and rollback
   context.
7. The complete WB14 interval state transition was duplicated between daily
   and continuation wrappers, with insufficient boundary parity vectors.

All findings are accepted for this package. No GO or terminal claim is carried
forward from the earlier bounded authority review.
