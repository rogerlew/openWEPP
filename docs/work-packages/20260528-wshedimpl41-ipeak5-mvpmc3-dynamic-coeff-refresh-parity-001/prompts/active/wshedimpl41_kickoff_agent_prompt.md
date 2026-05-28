Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/artifacts/wshedimpl40_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`

Files:
- `docs/work-packages/20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

Task: execute WSHEDIMPL41 objective end-to-end for declared scope by closing
`ipeak = 5` MVPMC3 dynamic-coefficient refresh parity gaps
(`GAP-ROUTE-011`, `GAP-SYSTEM-010`) using baseline-authoritative
`wshchr.for` lineage with contract-first sequencing.

Constraints:
- Contract-first sequencing is mandatory:
  1. contracts/index, 2. contract-derived tests, 3. pre-implementation gate,
  4. production runtime edits.
- Canonical `SC-*` contracts are implementation authority; package artifacts are
  evidence only.
- Preserve baseline provenance traceability to pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy physics substitutions in production runtime for migration
  closure claims.
- No silent defaults/coercion/clamping for domain violations; use typed
  fail-closed guards.
- Complete dual review and dual verification artifacts before disposition.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and run
required gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`).
