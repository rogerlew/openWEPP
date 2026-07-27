# CAL-04B Executor Schema

Status: `FROZEN PROSPECTIVE DESIGN`

Evidence class: `Static`

## Configuration Inventory

`candidate-configurations.csv` has one row per complete vector:

`candidate_id,temperature_pair_id,vpd_pair_id,photoperiod_pair_id,` followed by
the six native threshold fields, `boundary_class`, and three family saturation
flags. Candidate IDs are `GSI-0001` through `GSI-9261` in CAL-04A lexicographic
temperature/VPD/photoperiod order.

## Daily Trace

The producer writes no crossing, component, objective, or membership data. Its
results are the fixed `CAL04B03` binary daily trace, canonical identity
sidecar, lane manifest, keyed calendar, execution receipt, and typed
producer-failure ledger described in `intent-plan.md`.

The trace header is magic, candidate count, lane count, and retained days per
lane. Values are candidate-major, lane-major, year-major, then yday-major.
The nine canonical lanes are `1B`, `4B`, `4T`, `5B`, `5T`, `6T`, `7B`, `7T`,
and `HQ`. Each candidate/plot/year starts a fresh native `GsiState`, admits
only authenticated Daymet yday 1 through 180, and retains all 180 values.
Thus each lane has 6,480 values and the complete trace has 540,101,520 `f64`
values plus its 20-byte header. The protected `p10.cli` is not a calibration
input and remains native-proof-only.

## Reconstruction

The reconstructor independently derives:

- first plot-year spring crossing ending on yday 60 through 180 with
  `previous < 0.5 && current >= 0.5`; yday 1 through 59 are real warm-up and
  the yday 59-to-60 pair is eligible;
- closed-interval observation distance for every calibration record;
- annual mean squared distance over all admitted records across plots and
  species, plus annual RMSE;
- aggregate square root of the unweighted mean of annual MSE;
- species RMSE, observation/year median absolute distance, interval coverage,
  and failure counts;
- finite minimum, `minimum + 1.0 day` threshold, and complete accepted set.

A missing required candidate/plot/year crossing retains all affected records
and makes the candidate objective positive infinity. No midpoint substitution,
interpolation, plot broadcast, plot weighting, or state carry exists.
Crossings and observation joins retain
`candidate_id,plot_id,lane_index,year,record_id`. Both reconstructors emit and
exactly compare the crossing, observation, annual, diagnostics, objective,
failure, and accepted-membership ledgers. Objective and threshold values use
17-digit round-trip decimal serialization before freeze.

`candidate-diagnostics.csv` remains one canonical row per candidate. It
aggregates species RMSE across all admitted records, the overall observation
median absolute distance, the median of within-year absolute-distance medians,
overall interval coverage, failed-record count, and unique failed-year count.
Plot identity is retained in the crossing and observation ledgers rather than
substituting a plot-level diagnostics schema for this canonical summary.

Primary crossing, observation, annual, and candidate-diagnostics components
live under `/home/workdir/cal04b-objects/primary`; candidate, failure, and
accepted-membership ledgers live in the package artifacts directory. The
verification command receives these as distinct `--primary-components` and
`--primary-ledgers` roots and records both in its exact command receipt.

## External Object Retention

Large raw/compressed traces live under `/home/workdir/cal04b-objects/`, outside
Git. `execution-inventory.csv` and `input-and-authority-manifest.csv` retain
absolute path, bytes, SHA-256, command, source/config identity, and rebuild
procedure. Missing objects invalidate reconstruction and package closure.

## Observed Execution Ledger

`executor-command-plan.csv` is prospective authority only and cannot prove that
a command ran. `observed-command-contract.csv` binds every planned command to
prerequisite receipt IDs and exact output paths. Commands are launched through
`tools/observe.py run --command-id <id> -- <exact frozen argv>`.

The runner compares the observed token vector and working directory with the
frozen plan before launch. It exclusively creates durable stdout, stderr,
output-hash manifest, and receipt objects under
`/home/workdir/cal04b-objects/execution-ledger/`. Each receipt records source
and control hashes, exact observed argv JSON, environment delta, UTC start/end,
elapsed nanoseconds, exit status, and hashes for logs and declared outputs.
Existing objects or a missing/non-PASS prerequisite fail closed.

`tools/observe.py render` derives `command-log.csv` and
`execution-inventory.csv` from validated receipts. The immutable
`pre-freeze-snapshot.csv` is created after `summarize_pre_freeze` and frozen
with every receipt, log, and output manifest it names. No terminal artifact is
synthesized from the prospective plan.

## Saturation

`saturation-evidence.csv` uses the exact schema in `CTRL-SAT-01`. Completeness
is `9,261 × 3 = 27,783` rows.

## Later Stages and Propagation

`later-stage-membership.csv` is a four-row checksum index over external
stage-specific membership and parent-result ledgers. Each external membership
ledger has
`stage_member_id,design_id,gsi_candidate_id,bf_max_kg_m2,`
`structural_biomass_kg_m2,evergreen_fraction,xmxlai_m2_m2,`
`structural_cover_fraction,bb_m2_kg,parent_stage_member_id,`
`parent_membership_sha256,state`.

The index records each external path, row count, SHA-256, parent SHA-256,
design SHA-256, and `PASS` state. Only accepted rows from empirical
`EMP-BFBS-01` become BFBS parent membership.
`REC-BFBS-01` rows are synthetic-readiness evidence and never propagate as
empirical parameters. Every later empirical/readiness row retains the full
ordered empirical parent identity and parent-ledger checksum.

`later-stage-results.csv` has
`result_id,design_id,stage_member_id,evidence_role,operand_values,`
`observation_or_truth,objective_components,objective,sensitivity,`
`boundary_flags,failure,accepted_or_recovered,equifinal_set,evidence`.

## Holdout Isolation

No calibration or readiness command accepts a Harvard path. The holdout command
is the only program allowed to resolve Harvard content paths. Its output schema
contains per-observation interval distances, per-year components, species RMSE,
observation- and year-level median absolute distance, interval coverage,
failed-record/year counts, and aggregate score for the frozen accepted
membership. It cannot write candidate membership or calibration objectives.

The narrative `holdout-opening-record.md` may exist while sealed. The exclusive
token `holdout-opened-once.lock` must not exist before the holdout command.
