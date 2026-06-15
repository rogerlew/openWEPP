# CQR11 Disposition

Status: complete-with-warnings.

Static: CQR11 package objective was met for the scoped target.

Closed:

- `parse_yearly_perennial` CRAP reduced from `1406.0` to `4.0`.
- Every newly extracted perennial helper is CRAP `<= 9.0`.
- Focused perennial parser characterization passed before and after production
  refactor.
- Public API, parser-output shape, stable error IDs, typed error variants,
  compatibility branches, count/cardinality guards, and runtime/kernel-facing
  field meanings were preserved.
- Required Rust closure gates passed.

Warnings accepted:

- target-file coverage remains below the science-tier threshold;
- pre-existing out-of-scope management parser CRAP rows remain above `30`;
- the existing crate-level `too_many_lines` suppression remains unchanged.

No review finding remains undispositioned.

Package status: complete-with-warnings.
