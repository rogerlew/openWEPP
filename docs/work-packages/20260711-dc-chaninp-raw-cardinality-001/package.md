# DC CHAN.INP raw cardinality closure

Status: active
Evidence mode: Static and Ran as labeled
Queue item: `FQ-02`
Defect: `CHANINP-RAW-NCHNUM-CARDINALITY`

## Objective

Close the CHAN.INP compatibility defect end-to-end: preserve raw `nchnum`
source ownership and validate record-4 cardinality against that raw value before
topology normalization, while keeping the downstream network frame on
`nchnum_norm`. Complete science-tier A-H coverage and eligible CRAP closure in
the same package.

## Correction Authority Envelope

Owned semantic write set:

- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `docs/specifications/wepp-input-files/specs/chaninp.spec.md`
- `crates/openwepp-input-contract/src/parsers/chaninp.rs`
- `tests/integration/infile_chaninp_parser_contract.rs`
- `tests/fixtures/infile/chaninp/`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
- this package, work-package catalog, and follow-up ExecPlan

Pinned provenance is `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, files `wshinp.for`,
`cchrt.inc`, `pmxchr.inc`, and `chnrt.for`.

Allowed edits: raw/normalized contract clarification, A-H obligations,
contract-derived tests/fixtures, minimal parser order/field correction,
consumer assertions, and behavior-preserving post-coverage decomposition.
Channel-routing physics and unrelated writer/output policy are excluded.

## Required correction and sequence

The original compatibility fixture with raw `nchnum=99` and two IDs is invalid
and must fail exact `CHN-E-002`; normalization must not create validity. A
distinct valid compatibility fixture must contain 99 IDs, retain
`nchnum_input=99`, and expose the contract-ratified `nchnum_norm`. The network
frame must consume normalized topology while raw observability retains source
input.

Execute contracts, failing contract tests, independent pre-implementation PASS,
production correction, consumer validation, characterization, and only then
any decomposition. `HOLD` is exceptional and requires a proved out-of-envelope
authority/evidence boundary in `artifacts/hold-legitimacy-audit.md`; effort,
source reading, implementation, coverage, or validation remaining in-envelope
does not justify stopping.

## Exit criteria

- Raw cardinality and raw-vs-normalized ownership are canonical and tested.
- Invalid `99+2` fails `CHN-E-002`; valid raw-count closure retains 99 and
  normalizes only after validation.
- Network-frame consumer uses normalized fields; raw source value remains
  observable and no unrelated runtime-readiness claim is made.
- Science tier reaches at least 90% lines/regions, every function at least 75%
  regions or reviewed exclusion, A-H fully bound, eligible CRAP at most 30.
- Focused parser and WSHED-W5 tests, formatting, workspace clippy, full-profile
  nextest, deny, line-count, security, dual review/disposition, and dual
  verification all pass with no deferred current gate.

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent contract/technical reviewers, verification agents, and heavy
coverage/gate runners. Review/verification is read-only; heavy runners may
write only named package evidence and ordinary build outputs.
