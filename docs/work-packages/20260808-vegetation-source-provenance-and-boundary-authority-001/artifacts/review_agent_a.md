# Review Agent A — Primary Correctness And Science Architecture

Status: GO after remediation.

Evidence mode: Static + Ran on 2026-08-08.

Initial verdict: `HOLD`.

Findings:

| Finding ID | Severity | File/line reference | Scientific/governance impact | Proposed disposition |
|---|---|---|---|---|
| `A-001` | HIGH | `SC-VEGETATION-001.md:88-115,184-195` | Rate/amount ambiguity made the water and latent-energy identities dimensionally invalid. | Accept; use interval-integrated mass/energy operands and one exact `Q_T,s=-h_v*T_s` join. |
| `A-002` | HIGH | `SC-PLANT-001.md:386,501`; `SC-EVAP-001.md:237,269`; `SC-RESIDUE-001.md:286,313`; `SC-WATBAL-001.md:290,499`; `SC-LANDSURFACEENERGY-001.md:218-219,246-247` | Free-form adjacent claims bypassed owning invariant and guard schemas. | Accept; add each binding to the owning invariant table and guard map. |
| `A-003` | MEDIUM | `SC-VEGETATION-001.md:236-279,373-395` | Shorthand authority/vector IDs could not be resolved mechanically. | Accept; use exact canonical IDs and add reference-resolution testing. |
| `A-004` | MEDIUM | `artifacts/assurance-impact.md:13-18` | Reversed receipt labels obscured typed source-custody lineage. | Accept; bind each receipt to the correct source and generation. |
| `A-005` | MEDIUM | `package.md:134-151`; `artifacts/assurance-impact.md:20-28` | The typed DRAFT manifest rewrite was not explicitly authorized or reconciled. | Accept; amend the exact write set and record the machine-owned output without authority change. |
| `A-006` | MEDIUM | `tests/integration/vegetation_boundary_authority_contract.rs:208-376` | Literal-only checks permitted documentary anti-evasion and incomplete receipt/digest proof. | Accept; add structural reference, guard-map, digest, and receipt-chain checks. |
| `A-007` | MEDIUM | `artifacts/line-count-governance.md:1-16`; `artifacts/implementation-test-evidence.md:1-22` | Re-review found stale current-facing line counts and evidence wording. | Accept; recompute and reconcile all current-facing counts. |
| `A-008` | MEDIUM | `artifacts/operand-lineage.md:8-25` | Re-review found rate units still attached to interval transfer operands. | Accept; make every shared transfer interval-integrated and preserve distinct lineages. |

All findings were accepted. Remediation changed ledgers to interval-integrated
amounts, added aggregate layer admissibility, resolved full IDs, added owning
invariant/guard rows, corrected and extended the then-current four-receipt
assurance chain,
explicitly reconciled the typed manifest output, and expanded the focused test
from 6 to 8 structure-aware cases.

First re-review: `GO-WITH-AMENDMENTS` for `A-007` and `A-008` only. Both
evidence findings were accepted and corrected.

Final amendment check: no remaining findings. Final verdict: `GO`.

Terminal evidence remediation later appended two typed receipts for the final
PLANT/RESIDUE addendum cleanup; the six-receipt chain is verified separately.

Ran by reviewer: focused vegetation contract 8/8 and diff hygiene. The reviewer
did not inspect RHESSys source or run heavy profiles.
