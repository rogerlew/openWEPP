# Validation

Ran:

- `cargo nextest run -p openwepp-runner cqr_laned_active` — `12/12` passed.
- Runner shared coverage profile — `120/120` passed in `94.71s`.
- Focused runner Clippy, dedicated test rustfmt, and scoped diff check — PASS.

Invalid row/detail/step/limiter/TVD numerics and negative, nonclosing, or dry-
nonzero weights return `laned_active_trace_numeric` with exact indexed fields
and leave no output file. Nominal serialization remains byte-identical to the
serializer; dry full-detail and the `+5e-10` unit-sum tolerance edge pass.
Signed `mesh_end_storage_m3` (booked delta-storage) is finite-only; a `-0.75`
regression remains a numeric JSON value. TVD `signed_delta_m` is the only other
signed serialized value and remains finite-only.
