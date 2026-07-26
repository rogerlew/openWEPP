# Gate Evidence

Evidence class: `Ran unless marked Static`

| Gate | Result | Evidence |
| --- | --- | --- |
| observation source/corpus checksums | PASS | `sha256sum -c` from the corpus directory; 55/55 corpus IDs covered plus 3 explicit gap rows |
| required evidence classes/roles | PASS | 22 observation, 11 fitted operand, 7 derived diagnostic, 8 legacy comparison, 7 model output; one frozen role each |
| native YAML validation | PASS | 9/9 `openwepp-landuse-migrate --validate --to ow-lanuse-1` |
| protected fixture hashes | PASS | all 54 entries in `fixture-pair-manifest.json` independently verified |
| complete fixture execution | PASS | 9/9 direct-production CLIs; seven forest traces; two open controls `NOT_APPLICABLE` |
| real consumer research trace | PASS | seven forest lanes × 16,437 daily rows; producer/consumer aliases independently checked |
| foliar ledger closure | PASS | every traced forest lane/day |
| aggregate/cohort residue closure | PASS | every traced forest lane/day; 45 annual cohort rows per lane |
| deterministic rebuild | PASS | debug and release Hubbard traces independently rebuild identical annual/cohort hashes |
| analysis tests | PASS | `.venv/bin/python -m unittest tools/canopy_phenology/test_cal03_research.py`: 6/6 |
| focused runner tests | PASS | actual JSONL/schema/consumer run plus default-off, identity, finite-value, and typed I/O failure tests: 2/2 |
| management/migrator Nextest | PASS | run `06398618-77f6-44b8-9d31-399e3a75761b`: 38/38 |
| authority anti-evasion source guard | PASS | `check_authority_suite_antievasion.sh` |
| authority obligation Nextest | PASS | run `0566d408-4d3f-4946-85e5-2bc14a426b85`: 3/3 |
| `cargo check -p openwepp-runner` | PASS | no errors or warnings |
| Rust formatting and diff hygiene | PASS | `cargo fmt --check`; `git diff --check` |
| no parameter fitting/new physics | PASS | Static terminal-diff inspection; only uncalibrated fixture seed operands and diagnostic copies added |

Documentation lint is rerun against the staged terminal inventory before final
verification. Broad workspace, coverage, CRAP, and release gates were not
selected by the admitted plan. No pre-heavy audit applies.

## Line-count disposition

The existing included implementation file
`00c_day_input_builder_impl.rs` grows to 1,857 lines. The
trace writer remains colocated with the day-input producer because it needs the
private pending daily result and direct-frame consumer values; moving it would
expand private runtime interfaces for a default-off diagnostic. The new
analysis module is 367 lines and has a dedicated 6-case unit module. No file
crosses a newly introduced 2,000-line boundary.
