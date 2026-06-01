# AUTH12 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: FC rocky-soil physics closure + cohort promotion — the `cpm`
multiplier-policy change in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`,
the direct-θ FC cohort posture/obligations, and the anti-evasion guard state.
Follow-on to the AUTH11 review.

Evidence: **Static** (read diff/registry/fixture/obligations) and **Ran**
(commands executed by this reviewer).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — What the change does (factual)

Static. `legacy_correct_layer_moisture` now applies the `cpm` rock-fragment
multiplier to `thetfc`/`thetdr` and to `sm20c` only when
`FcWpRockMultiplierPolicy::ApplyToMeasuredFcWp`, and skips it for
`SkipForMeasuredFcWp`. `fc_wp_rock_multiplier_policy(ofe)` returns
`SkipForMeasuredFcWp` for disturbed-policy formats `9002`/`9003`/`9005` and
`ApplyToMeasuredFcWp` otherwise. The change is format-gated and preserves legacy
behavior for other formats. Ran (reviewer):
`cargo test --test auth07_fc_authority_cohort_contract` → 2 passed; the anchor
`valid_9002_reference` moved `expected_threshold_status: "exceeds" → "within"`.

## F-1b — Producer cross-check CONTRADICTS the fix's premise (correction to F-1 of the AUTH05 worked example)

Static. AUTH12's stated rationale is that 9002/9003/9005 carry SURGO measured
FC/WP "already rock-adjusted," so applying `cpm` is a second rock REDUCTION. The
producer
(`/workdir/wepppy/wepppy/soils/ssurgo/ssurgo.py:519-525`, marked
`# ERIN_ADJUST_FCWP`) does the opposite direction:

```
field_cap = (0.01 * wthirdbar_r) / (1.0 - min(50.0, rock)/100.0)
wilt_pt   = (0.01 * wfifteenbar_r) / (1.0 - min(50.0, rock)/100.0)
```

It DIVIDES by `(1 − rock)`, INFLATING FC/WP for rocky soils (×2 at rock=50%).
WEPP's `cpm` MULTIPLIES (reduces). These are inverse operations, and in the
established wepppy→legacy-WEPP pipeline legacy WEPP applies `cpm` (`scon.for`) —
so the producer's division and WEPP's `cpm` are a matched divide/multiply pair,
not two reductions. AUTH12's premise (a double rock reduction) is therefore
factually incorrect; the prior unconditional `cpm` multiply was not a second
reduction but the counterpart to the producer's division. openWEPP consumes the
same wepppy `.sol` files legacy WEPP consumes, so honoring the producer contract
requires applying `cpm` exactly as legacy `scon.for` does — i.e. the original
behavior, not AUTH12's skip. This also corrects the
AUTH05 worked example: its "−33 kPa authority" (`Σ fc_measured·dg = 223 mm` for
H1) is the producer's POST-division (pre-`cpm`) value, not an independent
whole-soil physical field capacity. The model's `cpm`-corrected value (~107 mm,
matching legacy ~114 mm) was on the basis legacy uses; my earlier conclusion that
"model FC is ~2× too low" used the producer intermediate as authority and is
withdrawn pending basis resolution.

## F-1c — The promoted gate is circular

Static. The cohort "authority" is `Σ(fc_measured·dg)` where `fc_measured` is the
producer's declared `field_cap` (the pre-`cpm` value). AUTH12 makes the model
skip `cpm`, so the published model FC now equals that same declared value by
construction. The gate passing (`within`) therefore reflects the model echoing
the producer's pre-`cpm` intermediate, not agreement with an independent physical
authority. As built, this Level-4 "constitutive" gate does not validate FC
physics; it validates that `cpm` was removed.

## F-2 — The promotion was procedurally clean, but it arms a circular gate

Static + Ran. The direct-θ cohort is promoted to `authority_level: 4`,
`gate_lane: required`, `failure_class: hard-fail`; the obligation binding for
`valid_9002_reference` was updated `exceeds → within`, and the anti-evasion guard
accepts it (Ran: `bash tools/release/check_authority_suite_antievasion.sh` →
PASS) because cases, fixtures, and threshold bound are preserved. So the
anti-evasion machinery worked as designed — no case removal this time. However,
per F-1c the gate the promotion arms is circular (model echoes the producer's
pre-`cpm` value), so "passes on merit" does not hold: the green required gate
certifies that `cpm` was skipped, not that FC is physically correct. The
procedural path was honest; the gate it produced is not yet a valid physics
check.

## F-3 — The production behavioral change was not validated against the 39-hillslope cohort

Static. This is the largest behavioral change in the FC/WP arc: it alters
FC/WP for every `9002`/`9003`/`9005` soil (the production format), and field
capacity governs the daily water balance (free-drainage cutoff, ET extraction
bound, lateral/drain thresholds). AUTH12 ran the constitutive cohort test and
workspace/release gates, but no 39-hillslope semantic rerun. Consequences:
- The over-drainage outcome is unmeasured. The arc's thesis is that the too-low
  FC drives the `Total-Soil`/`SoilWaterTotal` over-drainage (HPHYS0223:
  140.7 mm; AUTH05 worked example). Whether this fix reduces it is unverified.
- Broader regressions are unchecked: raising FC across all modern soils shifts
  storage, percolation, and ET on every column, including currently-passing
  ones.
- The package's `worker-handoff.md` records that a rerun should be done "if
  future FC/WP lineages are adjusted"; FC/WP lineages were adjusted here, and the
  rerun was not performed in-package.

This repeats the HPHYS0222 pattern (a cohort-affecting behavioral change landed
without a same-package rerun), at substantially larger scope.

## F-4 — The format scoping is a science-policy decision that should be contract-grounded

Static. The cpm-skip is keyed on disturbed-policy format
(`9002`/`9003`/`9005` skip; otherwise apply). This encodes an assumption that
those formats' measured FC/WP are pre-rock-adjusted and other formats' are not.
The assumption is consistent with the SURGO provenance observed on `p1.sol`, but
it is a constitutive policy that should be validated and stated per format
(including `7778`/`None`) in the science contract, not asserted only in the
kernel.

## F-5 — Commit state and gates

Static. AUTH12 is uncommitted (working tree; `git log` HEAD is `1b97d8f` =
AUTH11) despite the disposition's `complete`/`GO`; it changes production code
(`02_soil_slope.rs`, `08_tests.rs`, `mod.rs`). Ran (reviewer, working tree):
`cargo fmt --check` → exit 0; `cargo deny check` → exit 0 (non-blocking
duplicate/license-allowance warnings only); anti-evasion guard → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0; `cargo test
--workspace` → exit 0. (Gate-green does not bear on F-1b/F-1c, which concern
the physical validity of the change, not its compilation/test status.)
