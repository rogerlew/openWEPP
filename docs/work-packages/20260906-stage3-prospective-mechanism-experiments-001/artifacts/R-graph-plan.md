# R graph plan and reading map

Static: graph/test source cut; no Rust build, test execution or measurement.
Ran: independent Python graph-oracle hashing and Nix-shell Rust formatting.

Owned exact new paths in isolated R root `/tmp/openwepp-controlled-mechanisms-Hb6uS2/R`:
`crates/openwepp-land-surface-energy/src/solver_component_dependency_graph.rs`
and `crates/openwepp-land-surface-energy/src/solver_component_dependency_graph_tests.rs`.
This artifact is the only retained-tree write. `implement_r` owns module wiring
and all evaluator/custody consumers. No other agent edits are reverted.

`tools/agents/find-agents --for` on both repository-relative source paths and
this artifact resolved root AGENTS + crates/AGENTS for source, root AGENTS +
docs/work-packages/AGENTS for artifact. An initial absolute detached-path query
was rejected as outside the retained repository; equivalent relative paths
resolve the same source-copy instructions.

Required reading (full files unless a range is stated): root AGENTS (12599 B),
crates/AGENTS (5436), work-packages/AGENTS (26781), standards/AGENTS (4052),
science-contracts/AGENTS (5942), codex_exec_plans (20921), prompt-wording-guidance
(10508), testing-and-gate-strategy (22200), kernel-work-package-preparation
(15309), numerical-solver-architecture (6742): 130490 B core/conditional.
On-demand: kickoff (25549 B), current package (5340 B), prior
component-temperature-replay contract_ref (13669 B), and
SC-LANDSURFACEENERGY-001 lines 2860–3240 (31756 B, complete INV164/C020 amendment
and EXP-STAGE3-20260906-R binding). Required-reading total 206804 B: OK.
Source reads are restricted to solver module imports and crate dependency
manifest (sha2 already available). No authority amendments in this subtask.

Plan: author independent graph tests first; implement the exact normative node
and direct-edge expansion; derive stable inclusive probe closures once per
validated topology, project route/current-leaf/maximum-leaf selector masks,
then discard construction scratch. Unknown coordinate/node/read must never
prove independence. R reexecutes the remaining canonical tail even when
independent, which does not require extra graph edges.

Schema is `covered-component-temperature-dependency-v1`. Nodes use authenticated
zero-based occupancy/soil ranks and canonical component words. Residual families
expand to `[r]` in unchanged 10*N+3+S row order. Incident/stemflow custody nodes
exist at ranks 0 through N, including immutable top input and terminal custody.
Conservative explicit additions: `route.incident[N] -> result.ground_release`
and `route.stemflow[N] -> result.ground_stemflow`; terminal finalization also
retains both required direct result edges. These two source-real custody edges
are included in the schema hash and independent oracle. No route-to-leaf edges
are added: leaf_trial_state has no routing-preparation operand.

Hash serialization: SHA-256 over individually u64-big-endian-length-prefixed
UTF-8 fields: schema, decimal N, decimal S, decimal node count, lexically sorted
unique node IDs, decimal edge count, then each lexically sorted unique edge's
source and target as separate fields. Tests use a separate predicate over all
node pairs rather than the production edge builder, independently serialize
the records, pin N1/S1 and N2/S6 hashes, and delete/change each direct edge to
prove comparison rejection. Parent comparator owns all executable gates.

Source tests were authored before the graph module; Rust execution remains
NOT RUN by this worker (parent comparator responsibility). Independent
Python hashlib/struct expansion froze N1/S1 = 85 nodes, 254 edges,
`5119d5d7274807d5d871987d96a8ff1a75a8b9a2db790cf3fb83d1caa9ccf15f`;
N2/S6 = 163 nodes, 494 edges,
`cfd9d3c7f82c5871f8c6c953843f1d600f4929a7d9ba47c27f2494458e5a7b3b`.
Tests pin both golden hashes and also cover N3/S2 exact edges, N4/S3 selector
closure, every single-edge deletion/change, unknown endpoint conservatism,
noncomponent coordinates and out-of-range occupancies.

`nix develop -c rustfmt --edition 2024 <both exact source paths>` passed.
An initial plain `rustfmt` exited 127 because it is absent from ambient PATH;
the repository toolchain shell resolved it without changing dependencies.
Source line counts are 384 and 316, below WARN. Runtime API is `new(N,S)`,
`coordinate_eligible(coordinate)`, `route_reachable(coordinate,occupancy)`,
`leaf_current_reachable(coordinate,occupancy,sun:bool)`, and analogous maximum.
`schema_hash` joins the descriptor; immutable same-object borrowing preserves
exact typed graph identity. An unresolved edge endpoint disables all component
eligibility; unknown coordinates/target ranks conservatively return reachable.
Future newly added evaluator reads require authority/graph review; this static
descriptor does not claim automatic source-code dependency discovery.
