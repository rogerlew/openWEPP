# Independent Rust review: frozen-vector parity

Evidence class: `Static + Ran`

Reviewed commit: `646e95b40`

Verdict: `PASS`

Findings: none.

The reviewer confirmed that `LeafGasEnvironment` copies the existing
authority, pressure, and ambient-CO2 operands into all four production calls;
the refactor changes no equation, branch order, constant, guard, error, or
solver tolerance. The test reads the committed vectors, invokes the production
leaf routine, checks categorical parity, and compares `Ci`, `Ag`, `An`,
reconstructed `Rd`, and `rs` under the already admitted representation rule.
Both signed-zero inputs reach `ExactZeroPar`; the independent calculator and
definition/contract hashes remain separately bound.

Ran by the reviewer:

- focused Rust parity: PASS, 1/1;
- V10 authority/regeneration target: PASS, 3/3, Nextest run
  `13250fbb-d574-4e21-bdca-4f941eb2b872`;
- `git show --check 646e95b40`: PASS.

Optional non-blocking hardening was to assert parsed signed-zero input bits and
inactive exact-zero result bits explicitly. The reviewer found the current
evidence sufficient and identified no correctness blocker.
