# Review Disposition

Status: `passed`

Evidence mode: `Static:` review disposition plus `Ran:` focused fixes and
fresh full-fixture evidence.

## Findings

| Source | Finding | Severity | Disposition |
| --- | --- | --- | --- |
| Review A | `chanwb` closure was previously a tautology from unavailable channel operands. | High | Fixed. `chanwb` detail operands now emit null unless authoritative volume operands exist; conservation evidence no longer claims channel closure. |
| Review A | Formatting failed during an intermediate state. | Medium | Fixed. `cargo fmt` and `cargo fmt --check` passed after source updates. |
| Review A | Scaling and closure artifacts were stale after source changes. | Medium | Fixed. Fresh full fixture scaling artifacts use `/tmp/wshedw6_*_scaling_final`. |
| Review A | `ebe_pw0.precip` was not mapped through the writer. | Low | Fixed at writer projection level; current fixtures publish null because no authoritative precipitation operand is present in the typed frame. |
| Review B | Stale `particulate_pollutant_kg > 0.0` test assertion after nullable publication semantics. | Critical | Fixed. Test now asserts pollutant null and direct detachment/deposition fields separately. |
| Review B | Same-fixture pinned-legacy evidence missing. | Major | Fixed. `legacy-comparison-evidence.md` records full pinned legacy runs for both committed fixtures. |
| Review B | Contract-gate wording needed explicit disposition for null/area publication behavior. | Major | Fixed. W6 is schema-preserving and physics-preserving; source-slope area is actual geometry, unavailable operands are null, and no SC amendment was required. |
| Review A | Existing-pass manifest mode dropped available `publication_area_m2`, causing area-derived fields to publish null when manifest area was authoritative. | High | Fixed. Manifest publication area is returned by validation and used when no source runfile exists; focused pass+manifest regression asserts `Area`, `Runoff`, and `Q`. |
| Review B | Conservation reconstruction was too self-consistent to close conservation-sensitive acceptance by itself. | Medium | Fixed. Added independent source-runfile/slope-file reconstruction for all 32 and all 1,305 committed hillslopes. |

## Residual Risks

- Current fixture publication rows have zero detachment/deposition and sediment
  yield. Those values are actual pass-backed outputs for the fixture rows, not
  synthetic fills. Additional sediment-rich watershed fixtures remain useful
  follow-on coverage but are not required for W6 closure.
- Legacy and openWEPP timings are same-fixture completion evidence, but they
  are not a claim of output parity or a cross-scope speedup verdict.
