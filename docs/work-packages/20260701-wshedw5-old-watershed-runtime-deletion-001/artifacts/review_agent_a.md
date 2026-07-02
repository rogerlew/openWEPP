# Review Agent A

Status: `executed`

Evidence mode: `static + ran`

Reviewer: `rust_code_reviewer`

Findings:

1. `HIGH`: protected WS11/WS12 science coverage deleted without typed-route
   replacement.
2. `MEDIUM`: WS12 projection errors collapsed non-finite projection failures
   into domain violations.
3. `MEDIUM`: stale chan.inp/channel runtime-input error taxonomy remained after
   deleting old producers.

Disposition:

- Accepted and fixed by expanding
  `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` to 8 typed
  tests covering WS11 branches, WS18/WS20 transport-capacity sensitivity, WS12
  inactive/active behavior, and WS12 projection guard taxonomy.
- Accepted and fixed by mapping `WatershedRuntimeInputError` non-finite/domain
  variants to corresponding WS10 impoundment guard classes in `direct.rs`.
- Accepted and fixed by trimming `WatershedRuntimeInputError` to live WS12
  impoundment projection variants only.

Focused reruns after fixes:

- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`: `8 passed`
- `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`

Review focus: old-runtime deletion completeness, source guards, test backfill,
and production consumer-path proof.

Findings must use `accepted`, `rejected`, `deferred`, or `follow-up`
disposition.
