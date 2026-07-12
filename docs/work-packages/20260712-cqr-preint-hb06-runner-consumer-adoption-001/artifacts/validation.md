# Terminal Validation

Evidence class: **Ran + Static**

| Gate | Result | Evidence |
| --- | --- | --- |
| Summary accumulator focused tests | PASS | `cargo nextest run -p openwepp-summary-accumulator --profile quick`: `20/20` |
| Runner typed-adapter tests | PASS | Three selected direct-publication WAT tests: `3/3` |
| Producer alias-boundary test | PASS | Deliberate unequal aliases reject; canonical equal aliases accept: `1/1` |
| SIMIMPL04 executable consumer | PASS | The production streaming sink emitted `H5.wat.parquet`; readback proved exact schema order/type/nullability, two ordered rows, simulation-year keys, `Q == QOFE`, and `Total-Soil == SoilWaterTotal`. |
| SIMIMPL04 executable contracts | PASS | `5/5` across the three SIMIMPL04 binaries before the focused readback rerun. |
| Focused coverage | PASS | Lines `1144/1157` (`98.876%`), regions `1589/1651` (`96.245%`), functions `75/75`. |
| Function floor | PASS | `46` eligible production functions scored by `cargo-crap`; zero below `75%` coverage. |
| CRAP | PASS | Zero functions above `30`; worst is `Wb13DailyWaterBalanceRow::from_surface`, CC `28`, coverage `100%`, CRAP `28`. `from_input`: CC `7`, coverage `100%`, CRAP `7`. |
| Format, Clippy, diff | PASS | `cargo fmt --all --check`; scoped all-target Clippy with `-D warnings`; `git diff --check`. |

## Coverage Commands

    cargo llvm-cov clean --workspace
    cargo llvm-cov -p openwepp-summary-accumulator --lib --json --output-path .../coverage/hb06-adoption-final.json
    cargo llvm-cov -p openwepp-summary-accumulator --lib --lcov --output-path .../coverage/hb06-adoption-final.lcov
    cargo crap --path crates/openwepp-summary-accumulator --lcov .../coverage/hb06-adoption-final.lcov --threshold 30 --fail-above --format json --output .../coverage/hb06-adoption-final-crap.json

## Hashes

| Artifact/source | SHA-256 |
| --- | --- |
| Summary source | `20ed6937dff93a7422ad0ab661fccadb053930b46026f67ea5df47f00774c131` |
| Runner adapter source | `be24dc346a6cc2ad52f8848a07551acdbaf51df7ebd334ef8d73b9fcd087d78e` |
| Producer boundary source | `f695b655fddddb02a0a934aa67bda277ea4cca55e785f0595c48df98e3785c76` |
| Coverage JSON | `deda241cda2f76020f435926e458cf608c955f8bd1733d50baee14c9300cbec5` |
| Coverage LCOV | `688120f6803438bf72ac74715825e5cf0369641757c73939fd9d8cd0f5e60cd4` |
| CRAP JSON | `434f662a776437bce7d0a2d6ce14ab36d242ef00e56ea91273e6d861385a4138` |

The summary source is `1,767` lines, below the `2,000`-line WARN threshold.

