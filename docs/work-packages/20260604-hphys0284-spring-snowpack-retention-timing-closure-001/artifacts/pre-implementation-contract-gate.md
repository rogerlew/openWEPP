# Pre-Implementation Contract Gate

Status: complete
Evidence mode: Ran

## Ran: Contract Test Before Production Edit

Command:

```bash
cargo test --test hphys0284_negative_melt_snowpack_state_contract -- --nocapture
```

Result: failed as expected before production code changed.

Failure excerpt:

```text
runtime SWE must apply corrected legacy carried-depth state loss; runtime_swe=0.33736604, expected=0.32989843999999996, routed_melt=0.012633959999999994, raw_positive=0.016367760000000002, raw_negative=-0.0037338000000000002
```

## Gate Disposition

- Red gate confirmed the pre-fix runtime recomputed snowpack SWE from routed net melt alone.
- Production edit was made only after canonical contract amendments and this red gate.
