# CLI04 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Phase: A only.
Files:
- `docs/contracts/openwepp-runner-contract.md`
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `docs/contracts/README.md`
- `docs/work-packages/20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/artifacts/cli04-contract-implementation-evidence.md`
- `docs/work-packages/20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/artifacts/cli04-output-architecture-authority-and-guard-map.md`
- `docs/work-packages/20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/artifacts/cli04-crate-rename-and-shared-boundary-plan.md`
Task: implement CLI04 Phase A contract/architecture authority amendments for
shared output boundary and WAT schema metadata parity requirements.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) with explicit WAT output-authority
exception to post-`wepp_260430` `wepp-forest` lineage per stakeholder note;
typed guards; no silent defaults; no `arrow2` adoption for new implementation
work in this package; parquet architecture authority must use `arrow-rs`
ecosystem naming (`parquet` + `arrow-array` + `arrow-schema`) and treat
`arrow-schema` as companion crate, not alternative.
Outputs: update listed CLI04 artifacts for this phase only.

Mandatory sequencing constraints:
- Do not modify production runner/output code until:
  1. canonical contract/spec authority amendments are complete,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Do not implement placeholder/fallback output payloads for required contract
  surfaces.
- Preserve explicit metadata parity requirements for `H.wat.parquet`
  (`units`, `description`, and dataset metadata version keys).
