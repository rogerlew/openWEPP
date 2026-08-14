# Review Agent A

Status: `GO / bounded E19 ordering remediation`

Evidence mode: `not-run`

The original placeholder is superseded by the current bounded review below.

## 2026-08-13 Fresh E19 Rust Correctness Review

Evidence mode: `Static + Ran`

Initial disposition: `HOLD`. The first corrected allocator still rejected a
valid receipt when binary64 `internal_use + external_use` rounded one ULP above
`final_total_demand`. Neither canonical contract admits that aggregate
ordering guard; SC-VEGETATION-001@11 instead defines
`eta=min(1,Nused/Ndem_final)`.

The finding was accepted. The guard was removed and an exact adjacent-bit
regression now proves the canonical eta branch, `eta=1`, zero unsupported NSC,
and no alteration of either finalized-use operand.

Final disposition: `GO`. No remaining correctness finding. The review confirms:

- neither SC-VEGETATION-001@11 nor SC-BIOGEOCHEM-001 requires
  `Ndem_final<=Ndem_pot`;
- potential requests remain immutable and one global authorization occurs;
- `Fext` and every typed `F_N` remain within `F<=A<=D`;
- receipt-bound growth reconstructs final demand, debits internal use once,
  consumes external finalized use once, allocates six tissues with one eta,
  and retains unsupported carbon in NSC;
- no tolerance, clamp, request inflation, reauthorization, or second ordering
  guard remains;
- candidate work is clone-isolated and the public multi-owner candidate/commit
  remains fail-closed.

Ran on final reviewed bytes: vegetation 215/215, implementation contract
13/13, strict vegetation all-target Clippy, formatting, and diff hygiene all
passed. `carbon_nitrogen.rs` is 2,214 lines (WARN, below 3,000).
