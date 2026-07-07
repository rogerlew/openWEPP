# LANED-T3-AGG - Codex code-correctness review

Status: **EXECUTED** (2026-07-07). Verdict: **GO-WITH-AMENDMENTS**.

Evidence mode:

- **Static:** reviewed the uncommitted LANED-T3-AGG change set on top of
  `ef4172d5`, package evidence, `SC-OFEROUTE-001` rev 30, changed routing code,
  current call sites, and active-lane downstream consumers.
- **Ran:** read-only `git status`, `rg` call-site audits, line-anchored source
  inspection, and delegated `rust_code_reviewer` review. No cargo tests or
  H2637 runs were rerun by this review.

## Findings

### High

None.

### Medium

**C-M1 - `route_single_ofe_hybrid` does not locally enforce the hour-aligned
bin invariant required by the aggressive mask.**

Anchor: `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:471`
and `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:509`.

The rev-30 mask samples `seam_rate_at(source_rates_m_s, t0) == 0.0` only at
the bin start, then routes smooth bins with `zero_source`. That is correct for
the active runtime because it passes `LANED_ACTIVE_SAMPLE_DT_S = 900.0`, which
partitions the hourly seam exactly
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:32`
and `:501`). It is not enforced by the public `route_single_ofe_hybrid` API.

Concrete failure scenario: a caller supplies `sample_dt_s = 1000.0`,
`end_time_s = 4000.0`, `source_rates[0] = 0.0`, and `source_rates[1] > 0.0`.
The bin `[3000,4000)` samples hour 0 at `t0 = 3000`, is marked implicit, and
routes with zero source even though the source turns on at `t = 3600`.

Required amendment: fail closed unless the sample cadence partitions
`SEAM_SECONDS_PER_HOUR` exactly, or derive the mask from the source breakpoints
so every implicit-eligible bin is proven source-constant over the whole bin.

### Low

**C-L1 - Sub-noise terminal carry can return `Ok(())` with an unabsorbed
all-zero/near-zero remainder, while comments claim exact total.**

Anchor: `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:373`
and `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:386`.

`dispose_terminal_carry` permits a carry within `1e-9 * gross.max(1e-12)`,
then walks bins backward. If all bins are zero, for example `bins = [0.0]` and
`carry = -5e-22`, the noise floor is `1e-21`; `absorb_deficit` cannot consume
the carry, the loop exits, and the function returns `Ok(())`. The dropped
remainder is bounded below the declared floor, but the comment says the rule is
"exact total" without documenting this all-dry floor disposition.

Required amendment: either recheck `carry_m2` after backward absorption and
fail/handle the residual, or document and test the bounded all-dry drop as an
intentional sub-noise disposition.

## Adversarial Question Disposition

1. **Exact-total carry chain:** Holds through multiple bins and multiple
   explicit spans by the `absorb_deficit` identity, with the exception of the
   bounded all-dry sub-noise case in C-L1.
2. **Over-counting escape surface:** Current non-test consumer of
   `run_with_options_deficit_carry` is the hybrid composition at
   `cascade.rs:574`; it consumes bins through carry correction and discards the
   returned over-counting hydrograph. `pub(super)` is acceptable but still
   relies on sibling-module discipline.
3. **Sub-noise disposition:** Material carry fails closed. Sub-noise backward
   absorption preserves non-negative bins when positive mass exists. The
   all-zero floor case needs documentation or a guard.
4. **Attribution vs mass:** Downstream `UpstreamHandoff.bins_m2` and routed
   erosion weights consume the post-carry bins at `laned_active.rs:333` and
   `laned_active.rs:614`; non-negative/unit-sum guards remain intact. Larger
   cross-hour deficits remain fidelity-ratification risk, not a current closure
   blocker.
5. **Peak/hydrograph consistency:** Acceptable. Peak remains the physical
   pre-carry diagnostic; exported bins/hydrograph are conservative attribution.
6. **Aggressive mask exactness:** Current active path is safe at 900 s, but the
   hybrid API lacks local enforcement. See C-M1.
7. **Wrapper equivalence:** `run_with_options` delegates to the new variant and
   preserves the same `NegativeOutletBin` fail-closed behavior. Plain cascade,
   shadow, and default paths still call the wrapper.
8. **Missing synthetic solver-level vector:** Not a blocker for this
   experimental fix. The recorder seam and composition logic are pinned, and
   H2637 exercised the real class. A deterministic solver-level terminal-deficit
   vector remains useful ratification evidence if one is found.

