# Review Agent B — Science Contract QA And Governance

Status: GO after remediation.

Evidence mode: Static + Ran on 2026-08-08.

Initial verdict: `HOLD`.

Findings:

| Finding ID | Severity | File/line reference | Scientific/governance impact | Proposed disposition |
|---|---|---|---|---|
| `B-001` | HIGH | `SC-VEGETATION-001.md:236-279,373-395` | Unresolved authority/vector references prevented mechanical contract closure. | Accept; replace shorthand with full IDs and test resolution. |
| `B-002` | HIGH | `SC-PLANT-001.md:386,501`; `SC-EVAP-001.md:237,269`; `SC-RESIDUE-001.md:286,313`; `SC-WATBAL-001.md:290,499`; `SC-LANDSURFACEENERGY-001.md:218-219,246-247` | Adjacent-owner obligations could evade their invariant and guard schemas. | Accept; add owning rows and remove redundant free-form addenda. |
| `B-003` | HIGH | `SC-VEGETATION-001.md:88-115,184-195` | Ambiguous amount/rate bases invalidated water/energy reconstruction. | Accept; define interval-integrated operands and exact latent identity. |
| `B-004` | HIGH | `SC-VEGETATION-001.md:184-195,383-389` | Per-request bounds allowed aggregate same-layer overbooking. | Accept; require `sum_s U_s,l + W_comp,l <= A_l` on one transaction/area basis and add a poison vector. |
| `B-005` | MEDIUM | `tests/integration/vegetation_boundary_authority_contract.rs:208-376` | Literal-only assertions could pass while references, rows, digests, or receipts were stale. | Accept; make the focused suite structure-aware and identity-binding. |
| `B-006` | MEDIUM | `SC-EVAP-001.md:237,269`; `artifacts/assurance-impact.md:13-28`; `package.md:134-151` | Wrong EVAP ID/receipt mapping and an unreconciled manifest weakened exact custody evidence. | Accept; use `INV-EVAP-028`, correct all four receipts, and authorize only the typed DRAFT render. |
| `B-007` | MEDIUM | `artifacts/line-count-governance.md:1-16`; `artifacts/implementation-test-evidence.md:1-22` | First re-review found stale line/test counts in current-facing evidence. | Accept; recompute counts and focused results. |
| `B-008` | MEDIUM | `artifacts/assurance-impact.md:29-43`; `artifacts/implementation-test-evidence.md:1-22` | First re-review found stale assurance source-root/current-result evidence. | Accept; rerun typed validation and record the exact root and DRAFT/public counts. |

All findings were accepted. The remediated test recomputes the approved-spec
digest, resolves table references, requires adjacent invariant/guard rows,
binds the aggregate layer constraint, and validates the then-current
four-generation assurance chain.

First re-review: all substantive findings closed; `GO-WITH-AMENDMENTS` for
`B-007` and `B-008` only. Both evidence findings were accepted and corrected.

Final amendment check independently confirmed 8/8 tests, the then-current
407/433 line counts, then-current four-transaction wording, source root
`5548ed9b...`,
all-six unit PASS, and interval-integrated operand units. Terminal review later
strengthened registry metadata and the final two assurance receipts, bringing
the focused file to 444 lines without changing its eight-test count. No
science-review finding remained. Final verdict: `GO`.

The reviewer did not inspect RHESSys source or run heavy profiles.
