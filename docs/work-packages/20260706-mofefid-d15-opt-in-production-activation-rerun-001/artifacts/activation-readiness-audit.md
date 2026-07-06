# Activation Readiness Audit

Status: **EXECUTED-HOLD**.

Evidence mode: Static + Ran (timing/test failure cross-reference).

## Audit Checklist

| Preconditions | Status | Evidence |
|---|---|---|
| `INV-OFEROUTE-010` subsystem-off protected-output byte identity | PARTIAL / HOLD | Shadow-only identity exists: `OPENWEPP_LANED_SHADOW` collector is optional in `05_runner_execution_and_outputs.rs:89-98`; H2637 tests assert no shadow key off and byte identity on/off when the shadow runs. Missing: byte-flat proof for a future active selector. |
| `INV-OFEROUTE-011` D-val / `GAP-OFEROUTE-005` D10B closure | PASS STATIC | `SC-OFEROUTE-001` rev 25/26 closes Case 4 / `GAP-OFEROUTE-005`; D10B contract-derived tests cover oracle convergence, TVD mass neutrality, conservative handoff, and 19-OFE exact conservation. |
| `INV-OFEROUTE-012` active `ui_SCrunf` source term | PARTIAL / HOLD | Seam helper exists at pure/solver tier: `seam_source_rate_series` consumes `wb14_hourly_excess_m + ui_SCrunf` and divides by 3600 (`seam.rs:36-59`). Missing: active production runtime wiring. |
| `INV-OFEROUTE-012` active `latqcc` bypass closure operand | PARTIAL / HOLD | Closure helper includes `latqcc_outlet_m3` (`seam.rs:145-177`). Missing: active production closure operand construction and hard-fail. |
| `INV-OFEROUTE-012` runtime closure hard-fail in active mode | HOLD | No active production path invokes the closure helper. `ofe_routing.rs:5-7` states no production phase-span wiring exists. |
| DC01 daily-lump runon disabled for active routed lanes | HOLD | Current production WB14 path unconditionally calls `apply_dc01_runon_supply_admission()` (`runoff.rs:205-209`), which injects runon into producer inputs (`runoff.rs:632-646`). The seam double-count guard exists only as pure/test machinery. |
| Rev-21 friction operands consumed by active production path | HOLD | Static/dynamic operands feed the shadow path (`00_builders_and_authority.rs:23-32`, `00c_day_input_builder_impl.rs:355-370`, `laned_shadow.rs:62-80`). Missing: active production consumer proof. |
| D12 source-shape limbs consumed by active production path | HOLD | D12 source-shape exists through DC01/shadow (`runoff.rs:1398-1410`, `laned_shadow.rs:1-8`). Missing: active routed producer source series independent of diagnostic DC01-weight reconstruction. |
| D13 routed hydrograph shape feeds erosion active consumer | PARTIAL / HOLD | Candidate consumer exists and fails closed (`erosion.rs:98-103`, `erosion.rs:434-462`); tests prove routed shape supersedes DC01 (`direct_runtime_wave1_continuity.rs:794-820`). Missing: production builder still selects `Dc01SourceShape` and `None` (`00_builders_and_authority.rs:1668-1678`); no routed producer feeds it. |
| Missing active selector or stale shadow-only paths | HOLD | Current routed subsystem module doc states diagnostics-only shadow and no production phase-span wiring (`ofe_routing.rs:5-7`). |

## Findings

1. `INV-OFEROUTE-011` is no longer the D15 blocker. D10B resolved the source
   authority and Case-4 acceptance surface.
2. The current tree is activation-candidate/shadow-ready, not active
   production-ready. There is no active Lane D production selector/path.
3. The required D14 timing refresh is blocked by `NegativeOutletBin` on the
   H2637 shadow path before endpoint completion.
4. The existing seam/DC01/erosion pieces are necessary but not sufficient:
   they are pure helpers, diagnostics-only shadow plumbing, or active-candidate
   consumer tests. They do not prove a real active downstream consumer reads the
   routed path.

## Decision

Do not implement a partial activation flip in this package. Phase C would have
to create the active owner path, disable DC01 on active lanes, solve the
day-boundary/terminal-bin evidence blocker, construct active closure operands,
and feed the D13 consumer. The current timing failure and missing active
selector make a complete, authority-backed activation claim unavailable.
