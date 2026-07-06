# Friction Builder Evidence

Status: not-implemented-held
Evidence mode: Static

No active/shadow friction operand builder was implemented in D11.

Current code state:

- `laned_shadow.rs` constructs each OFE mesh with `CellParameters::bare(...)`
  and `LANED_SHADOW_KO = 500`.
- The shadow passes a local `rainfall_intensity_m_s` closure that returns `0.0`.
- Form, wave, and vegetation operands are zeroed by `CellParameters::bare`.

Why no builder was landed:

- A builder that consumes only `I` would still silently use unsupported
  `k_o` and roughness/vegetation defaults.
- A builder that maps residue/roughness/canopy fields to Papanicolaou
  `C_d`, `D_r`, or `lambda` would be a surrogate physics mapping.
- The package found no operator-approved or contract-ratified bounded default
  set.

Hold-lift builder requirements:

1. Create an upstream builder surface before `LanedShadowCollector::new` or
   its activation successor.
2. Feed `CellParameters` and the rainfall-intensity forcing from
   source-authorized operands.
3. Include typed fail-closed errors for missing/invalid operands.
4. Prove a friction-sensitive routed metric changes under a sentinel operand
   and that the real consumer reads the builder.
