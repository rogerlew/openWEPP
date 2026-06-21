# R6G - Direct WAT Producer Authority

Status: scaffolded.

Package type: Defect-Closure ExecPlan / R6 direct publication producer authority.

Defect ID: `R6G-DIRECT-WAT-PRODUCER-AUTHORITY`.

## Defect-Closure Rule

If this package identifies a reproducible WAT publication root cause inside the
declared write set and the expected behavior is supported by canonical `SC-*`
authority, pinned-baseline provenance, or contract-authorized physical
invariants, it must land the contract-first correction in this package. It may
not stop at another diagnostic handoff, reduced mismatch list, or missing
producer label while the correction remains in-envelope.

## Purpose

Close the R6F hold marker:

`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`

R6F proved HBP byte identity for the inherited near-zero runoff fixture and
reduced the next cutover blocker to WAT row fields that require typed direct
process producers: `wepp_id`, output simulation `year`, `Es`, `Total-Soil`,
`SoilWaterTotal`, `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and
`ProfileWPStore`.

R6G must implement the production runner binding that feeds
`DirectPublicationDayInput` from parsed typed inputs and direct runtime state,
then continue the R6 publication loop through WAT parity. Compatibility WB13
rows, compatibility runtime surfaces, writeback payloads, and output rows are
not valid direct authority.

## Required Outcome

Terminal states:

- `COMPLETE-R6G-DIRECT-WAT-PRODUCER-AUTHORITY`: WAT row/schema/metadata parity
  passes from typed direct projection only, current-fixture HBP identity remains
  green, no forbidden compatibility authority is used, and R6F can resume with
  nonzero HBP fixture, PASS/loss/manifest/public-write gates.
- `HOLD-R6G-<SPECIFIC-BOUNDARY>`: only allowed after field-level reduction,
  attempted or ruled-out in-scope corrections, dual review, and a new exact
  follow-on package.

## In Scope

- Add typed direct producer binding for production
  `DirectPublicationFrameCutover` day inputs:
  - direct publication identity (`wepp_id`, simulation year, OFE/day keys);
  - direct percolation layer state and initial storage;
  - direct subsurface compute inputs;
  - direct evapotranspiration compute inputs, including PMET/Priestley-Taylor
    branch operands under `SC-EVAP-001`;
  - direct hydrology projection profile inputs;
  - lane-to-lane carried layer state already exposed by R6F.
- Use parsed source inputs, canonical direct runtime state, and amended or
  existing `SC-*` authority only.
- Keep current-fixture HBP identity green while closing WAT, and do not claim
  full HBP closure until nonzero peak-runoff/event-duration fixture coverage is
  added in R6 continuation work.
- Add independent reconstruction and anti-alias tests proving WAT does not read
  WB13 rows or compatibility runtime surfaces.

## Intended Write Set

- `crates/openwepp-runner/src/hillslope/*`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/*`
- Focused runner and orchestrator tests under the touched crates.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` and
  `SC-SYSTEM-001.md` only if required to clarify existing WAT operand authority.
- Package-local artifacts under
  `docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/`.

## Out of Scope

- Default activation of direct publication.
- Treating compatibility WB13 rows, compatibility runtime surfaces, or writer
  rows as direct authority.
- PASS/loss/manifest cutover except as follow-on blocker reduction after WAT
  is closed.
- Provisional process-physics formulas without contract-first authority.

## Security Impact Gate

This package must preserve fail-closed publication cutover behavior. It must not
add silent fallback wrappers around missing typed inputs, compatibility WB13
rows, runtime surfaces, or writer rows. Any new direct-publication input
producer must fail closed with a typed error or stable hold marker when required
authority is unavailable.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/architecture/array-native-runtime-specification.md` section 5.2.1
- R6F disposition, blocker ledger, operand lineage, no-compatibility proof, and
  worker handoff.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- Any `SC-*` contract governing changed process inputs, units, or publication
  semantics.

## Required Artifacts

- `artifacts/r6g-blocker-ledger.md`
- `artifacts/r6g-operand-lineage.md`
- `artifacts/r6g-independent-reconstruction.md`
- `artifacts/r6g-anti-alias-fixtures.md`
- `artifacts/r6g-no-compatibility-proof.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`

## Validation Gates

- `cargo fmt --check`
- `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner`
- `cargo clippy --workspace --all-targets -- -D warnings`
- Focused direct runtime typed-input/carry tests.
- Focused runner HBP identity and WAT parity tests.
- CLI `DirectPublicationFrameCutover` fail-closed or cutover-write test,
  depending on whether later gates remain.
- Static no-compatibility scan for direct WAT producer and consumer paths.
- Independent WAT operand reconstruction.
- `cargo test --workspace`
- `cargo deny check`
- Dual review and disposition.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to Rust code-review and verification subagents for R6G
direct WAT producer authority review and gate verification; expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is read-only for review and
verification agents.

## Exit Criteria

- Current-fixture HBP identity remains green, with fixture limits stated.
- WAT row/schema/metadata parity passes from typed direct projection only, or a
  stable `HOLD-R6G-*` marker identifies a boundary outside the write set after
  in-envelope corrections are attempted or ruled out.
- No compatibility WB13 row, runtime surface, writeback payload, or writer row
  is used as direct WAT authority.
- Review findings are dispositioned as accepted, rejected, deferred, or
  follow-up, and verification artifacts independently check gate legitimacy.

## Progress

- [x] Scaffolded from R6F hold evidence.
- [ ] Implement parsed-input typed producer binding.
- [ ] Prove WAT parity without compatibility authority.
- [ ] Update R6 blocker chain and handoff.
