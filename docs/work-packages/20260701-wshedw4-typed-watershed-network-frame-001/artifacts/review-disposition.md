# Review Disposition

Status: `EXECUTED-HOLD`

Dual independent review plus science-contract review is required before
closure. Record each finding as `accepted`, `rejected`, `deferred`, or
`follow-up`, with evidence paths and parent disposition.

Reviews must check gate legitimacy, consumer-path proof, conservation/output
acceptance, and line-count governance.

## Reviewers

Evidence class: `Static:` plus `Ran:`

- `rust_code_reviewer` (`019f1fce-991f-7a42-bedb-32af2c27d5fc`): static
  review plus `git diff --check`.
- `rust_qa_reviewer` (`019f1fce-99e3-7de2-8d7c-9987cba20716`): static review
  plus `git diff --check`, `cargo fmt --check`, and the focused W4
  source-marker test.
- `science_contract_reviewer`: unavailable in the current subagent role list.
  Local science-contract disposition was performed against
  `docs/specifications/science-contracts/AGENTS.md` and the pre-edit operand
  lineage artifact. No canonical `SC-*` contract amendment was made.

## Findings

| Finding | Disposition | Parent disposition |
| --- | --- | --- |
| Production routing is not a typed cutover because the public CLI still calls `network_frame.compatibility_writeback_surface()` and passes a `WatershedWritebackSurface` to `execute_watershed_dispatch_with_kernel`. | `accepted` | This is the primary hold blocker. Package closes `EXECUTED-HOLD-TYPED-ROUTING-KERNEL-WRITEBACK-REMAINS-COMPATIBILITY-EDGE`, not `EXECUTED-COMPLETE-WSHED-W4`. |
| The W4 source guard is partial and currently blesses the hold-path shape by requiring `compatibility_writeback_surface`; it proves CLI direct old-surface construction/publication removal, not routing cutover. | `accepted` | `artifacts/source-guard-evidence.md` remains valid only for the partial handoff claim. Hold lift must replace this with a negative guard forbidding public routing through the compatibility projection and covering orchestrator production routing reads/writes. |
| Conservation/publication acceptance evidence is incomplete: no committed-fixture final identity, independent reconstruction, anti-alias proof, or closure/magnitude audit was recorded. | `accepted` | `artifacts/protected-output-evidence.md` and `artifacts/gate-results.md` mark these gates blocked until typed routing no longer depends on the old writeback projection. |
| Typed publication still harvests compatibility results from `WatershedKernelExecutionReport.writeback_surface`, and missing compatibility symbols default to zero through `map_or(0.0)`. | `accepted` | Identity-compatible for the landed partial implementation, but not fail-closed evidence for complete typed publication. Hold lift must replace silent compatibility harvest/defaults with typed routed-state/publication validation or explicit typed errors. |
| Typed frame builders duplicate old runtime projection/domain logic without all guard parity, so future typed routing could consume values that existing projection helpers reject. | `accepted` | Hold lift must centralize guard authority or add equivalent fail-closed typed-builder validation and tests before typed routing consumes these fields directly. |
| Line-count and gate truthfulness needed final recording after code edits. | `accepted` | `artifacts/line-count-governance.md`, `artifacts/gate-results.md`, `artifacts/verification.md`, and this review disposition record the executed hold state. |

## Conclusion

Both independent reviewers approve only the held disposition. No reviewer
approved `EXECUTED-COMPLETE-WSHED-W4`.
