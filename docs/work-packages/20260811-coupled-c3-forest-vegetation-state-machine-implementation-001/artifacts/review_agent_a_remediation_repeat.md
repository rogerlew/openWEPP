# Repeat Independent Rust Correctness Review

Status: `FAIL / material findings remain at authority HOLD`

Evidence mode: `Static / independent reviewer findings recorded by parent`

The independent Rust reviewer inspected the evolving exact remediation bytes
and reported the findings below through the review channel. The reviewer did
not complete its own file write before the authority HOLD was adjudicated; this
artifact preserves its reported findings without upgrading their disposition.

## Confirmed corrections

- The public radiation seam now uses one exact piecewise column solver; the
  duplicate single-layer science implementation was removed.
- Direct/diffuse components use the same linear column system.
- Configuration now rejects nonfinite crown, liquid-capacity, and GSI fields.
- Beginning and accepted state enforce LAI/SLA, SAI, and root-area identities.
- Tile-local radiation area is derived from stand-ground area and `C_s`, then
  aggregated with `f_t`.

## Material findings

1. `CRITICAL`: E04 interception remains non-reducible for heterogeneous tiles;
   the science review independently adjudicated this as missing canonical
   authority. The public path now fails closed for the affected topology.
2. `CRITICAL`: numerical failure errors do not carry required residual and
   iteration diagnostics; aggregate diagnostics omit nested `ci`/outer solve
   components, and mixed-unit energy residual components are not independently
   normalized to the canonical acceptance criterion.
3. `CRITICAL`: final upper-rank condensation/drainage is not recomputed through
   a tile-resolved lower-rank liquid handoff.
4. `CRITICAL`: energy and BGC owner validation remains producer-consistent
   rather than genuinely independent reconstruction from separately owned
   operands/receipts.
5. `HIGH`: the public vegetation-only `validate_and_commit` API permits a
   commit outside the complete water/BGC/energy owner set.
6. `HIGH`: arbitration does not reject duplicate request identity before
   deterministic proportional summation.
7. `HIGH`: stem shortwave uses an unauthorized unconditional `.max(0.0)` rather
   than a provenance-bound roundoff threshold and typed material failure.
8. `WARN`: `transaction.rs` exceeds the 2,000-line warning threshold and needs
   the decomposition recorded in `line-count-governance.md`.

All findings are accepted. None is claimed fixed by the HOLD disposition. They
remain work in this package after the canonical E04 topology rule is supplied.

## Verdict

`FAIL`. Heavy gates and terminal verification are not legitimate. The package
has an independently confirmed authority HOLD and unresolved Rust correctness
work; it is not implementation-complete.
