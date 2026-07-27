# Line-Count Governance

Status: `PASS WITH WARN-SIZE RETENTION`

Evidence class: `Ran`

Current owned Rust line counts:

| File | Lines | Disposition |
|---|---:|---|
| `growth.rs` | 1,925 | below warning threshold |
| `erosion.rs` | 1,249 | below warning threshold |
| `00_core_frames.rs` | 2,704 | existing WARN-size aggregate; six trace-field/initializer lines only |
| `03_executor.rs` | 1,774 | below warning threshold |
| `laned_active.rs` | 2,035 | existing WARN-size active-route aggregate; three exact-consumer trace/test lines |
| `00_builders_and_authority.rs` | 2,971 | existing WARN-size aggregate; bounded test/typed-context trace additions |
| `00a_snow_frost_authority_impl.rs` | 685 | below warning threshold |
| `00c_day_input_builder_impl.rs` | 1,894 | below warning threshold |
| runner `03_tests.rs` | 2,656 | existing WARN-size integration-test aggregate |
| `direct_publication_source_guards.rs` | 667 | below warning threshold |

No edited Rust file reaches 3,000 lines. The two production WARN-size
aggregates were not split because the correction adds only co-located typed
handoff/trace fields; moving unrelated frame or authority content would expand
this critical defect's write set and obscure the consumer-path diff. Follow-up
modularization remains repository structural work, not a condition of this
science-defect closure.

Concrete follow-on split intent:

- split `DirectDayFrame` state families and their zero initialization from
  `00_core_frames.rs` into process-owned frame modules;
- split native canopy trace/test-only custody and frost typed-authority
  construction from `00_builders_and_authority.rs`;
- split Lane D trace records/diagnostics and routing test support from
  `laned_active.rs`, leaving the active route kernel and guard together;
- split the native-canopy real-consumer integration scenario from runner
  `03_tests.rs` into a dedicated `tests03/native_canopy_consumers.rs` module.

That follow-on must preserve public paths and run the full-workspace gate; it
is deliberately excluded here because it is a mechanical ownership refactor,
not part of `CAL04B-NATIVE-001` physics or consumer correction.
