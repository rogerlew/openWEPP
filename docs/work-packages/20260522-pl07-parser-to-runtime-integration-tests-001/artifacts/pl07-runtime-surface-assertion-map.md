# PL07 Runtime Surface Assertion Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL runtime projections must include schedule, growth, and decomposition/resup families plus canonical seed aliases.

Ran:
- Family-completeness assertions and fixture coverage checks pass in integration harness.

| assertion_id | family | assertion scope | anchor |
|---|---|---|---|
| `PL07-ASM-001` | schedule | ordering preconditions (`pl_order_decomp_before_soil`, `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth`) | `assert_pl_ordering_flags` |
| `PL07-ASM-002` | schedule + growth + decomp | per-OFE seed symbol projection (`initial_ref`, `lanuse`, `imngmt_seed`, `rtyp_seed`, `iresd_seed`, `sumrtm_seed`, `sumsrm_seed`, optional understory seeds) | `assert_pl_ofe_seed_coverage` |
| `PL07-ASM-003` | schedule | per-slot topology/year closure symbol projection | `assert_pl_slot_projection_coverage` |
| `PL07-ASM-004` | schedule + growth | per-slot/per-crop core symbol projection (`yearly_ref`, `itype`, `tilseq`, `conset`, `drset`, `imngmt`, growth class selectors) | `assert_slot_crop_schedule_symbols`, `assert_slot_crop_growth_common_symbols` |
| `PL07-ASM-005` | growth + decomp/resup | branch-specific projection: annual (`jdharv`, `jdplt`, `rw`, `resmgt`), perennial (`jdharv`, `jdplt`, `jdstop`, `rw`, `mgtopt`, `ncut`, `ncycle`) | `assert_slot_crop_branch_symbols` |
| `PL07-ASM-006` | merged alias continuity | canonical seed aliases remain present in merged runtime surface (`lanuse`, `itype`, `imngmt`, `jdharv`, `jdplt`, `rw`, `resmgt`, `iresd_seed`, `sumrtm_seed`, `sumsrm_seed`) | `assert_merged_pl_seed_aliases` |

Reference block:
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:530`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:542`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:821`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:830`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:848`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:899`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:938`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:959`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:972`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:1024`
