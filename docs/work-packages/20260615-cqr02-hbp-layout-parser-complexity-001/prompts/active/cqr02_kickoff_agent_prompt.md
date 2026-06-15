# CQR02 Kickoff Agent Prompt

Scope: local repository flat-file edits only in `/home/workdir/openWEPP`.

Autonomy: execute the package end-to-end through source refactor, validation,
artifact updates, dual review/verification, and disposition unless a declared
hard blocker is reached.

Objective: decompose
`crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs` so
`parse_layout` no longer concentrates all HBP layout parsing branches in one
high-CRAP function. Preserve all parser behavior, byte-read order, cursor
advancement, checksum windows, error codes, error messages, and public HBP
parser APIs.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to package
artifact files. If subagents are unavailable or tool policy does not allow
delegation from this turn, perform equivalent independent local reviews and
record that path.

Required reading map: `artifacts/required-reading-map.md`.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/standards/AGENTS.md`
- `/home/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/code-quality-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/module-test-enhancement-authoring-guide.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`

On-demand:

- `/home/workdir/openWEPP/tests/integration/infile_hbp_parser_contract.rs`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp/mod.rs`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`

## Execution Instructions

1. Record baseline line counts, function spans, coverage, and CRAP before
   editing production code.
2. Run focused HBP characterization before production edits:
   `cargo test --test infile_hbp_parser_contract`.
3. Extract cohesive internal blocks from `parse_layout` into private helpers in
   the same file. Preserve byte-read order, cursor advancement, checksum
   windows, error codes, and error messages.
4. Run focused checks, then the required closure gates:
   `cargo fmt --check`;
   `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`;
   `cargo deny check`.
5. Re-measure target coverage and CRAP; every eligible target function must be
   CRAP `<= 30`.
6. Update all package artifacts with `Static:` and `Ran:` evidence labels.
7. Complete dual reviews, finding disposition, dual verification, final
   disposition, and worker handoff.

Stop conditions:

- Focused HBP characterization fails before production edits.
- A necessary change would alter parser behavior, error authority, public API,
  or checksum/binary layout semantics.
- Required tooling is unavailable and no package-conforming fallback can record
  evidence truthfully.
