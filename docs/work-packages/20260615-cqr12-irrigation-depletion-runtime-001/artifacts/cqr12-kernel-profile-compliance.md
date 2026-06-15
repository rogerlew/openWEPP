# CQR12 Kernel Profile Compliance

Status: complete-with-warnings.

Static: CQR12 is kernel-affecting because depletion-scheduled irrigation
runtime projection publishes kernel-facing state.

Kernel profile checklist:

- Science authority changed: no.
- Public API changed: no.
- Parser compatibility changed: no.
- `irrigation.depletion.*` symbol names changed: no.
- Units changed: no.
- Typed error variants changed: no.
- Stable field names and `allowed` strings changed: no.
- Depletion period iteration changed: no.
- Sprinkler/furrow formula or value meaning changed: no.
- Float expression order/grouping changed: no intentional numeric expression
  regrouping.
- Bounded canonicalization added: no.
- Conservation-output formula changed: no.

Ran:

- focused CQR12 tests before and after refactor, exit `0`;
- full workspace Rust gates, exit `0`.

Warning: target-file coverage remains below the ADR-0021 science-tier coverage
threshold. This package closes the scoped CRAP burn-down target only.
