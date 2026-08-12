# Contract Amendment Evidence

Status: `authored; focused validation passing`

Evidence mode: `Static`

- `SC-VEGETATION-001` v5 admits `OPENWEPP_C3_WOODY_V1`, invariants
  `INV-VEGETATION-062`--`072`, equations, causal/state ownership, typed
  exclusions, caller configuration/state, solvers and vectors.
- New `SC-BIOGEOCHEM-001` v1 owns mineral-N arbitration and litter/CWD
  C/N/dry-material receipts while making soil transformations an explicit
  dependency rather than a temporary source.
- The canonical registry contains both exact lifecycle rows.
- Existing adjacent contracts already assign soil mutation to hydrology,
  latent-energy identity to land-surface energy, and dead-material receipt to
  residue; v5 narrows only the new prospective vegetation/BGC handoffs. No
  adjacent canonical mutation was necessary before a real implementation
  exists. Prospective implementation dependencies are named in both successors.
