# Claude Code Review — HPHYS0313 Settling-Route Misattribution

Status: complete

Reviewer: Claude Code (proactive science-contract review)

Evidence mode: static + ran-arithmetic

> Scope note: this artifact surfaces findings and evidence only. Architecture,
> contract wording, and disposition are Codex's to decide. Nothing in the
> working tree was edited to produce this review.

## Evidence class

- Static: read `SC-SNOWFREEZE-001` / `SC-WATBAL-001` amendments, the runner
  `hphys0313_snowpack_settling_carry_recursion.py`, the instrumentation patch,
  pinned-baseline `/workdir/wepp-forest_260430_baseline/src/snowd.for`, and
  openWEPP `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`.
- Ran: arithmetic on the committed
  `artifacts/snowpack-settling-carry-recursion-ledger.json` (delta
  computation only; the comparator and kernel were not executed).

## Posture is correct

- Disposition is `HOLD`, `production_edit_authorized: false` on all six rows.
- Gates are green; the runner fails closed on missing source lines.
- No kernel edit shipped. The damage below is in evidence/authority text, not
  in production code.

## Finding 1 (Blocker for the next package) — the settling route's "driftg" root-cause is contradicted by the runner's own data

The three `settling-depth-update-hold` rows (H1/H7/H39 → target 2014) are
classified `cold-driftg-addition-lineage-hold` with the conclusion:

> "the material final-state delta is the baseline cold no-snow `driftg`
> addition (`snowd.for:145-146`) absent from openWEPP's homologous final-depth
> lane."

This is refuted on the baseline side alone. From the ledger (H1, 2013 day 11
hour 11):

| quantity | value | source |
|---|---|---|
| baseline `hrsnow_m` | `7.4545e-4` | `H313_WDAY` observe tag |
| `inferred_driftg_addition_m` (final − post-settling) | `7.4546e-4` | runner py:588 |
| implied `driftf + driftg` (difference) | `≈ 5.9e-9` (≈ 0) | derived |
| openWEPP `snowfall_depth_m`, same hour | `0.0` | runner py:620 |

The no-snow branch at `snowd.for:145-146` is guarded by
`if (hrsnow(hour) .le. 0.0)`. The runner observed `hrsnow = 7.45e-4 > 0`, so
**baseline executed the snowing branch at `snowd.for:167`**
(`snodep = snodpt + hrsnow + driftf + driftg`), not the no-snow branch. The
depth gain the runner labeled "driftg" is almost entirely **fresh hourly
snowfall (`hrsnow`)**; the true `driftf + driftg` contribution is `~6e-9 m`.

Root cause of the mislabel: `inferred_driftg_addition = depth_after_cold_branch
- depth_after_settling` (py:588) is named "driftg" unconditionally, without
gating on the `hrsnow` value the runner already parsed from `H313_WDAY`.

### What the real divergence is

The runner also recorded the actual divergent surface but did not name it:
baseline `hrsnow = 7.45e-4 m` versus openWEPP `snowfall_depth = 0` at the same
hour. This is an **hourly snowfall input / phase-partition mismatch**, not a
missing snow-drift depth term.

Corroborating: openWEPP's cold snowing branch
(`03_kernel_support_00_support_helpers.rs:3918`, `snodep = snodpt + hrsnow`) is
equation-faithful to baseline `snowd.for:167` (modulo drift, which is ~0). So
this is not a settling/accumulation kernel-equation defect at all — openWEPP
simply saw no snow that hour. The divergence is upstream of `snowd`, in the
hourly snowfall lane.

## Finding 2 (Blocker) — the misattribution propagated into authority surfaces

Because the conclusion is wrong, three durable surfaces now mis-specify the
follow-up obligation:

1. `SC-SNOWFREEZE-001#INV-SNOWFREEZE-038` and `SC-WATBAL-001#INV-WATBAL-086`
   require future evidence to reconstruct "cold-branch final depth after
   `snowd.for:145-146` `driftg` addition for 2013 day 11 hour 11" — wrong branch,
   ~0 term.
2. `worker-handoff.md` directs the next package to "prioritize
   baseline-authoritative migration of the snow-drift cold-branch final-depth
   lane (`driftf`/`driftg`)" — porting a `~6e-9 m` term.
3. `hphys0313_..._contract.rs:103` asserts the ledger contains
   `"cold-driftg-addition-lineage-hold"`, locking the mislabel into the test.

## Finding 3 (Caveat to verify) — confirm openWEPP snowfall=0 is real

`hour_value(row, "snow_hourly_snowfall_depth_m", …)` returning `0.0` for an
absent key is indistinguishable from a genuine zero. Before the next package
treats the input mismatch as established, prove openWEPP's hourly snowfall at
2013 day 11 hour 11 is really `0` and not a missing-key default.

## Finding 4 (Minor) — threshold-grazing divergence

The only "material" channel is depth (`7.46e-4` vs `5e-4 m` tolerance); the
density delta (`0.148`) is below the `0.5 kg m^-3` tolerance. A single
fresh-snow hour barely crossing one tolerance is weak ground for a "missing
physics term" conclusion and should invite skepticism on its own.

## Year-start route — no analogous defect found

The three `recursive-year-start-inherited-state-hold` rows (target 2016, scan
2014) are supported by the ledger: first material divergence already present at
2014 day 1 hour 1, `last_within_tolerance_state_before_first_divergence: null`,
recurse-to-2013 conclusion. Only minor narrative drift: package/contract framing
says "2014 carry feeds 2015 day 1 hour 1" while the ledger scanned 2014 and found
divergence at its own year-start — consistent, just relabel for clarity.

## Suggested actions (Codex's call)

1. Gate `inferred_*_addition` on the actual M3 branch: `hrsnow ≤ 0` ⇒ driftg
   lane; `hrsnow > 0` ⇒ snowfall lane. Reclassify the settling route as an
   hourly-snowfall (`hrsnow`) input divergence.
2. Correct `INV-SNOWFREEZE-038` / `INV-WATBAL-086` and `worker-handoff.md` to
   point at the snowfall / phase-partition surface, not `snowd.for:145-146`.
3. Update the test assertion that hard-codes `cold-driftg-addition`.
4. Prove openWEPP `snowfall_depth = 0` at the key hour is a real value, not an
   absent-key default.
