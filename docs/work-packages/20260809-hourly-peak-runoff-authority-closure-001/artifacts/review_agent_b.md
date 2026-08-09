# Independent Hydrology/Science Re-Review B

Status: `executed`

Evidence class: `Static: exact implementation commit 7820953c1b5258564200bd167e0c4994a69b3065, base-to-anchor diff a65cc3973..7820953c, contracts, focused tests, consumer paths, and committed package evidence`

Verdict: `HOLD`

Reviewer independence: Reviewer A's report was not consulted in reaching this verdict.

## Severity-Ranked Findings

### `SCI-B3-001` — CRITICAL — daily same-pass infiltration still manufactures hourly timing

The runtime computes a daily `additional_same_pass_infiltration_m` correction
after the hourly WB14 excess profile has already been produced, then removes
that daily depth from the earliest positive hourly bins
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:234-246`
and `:1912-1932`). The new custody guard correctly hard-fails when runon is
mixed into that ledger, but it permits the same earliest-bin operation for a
local-only ledger.

This is not source custody at producer resolution. A daily correction contains
no evidence that infiltration occurred in the earliest runoff hours. Selecting
those hours can reduce or eliminate an early maximum, move the maximum to a
later hour, and change the HBP/pass peak while preserving daily volume. It is
therefore a temporal process assumption rather than arithmetic reconciliation.
It also conflicts with the operative wording of `INV-WATBAL-103`, which says
that neither daily debit may be distributed across positive bins to manufacture
a peak. The sentence permitting a local-only daily correction does not supply
authority for an earliest-hour allocation.

Proposed disposition: `accepted / closure-blocking`. Produce the additional
same-pass infiltration debit within the hourly WB14 owner and retain its
producer-timed hourly ledger. Until that exists, hard-fail authoritative hourly
peak publication whenever the daily-only correction is material and positive.
Do not substitute proportional, earliest-first, latest-first, or largest-bin
allocation.

### `SCI-B3-002` — HIGH — the complete mutation census is not reconciled to the exact anchor

The committed `artifacts/mutation-study.md` remains explicitly bounded to a
single corrected probe at implementation anchor `949349e70` and states that
the complete cohort is still running. That wording is appropriately
disciplined and does not overclaim acceptance. The committed full-census log
does end successfully and contains 1,913,199 paired event rows, zero invalid
maximum-hour fractions, zero zero-runoff topology mismatches, and zero cases
with volume within five percent while peak changes by at least twofold.
However, the log itself contains no exact source commit, binary hash/path, or
input-manifest identity tying those results to `7820953c`; the mutation-study
artifact does not adopt or interpret it.

Proposed disposition: `accepted / closure-blocking for package evidence`. Run
or positively identify the full cohort against an exact-anchor release binary,
record source commit, binary path/hash, cohort/input identity, command and exit
status, then reconcile the required acceptance metrics in
`artifacts/mutation-study.md`. The raw log alone is not exact-anchor evidence.

## Prior Blocker Rechecks

### Partial frost-retention timing — `PASS`

The proportional allocation is gone. A complete frost debit may clear the
entire hourly series only when no positive local runoff remains; a material
partial debit that leaves positive runoff now returns typed
`MissingDirectUpstream` for producer-timed frost retention. Focused tests cover
both full clearing and partial-positive rejection. No daily-only frost scalar
is allowed to shape an authoritative positive peak.

### `TOL-WATBAL-009` provenance and behavior — `PASS`

The aggregate `24 * 1e-9 m` bound is now explicitly derived from the 24 WB14
intervals and `SC-RUNOFFPART-001#TOL-RUNOFFPART-007`. Reconciliation compares
two independently accumulated ledgers only; it neither fills an empty ledger
nor mutates an hourly bin. The exact aggregate boundary is accepted, a value
just above it hard-fails, and tests confirm the accepted ledger remains
unchanged. This is bounded numerical adjudication, not process timing.

### Synthetic runon timing and mixed-source custody — `PASS`

Positive runon requires a producer hourly shape, is admitted to WB14 exactly
once, and cannot fall back to a uniform or rainfall-derived distribution. The
new mixed local/runon same-pass guard also prevents a daily local correction
from being applied to a merged source ledger. This does not cure
`SCI-B3-001` for the remaining local-only path.

### Melt and WB19 subsurface-return timing — `PASS`

Routed melt is an hourly liquid supply entering WB14 once, rather than a
post-partition runoff limb. WB19 surface-saturation return retains its produced
hour in the shared series. Tests cover melt that becomes runoff and melt that
fully infiltrates, preventing the former double count.

### Consumer area and unit custody — `PASS`

The internal peak is depth rate (`m s^-1`). Outlet publication reconciles it
to the same run-volume depth basis and multiplies by the positive lane area
once to obtain `m^3 s^-1`; HBP/pass serialization consumes that value without
a second area conversion. The multi-OFE integration reconstructs outlet volume
and `max(hourly volume) / 3600 s` from the real pass row.

## Verdict Rationale

Commit `7820953c1b5258564200bd167e0c4994a69b3065` genuinely closes the prior
proportional-frost and tolerance-provenance findings and preserves runon, melt,
WB19 return, area, and consumer custody. `HOLD` remains because the local-only
daily same-pass infiltration correction still selects runoff hours without a
producer clock and can directly determine the public maximum-hour flow. The
full mutation evidence also remains unbound to the exact reviewed anchor in
the governing artifact.
