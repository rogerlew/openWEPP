# Characterization

Ran: characterization added and focused oracle passed.

Scope:

- Added `latest_event_state_represents_non_runoff_subevent_payload`.
- Added a schema-1 non-runoff subevent fixture builder.
- The fixture uses event kind `1` and payload-minor `0`, then encodes six
  scaled `i64` values plus the required state snapshot entries.
- The test proves the public latest-event-state API parses the non-runoff
  subevent as `HbpLatestEventState::NoEvent` with source kind `Subevent`.
- The test also proves the compatibility latest-event-payload API does not
  synthesize a runoff `EventPayload` from a non-runoff subevent.

Production refactor: none. The target production source was unchanged, so a
detached test-first patch before production edits was not applicable.

Focused command:

`cargo nextest run --test infile_hbp_parser_contract --profile quick`

Result:

- exit `0`
- 26 tests run, 26 passed, 0 skipped

Focused clippy:

`cargo clippy -p openwepp --test infile_hbp_parser_contract -- -D warnings`

Result:

- exit `0`
