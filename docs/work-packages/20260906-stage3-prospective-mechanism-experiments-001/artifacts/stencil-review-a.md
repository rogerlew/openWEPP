Static: independent source/domain/lifecycle review A of the offline stencil
oracle and actual A trace. Ran: read-only JSON inspection and in-memory
mutation checks on the first complete map's 1,345-event trace. No Rust build,
simulation, timing or memory workload run by this reviewer.

## Findings

| ID | Severity | Path/line | Finding |
| --- | --- | --- | --- |
| SA1 | HIGH | `artifacts/reproduction/audit_stencils.py:60` | Start/end membership is not lifecycle validation. The oracle accepts a Solve End before its first iteration, arbitrary repeated iteration ordinal 800, and removal of the MapJoin that authenticates the final solve's attachment to the ended potential map. Validate active typed nesting/order, independent IDs, per-solve iteration ordinals/current iteration, and source-real map join/final-solve attachment. Retain separate per-map/per-solve and potential/fixed-final accounting, not only global totals. |
| SA2 | HIGH | `artifacts/reproduction/audit_stencils.py:104` | Failed-prefix comparison does not bind probe outcomes to stopping or stencils. It accepts a failed probe followed by all later probes and a successful sweep; it also accepts a failed sweep with the full probe stream and no Stencil events. Require each probe to follow its actual stencil, enforce canonical minus/plus ordering and first-error termination, and reconcile sweep success/failure with attempted/completed/error children. Never invent a ShortCircuited state. |

Each listed invalid trace independently returned PASS after recomputing only
the counters affected by that mutation. The unmodified first-map trace also
returned PASS. These are oracle sensitivity failures, not evidence that the
actual A producer emitted those invalid sequences. They block use of this
oracle as complete INV-164/C-020/EXP-R lifecycle and first-error evidence; they
do not block already-reviewed prospective treatment authoring.

## Source and evidence assessment

The independent binary64 domain enumerator correctly reproduces hydraulic
scale 1000, beta bounds [0,1], liquid-vapor component temperatures [273.15,350],
dry stem/shared-air/soil bounds [200,350], humidity [0,0.1] with scale 0.001,
and the conditional liquid-ground bound. It constructs canonical h and signed
values itself rather than copying stencil validity or producer class counts.
The base shape is independently checked against 10*N+3+S. Completed-sweep
ordered coordinate/value/class comparison is useful independent evidence.
Finite topologies, bounded-domain cases, signed zero and invalid bases remain
appropriate small checker tests. Graph eligibility is explicitly conditional
on the separate complete direct-edge oracle, not claimed proved by coordinates.

Actual A trace: N=2/S=6, represented snow, non-liquid ground on all 2,000 sweeps;
400 maps, 800 solves, 2,400 iterations, 106,800 probes, all completed. Its
78,800 complete plus 28,000 identity-anchor starts reconcile exactly. The
retained offline result has 200 sweeps of 48/14/0/34 and 1,800 of 54/14/0/40.
This is lawful stencil-aware evidence, not a missing fully centered population.
All 608,624 raw events are retained, overflow=false and dropped_events=0.
The review independently inspected the complete JSON summary/topology/parent
relationships; it did not rerun the original whole-trace offline checker.

The current checker rejects nonzero dropped_events, overflow, unknown event
classes, missing end IDs, and aggregate counter mismatch. Those useful guards
do not discharge SA1/SA2. Identity-anchor metadata is currently ignored; its
presence/coordinate classification should be checked, while exact anchor
operand physics remains the forced-complete oracle's responsibility.

Reviewed SHA-256 identities:

- `audit_stencils.py`: `14e24d640ccc83259562103bb43d157cb1d24c7a563cfae8fb50e3cbc9edfc4b`.
- `raw/A-trace-01/lse-0.json`: `5ff2ccfec4bb7c555c52551a9d9d792c286e2c0e1f969067633194ba85288d92`.
- `raw/A-stencil-audit.log`: `a121dd11d0850e9ea527df3172f9068bc476f40f80dcb3ec564467f60d319876`.

## Disposition

HOLD complete stencil/lifecycle oracle admission for bounded SA1/SA2 correction.
Require positive source-real trace cases and negative mutations for nesting,
map joins, ordinals, duplicate/cross-sweep events, first-error stopping, missing
stencils and dropped records. Then rerun the corrected oracle against immutable
A and, when available, R traces. R must independently show nonzero completed
replay and all other current-arm graph/field/guard/closure/validation gates.

## Corrective audit cut and reusable mutation evidence

Ran: `test_stencil_mutations.py` with the authentic A trace, both the first-map
prefix and `--full-trace`. The full trace passed with unchanged 400/800/2400/
2000 map/solve/iteration/sweep counts and the original lawful histogram.
The corrected oracle hash is
`a865b85f62b37a4457f66948a855eccf1bd23f6aca776f5d4018da752092b08e`.
The artifact-only reusable mutation script hash is
`c3f600afd6cdff6dcb2fdea3bb00d78571af3997add8c527ad011052edacd932`.

All original five mutations now reject. Additional missing-complete-Evaluation,
changed immutable anchor within one solve, duplicate MapJoin, child/parent
outcome mismatch and dropped-record mutations also reject. The stronger failed
probe mutation changes both its Evaluation and Probe outcomes, ensuring the
failure comes from first-error continuation rather than only result mismatch.
Complete and replay probes each truthfully enter one shared Evaluation wrapper;
identity anchors enter none. Replay leaf savings are separate evidence.

SA1's active nesting, map attachment, current-iteration and ordinal issues are
resolved on this cut. SA2 remains HOLD for one exact residual: after the first
Probe/Evaluation error, close its sweep as failed and remove all remaining
events inside that sweep, but keep later iterations/sweeps and successful
Solve/Map completion. `failed_sweep_continues_solve` incorrectly returns PASS.
The canonical probe error exits the solve via `?`; local sweep-prefix validity
must not permit subsequent work or a successful parent solve. The reusable
script exits 1 with this sole ACCEPTED_INVALID case (10 other negatives reject).
Bounded correction: bind failed child propagation and termination through the
actual enclosing solve/map path, then rerun the same script and immutable A.

## Final bounded correction: GO for this oracle cut

Ran again with `--full-trace`: all 11 malformed copies REJECTED, unmodified
first-map and full A trace PASS. Corrected oracle SHA-256:
`dcd09ca26e272af41a25bfb5bf90f237c38bfc51f4bd2e3607ee164fc461a973`.
Mutation script remains `c3f600afd6cdff6dcb2fdea3bb00d78571af3997add8c527ad011052edacd932`;
trace remains `5ff2ccfec4bb7c555c52551a9d9d792c286e2c0e1f969067633194ba85288d92`.

Static: failed Probe/Sweep/Solve now requires the immediate failed End of its
active parent, cascading without subsequent work. A caught line-search
Evaluation error is not incorrectly made an unconditional solve abort. A
joined fixed-final solve has no active potential Map scope, so its failure does
not rewrite the prior successful potential map. SA2 is resolved on this cut.

Decision: GO for the corrected source/domain/lifecycle checker on the reviewed
A workload and sensitivity corpus. No remaining SA1/SA2 blocker. Distinct raw
map/solve/iteration/sweep identities remain retained; aggregate histograms are
not a substitute for those records. This does not claim actual R execution,
source-real error-vector runtime parity, physical anchor/graph correctness or
measurement admission; those independent current-arm gates remain required.
