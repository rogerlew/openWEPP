# Kernel Profile Compliance Checklist

Evidence class: Static + Ran.

- Contract-first amendment completed before production physics wiring.
- No provisional or surrogate production physics was introduced.
- The opt-in formula is explicitly ratified by `SC-SNOWFREEZE-001` v79.
- The default production path remains `legacy_coe`.
- Missing active opt-in state fails typed and does not silently fall back.
- Radiation forcing is consumed from the existing 05B source without snow-only
  scaling, clipping, or fitted multipliers.
- The typed path exposes enough lineage to reconstruct raw melt,
  redistributed melt, routed melt, SWE loss, WB12 `S`, and WB13 liquid forcing.
- Compatibility, rollback, and default behavior remain available.
- Required Rust, deny, contract, and anti-evasion gates passed.
