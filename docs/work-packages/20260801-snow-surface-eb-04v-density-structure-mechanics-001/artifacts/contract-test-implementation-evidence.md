# Contract-Test Implementation Evidence

Status: `PASS`.

Evidence mode: `[Ran]`.

The pre-production contract cycle first passed the amended authority structure
and failed the deliberately absent runtime/consumer surface. After
implementation:

- `snow_surface_eb04v_density_process_diagnostics_contract`: `2 passed`;
- `hphys0296_snow_rm_acceptance_authority_contract`: `3 passed` after the
  additive trace schema moved from v1 to v2;
- focused orchestrator `snow_density` selection: `12 passed`, including direct
  fresh-snow anti-alias, isolated wet/PTM/POC vectors, independently calculated
  two-layer bulk-space cap attribution, all-field finite guards, climate-class
  snow-free neutrality, and ledger closure vectors;
- `tools/release/check_unit_registry.sh`: passed its registry tests and
  workspace check.

The retained consumer reconstruction read every v2 JSONL row independently.
Maximum closure was `3.411e-13 kg m^-3`; the largest difference from the
runtime-emitted residual was `5.686e-14 kg m^-3`. Omitting overburden produced a
`22.233 kg m^-3` residual, and 100,824 fresh-density rows differed materially
from final density, so the known wrong aliases fail.

The terminal analyzer also reproduces the retained EB-04R B-cell observation
operator for all nine lanes: exact paired counts and maximum KGE-component
difference `4.441e-16`.
