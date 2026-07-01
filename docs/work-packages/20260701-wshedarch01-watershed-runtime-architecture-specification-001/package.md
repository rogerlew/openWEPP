# WSHEDARCH01 Watershed Runtime Architecture Specification

Status: `EXECUTED-COMPLETE-DRAFT-SPEC-REV4-CLAUDE-STATIC-VERIFICATION-DISPOSITIONED`

Date opened: `2026-07-01`

## Objective

Create the watershed-runtime counterpart to
`docs/architecture/array-native-runtime-specification.md`, grounded in
WSHEDPERF01 evidence and current watershed CLI/orchestrator seams.
Revision 1 incorporates the user-directed hillslope performance lesson: specify
watershed work as a ground-up runtime rewrite with full deletion of the existing
runtime and old-surface tests, then backfill protected tests against the new
runtime.
Revision 2 dispositions dual-review findings by tightening pass-payload
validation, current WSHEDPERF evidence, legacy-comparison wording, `--jobs`
authority, consumer-path proof, deletion-test coverage, and implementation gate
requirements.
Revision 3 records the user-directed fixture strategy: arboreal-dendrite is too
small to be the sole development target, carnivorous-adobo is the preferred
near-term 32-hillslope development fixture, larger 1,000+ hillslope fixtures are
needed after runtime progress, and adopted fixtures must be committed to the
repo for auditability.
Revision 4 dispositions Claude static verification by tightening benchmark
taxonomy around sidecar discovery, adding the ROADMAP watershed-runtime queue,
annotating `chan_out`, and making the latest-event `NoEvent` question a
contract-first follow-up.

## Rationale

WSHEDPERF01 showed that arboreal-dendrite routed-stage watershed execution is
already very small, while the practical openWEPP full command chain is dominated
by serial hillslope subprocess execution and handoff pathing. The architecture
direction should therefore define CPU-scalable watershed supervision before
optimizing the routed channel stage.

## Included Scope

- Review WSHEDPERF01 timing/profiling artifacts.
- Inspect current watershed CLI and orchestrator state/writeback seams.
- Author a draft architecture spec at
  `docs/architecture/watershed-runtime-architecture-specification.md`.
- Update `docs/architecture/README.md` with the new architecture pointer.
- Update `docs/ROADMAP.md` with the forward watershed runtime queue required
  before ratification.
- Record package-local review, gates, and disposition.

## Excluded Scope

- No production Rust edits.
- No benchmark reruns.
- No ADR ratification.
- No final performance claim beyond recorded WSHEDPERF01 evidence.

## Intended Write Set

- `docs/architecture/watershed-runtime-architecture-specification.md`.
- `docs/architecture/README.md`.
- `docs/ROADMAP.md`.
- `docs/work-packages/20260701-wshedarch01-watershed-runtime-architecture-specification-001/**`.
- `docs/work-packages/README.md` execution-log pointer.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer` and `rust_qa_reviewer` subagents for
read-only dual review of
`docs/architecture/watershed-runtime-architecture-specification.md`; expected
outputs are compact review findings with file/line references and severity; write
access is read-only for subagents, with parent disposition recorded in
`artifacts/dual-review-disposition.md`.

## Deliverables

- Draft watershed runtime architecture specification.
- Package review artifact.
- Dual review disposition artifact.
- Gate/disposition artifacts.

## Exit Criteria

- The spec names WSHEDPERF01 measured constraints.
- The spec states the selected first architecture lever.
- The spec requires ground-up replacement and full deletion of the existing
  runtime after validation.
- The spec defines the fixture ladder and requires committed adopted fixtures.
- The spec defines the sidecar/input-discovery benchmark axis.
- The roadmap carries the WSHED-ADR/W2-W6 forward queue.
- The spec separates process-level CPU scaling from later typed watershed
  network-frame cleanup.
- The spec does not claim ratification.
- Docs-only validation is recorded.

## Security and Safety

Docs-only work. No network, branch changes, production services, or secrets are
in scope.
