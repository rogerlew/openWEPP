# Implementation evidence

Status: PASS
Evidence mode: Static and Ran

Static: `parse_f64` now rejects any parsed non-finite value with
`FieldRangeError { field, value, expected: "finite" }`, which maps to
`FDIR-E-005`. Strict `datver` and every sprinkler/furrow real field use this
boundary. The compatibility single-token datver probe retains its historical
`datver_or_header` syntax error for nonnumeric input but now rejects parsed
non-finite values with the same `datver` finite-domain error.

Ran: `cargo nextest run --test infile_irrigation_fixeddate_parser_contract`
passed 27/27 after correction.

Static consumer audit: repository search shows no production consumer of
`FixedDateIrrigationFile`; this is parser-boundary-only closure and makes no
runtime-readiness claim.

After characterization closure, `parse_fixeddate_str` was decomposed by moving
the complete preamble policy into `parse_preamble` and the complete event loop
into `parse_events`. Statement order, comparisons, float parsing, warning order,
event order, and allocation behavior are preserved. CC/CRAP fell from
`39/39.713` to `17/17`; both helpers are independently below 30.

Final-review source hash:
`70aa60e562f7e5d972ec53330e856122ac38ee4b8c9a0a4623834599a04a4b45`.
