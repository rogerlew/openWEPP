# HPHYS0216C Remediation Streams

Status: completed
Evidence mode: Static + Ran

## Recommended immediate follow-up package
`HPHYS0216D` (proposed):
- slug: `hphys0216d-profilefc-normalized-tail-authority-reconciliation`
- purpose: reconcile FC layer-authority publication with normalized-depth tail
  authority so layer-based publication and seed-profile lineage are consistent.

## Required scope for HPHYS0216D
1. Contract amendments:
   - make normalized-tail authority explicit for FC layer publication path.
   - define whether tail is represented as:
     - extended layer symbols, or
     - explicit tail contribution symbols consumed by WB13 publication.
2. Contract-derived tests:
   - fail if FC publication omits normalized tail.
   - fail if FC publication reintroduces seed-only fallback.
3. Production changes:
   - update runtime-input symbol projection and/or WB13 publication aggregation
     to satisfy the contract amendment.
4. Diagnostics:
   - rerun 39-hillslope semantic lane and require `ProfileFCStore` improvement
     versus HPHYS0216 (`39/39` fail).

## Guardrails
- Do not revert to raw parser-theta fallback authority.
- Keep fail-closed typed guard posture.
- Preserve `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.

## Sequencing
`hphys0216d` must execute before advancing `hphys0217` closure claims.
