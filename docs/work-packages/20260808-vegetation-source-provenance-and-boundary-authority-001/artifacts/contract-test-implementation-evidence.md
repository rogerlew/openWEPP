# Contract-Test Implementation Evidence

Status: PASS; dual independent review complete.

Evidence mode: Ran on 2026-08-08.

Added and registered
`tests/integration/vegetation_boundary_authority_contract.rs`. Its eight tests
bind:

1. the complete contract schema and canonical index row;
2. configuration/state separation and exact native-stratum topology;
3. Stage A/B/C order, bounded reason-coded allocation, sole mutators, and
   atomic commit;
4. transpiration/latent identity and distinct water/energy/element operands;
5. canopy-snow, compatibility, calibration, licensing, and all ten gap holds;
   and
6. canonical authority/test-vector reference resolution;
7. every required adjacent-contract invariant and guard-map row; and
8. digest recomputation plus the complete typed assurance generation chain.

Historical pre-review gate: 6/6 passed before production edits (none followed).
Ran after review remediation:
`cargo nextest run --test vegetation_boundary_authority_contract` — PASS, 8/8,
0 skipped. The first authoring run exposed two overly specific literal
assertions; those test strings were corrected to the actual authoritative
wording without weakening coverage.
