# Independent Verification A: Growth-State CRAP Regression

Evidence class: **Ran + Static**

Final recommendation: **PASS**.

Reviewer A found no growth implementation defect in the initial review. This
verification confirms that the shared gate findings are closed and that the
growth extraction remains identical to the reviewed source and terminal metric
evidence.

## Finding And Dependency Status

| Surface | Status | Verification |
| --- | --- | --- |
| Growth initial review | **PASS / no findings** | The terminal diff still moves only the complete root mass/root depth candidate block into one private helper. Comparison, branch, tuple, and floating-point expression text/grouping remain unchanged. |
| Shared `A-GATE-001` | **PASS / closed** | The growth source and all Rust measurement inputs are sealed by the identical before/after/final `216/418` manifests; an independent current manifest still matches them exactly. |
| Shared `A-GATE-002` | **PASS / closed** | The fresh target result uses the canonical, reviewed registry only; neither the growth function nor its helper is adjudicated. |
| Shared `A-GATE-003` | **PASS / closed** | The post-review result is a fresh, closure-eligible 17-crate census with a complete success envelope and sealed artifacts, not retained assessment evidence. |
| Shared `A-GATE-004` | **PASS / closed** | The report explicitly records `M` for the growth source and no actionable row in or outside the touched set. |

## Contract And Numeric Identity

The current source SHA-256 remains
`1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.
Static comparison reconfirms SC-PLANT-001 `INV-PLANT-027` fidelity:

- the perennial `root_mass_kg_m2 >= rtmmax` saturated-cap test precedes the
  incremental mass expression and division by `rtmmax`;
- the saturated branch still publishes `(rtmmax, min(rdmax, soil_depth))`;
- annual/perennial selection, clamp/floor operations, arithmetic grouping, and
  tuple field order are text-identical to the pre-extraction block;
- `growth.root_depth_upper_bound_m` validation remains before computation,
  `growth.root_depth_candidate_m` validation remains after it, and state output
  order is unchanged;
- the helper is private and adds no allocation, clone, tolerance, fallback,
  conversion, error, or public API.

## Ran Evidence

- Independent focused Nextest selection passed `3/3` (run id
  `0ca5305f-0ac9-4f0a-b323-d18144f243ca`):
  - exact-zero perennial cap and division bypass;
  - ordinary annual incremental root path and R4N publication;
  - ordinary positive-cap perennial path and publication identity.
- Fresh post-review CRAP rows are:
  - `compute_equation_growth_state`: CC `27`, coverage
    `97.22222222222221%`, CRAP `27.015625`;
  - `compute_root_mass_and_depth_candidates`: CC `5`, coverage `100%`, CRAP
    `5`.
- The workspace report is fresh and closure-eligible with `2` raw, `2`
  adjudicated, `0` actionable, and the target source recorded as modified.
- Terminal Rust gates passed on this source identity: format, workspace Clippy,
  full Nextest (`1,960/1,960` executed tests), and deny.
- The file remains `1,668` lines, below both line-count thresholds.

## Residual Risks

Ordinary annual/perennial tests use the repository's established numerical
assertion helper, while the exact-zero cap test uses bit comparisons. For this
mechanical extraction, unchanged source expression text and grouping provide
the additional bit-identity argument required by the CQR standard. No formula
or reassociation risk remains visible.

## Final Disposition

All `CQR-GR-001` through `CQR-GR-007` evidence is present and passing from
Reviewer A's perspective. The growth package is eligible for final `PASS`
closure after Reviewer B's independent verification and parent bookkeeping.

## Final Residual-Fix Verification (2026-07-14)

Evidence class: **Ran + Static**

Final residual-fix recommendation: **PASS**.

This verification was performed independently without reading Reviewer B's
verification. The shared closure gate's final sealed evidence now uses the
manifest-v2 source snapshot at SHA-256
`2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483`, with
identical before/after/final manifests, `216` production sources, `419`
measurement inputs, active Cargo and Rust compiler provenance, an exact `17/17`
production-crate census, and a passing `2` raw / `2` adjudicated / `0`
actionable report. Every sealed checksum passes, and the focused gate suite
passes `17/17`.

The growth source remains unchanged at SHA-256
`1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041` and
`1,668` lines. Static comparison again confirms that the helper extraction
preserves the original comparisons, branch order, arithmetic grouping, error
order, tuple publication, and SC-PLANT-001 `INV-PLANT-027` identity. No growth
symbol is adjudicated. The final sealed metrics remain:

- `compute_equation_growth_state`: CC `27`, coverage
  `97.22222222222221%`, CRAP `27.015625`;
- `compute_root_mass_and_depth_candidates`: CC `5`, coverage `100%`, CRAP `5`.

An independent focused Nextest selection passed `3/3` under run id
`b9dbe80f-1dd1-4c64-b1c0-977fb198112e`. The shared residual fixes therefore do
not alter the reviewed growth implementation or its contract evidence. No
Reviewer A blocker remains, and the growth-state package retains its final
**PASS** recommendation.
