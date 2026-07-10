# Coverage After

Evidence label: Static/Ran.

Status: `EXECUTED`

Sources:

- LCOV: `/tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov`
- JSON: `/tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-llvmcov.json`

Commands:

- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov`
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --ignore-run-fail --json --output-path /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-llvmcov.json`

Command result:

- PASS, `82` lib tests passed for each coverage run.

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`

Final target coverage:

- Lines: `1331/1373` (`96.94100509832484%`)
- Regions: `1348/1399` (`96.35453895639743%`)
- Functions: `45/45` (`100.0%`)
- Instantiations: `46/73` (`63.013698630136986%`)
- Branches: `0/0`

Artifacts:

- LCOV bytes: `306078`
- LCOV sha256:
  `d8d5dfd4cb260b91b0d4985181c56c4962594d4f7dd654e67c7d1299078cce1a`
- JSON bytes: `12066080`
- JSON sha256:
  `75e6685ee560fb7257bdfe5ea12e20d3047749459d51706189088a400d909c25`
