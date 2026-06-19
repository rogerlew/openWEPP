# Artifacts

Status: scaffolded 2026-06-19 (queued; not executed).

Required deliverables:

- `perfdeep05-implementation.md` - summary of production changes and ownership
  boundary.
- `perfdeep05-static-hot-loop-proof.md` - static proof that full lane-dense
  resync is removed from the opt-in daily hot loop.
- `perfdeep05-identity.md` - H2637 identity and roundtrip evidence.
- `perfdeep05-endpoint.md` - real H2637 endpoint and RSS against `669.97 s`.
- `perfdeep05-profile.md` - profile evidence that the PERFDEEP04 sync hotspot
  is gone or dispositioned.
- `perfdeep05-gate-results.md` - Rust gates, markdown gate, and line-count
  governance.
- `perfdeep05_disposition.md` - final `CONTINUE`, `NO-GO`, or `HOLD`
  disposition.

Runfiles:

- `runfiles/perfdeep05-h2637.run` - opt-in H2637 endpoint/profile runfile.
- `runfiles/perfdeep05-h2637-default.run` - default-disabled comparison
  runfile.
