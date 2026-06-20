# Worker Handoff

Status: complete.
Evidence mode: Static.

R4A is closed. The direct runtime now has:

- R3A: lane/day transfer-input accounting;
- R3B: lane/day water-ledger diagnostic chain;
- R3C: run-level multi-lane transfer/topology propagation;
- R4A: first direct hydrology-process span for runoff-partition closure.

Recommended next route: R4B, a narrow follow-on that either expands the direct
runoff-partition process boundary one step upstream into direct liquid/input
assembly or one step downstream into direct storage reconciliation. Do not jump
to publication cutover until the direct path owns enough WB12/WB14/WB18/WB19
state to prove output identity without compatibility compensation.

Carry forward gates:

- canonical `SC-*` authority before process math;
- no default activation;
- no publication/schema cutover unless explicitly authorized;
- direct-runtime forbidden-token source scan;
- scheduler no-diff check;
- explicit opt-in direct counters positive;
- default-disabled direct counters zero;
- default-disabled H2637 median `<= 676.67 s`;
- full Rust gates: fmt, clippy, test workspace, deny.

Carry forward caution:

R4A direct runoff state is not a public output authority. It is a direct runtime
process result and must stay shadow-only until a later publication/cutover
package proves anti-tautological identity and closure.
