# CQR29 Behavior Equivalence

Ran: added
`cqr29_phase_guard_error_surface_preserves_codes_classes_and_display` and
`cqr29_erod_guard_error_surface_preserves_codes_classes_and_display`.

Static: the characterization asserts exact outputs for all 15
`Wb11HydrologyKernelGuardError` variants:

- `code()`
- `boundary_class()`
- `to_string()`

Static: production changes are private-only. No public enum variant, public
method signature, error ID, boundary class, or display format changed.

Ran: focused test command passed:

```text
cargo test -p openwepp-hillslope-orchestrator cqr29
```
