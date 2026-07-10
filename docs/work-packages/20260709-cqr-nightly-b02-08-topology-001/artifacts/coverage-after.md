# Coverage After

Command:

`CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-cov4 cargo llvm-cov --workspace --test topology_graph_validation_gate --lcov --output-path /tmp/openwepp-cqr-b02-t08-final4.lcov`

Result: PASS, 13/13 tests passed under coverage.

Evidence:

- LCOV: `/tmp/openwepp-cqr-b02-t08-final4.lcov`
- LCOV SHA-256:
  `7bf6b81aefe36bc8dada1991aef481e380472684372520e45136ed48686a4692`
- llvm JSON export command:
  `$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's|host: ||p')/bin/llvm-cov export -format=text -instr-profile=/tmp/openwepp-cqr-b02-t08-cov4/llvm-cov-target/openWEPP.profdata /tmp/openwepp-cqr-b02-t08-cov4/llvm-cov-target/debug/deps/topology_graph_validation_gate-52d27325d89396ce > /tmp/openwepp-cqr-b02-t08-final4-coverage.json`
- llvm JSON export:
  `/tmp/openwepp-cqr-b02-t08-final4-coverage.json`
- llvm JSON SHA-256:
  `2d9e0ef09ceeba5bf534678368dc19b73eeaa0fdce2201aa9d40ae23c28809d8`

Target source coverage:

| Metric | Covered | Total | Percent |
|---|---:|---:|---:|
| Lines (`/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`) | 710 | 746 | 95.1743% |
| Regions (`/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`) | 841 | 874 | 96.2243% |
| Functions (`/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`) | 70 | 72 | 97.2222% |

Notes:

- The LCOV export for this focused command reports `BRF:0`/`BRH:0`; region
  coverage is therefore taken from the Rust toolchain `llvm-cov export` JSON.
- The package is glue-tier for ADR-0021 purposes: parser/validation wiring with
  typed fail-closed status, no science formula or conservation output changes.
