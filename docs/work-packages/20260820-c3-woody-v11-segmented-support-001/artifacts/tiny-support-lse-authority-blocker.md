# Tiny-Support LSE Authority Blocker

Date: `2026-08-20`

Frozen implementation checkpoint at investigation start:
`f265e116f16179c110aeddc55e09ca9908674dfb`.

## Trigger

The actual default-off V11 consumer accepts `600+1200`, `1200+600`, three
unequal supports, and forcing-order-sensitive execution. Its required `1 ns`
positive case reaches the unchanged `OPENWEPP_SNOW_FREE_LSE_V1` potential
`JointCanopyGround` solve and returns `LSEB-E-034` after 50 Newton iterations
and 736 backtracks. The `1e-9 s` heat-capacity storage terms produce matrix
infinity norm `2.485478572575206e15`.

Reproduction:

```text
nix develop --command bash -lc \
  'cargo test -p openwepp-hillslope-orchestrator \
   v11_actual_stack_accepts_one_nanosecond_edge_supports -- --nocapture'
```

## Bounded prototypes

All prototype Rust changes were disposable and were reverted after each
result; `crates/openwepp-land-surface-energy/src/solver.rs` has no retained
diff.

1. The existing algebraically root-preserving coordinate preconditioner was
   provisionally enabled for tiny duration. Pivot evidence increased from
   about `108.64` to `108639`, but the matrix norm, 50-iteration trajectory,
   residuals, and rejection were materially unchanged.
2. A descending-duration coordinate-only continuation started at `600 s`,
   reused only each accepted provisional unknown vector, and reevaluated the
   unchanged equations at successively smaller support. It reached a
   few-nanosecond near-root but failed after 5 iterations and 20 backtracks:
   normalized `soil-1=1.0321709530518652` and
   `soil-dry=1.0370899684347248`, while temperature steps were about `1e-20 K`
   and hydraulic steps about `1e-18`; no representable trial strictly reduced
   the norm.
3. Repeating continuation with a finer `0.75` duration ratio reached the same
   binary64 precision wall and `NumericalBacktrackingLimit`.

## Exact contradiction

Coordinate prediction and linear-system scaling do not suffice to admit the
required `1 ns` slab while simultaneously preserving:

- the existing rate-form storage residual evaluation;
- unchanged physical residual tolerances;
- strict residual decrease;
- the exact target-support coupled solve; and
- immutable historical V10/full-support behavior.

At nanosecond duration, `C/dt` storage evaluation loses enough binary64
resolution that the remaining soil residual is just outside its unchanged
acceptance threshold, while the computed Newton step is below representable
state progress. A continuation-only `SC-LANDSURFACEENERGY-001` amendment was
therefore withdrawn before review.

Future authority work must decide a separately reviewed numerical surface,
such as algebraically equivalent interval-integrated energy residuals with
declared compensated arithmetic, or a physically justified tiny-support
asymptotic rule. It may not freeze state, scale outputs, loosen acceptance,
publish provisional work, or treat typed rejection as satisfying the positive
vector.

## Targeted V11 gate after compatibility reconciliation

The full-support gate now compares a generated typed projection of the entire
serialized V8 physical payload. It removes only the explicitly released
successor identity/chronology paths (model, configuration, state and outer
transaction identities; stratum transaction identity; occupancy accepted
transaction identity) and compares every remaining field exactly without
mutating either execution result.

```text
cargo test -p openwepp-hillslope-orchestrator v11_
4 passed; 1 failed
```

Passing: full-support exact physical compatibility, rejected-attempt no-op,
sequential unequal supports, and forcing-order observability. The sole failure
is the documented `1 ns` `LSEB-E-034 NumericalIterationLimit` (50 iterations,
736 backtracks); no additional compatibility or custody failure remains.
