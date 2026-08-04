# Line-Count Governance

Status: `PASS / terminal inventory reviewed`

Evidence mode: `Static`

Scaffold counts:

| File | Lines | Initial disposition |
|---|---:|---|
| `03_kernel_support_00_support_helpers.rs` | 931 | below WARN |
| `infiltration_reconciliation.rs` | 2392 | WARN; no unreviewed growth |
| `runoff_reconciliation.rs` | 2632 | WARN; extract ledger/capture seam |
| `00c_day_input_builder_impl.rs` | 2575 | WARN; extract trace/capture seam |
| `00f_snow_accumulation_melt_trace.rs` | 176 | below WARN |

Terminal directly implicated counts:

| File | Scaffold | Terminal | Delta | Disposition |
|---|---:|---:|---:|---|
| `03_kernel_support_00_support_helpers.rs` | 931 | 926 | -5 | compact result fields replace eager payloads |
| `hydrology/02_guard_errors.rs` | 725 | 776 | +51 | typed ledger error category/code/source and focused tests |
| `infiltration_reconciliation.rs` | 2392 | 2485 | +93 | WARN; capture-aware hourly solve retains original arithmetic beside private pack state |
| `runoff_reconciliation.rs` | 2632 | 2724 | +92 | WARN; authoritative conservation sequence constructs, links, and validates both ledgers and the optional payload |
| new `snow_mass_transition.rs` | 0 | 298 | +298 | bounded immutable ledger, outcome, capture, resolution, and validation module |
| `direct_runtime/00_core_frames.rs` | 2712 | 2712 | 0 | WARN; only the already-optional snow shadow record is boxed |
| `direct_runtime/runoff.rs` | 2528 | 2541 | +13 | WARN; consumers select exact ledger operands rather than aliases |
| `00c_day_input_builder_impl.rs` | 2575 | 2579 | +4 | WARN; selection/writer plumbing remains while request ownership moved to bounded `00g` |
| `00f_snow_accumulation_melt_trace.rs` | 176 | 181 | +5 | below WARN |
| new `00g_snow_diagnostic_capture.rs` | 0 | 136 | +136 | bounded request/row-context ownership and selector tests |

Other reviewed remediation hosts remain below 2000 lines:
`direct_runtime/04_audit_error_helpers.rs` is `944` (`+28`) and
`direct_runtime/storage.rs` is `1780` (`+25`). All other touched Rust files
remain below 2000 lines. No file reaches 3000 lines.

The two principal WARN-host increases do not duplicate general-purpose logic:
moving the hourly capture branches would expose private pack state, while
splitting runoff construction would break the required single visible
conservation sequence. The runner host grew only four lines after request and
row-context extraction. Both independent reviewers accepted this disposition;
the 48-byte constructor and live-frame layout headroom is recorded as
non-blocking debt.
