# Implementation

Static: test-only characterization; production source unchanged.

Production source:
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`

- Before SHA-256:
  `f8b2276b8e15de51f46e343fcf0ff7b49a2537fd048853b1e5e51ff74b993585`
- After SHA-256:
  `f8b2276b8e15de51f46e343fcf0ff7b49a2537fd048853b1e5e51ff74b993585`
- Lines: 730 before, 730 after.
- Production refactor: none.

Test file:
`tests/integration/infile_hbp_parser_contract.rs`

- Before SHA-256:
  `d7815ecede4d7aa0f0dee21ecea8a60b435e455dac166a40726c183acb9d145a`
- After SHA-256:
  `4e518ef8e836242ade8ce94edf6dc47b10e1bc0ac803557be8bf02b17b90da6b`
- Lines: 1593 before, 1677 after.

Changes:

- Added `build_non_runoff_event_payload`.
- Added `build_schema1_non_runoff_fixture`.
- Generalized schema-1 fixture assembly with
  `build_schema1_fixture_with_payload`.
- Added `latest_event_state_represents_non_runoff_subevent_payload`.

Reason this closed CRAP without production edits: CRAP penalizes uncovered
complexity. The only baseline row above 30 was already isolated production
logic with no behavior change required; direct characterization coverage was
the safe CQR action.
