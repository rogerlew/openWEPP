# Disposition: SC-SUBHYD-001

Status: complete
Date: 2026-05-20 UTC
Canonical contract: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
Reviewed commit SHA: `28b09da559f40c3a13ea2a9f14d7e1942bcd3f33`
Review snapshot: `12fc4173d9f2f8a44149d4a36e7fa5dc95c804649e417393711b5eb8ed278633`
Post-fix snapshot: `004f2c92925a7d7429562678dfb3715548a96e656c00591b411ee71343bb26a9`

Disposition table:

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
|---|---|---|---|---|---|---|
| `A-001` | `agent_a` | `medium` | `accepted` | Added explicit daily continuity closure identity section for Eq. [6.2.1] with residual term `εsubhyd` bound to `TOL-SUBHYD-001`. | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:97`, `:102`, `:214` | Clarifies accounting boundary for independent implementations and comparator replay. |
| `A-002` | `agent_a` | `medium` | `accepted` | Added drainage-capacity invariant (`INV-SUBHYD-011`) and runtime guard enforcing `Qdd <= D.C.` with explicit producer obligation. | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:125`, `:141`, `:186` | Encodes Chapter-6 hydraulic-capacity cap as normative runtime behavior. |
| `B-001` | `agent_b` | `medium` | `accepted` | Added `D.C.` to Variables and Units and Symbol Alias Map coverage. | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:93`, `:156` | Restores canonical-symbol continuity for drainage-capacity semantics. |
| `B-002` | `agent_b` | `low` | `accepted` | Added explicit cap-boundary tolerance `TOL-SUBHYD-006` for `Qdd` versus `D.C.`. | `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md:219` | Improves comparator interpretation at the hydraulic-cap boundary. |

Final disposition note:
- All reported findings were accepted and addressed in the post-fix snapshot.
- Contract lifecycle remains `in_review`; promotion remains constrained by open
  non-promotable gap entries in the contract gap register.
