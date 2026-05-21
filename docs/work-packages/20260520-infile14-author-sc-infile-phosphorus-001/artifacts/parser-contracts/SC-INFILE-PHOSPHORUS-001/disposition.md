# Disposition — SC-INFILE-PHOSPHORUS-001

Evidence mode: `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `PHOS-A-001` | `review_agent_a` | high | `amended_closed` | Replaced grouped `tmps*` fanout rows with symbol-level rows in field, propagation, and boundary mapping tables. | `docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:86`, `:87`, `:88`, `:89`, `:105`, `:106`, `:107`, `:108`, `:169`, `:170`, `:171`, `:172` | Per-symbol propagation coverage is now explicit and complete. |
| `PHOS-A-002` | `review_agent_a` | medium | `amended_closed` | Removed strict trailing-token rejection conflation by adopting canonical numeric-leading + optional trailing-text policy and tokenization-specific guard semantics. | `docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:55`, `:56`, `:57`, `:58`, `:176`, `:195` | `PHOS-E-002` remains record-count-only. |
| `PHOS-A-003` | `review_agent_a` | medium | `amended_closed` | Added explicit header text model surface and propagation row so header literal policy is auditable at model level. | `docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:76`, `:95`, `:102` | Header policy now has direct source-field representation. |
| `PHOS-B-001` | `review_agent_b` | high | `amended_closed` | Added non-negative domain guard linkage (`G-PHOS-003`) on `srp/slfp/bfp/scp` propagation rows. | `docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:96`, `:97`, `:98`, `:99` | Guard-linkage completeness restored for concentration fields. |
| `PHOS-B-002` | `review_agent_b` | medium | `amended_closed` | Closed grouped-rows/unit-fidelity gap via explicit per-symbol `tmps*` unit-preserving rows and field mappings. | `docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:86`, `:87`, `:88`, `:89`, `:105`, `:106`, `:107`, `:108`, `:169`, `:170`, `:171`, `:172` | `mg/L` vs `mg/kg` fanout fidelity is now visible per field. |

## Status
- Closed findings: `PHOS-A-001`, `PHOS-A-002`, `PHOS-A-003`, `PHOS-B-001`, `PHOS-B-002`.
- Open high-severity findings: none.
