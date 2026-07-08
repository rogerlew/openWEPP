# Kickoff Prompt

Scope: local repository documentation work; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/package.md`

On-demand:

- `crates/openwepp-input-contract/src/parsers/management.rs`
- `tests/integration/infile_management_parser_contract.rs`
- `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man`
- `tests/fixtures/disturbed_native_route_coefficients/p1.man`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

Required-reading budget: `421001` bytes, `WARN`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/`

Task: execute the package objective end-to-end for the declared docs-only scope.

Constraints: no parser/runtime/test/fixture/contract edits; no silent authority
changes; no new coefficient defaults; no legacy-field-to-routing-coefficient
bridge; keep all authority claims aligned to `SC-INFILE-MANAGEMENT-001` and the
management-lanuse authority contract.

Subagent requirement: none.

Subagent authorization: none.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases.
