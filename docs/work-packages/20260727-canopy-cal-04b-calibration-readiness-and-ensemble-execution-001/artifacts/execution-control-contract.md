# Execution Control Contract

Status: `DIRECT EXECUTION CONTROL`

Evidence class: `Static`

`direct-execution-plan.json` is the prospective command inventory. The
package-local executor launches its literal argv arrays without a shell,
records command evidence directly, and fsyncs the first failure before
returning. It has no planner, CI, transition, receipt, or lifecycle dependency.
The command-plan CSV and observed-runner artifacts are retained only as
historical execution evidence and are not prospective control authority.

## CTRL-NATIVE-01 Production Consumer Chain

The native proof command is frozen during executor review and records its exact
binary, arguments, environment, toolchain, and checksums. It operates on
package-local copies of:

- `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.native.run.toml`;
- `p10.man.yaml`, changing only the six named GSI threshold fields; and
- the other files named by that runfile, byte-identical to the protected fixture.

The proved chain is copied YAML threshold fields -> management-schema parser ->
runtime forest-phenology projection -> `openwepp-cli-hill
--direct-production-executor` -> `openwepp-canopy-research-daily-v1`
`/gsi/gsi21`. The trace key is `(candidate_id, calendar_date, lane_index)`.
For interior, q00/q100 double-boundary, saturated, and native-default cases,
every daily GSI value must match a direct `openwepp-plant-phenology` kernel run
bit-for-bit. Additional copied configurations vary `Bf,max`, `Bs`, `fe`,
`xmxlai`, `Cs`, and `bb` and bit-compare trace fields for evergreen, deciduous,
total foliar, structural, and total aboveground biomass, LAI, and canopy cover
against direct native realizations. The invalid unordered-threshold case must
fail before day publication with the production typed threshold-order error and
no trace. Direct-kernel results alone carry only a kernel-execution claim.

## CTRL-ID-01 Custody and Preflight

The pre-open input manifest must bind:

- all nine CAL-04A Daymet files and `SHA256SUMS`;
- Hubbard runfile, YAML management, climate, soil, and slope files;
- expected Harvard identities copied from the admitted authority manifest,
  without resolving or reading Harvard content;
- the admitted timing ledger and frozen observation/operator document;
- CAL-04A domain grid and proposed design;
- producer, reconstructor, freeze, and holdout sources and executables;
- Rust/Python/toolchain identities and exact commands.

Calibration latitude is read from and checksum-bound to CAL-04A's
`hubbard-plot-geometry.csv`; it is never an unchecked command literal. The
Daymet derivation must contain exactly 365 ordered real rows for each of nine
plots and 36 years. Each plot-year is an explicit independent cold-start run:
the producer admits yday 1–180, carries no state across plot or year, and
creates no synthetic leap-day or history prefill. Days 1–59 are retained
warm-up and cannot be selected as crossings. Malformed fields, duplicates,
gaps within the admitted sequence, nonchronology, checksum mismatch, identity
mismatch, missing dates, or unexpected dates fail preflight before outputs are
created. The protected `p10.cli` member is used only by `CTRL-NATIVE-01`.
Actual Harvard content hashes and chronology are validated only after the
durable `OPENED_ONCE` transition and must equal the expected identities.

## CTRL-RECON-01 Independent Reconstruction

`calibration-forcing-authority-resolution.md` prospectively replaces only the
older Hubbard composite-lane/first-any-day binding. It preserves the
closed-interval and equal-year objective while making the modeled date
plot-specific and limiting eligibility to CAL-04A's admitted yday 60–180
support window.

Producer daily evidence has key
`(candidate_id, plot_id, year, yday, lane_index)` and contains native `gsi21`.
Lane identity binds the canonical plot ID and source-EML latitude. The
independent reconstructor may read only the immutable daily trace, frozen
configuration inventory, and admitted observation ledger. It must not read
producer crossings, distances, annual components, aggregates, accepted flags,
or producer scoring code.

For every candidate/plot/year, it scans dates once in order. Spring is the
first eligible yday 60–180 pair satisfying
`previous < 0.5 && current >= 0.5`; warm-up days are never crossings. Fall is the first
`previous > 0.5 && current <= 0.5`. Equality belongs to the current day, there
is no interpolation, assignment cannot be overwritten, and a missing required
crossing retains a failed year and makes the candidate objective `+infinity`.

The reconstructor joins every admitted observation exactly once by
`(candidate_id, plot_id, year, record_id)`, derives closed-interval distance, species
diagnostics, annual mean squared distance, aggregate equal-year RMSE, and
accepted membership. Completeness joins must prove all 9,261 grid identities,
all nine lanes and required plot-years, and all admitted observations. The producer has no scoring
or membership output. A primary reconstructor and a separately implemented
verification reconstructor both consume only raw daily trace, configurations,
and observations; their complete derived ledgers and membership must match
exactly. Missing/extra keys block.

## CTRL-HOLDOUT-01 Atomic Opening

The freeze command creates one canonical LF-terminated UTF-8 CSV manifest,
sorted by `identity_id`, with verifier receipts excluded, and hashes those exact
bytes. Two read-only verifiers inspect identities and calibration evidence
without reading Harvard content, then produce separate immutable receipts
containing the same freeze digest, verifier identity, command, timestamp, and
PASS.

The repository `holdout-opening-record.md` is narrative state, not the
exclusivity token. The holdout command exclusively creates and durably syncs a
previously absent `holdout-opened-once.lock` containing `OPENED_ONCE`, the
freeze digest, and exact command before its first Harvard content read. An
existing token or active lock, digest mismatch, non-PASS receipt, empty
ensemble, or incomplete manifest refuses execution. A crash
after the durable transition remains `OPENED_ONCE / INCOMPLETE`; rerun is
forbidden and requires a separately authorized incident disposition. The
holdout process has no calibration-output write path. It runs under
`bubblewrap` with the repository, Harvard fixtures, calibration artifacts, and
executables read-only. Only the custody root and a separate initially empty
holdout-output root are writable. Missing `bubblewrap` fails closed.

## CTRL-SAT-01 Saturation

Family saturation uses all nine checksum-bound CAL-04A Daymet members and only
plot-days from yday 60 through each applicable Hubbard interval upper bound.
The exact factor-range predicate is `max - min <= 1e-12`. The population,
window, family, and result are retained per candidate.

`saturation-evidence.csv` retains
`candidate_id,family,plot_year_windows,population_rows,minimum_yday,`
`maximum_interval_upper_doy,factor_min,factor_max,factor_range,result`.
