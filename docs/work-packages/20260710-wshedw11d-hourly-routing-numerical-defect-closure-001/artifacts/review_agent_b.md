# Review Agent B

Status: `EXECUTED-PASS-RECOMMENDATION`

Evidence mode: `Static + Ran`

Reviewed at UTC: `2026-07-11T05:32:14Z`

Role: independent parser, integration, publication, and real-consumer review.
This artifact recommends a result; it does not set package disposition.

## Finding summary

| Severity | Count | Result |
|---|---:|---|
| High | 0 | none |
| Medium | 0 | none |
| Low | 0 | none |

Recommendation: `PASS`.

## Static review

### `chan.inp nchnum=0`

- `SC-INFILE-CHANINP-001` lines 65-69 and 155-163 define the conditional
  fourth record and preserve `CHN-E-002` for missing/extra nonempty records.
- `crates/openwepp-input-contract/src/parsers/chaninp.rs:366-390` keeps strict
  rejection and compatibility-default policy explicit. Lines 399-405 admit
  the three fixed records; lines 468-488 derive the record count from parsed
  `nchnum` and construct an empty ID list for zero count. No silent 60-second
  default is taken for canonical three-record input.
- `tests/integration/infile_chaninp_parser_contract.rs:63-117` distinguishes
  strict and compatibility parsed branches from the rejected extra-record
  alias and asserts the requested 600/3600-second grids.

### Terminal selection and extensive publication

- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs:657-665`
  reduces public event runoff over `outlet_channel_ids`, while lines 666-693
  retain all-channel water diagnostics separately.
- Lines 1025-1063 construct terminals by removing only channels consumed by
  another channel. The explicit lines 1035-1047 rule preserves a channel that
  feeds an impoundment as the terminal channel-oriented output.
- Lines 739-800 integrate direct terminal sediment rates to mass and sum only
  terminal masses. Lines 803-833 reconstruct serial channel contributor
  ancestry in dispatch order. Lines 835-919 use the inclusive superposed
  hourly sediment span, and lines 922-998 reproduce the direct-event fallback
  duration from `dtchr`, direct contributors, and dependency duration.
- The unit vector at lines 1339-1384 covers a serial internal channel, two
  independent terminal channels, and the channel-to-impoundment boundary. It
  asserts terminal IDs `{2,3}` and a 360 kg terminal-only mass, numerically
  distinct from including internal channel 1.
- Interval-lane terminal state remains mass-valued; event-scalar state alone
  uses the rate-times-duration reduction. Negative/non-finite mass, duration,
  pair cardinality, or missing ancestry fails through the typed
  `WatershedNetworkFrameError::InvalidTerminalPublication`
  (`network_frame.rs:49-53`, `100-107`, `784-799`, `852-897`).

### Real consumer and protected behavior

- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs:212-235`
  launches the real watershed CLI, then reads EBE Parquet and proves channel
  2, 7,200 m3 terminal water, and 240 kg terminal sediment for the serial
  CREAMS case. Those values reject the old channel-1, 14,400 m3, rate-as-mass
  path.
- Lines 89-135 exercise the canonical three-record zero-count sidecar through
  that same CLI after directly proving `ParsedBranch`, `dtchr=600`,
  `ntchr=144`, empty IDs, and output disabled. The runtime handoff reads those
  normalized fields directly at `network_frame.rs:1124-1144`.
- Lines 137-209 retain the protected W11B two-channel same-grid sediment and
  water-closure consumer checks on the now-authorized KW branch. Lines 238-265
  separately require typed `WKERNEL-WS10-CHANNEL-E-003` rejection for every
  active W11C static/variable MC grid, so changing the protected success vector
  does not hide the newly authorized MC disposition.
- The write set remains inside the package envelope. Production changes add no
  network access, dependency, shell interpolation, `unsafe`, secret handling,
  or untyped error wrapper. CLI tests continue to use `Command::new` with
  explicit arguments (`mt3_hbp_hourly_consumer_contract.rs:976-999`).

## Ran evidence

| Command | Result |
|---|---|
| `cargo nextest run --test infile_chaninp_parser_contract --no-fail-fast` | PASS, 20/20 |
| `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d --no-fail-fast` | PASS, 3/3 |
| `cargo nextest run -p openwepp-watershed-orchestrator --no-fail-fast` | PASS, 107/107 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract --no-fail-fast` | PASS, 6/6 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11d_creams_serial_publication_uses_terminal_extensive_outputs --no-fail-fast` | PASS, 1/1 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11d_cli_accepts_three_record_zero_count_chaninp_without_defaulting --no-fail-fast` | PASS, 1/1 |
| `git diff --check` | PASS |

These reviewer runs used Cargo's test-built CLI binary. Release-binary
provenance and full-workspace closure remain the package-authorized heavy
runner's separately recorded gates; they are not relabeled as reviewer runs.

## Review conclusion

Static and executed evidence agree: canonical zero-count input retains its
routing timestep; malformed closure remains typed; serial internal flow is not
published twice; independent/channel-before-impoundment terminals are retained;
direct sediment is a duration-integrated mass; and the actual CLI consumes the
corrected publication path. No H/M/L finding blocks final disposition.
