# SNOWDENSITY-10.3.21 Active Prompt

Scaffold and execute SNOWDENSITY-10.3.21: post-partition residual
decomposition and frost-attribution-threshold input, diagnostic-only.

Primary constraints:

- Consume strategy section 10.3 step 8, 10.3.20 mechanism-family exhaustion
  review, 10.3.18 pre-partition decomposition, `SC-SNOWFREEZE-001`
  `INV-SNOWFREEZE-050`, ADR-0028, and the SNOTEL plus `cancov_forest` corpora.
- Decompose the current no-env default residual `15` robust fails / `179`
  robust score by signature, climate, mass/SWE, density, depth, and
  persistence direction.
- Classify each residual cluster as forcing-limited/irreducible or as a later
  new mechanism class candidate.
- Produce frost-attribution-threshold input only.
- Do not change production/default/cap/schema/fixture/frost behavior and do not
  add selectors or decide/unblock frost.
