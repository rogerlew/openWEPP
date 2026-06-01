# AUTH12 Preimplementation Contract Gate

Status: complete  
Evidence mode: Ran

## Red-State Capture (Before Production Fix)

Command:

```bash
cargo test --test auth07_fc_authority_cohort_contract
```

Observed failure (pre-fix):

- `valid_9002_reference threshold status mismatch`
- expected: `within`
- observed: `exceeds`
- `rel_err=0.5316351316472674` (earlier capture), threshold `0.35`

This red-state capture was recorded before AUTH12 production runtime changes.
