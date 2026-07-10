# Coverage After

Evidence label: Static/Ran.

Status: `EXECUTED`

Targeted coverage source:

- LCOV: `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted.lcov`
- JSON:
  `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-llvmcov.json`

Commands:

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p openwepp-runner --lib --lcov --output-path /tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted.lcov -- laned_shadow`
  - PASS, `15` passed, `83` filtered.
- `cargo llvm-cov -p openwepp-runner --lib --json --output-path /tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-llvmcov.json --no-clean -- laned_shadow`
  - PASS, `15` passed, `83` filtered.

Artifacts:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| LCOV | `507904` | `e09a39365ce1413bb9bfdcbbf70bc4a7a3a02536c34e126e51fba50d2bf4ecd7` |
| JSON | `5339448` | `0e9dcedd6889b63c49578543e38b9cb0e78ed769b9748a2ee3536f6e8a99f31b` |

Final target coverage:

- Lines: `684/699` (`97.85407725321888%`)
- Regions: `842/877` (`96.00912200684151%`)
- Functions: `47/52` (`90.38461538461539%`)
- Instantiations: `47/52` (`90.38461538461539%`)
- Branches: `0/0`

Production/test split at `#[cfg(test)]` (`laned_shadow.rs:578`):

- Production lines: `321/330` (`97.27272727272728%`)
- Production regions: `406/437` (`92.90617848970251%`)
- Test lines: `348/349` (`99.7134670487106%`)
- Test regions: `430/434` (`99.07834101382488%`)

Baseline comparison:

- Baseline lines: `251/452` (`55.53097345132743%`)
- Baseline functions: `23/39`
- Final lines: `684/699` (`97.85407725321888%`)
- Final functions: `47/52`
