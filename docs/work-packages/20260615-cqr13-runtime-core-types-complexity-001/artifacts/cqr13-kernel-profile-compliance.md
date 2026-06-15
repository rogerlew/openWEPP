# CQR13 Kernel Profile Compliance

Status: complete.

Static: CQR13 is kernel-affecting because runtime-input core types define
fail-closed projection errors and request surfaces consumed by kernel-facing
hillslope runtime publication.

Kernel profile checklist:

- Science authority changed: no.
- Public API changed: no.
- Error IDs changed: no.
- Display text changed: no.
- Typed error variants changed: no.
- Parser compatibility changed: no.
- Symbols, aliases, or units changed: no.
- Numeric expression order/grouping changed: no.
- Bounded canonicalization added: no.
- Conservation-output formula changed: no.
- Production Rust changed: no.

Ran:

- before/after metric gates, exit `0`;
- full workspace Rust gates, exit `0`.

Conclusion: no kernel-profile behavior changed.
