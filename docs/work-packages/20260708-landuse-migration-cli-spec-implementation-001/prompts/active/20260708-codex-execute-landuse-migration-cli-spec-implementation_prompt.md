Scope: local repository Rust/docs implementation task; flat-file reads/edits
only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md`
sequentially through disposition.

Required reading:

Core:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/artifacts/required-reading-map.md`

Conditional:
- `docs/specifications/science-contracts/AGENTS.md` if any contract changes.
- `tests/AGENTS.md` before integration tests.
- `crates/openwepp-input-contract/src/parsers/management.rs` before flat-source parser integration.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` before runtime consumer proof.
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py` before table embedding.

Files:
- `docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md`
- `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`
- `docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001/**`
- implementation files declared by `package.md`.

Task: implement the Rust landuse migration library/CLI and finalize the CLI
specification. The first target is frozen legacy cropland flat `.man` to
coefficient-complete `ow-lanuse-1` canonical management YAML, requiring
disturbed-class authority. Also provide flat `ow-lanuse-1` to YAML and native
YAML `ow-lanuse-1` to `latest` migrator scaffolding.

Constraints:
- contract/spec-first sequencing;
- no sidecars;
- no compatibility-only output mode for pre-native datvers;
- no coefficient projection from legacy fields;
- no native flat `.man` writer;
- producer outputs must end in lowercase `.yaml`;
- omitted `--output` defaults to appending `.yaml` to the input filename
  (`field.man` -> `field.man.yaml`);
- typed errors and fail-closed behavior;
- `--validate` support;
- crates.io distribution readiness;
- real runtime consumer proof for emitted YAML before closure.

Subagent requirement: REQUIRED for implementation review/verification when
available. This prompt explicitly authorizes subagent spawning/delegation to
read-only review and verification subagents for spec, implementation, runtime
consumer, and crates.io-readiness review. Outputs: `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access: read-only unless a later prompt
assigns a bounded implementation write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
