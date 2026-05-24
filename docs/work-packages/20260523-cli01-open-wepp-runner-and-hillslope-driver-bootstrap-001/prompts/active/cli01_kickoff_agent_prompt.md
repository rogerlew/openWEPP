# CLI01 Kickoff Agent Prompt

Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.
Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260523-cli01-open-wepp-runner-and-hillslope-driver-bootstrap-001/package.md


You are executing
`20260523-cli01-open-wepp-runner-and-hillslope-driver-bootstrap-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Scope note:
- This task is local repository contract/spec + Rust runner/CLI implementation
  work.
- Operate on flat files in this repository/worktree only.

Objectives:
1. Implement in-repo `open_wepp_runner` launch boundary for openWEPP binaries.
2. Implement `openwepp-cli-hill` executable path that emits
   `H5.wat.dat` + `H5.plot.dat` from openWEPP runtime execution.
3. Implement blind run-directory sidecar discovery with typed strict/compat
   `openwepp-legacy-bridge` behavior.
4. Emit deterministic run-provenance manifests and schema-valid release
   metadata sidecars.

Mandatory sequencing constraints:
- Do not modify production runner/CLI code until:
  1. canonical contract/spec amendments are implemented (when needed), and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- CLI01 sidecar behavior must remain blind run-directory discovery for this
  contract revision; do not switch to `.run` sidecar declarations.
- Launcher boundary must use explicit argument arrays only; no shell
  interpolation and no silent fallback to legacy binaries.
- Missing required sidecars and missing required outputs must remain typed
  hard failures.
- Contract tests must be authored from canonical contract/spec authority, not
  from current implementation behavior.
- Complete dual code review and dual verification artifacts before final
  disposition.

Required outputs are listed in `package.md` Deliverables.
