# Review Agent B — Hydrology And Ownership

Evidence class: `Static` exact-worktree review. Verdict: **FAIL**.

The complete agent mailbox review is retained in the campaign thread. This
artifact preserves its exact reported reviewed hash prefixes:

- `SC-LANDSURFACEENERGY-001.md`: `b1e2d15...`
- `SC-VEGETATION-001.md`: `f39fe2b...`
- `SC-VEGETATIONTRANSACTION-001.md`: `c610648...`
- `SC-WATBAL-001.md`: `c30b7c2...`
- `openwepp_snow_free_lse_v1_definition.json`: `83f2425...`
- `reference_calculator.py`: `3654a5d...`
- `openwepp_snow_free_lse_v1_vectors.json`: `ca78c8a...`

| Finding | Severity | Accepted review conclusion |
|---|---|---|
| `OWN-CRITICAL-001` | critical | Water-mass and thermal ownership conflict; the draft permits an LSE mass/temperature copy alongside hydrology ownership. |
| `OWN-CRITICAL-002` | critical | Condensation is absent from canonical request/use/credit identities and ending-store reconstruction. |
| `OWN-CRITICAL-003` | critical | Potential/final ingress changes can invalidate the supply snapshot backing authorization. |
| `OWN-CRITICAL-004` | critical | The real owning-hydrology arbitration/debit/receipt protocol is not canonically admitted. |
| `OWN-CRITICAL-005` | critical | Precipitation, runon, infiltration, and runoff energy lack exact source/receiver custody, including routed transfers. |
| `OWN-HIGH-006` | high | OFE-local tile fractions, stand/OFE bases, and multi-OFE routed area conversion are underspecified. |
| `OWN-HIGH-007` | high | Strict owner/configuration/state/forcing schema is descriptive rather than exact. |
| `OWN-HIGH-008` | high | Precipitation and runon temperature providers and missing-lineage failure posture are incomplete. |
| `OWN-CRITICAL-009` | critical | The fixture lacks a real ownership transaction with D/A/F, credits, receivers, rollback, and owner candidates. |

All findings are accepted. No finding in this artifact is marked fixed, PASS,
or release-eligible. Corrections require a fresh review against stable exact
bytes.
