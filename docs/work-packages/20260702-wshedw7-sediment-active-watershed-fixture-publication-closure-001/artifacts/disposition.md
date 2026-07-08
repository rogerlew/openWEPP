# Disposition

Status: `EXECUTED-COMPLETE-W7R-SEDIMENT-ACTIVE-PUBLICATION-CLOSURE`

Evidence mode: `Ran:` release fixture runs, focused test, fixture manifest, and
local gates; `Static:` source-path review.

## Summary

W7 resumed and closed on current main through W7R.

What changed:

- Adopted committed fixture
  `tests/fixtures/watershed/p102-sediment-active/`, a complete one-channel
  watershed wrapper around the real W7DC01 p102 two-OFE sediment producer.
- Added focused guard
  `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`.
- Proved release `openwepp-cli-watershed` runs the fixture with `--jobs 1` and
  `--jobs 4`, generating production HBP/pass artifacts through
  `openwepp-cli-hill`.
- Proved decoded schema and row identity across all 14 required public parquet
  outputs.
- Reconstructed public detachment/deposition from the generated HBP latest
  event payload and rejected zero-fill and detachment-minus-deposition aliasing
  for routed `sed_del`.

Historical note: the original 2026-07-02 W7 execution held because inspected
fixtures emitted zero production HBP sediment. Later E.3/E.4 work closed that
producer-side blocker on the W7DC01 p102 substrate. W7R is the current-main
resume and closure evidence.

## Release Evidence

- Current commit under test: `97b23132b85c579041dee5de530d0b5aa319fbd7`.
- `target/release/openwepp-cli-hill`
  SHA-256: `e88c5552f6fa98fae4282eb87095fb271a8dd5c0cf30a97431a483c46a8694e7`.
- `target/release/openwepp-cli-watershed`
  SHA-256: `160f7f5d54d5aef4a1d2c12d82ada09f9326c2a6cf60840bf6882766675e6996`.
- Serial release run:
  `target/release/openwepp-cli-watershed --run-dir tests/fixtures/watershed/p102-sediment-active/runs --run-file case.run --output-dir /tmp/wshedw7r_p102_fixture_jobs1 --policy compat --jobs 1 --hillslope-binary target/release/openwepp-cli-hill`
  completed: `wall=0:00.78`, `maxrss=20516`.
- Parallel release run:
  same command with `--jobs 4` and
  `/tmp/wshedw7r_p102_fixture_jobs4` completed: `wall=0:00.74`,
  `maxrss=20492`.

## Public Sediment Result

`/tmp/wshedw7r_p102_fixture_jobs1/interchange/totalwatsed3.parquet`:

- rows: `1`
- `tdet = 584.2332653870001 kg`
- `tdep = 282.14618621700004 kg`
- `tdet - tdep = 302.08707917000004 kg`
- `sed_del = 0.08391307754719238 kg`
- `runvol = 2329.7636065586953 m^3`
- `Runoff = 63.1600928292383 mm`

Generated pass parquet:

- rows: `3652`
- `sum(tdet) = 41531.85795763501 kg`
- `sum(tdep) = 29195.4647928195 kg`
- `sum(tdet - tdep) = 12336.39316481551 kg`
- all five `sedcon_*` sums are nonzero.

## Closure

All W7R acceptance claims are closed. No surrogate sediment physics, manual pass
edits, watershed/channel routing changes, or publication schema changes were
introduced.
