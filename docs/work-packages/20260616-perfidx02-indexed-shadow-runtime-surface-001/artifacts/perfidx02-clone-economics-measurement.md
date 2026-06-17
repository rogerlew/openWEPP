# PERFIDX02 Clone-Economics Measurement

Status: PASS 2026-06-16
Evidence mode: **Ran** + **Static**

## Verdict

PERFIDX02 passed the ADR-0022 Amendment 1 go/no-go gate.

The chosen Stage 2 shadow representation is `sparse_sorted_vec`: a
working-set-sized sparse sorted `Vec<(SymbolId, BoundaryValue)>`, not a dense
global-`SymbolId` vector. The BTreeMap remains authoritative.

On real H2637 clone-source surfaces with 4,087 present entries:

| Case | BTreeMap clone ns | Sparse clone ns | Sparse speedup | Compact-value clone ns | Compact-value speedup | Sparse lookup speedup | RSS HWM KB |
|---|---:|---:|---:|---:|---:|---:|---:|
| H2637 without UI | 302,463.058 | 4,328.210 | 69.882x | 2,637.913 | 114.660x | 4.245x | 99,424 |
| H2637 with UI | 280,500.548 | 5,185.229 | 54.096x | 3,019.223 | 92.905x | 4.420x | 98,660 |

Both sparse and compact-value candidates keep clone economics comfortably ahead
of the current `BTreeMap::clone` at real H2637 scale.

## Representation Choice

Sparse sorted vector is the Stage 2 choice because it is already a complete
shadow representation:

- it is sized to present entries, not global registry capacity;
- it carries `SymbolId` beside each value, so id-ordered export is direct;
- it preserves the sorted logical `BoundarySymbol` export seam;
- it avoids adding a local-id mapping contract before authority flip.

The compact-value candidate is faster in the microbench, but the measured
candidate intentionally stores only cloned values. A complete compact-local
authority representation would also need local-id/export metadata. That remains
a Stage 3 or later optimization if hot-path O(1) lookup justifies the added
contract surface.

## Real Runs

Final corrected shadow-only timing:

```text
PERFIDX02_H2637_SHADOW_FIXED elapsed_s=892.67 user_s=892.09 sys_s=0.42 maxrss_kb=237224
PERFIDX02_UI_SHADOW case=h2637_with_ui elapsed_s=908.83 user_s=908.12 sys_s=0.54 maxrss_kb=236356
```

The H2637 reports:

```text
/tmp/perfidx02/shadow/h2637.json
/tmp/perfidx02/shadow/h2637_with_ui.json
```

## Method Notes

An earlier H2637 attempt validated every prepared per-lane shadow surface and
was terminated after `1713.34s`; that validation granularity was too expensive
for H2637. The final implementation observes every persistent clone source for
economics and validates daily outlet/final surfaces for shadow equality.

An earlier report also showed single-digit-nanosecond sparse clone timings. That
was rejected as invalid because LLVM optimized away the cloned vector work. The
benchmark was corrected by black-boxing the cloned value itself and increasing
minimum clone repeats. The invalid report is preserved only as local scratch at:

```text
/tmp/perfidx02/shadow/h2637-optimized-away-invalid.json
```

## Go/No-Go

GO. Working-set sparse clone is a measured win at H2637 scale. Stage 3 may
evaluate authority flip mechanics, but PERFIDX02 did not flip storage authority.
