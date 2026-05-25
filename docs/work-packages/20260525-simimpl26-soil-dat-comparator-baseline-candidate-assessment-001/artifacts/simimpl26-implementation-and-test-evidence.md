# SIMIMPL26 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Production code changes: none.
- SIMIMPL26 write activity was limited to work-package artifacts.

## Ran
Required package gates executed:
1. `cargo test -p openwepp --test infile_soil_parser_contract`
   - result: pass (`8 passed; 0 failed`)
2. `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
   - result: pass (`8 passed; 0 failed`)
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass (warnings only)
   - warnings observed:
     - duplicate lock entries: `getrandom`, `hashbrown`, `twox-hash`
     - license-not-encountered allowances: `ISC`, `Unicode-DFS-2016`
     - summary line: `advisories ok, bans ok, licenses ok, sources ok`

Additional evidence commands:
- soil-file hash/size/header capture commands for PL08 and PL14R lane roots
- `cmp -s` byte-identity checks for comparable `p5.sol` files
- `find` scan for `soil.dat`/`*.sol` coverage in selected lanes
