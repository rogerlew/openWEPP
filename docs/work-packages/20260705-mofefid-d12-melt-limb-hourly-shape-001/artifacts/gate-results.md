# Gate Results

Status: **COMPLETE**.

Final D12 gates were run after the snow-coupling layout fix, DC01 test split,
and package-boundary artifact update.

| Gate | Evidence | Status | Notes |
|---|---|---|---|
| `git diff --check` | Ran | PASS | No whitespace errors. |
| Markdown lint | Ran | PASS | `markdown-doc lint` on touched docs/package/contract paths: `29` files scanned, `0` errors, `0` warnings. |
| Contract/profile/BEI checks | Ran | PASS-DEFERRED | `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`: `5` BEI rows, `4` science-review-follow-on rows; non-strict pass. |
| Unit-governance applicability | Static | PASS | D12 added no new unit conversion; all added hourly limbs are depths in meters and the pre-existing `/3600 s` rate seam remains unchanged. |
| Focused D12 tests | Ran | PASS | Producer closure, DC01 source shape, R4G nonclosure, size-layout guard, runner dynamic operands, snowdensity opt-in, and H2637 fail-closed guard all passed. |
| H2637/Lane D shadow evidence | Ran | PASS | Popper rerun: `cargo test --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`; `324.83 s`; manifest `/tmp/laned_shadow_h2637_native_on_10668/manifest.json`; `days_uniform_shape_with_routed_melt=0`, `days_uniform_shape_without_routed_melt=6`; protected identity max abs `0.0 mm`. |
| Default-off protected-output identity | Ran | PASS | H2637 native shadow off/on protected HBP and pass parquet bytes unchanged; manifest identity statuses all `pass-published-per-ofe-wb13-records`. |
| `cargo fmt --check` | Ran | PASS | Final post-split run passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Ran | PASS | Final post-split run passed. |
| `cargo nextest run --workspace --profile full` | Ran | PASS | Final run: `1378` tests passed, `2` skipped, `1` slow; `579.374 s`. |
| `cargo deny check` | Ran | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Source-level anti-evasion applicability | Static | NOT REQUIRED | D12 touched H2637 test harness/provenance code but did not change required-case fixture files, authority-suite posture, cohort fixtures, or required-case bindings. |

Prior non-final gate result:

- A delegated full nextest run before the layout fix failed only
  `r7b_constructor_type_size_layout_is_bounded` with
  `DirectDayFrame=15896 > 15456`. D12 removed duplicate hourly routed-melt
  storage from `DirectSnowCouplingState`/`DirectSnowCouplingShadowProjection`
  and boxed the downstream operand vector. The focused size guard now passes
  with `DirectDayFrame=15328`, and the final full nextest gate passed.
