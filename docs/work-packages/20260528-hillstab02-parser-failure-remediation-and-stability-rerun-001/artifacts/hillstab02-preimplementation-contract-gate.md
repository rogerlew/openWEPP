# hillstab02-preimplementation-contract-gate

Status: complete  
Evidence mode: Ran

## Command
```bash
cargo test --test infile_soil_parser_contract --test infile_management_parser_contract
```

## Gate Result (Pre-Code-Edit)
- Expected: fail (new compatibility behavior not yet implemented).
- Observed: fail at
  `compatibility_mode_accepts_tilseq_zero_when_nseq_nonzero`.
- Observed error:
  `MAN-E-009: DanglingScenarioReference { field: "tilseq", value: 0, max_allowed: 1 }`.

## Conclusion
- Pre-implementation contract gate produced the expected failing signal, so
  production parser edits proceeded under contract-first sequencing.
