# Review Agent A Second Repeat Review

Status: `HOLD / one material oracle finding remains`

Evidence mode: `Static + Ran`

Review role: independent canopy-interception/topology/energy science reviewer.

This review reassessed the complete current Stage-A authority and acceptance
envelope, including every historical Agent-A finding and both first-rereview
findings. Historical review artifacts were not modified.

## Evidence Run

- Independent Python oracle regeneration: PASS; output is byte-identical to the
  committed fixture, SHA-256
  `e487413142c463a81a4e29d4887cdf4fa339eadeaeeda0a4cf92ffbf2ceb76a7`.
- Both V2 definition copies: byte-identical canonical JSON, recursively lexical
  object keys, SHA-256
  `b2b01f965f83a52f4c800c489079c88d97179ed6a8191734b541115308b97a5c`.
- Historical V1 definition: unchanged SHA-256
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Shared transaction contract: SHA-256
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`.
- Vegetation authority suite: PASS `14/14`.
- Focused authority-test Clippy with `-D warnings`: PASS.
- Authority anti-evasion: PASS.
- AUTH11: PASS `3/3`.
- Unit compliance: PASS for `SC-VEGETATION-001` and
  `SC-VEGETATIONTRANSACTION-001`.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.

Science admission was not rerun as a passing gate because the proposed V2
contracts correctly remain `in_review/draft` until review, disposition, and
verification finish.

## Finding Reassessment

| Finding | Second repeat-review status | Assessment |
|---|---|---|
| `A-CRITICAL-001` | canonical authority corrected | `SC-VEGETATION-001.md:563`--`580` exactly binds the potential column, immutable arbitration, and final capped rebuild from beginning state. |
| `A-CRITICAL-002` | corrected | The digest-bound shared transaction contract now supplies cross-owner occupancy, basis, independent reconstruction, and atomicity authority. |
| `A-HIGH-003` / `A-RR-HIGH-002` | corrected | `SC-VEGETATION-001.md:180` and `:601`--`:615` consistently use `mm H2O`, reject MPa, and select recursive lexical state serialization. The fixture exposes exact serialized state bytes and digest, and Rust reconstructs both. |
| `A-HIGH-005` | corrected | V2 and the new transaction contract remain `in_review/draft`; V1 remains the approved historical identity until promotion. |
| `A-RR-HIGH-003` | corrected | The authority test was decomposed; focused Clippy now passes with warnings denied. |
| `A-HIGH-004` / `A-RR-CRITICAL-001` | materially improved but still open | The same lower stratum now occurs in both heterogeneous columns; routing and resource swaps execute typed rejection; rollback includes vegetation state/warm starts, water, BGC, energy, and transaction bytes; N arbitration is keyed by layer/species; state serialization is exact; and Rust independently reconstructs substantially more fixture structure. The remaining defect is below. |

## Remaining Finding

### A-RR2-CRITICAL-001: The authorization-sensitive “coupled” oracle still introduces substitute physics

The canonical V2 order requires the complete admitted E11--E15 local coupled
solve during both potential and capped column passes
(`docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md:563`--`580`).
The independent fixture instead defines
`coupled_vapor_under_cap()` as
`potential_vapor - 0.36 * (1 - authorization_fraction)` at
`artifacts/reference_calculator.py:119`--`128`. The coefficient `0.36` and this
linear cap-to-vapor response are not in V1, V2, the numerical contract, the V2
definition, or cited scientific authority. The function's docstring calls the
result the independently expected final coupled-solver response, and that value
drives final upper condensation/second drainage and descendant incident at
`artifacts/reference_calculator.py:206`--`222`.

This is exactly the class of proxy response the package prohibits. It proves
that manually changing an upper vapor scalar changes a descendant, but it does
not prove the admitted potential-request/arbitration/capped-E11--E15/final-E04
path. The resulting fixture can agree with an implementation that uses the same
invented linear response while the canonical coupled solver is absent or wrong.

The nonlinear locality evidence has a related authority problem. The supposed
wet-energy poison applies a second arbitrary `2/3` power to the already-derived
E04 wet fraction at `artifacts/reference_calculator.py:313`--`322`; it is not an
admitted wet-energy residual. The FvCB helper at
`artifacts/reference_calculator.py:131`--`143` takes a direct `min` of limits
and omits the two admitted co-limitation quadratics, yet its docstring calls it a
digest-bound V1 FvCB response. These comparisons demonstrate generic
nonlinearity, not conformance to the digest-bound nonlinear consumers claimed by
the fixture ledger.

Rust independently verifies that the produced numbers differ at
`tests/integration/vegetation_boundary_authority_contract.rs:852`--`881`, but it
does not supply independent correctness authority for the invented response.
Likewise, the water reconstruction sums requests and authorizations and checks
the supply at `tests/integration/vegetation_boundary_authority_contract.rs:820`--`838`,
but does not reconstruct each proportional authorization or prove that final
vapor came from the canonical capped solver.

Scientific impact: Stage A would release a digest-bound implementation oracle
containing an uncited constitutive shortcut precisely at the load-bearing join
that caused the original HOLD. Passing local and stand water closure does not
make that response scientifically admissible.

Disposition recommendation: `accepted`. Remove the linear cap-to-vapor helper
and the extra wetness response. Either:

1. evaluate the exact already-admitted independent E11--E15 oracle for the
   potential and capped occupancy states, including its exact FvCB
   co-limitation, energy, hydraulic complementarity, and diagnostics; or
2. if Stage A intends only to test routing, label vapor as an explicit
   externally supplied controlled operand and do not claim it validates the
   coupled authorization response. In that case, retain the canonical ordering
   as static authority and move the complete coupled-result acceptance vector
   to a named Stage-B gate without calling the Stage-A scalar response
   digest-bound physics.

For the locality poisons, use exact admitted E04 wet fraction/store operands and
the complete existing independent FvCB equations, or narrow the claims to the
specific algebra actually executed. Rust should reconstruct each proportional
authorization by exact key and compare the final coupled response to the
authority-backed expected vector.

## Final Recommendation

`HOLD`

The topology selection, state and migration schema, units, serialization,
owner authority, V1/V2 identity, lifecycle posture, rollback envelope, and
focused code-quality gate are now sound. A material scientific-acceptance defect
remains because the authorization-sensitive and wet/FvCB locality fixture uses
uncited substitute response math while claiming digest-bound coupled behavior.
Stage A should not release implementation authority until that claim is either
backed by the exact admitted oracle or truthfully narrowed.
