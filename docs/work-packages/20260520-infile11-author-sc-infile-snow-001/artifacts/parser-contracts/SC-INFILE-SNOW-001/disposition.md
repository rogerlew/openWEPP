# Disposition — SC-INFILE-SNOW-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `SNOW-A-001` | `review_agent_a` | high | `amended_closed` | Added explicit unsupported prefix/version policy with typed error and guard; compat path now rejects prefix variants to avoid semantic record-shift masking. | `docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:39`, `:133`, `:161`, `:168`, `:183`; `docs/specifications/wepp-input-files/specs/snow.spec.md:43`, `:119` | Prefix handling now deterministic across modes. |
| `SNOW-A-002` | `review_agent_a` | medium | `amended_closed` | Split strict failure classes for missing vs surplus vs trailing-token cases and updated guard mapping accordingly. | `docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:127`, `:131`, `:132`, `:179` | Eliminates prior `SNOW-E-002` overloading. |
| `SNOW-B-001` | `review_agent_b` | medium | `amended_closed` | Same taxonomy/guard correction for strict surplus/trailing-token behavior made explicit and executable. | `docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:160`, `:179` | Error-class precision restored. |
| `SNOW-A-003` | `review_agent_a` | medium | `amended_closed` | Added explicit per-line trailing-token provenance field and propagation for compatibility warnings. | `docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:81`, `:94`, `:166` | Warning observability now line-granular. |
| `SNOW-B-002` | `review_agent_b` | medium | `amended_closed` | Added explicit guard linkage for cross-file constraints (`G-SNOW-007..009`). | `docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:142`, `:143`, `:144`, `:180`, `:181`, `:182` | Cross-surface invariants now enforceable. |

## Status
- High-severity findings closed in this pass: `SNOW-A-001`.
- Open governance HOLDs (not unresolved reviewer findings): `SNOW-GAP-001`, `SNOW-GAP-002`, `SNOW-GAP-003`, `SNOW-GAP-004`.
