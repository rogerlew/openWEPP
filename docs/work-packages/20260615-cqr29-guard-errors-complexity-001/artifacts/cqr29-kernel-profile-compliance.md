# CQR29 Kernel Profile Compliance

Static: CQR29 touches kernel-facing guard error support code, not process
physics formulas or runtime publication math.

Static: protected surfaces preserved:

- Public `Wb11HydrologyKernelGuardError` enum variants.
- Stable `HKERNEL-*` error IDs.
- Boundary class mapping.
- Display strings.
- Typed error implementation.
- Parser compatibility and science-contract behavior.

Static: no fallback wrapper, silent dependency masking, numeric
canonicalization, unit conversion, output formula, or float expression-order
change was introduced.

Ran: characterization and workspace gates exercise the public error surface
after private helper extraction.
