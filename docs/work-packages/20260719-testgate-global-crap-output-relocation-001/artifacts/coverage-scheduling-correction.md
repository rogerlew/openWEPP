# Coverage Scheduling Correction

Evidence class: `Ran` unless marked `Static`.

## Reproducer

The preserved exact terminal receipt passed the ordinary full workspace and
failed only the fresh LLVM-coverage run. That run started all 25
`assurance_v2_publication_contract` cases together. The three heaviest matching
cases took `505.237s`, `567.956s`, and `584.723s` without instrumentation, then
`719.563s`, `720.009s`, and `720.013s` with instrumentation. The latter two
timed out at the unchanged 720-second full-profile ceiling.

## Correction

`.config/nextest.toml` now binds the complete publication contract binary to
the named `assurance-publication` group. The group has eight resource slots and
each matching case reserves two, permitting at most four matching cases at
once. The override contains no timeout, retry, or selection setting, so each
profile retains its existing behavior. `profile.full` continues to inherit
`default`, select `all()`, and use `90s * 8`.

Static: no runner, adapter digest, coverage command, CRAP threshold,
adjudication registry, or assurance-publication test implementation changed.
One 21-line TESTGATE scheduler-policy regression brings that integration
contract to 579 lines, below the 2,000-line warning threshold.

## Focused Evidence

- `cargo fmt --check`: PASS.
- `cargo nextest run --test testgate_ci_executor_contract`: PASS, 4/4.
- `cargo clippy --test testgate_ci_executor_contract -- -D warnings`: PASS.
- `cargo nextest show-config test-groups --workspace --profile full --groups
  assurance-publication --no-pager`: PASS; the exact binary filter resolves all
  25/25 publication cases to the group.
- Full-profile JSON inventories generated with the authority-base Nextest
  config and the corrected config are byte-identical under `cmp`: PASS. The
  current source tree lists 2,175 cases total, including the newly added
  scheduler regression, with five configured skips.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`:
  PASS, 3/3.
- `git diff --check`: PASS.

The expensive instrumented publication binary was not run as a separate
focused rehearsal. Doing so would duplicate the exact fresh-coverage terminal
gate without producing terminal receipt authority. The next committed exact
plan is the acceptance run for the four-case cap.
