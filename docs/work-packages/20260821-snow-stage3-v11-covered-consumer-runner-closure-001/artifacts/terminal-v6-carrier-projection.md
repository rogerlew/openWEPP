# Terminal V6 carrier projection coverage

The carrier is not serialized wholesale. Compiler-indexed top-level coverage:

| field | disposition | selected purpose |
|---|---|---|
| `transition` | `exclude` | terminal-solver transition outcomes |
| `ending_candidates` | `select` | joint receipt and trial snow-soil receipt digests |
| `precipitation_sets` | `select` | canonical ordered set digest |
| `carrier_envelope` | `select` | transaction and selected provider identities only |
| `complete_lower_boundaries` | `select` | canonical ordered lower-boundary digest |
| `carrier_source_receipts` | `select` | canonical ordered source-receipt digest |
| `covered_lse_states` | `select` | canonical ordered state digest |
| `soil_candidate` | `select` | canonical soil snapshot digest |
| `soil_top_boundary_credit` | `select` | canonical credit digest |
| `wb14_child_receipt_set_sha256` | `select` | parse exact lowercase digest |
| `wb14_parent_receipt_set_sha256` | `exclude` | parent evidence outside rejected child |
| `wb14_child_replay_bytes` | `select` | digest explicit bytes; not native wire |
| `wb14_parent_replay_bytes` | `exclude` | parent evidence outside rejected child |
