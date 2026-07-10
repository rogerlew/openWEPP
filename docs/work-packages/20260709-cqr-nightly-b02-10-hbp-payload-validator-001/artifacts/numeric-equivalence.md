# Numeric And API Equivalence

Static/Ran: production behavior preserved; new test records public API behavior.

Behavior-preserving HBP payload-validator CQR must preserve byte-read order,
cursor consumption, numeric scaling, typed errors, guard semantics, public API
shape, and schema meaning.

Production source was unchanged; therefore byte-read order, cursor consumption,
typed errors, guard semantics, schema meaning, and public parser APIs are
identical to the scaffold baseline.

The added characterization fixture asserts:

- encoded scaled `i64` values `1_000_000_000` through `6_000_000_000`
  project to public values `1.0` through `6.0`;
- non-runoff subevents surface through `HbpLatestEventState::NoEvent`;
- compatibility `parse_hbp_from_bytes_with_latest_event_payload` returns
  `None` for non-runoff subevents rather than fabricating runoff payloads.

No science formulas, runtime publication paths, binary schema fields, or
thresholds were changed.
