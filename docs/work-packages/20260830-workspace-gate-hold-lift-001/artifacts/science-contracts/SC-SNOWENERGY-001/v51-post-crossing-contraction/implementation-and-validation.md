# V51 post-crossing contraction implementation and validation

Status: `IMPLEMENTED; DUAL REVIEW APPROVED; CANONICAL R133 PENDING`

Evidence mode: `Static + Ran`

## Exact correction

R132 proved one adjacent, direction-consistent canonical phase-predicate
crossing followed by finite corrections entirely inside the entered phase.
The binary64 enthalpy steps are
`+5069.96060020145`, `+2965.9686717161603`,
`-997.5636528494651`, and `+340.4305842210815 J m^-2`; their absolute
magnitudes strictly decrease and the within-phase corrections alternate.
V41 rejected this lawful sequence solely because it required one signed
enthalpy direction after the phase crossing.

V51 adds one bounded eligibility posture. It first constructs a shared fully
validated V41/V51 trace. That trace validates every window's exact support,
static joins, two-map cadence, promoted-root chain, opposite vapor side,
finite/nonstagnant/no-A-B-A raw owner, private posture, shared-budget reserve,
and every lane's constant bit-exact water and recorded canonical predicates.
V41 and V51 then consume that same trace, preventing an early lane-0
`NonDescent` from bypassing later-lane guards.

Across the complete validated observed-lane set V51 requires exactly one
adjacent direction-consistent predicate crossing. All pre-crossing and
noncrossing-lane steps retain one exact direction. Only the crossing lane may
reverse afterward, only inside the entered predicate, and only through finite,
positive, nonstagnant, alternating corrections with strictly decreasing exact
absolute step magnitude. The result remains private eligibility for the
unchanged physical coupled solve, root polish, exact receipt stabilization,
same-input replay, and authentic finalization. No tolerance, equation, physics,
budget, floor, support, event, custody, receipt, closure, rollback, persistence,
diagnostic, acceptance, or publication rule changed.

## Ran evidence

- Contract/source expected red: `a360f4cd-ddac-4c82-b693-068142d64692`;
  contract 1/1 passed and production seam 0/1 failed as expected.
- Final V41/V51 behavior after shared validation and exact R132 assertions:
  `650abc75-16bc-4699-84ad-dd10166fa09d`, 10/10 passed.
- Final V51 contract/source seam:
  `3816f371-e3dc-480f-be23-6291cb21fa8b`, 2/2 passed.
- Retained V41/V45/V46/V51 runtime:
  `2946eecb-15e9-47ee-8eef-90df5acdc648`, 28/28 passed.
- Complete snow source-contract target:
  `595ead12-4c1f-4b14-8391-dd05959473bb`, 46/46 passed.
- Orchestrator all-target/all-feature check: passed.
- Workspace formatting and scoped `git diff --check`: passed.
- Production diagnostic scan: no `DFF_V51`, `R132_`, `eprintln!`, or `dbg!`
  seam in the V51 production sources.
- Retained r130 SHA-256:
  `43aee720db2758e47b166f96e726e307152c4fa14c82321564422062b9df728a`.
- Retained r132 SHA-256:
  `db16c87e296f1a4756d9467e38fb1b36d7611df51b8275a483c3c33584600dbf`.

A warnings-denied whole-orchestrator Clippy run was attempted after the
all-target check. It is nonterminal because the shared in-progress crate has
1,108 pre-existing warnings across unrelated owned files. No V51-owned lint
diagnostic was reported; the attempt is not recorded as a pass.

## Behavior evidence

The positive vector asserts exact ordinals `4/6/8/10`, exact water
`0.3272909355676788 kg m^-2`, endpoint enthalpies, predicate chain
`0/1/1/1/1`, signed binary64 steps, seed coordinates, one crossing, private
posture, and unchanged authentic-only admission. Negative vectors isolate
pre-crossing reversal, no crossing, skipped and repeated crossings,
nonalternation, equal/noncontracting/growing magnitude, adjacent stagnation,
nonfinite enthalpy, nonfinite raw owner, later-lane water and stored-predicate
substitution, static join, cadence, opposite-side, raw-owner A/B/A, budget,
and publication poisons.

## Governance

Terminal line counts are `phase_consistent_coupled_solve.rs` 2,359 (`WARN`),
`open_snow.rs` 2,770 (`WARN`), `open_snow_convergence_tests.rs` 2,999
(`WARN`, below 3,000), the V51 exact-move test split 226 (`PASS`), and the
source-contract target 1,778 (`PASS`). No file reaches the 3,000-line limit.
The V51 test body was exact-moved before the retained test file crossed the
limit; the active coupled numerical core remains contiguous. Existing
split-before-3,000 intent remains binding; no exception is requested.

Independent Rust correctness and QA reviews approved the corrected
shared-validator head. The QA reviewer independently reran retained
V41/V45/V46/V51 behavior 28/28 and the full source contract 46/46. The
parent-owned canonical R133 run remains pending.
