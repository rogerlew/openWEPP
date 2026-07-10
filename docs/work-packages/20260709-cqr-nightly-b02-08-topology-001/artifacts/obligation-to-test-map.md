# Obligation-to-Test Map

No `SC-*` process-physics obligation is altered. The package is behavior-
preserving topology glue CQR.

| Obligation / behavior | Test binding |
|---|---|
| Canonical valid topology fixture parses and validates as nominal. | `canonical_topology_fixture_passes_pre_execution_validation` |
| Missing fixture path remains a typed `ReadError`, display prefix is stable, and parser error source behavior is stable. | `missing_fixture_file_returns_typed_read_error` |
| Topology validation status errors remain wrapped with stable display/source behavior. | `topology_validation_error_wraps_status_source` |
| Required headers, malformed headers, unknown headers, invalid header values, and duplicate headers fail with typed parser errors and exact display strings. | `parser_reports_required_header_and_header_format_errors`; `parser_reports_header_value_and_duplicate_header_errors` |
| Node row length, node markers, unknown node kind, forbidden hillslope node rows, invalid numeric fields, and duplicate nodes fail with typed parser errors and exact display strings. | `parser_reports_node_record_and_kind_errors`; `parser_reports_node_value_and_duplicate_node_errors` |
| Existing validation message IDs for disconnected nodes, declared channel mismatch, out-of-domain channel reference, and directed cycle remain stable. | Existing validation tests in `topology_graph_validation_gate.rs` |
| Hillslope count, impoundment count, hillslope reference domain, zero channel upper-bound, missing channel existence, and missing impoundment existence violations remain stable. | `validation_reports_hillslope_and_impoundment_count_failures`; `validation_reports_reference_domain_and_existence_failures` |

Coverage closure binds the public parser/display/validation behavior used for
the private decomposition. No fixture grammar or validation policy is changed.
