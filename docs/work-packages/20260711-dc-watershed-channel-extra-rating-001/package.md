# DC watershed-channel extra rating-row closure

Status: active
Evidence mode: Static and Ran as labeled
Queue item: `FQ-03`
Defect: `CHN-E006-EXTRA-RATING-ROW`

## Objective

Ratify and implement an unambiguous structural recognition rule for a
prohibited rating-curve record following `icntrl != 4`, so only a
contract-recognized extra rating record emits `CHN-E-006`. Preserve valid
single-/multi-channel parses, numeric-leading comment text, rating physics, and
all public typed data.

## Correction Authority Envelope

Owned semantic write set:

- `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
- `tests/integration/infile_watershed_channel_parser_contract.rs`
- `tests/fixtures/infile/watershed_channel/`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
- this package, catalog, and follow-up ExecPlan

Pinned provenance is `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, files `infile.for`,
`wshinp.for`, `inidat.for`, and `verchk.for`. Targeted legacy citations must be
re-anchored to this baseline before production edits.

Allowed edits are contract/spec recognition policy, contract-derived vectors,
typed error classification, minimal parser structure, and consumer
non-regression assertions. Rating-curve physics, accepted grammar, channel
routing, unrelated policy, and lexical “three floats means rating” heuristics
are excluded.

## Required sequence

Amend canonical authority and add intended-red one-channel EOF two-/three-/
four-token, multi-channel boundary, and numeric-comment vectors. Obtain dual
independent pre-implementation PASS before production edits. Then implement the
smallest structurally governed correction, validate real consumer stability,
close A-H and science-tier coverage, and only after the safety net decompose
eligible CRAP above 30.

`HOLD` is exceptional and requires a proved out-of-envelope authority/evidence
boundary in `artifacts/hold-legitimacy-audit.md`; ambiguity must first be
resolved contract-first within this envelope.

## Exit criteria

- Canonical rule uses declared channel-block structure and valid suffix
  recognition, not a three-float lexical heuristic.
- Only contract-recognized prohibited rating rows emit exact `CHN-E-006`;
  generic extra rows retain `CHN-E-002`.
- Valid numeric comments, next-channel records, and all valid typed outputs are
  unchanged through the real frame consumer.
- Science tier reaches at least 90% lines/regions, every named function at
  least 75% regions or reviewed exclusion, A-H fully bound, eligible CRAP at
  most 30.
- Focused parser and WSHED-W5 tests, formatting, workspace clippy, full-profile
  nextest, deny, Markdown, line-count, security, dual review/disposition, and
  dual verification pass with no deferred current gate.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to independent contract/technical reviewers, verification
agents, and heavy coverage/gate runners. Expected outputs are package review,
verification, and gate artifacts; reviewers/verifiers are read-only except for
their named artifact, and runners may write only named evidence and ordinary
build outputs.
