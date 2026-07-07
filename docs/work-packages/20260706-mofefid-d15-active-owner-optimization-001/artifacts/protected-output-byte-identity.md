# Protected Output Byte Identity (D15A-P4, `INV-OFEROUTE-010`)

Status: **EXECUTED**.

Evidence mode: **Ran**.

## Default/off identity across the whole package

The same native-patched H2637 fixture (recipe in `baseline-profile.md`) was
run subsystem-off at three points, all pinned (`taskset -c 4`, release):

| Point in the package | H2637.hbp | H2637.loss.json | H2637.pass.parquet |
|---|---|---|---|
| S0 baseline (pre-optimization, pre-active-owner) | `948faf82…` | `725f5723…` | `f0d1be11…` |
| post-S4 (optimizations landed) | `948faf82…` | `725f5723…` | `f0d1be11…` |
| post-P2 (active owner landed, env UNSET) | `948faf82…` | `725f5723…` | `f0d1be11…` |

Full SHA256 values are in the package logs
(`baseline-profile.md` carries the full digests). The default path is
byte-identical through both the S4 hot-path optimizations and the entire
active-owner integration. Structural reasons (Static): the S4 optimizations
are bit-identical value reuse inside the solver; the executor's default loop
is textually untouched (the active loop is a separate function selected only
when `frame.laned_active` is set, which requires `OPENWEPP_LANED_ACTIVE=1`);
`run_day_spans` split into two halves called back-to-back preserves the exact
span sequence.

## Shadow-mode bit identity (the `route_single_ofe` refactor witness)

Shadow-on (`OPENWEPP_LANED_SHADOW=1`) post-implementation reproduces the S0
baseline exactly: identical protected-output hashes AND a JSON-identical
manifest `laned_shadow` block (all conservation figures, volumes, and day
classes). The cascade refactor that the active path shares is therefore
proven behavior-preserving on the real 2-year fixture.

## Active-mode expectations (negative posture)

Active-mode protected outputs are EXPECTED to differ (the contract-authorized
supersede changes downstream-lane hydrology): `H2637.hbp ddcadc23…`,
`H2637.pass.parquet 88d71e61…` (active), vs the off hashes above.
`H2637.loss.json` is byte-identical off/active because it carries only the
static run summary (climate metadata), not water/erosion surfaces — verified
by inspection. The manifest carries the `laned_active` block ONLY under the
selector; the off-manifest has no active or shadow keys.
