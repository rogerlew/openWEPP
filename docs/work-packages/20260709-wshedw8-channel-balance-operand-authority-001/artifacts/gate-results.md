# Gate Results

Status: `EXECUTED-COMPLETE`
Evidence: `Ran`

## Focused Gates

| Gate | Result | Evidence |
|---|---|---|
| `cargo test -p openwepp-watershed-output typed_publication_writer --lib` | PASS | `2 passed; 0 failed` |
| `cargo test --test wshedw5_typed_watershed_runtime_contract typed_publication_projects_non_aliased_channel_balance_operands -- --nocapture` | PASS | `1 passed; 0 failed` |
| `cargo test --test wshedw5_typed_watershed_runtime_contract` | PASS | `12 passed; 0 failed` |

## Closure Gates

| Gate | Result | Evidence |
|---|---|---|
| `cargo fmt --check` | PASS | exited `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exited `0` |
| `cargo nextest run --workspace --profile quick` | PASS | `1393 tests run: 1393 passed; 26 skipped` |
| `cargo nextest run --workspace --profile full` | PASS | `1468 tests run: 1468 passed; 3 skipped` |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/20260709-wshedw8-channel-balance-operand-authority-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | PASS | `11 files validated, 0 errors, 0 warnings` |
| `git diff --check` | PASS | exited `0` |

## Notes

- The first clippy attempt failed after `RoutedChannelState` grew enough to
  trigger `clippy::large_enum_variant` on `DirectWatershedKernelOutput`.
  Closure boxed the channel variant instead of suppressing the lint.
- The second clippy attempt failed on an exact float comparison added in the
  integration test. Closure replaced it with a tolerance assertion.
