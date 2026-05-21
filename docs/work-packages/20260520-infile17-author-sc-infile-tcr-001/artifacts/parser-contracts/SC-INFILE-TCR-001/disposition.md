# Disposition — SC-INFILE-TCR-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `TCR-A-001` | `review_agent_a` | high | `amended_closed` | Added explicit cross-file dependency surfaces (`nchan`, `channel_element_ids`, `chnslp_terminal(i)`) in both field and propagation tables, and exported them via boundary mapping for executable guard closure. | `docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:82`, `:83`, `:84`, `:102`, `:103`, `:104`, `:162`, `:172`, `:197`, `:201` | Closes missing dependency-model completeness for `G-TCR-005`/`G-TCR-009`. |
| `TCR-A-002` | `review_agent_a` | medium | `amended_closed` | Split relational invariant handling into strict typed failure (`TCR-E-009`) and compatibility warning+preserve branch (`TCR-W-003`), with explicit warning surface/derivation and guard linkage. | `docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:92`, `:112`, `:129`, `:136`, `:147`, `:154`, `:171`, `:185`, `:195` | Contract now aligns with paired spec strict-vs-compat behavior. |
| `TCR-B-001` | `review_agent_b` | high | `amended_closed` | Reconciled contract/spec policy mismatch for `taumin>taumax` by preserving compat flow with explicit warning while keeping strict-mode failure typed and deterministic. | `docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:147`, `:154`, `:185`, `:195` | High-severity strict/compat divergence resolved. |

## Status
- Closed findings: `TCR-A-001`, `TCR-A-002`, `TCR-B-001`.
- Open high-severity findings: none.
