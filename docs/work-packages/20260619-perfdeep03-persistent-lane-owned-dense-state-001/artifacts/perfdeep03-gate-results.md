# PERFDEEP03 Gate Results

Evidence class: Ran.

## Required Rust Gates

Ran:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: passed.

Ran:

```text
cargo test --workspace
```

Result: passed.

Ran:

```text
cargo deny check
```

Result: passed (`advisories ok, bans ok, licenses ok, sources ok`).

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

## Focused Tests

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator \
  perfdeep03_ofe_sequence_uses_lane_owned_compact_dense_state -- --nocapture
```

Result: passed.

Ran:

```text
cargo test -p openwepp --test mofe01_per_ofe_state_contract
```

Result: passed after updating the structural source-path list for the
`scheduler/water_balance.rs` split.

## Package Gates

| Gate | Result | Evidence |
|---|---|---|
| Lane runtime owns persistent dense frame | PASS | `HillslopeLaneDenseState`, carried through `OfeLanePersistentState` |
| Hot working set, not full registry, is carried | PASS | compact state/flux symbol tables and slot views |
| No default activation | PASS | requires `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1` |
| H2637 HBP/WAT identity and PASS Arrow equivalence | PASS | `perfdeep03-identity.md` |
| Roundtrip zero-mismatch preserved | PASS | 235961 diagnostic rows, no mismatch rows |
| Real opt-in H2637 endpoint beats `669.97 s` | FAIL | measured `1147.96 s`, `229580 KB` |
| Default endpoint flatness | FAIL | measured `697.36 s` and `707.80 s`; identity passed but flatness not proven |

Because the load-bearing endpoint gate failed, PERFDEEP03 closes as `NO-GO`
rather than `CONTINUE`.

## Markdown Gate

Ran:

```text
markdown-doc lint \
  --path docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001 \
  --path docs/work-packages/README.md \
  --path docs/ROADMAP.md \
  --path docs/architecture/array-native-runtime-specification.md \
  --format plain
```

Result:

```text
10 files validated, 0 errors, 0 warnings
```
