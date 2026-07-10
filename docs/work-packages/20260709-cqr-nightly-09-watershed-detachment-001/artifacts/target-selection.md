# Target Selection

Evidence label: Static.

Status: `SCAFFOLDED`

Selected target:

- Rank: `9` of `10`
- Module:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`
- Quality dimension: `CRAP/cyclomatic-complexity`
- Production line count: `867`
- Baseline target LCOV: `193/749` lines (`25.767690253672%`)
- Baseline function coverage: `7/52`
- Baseline CRAP rows above `30`: `4`
- Baseline max CRAP: `272.0`
- Baseline total excess over `30`: `386`

Nightly measurement inputs:

- Coverage source: `/tmp/openwepp-cqr-nightly.lcov`
- CRAP source: `/tmp/openwepp-cqr-nightly-crap.json`
- Target selection follows
  `docs/work-packages/cqr-nightly-burndown-execplan.md`.

Rationale:

The module is the ninth eligible nightly CQR module by live CRAP burden. The
target is below the line-count WARN threshold and has four zero-covered
high-CRAP helpers whose branch structure can be characterized before
behavior-preserving decomposition.

Instruction discovery:

`tools/agents/find-agents --for
crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs
crates/openwepp-watershed-orchestrator/src/lib.rs
docs/work-packages/README.md tests/integration` reported:

- target module: `AGENTS.md`, `crates/AGENTS.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`: `AGENTS.md`,
  `crates/AGENTS.md`
- `docs/work-packages/README.md`: `AGENTS.md`,
  `docs/work-packages/AGENTS.md`
- `tests/integration`: `AGENTS.md`, `tests/AGENTS.md`
