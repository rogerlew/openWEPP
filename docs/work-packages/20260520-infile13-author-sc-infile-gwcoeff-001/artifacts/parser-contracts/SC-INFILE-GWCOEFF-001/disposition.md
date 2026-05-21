# Disposition — SC-INFILE-GWCOEFF-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `GWC-A-001` | `review_agent_a` | high | `amended_closed` | Aligned strict-mode canonical shape with spec authority by accepting numeric-leading lines with optional trailing tokens; removed strict trailing-token rejection conflict. | `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:45`, `:50`, `:157`; `docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:121` | Contract/spec authority is now consistent for canonical trailing-text fixtures. |
| `GWC-A-002` | `review_agent_a` | medium | `amended_closed` | Separated tokenization policy from record-count errors; `GW-E-002` remains record-count only and tokenization enforcement uses `G-GW-006 -> GW-E-001`. | `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:128`, `:170`, `:174` | Typed outcomes are now executable and taxonomy-precise. |
| `GWC-B-001` | `review_agent_b` | high | `amended_closed` | Made `lr_bf` branch derivation parse-success based and added explicit `parse_outcome`; malformed present-file branch now terminates with typed error and no normalized enable-state emission. | `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:79`, `:80`, `:94`, `:113`, `:120` | Resolves present-but-malformed ambiguity for branch state. |
| `GWC-B-002` | `review_agent_b` | medium | `amended_closed` | Closed same taxonomy conflation by codifying canonical tokenization guard behavior and keeping arity closure in its own error class. | `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:128`, `:170`, `:174` | No open taxonomy ambiguity remains. |

## Status
- Closed findings: `GWC-A-001`, `GWC-A-002`, `GWC-B-001`, `GWC-B-002`.
- Open high-severity findings: none.
