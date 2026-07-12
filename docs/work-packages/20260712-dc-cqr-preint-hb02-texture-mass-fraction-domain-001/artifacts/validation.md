# Validation

Evidence class: **Ran**

| Gate | Result | Evidence |
| --- | --- | --- |
| Contract-derived HB-02 tests | PASS | `4/4` |
| Focused erosion-operands suite | PASS | `26/26` |
| Real Wave-1 continuity consumer | PASS | `1/1` |
| Real Yalin transport consumer | PASS | `1/1` |
| Same-source LCOV/JSON/CRAP | PASS with attributed unrelated noise | 383 passed; two source-unchanged audit-counter failures; no retry |
| Science coverage/floors/CRAP | PASS | 98.020% slice lines; 97.318% regions; minimum floor 93.548%; maximum CRAP 14.042 |
| Focused Clippy/format/diff | PASS | `-D warnings`; `cargo fmt --check`; `git diff --check` |
| Line governance | PASS | Production 944 lines; tests 621 lines |

Primary artifacts, hashes, failure identities, and source provenance are in the
HB-02 module record. The final capture source exactly matches the final
production SHA; no edit followed measurement.
