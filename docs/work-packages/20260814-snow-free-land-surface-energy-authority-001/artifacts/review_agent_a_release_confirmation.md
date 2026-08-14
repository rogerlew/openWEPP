# Release Land-Surface Science Confirmation Review

Evidence class: `Static + Ran`; fresh independent confirmation against the
frozen candidate bytes listed below. Prior failed reviews were read as
historical evidence, not reused as acceptance evidence.

Verdict: **NO-GO / FAIL**.

The `A5-HIGH-001` remediation is correct on these bytes. The diagnostic schema
now enforces the canonical `LSEB-E-030`/unsupported-domain and
`LSEB-E-034`/numerical pairings in both directions, all forty incompatible
code/kind pairing poisons reject, and all eleven emitted failures use the
canonical pair. The earlier frozen-cap defect is also corrected. One
load-bearing energy-conservation defect remains: the accepted infiltration
enthalpy receipt is not applied to the receiving soil node's ending state.

## Exact Reviewed Bytes

- LSE definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- top-level calculator:
  `9c1dc79631af4840d69f04a619e0e5d6bcbd9210b70a0e669e58f57ec89cae79`;
- exact joint core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- frozen vectors:
  `ad2e3e7e39a594e297eaf9f4c5d4549fa27f531f9edb7e9bba1c5197d43282da`;
- diagnostics schema:
  `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`.

The reviewed `SC-LANDSURFACEENERGY-001.md` SHA-256 was
`67b51fde024e85668d1bb605bbb54fd58ea6b7a0e798b68db1293ebbb93a0a62`.
The immutable V8 definition remained
`622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`.

The calculator began changing for the accepted finding after this review
snapshot was frozen. This report is historical evidence for the exact hashes
above and does not review those later bytes.

## Commands And Independent Regeneration

Ran exactly one independent generator process to completion in the absolute
temporary directory
`/tmp/lse-child1-science-confirm-QGgMRF`:

```text
.venv/bin/python .../reference_calculator.py \
  --write /tmp/lse-child1-science-confirm-QGgMRF/vectors.json
sha256sum /tmp/lse-child1-science-confirm-QGgMRF/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
cmp -s /tmp/lse-child1-science-confirm-QGgMRF/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
```

Both files had SHA-256 `ad2e3e7e...`; `cmp` returned zero. A mistakenly
started duplicate retry was terminated while the original process was still
running and produced no fixture; it is not science evidence and did not alter
the completed one-run comparison.

Ran:

```text
cargo nextest run \
  --test land_surface_energy_balance_authority_contract \
  --profile quick
```

Result: **7 passed, 0 skipped**.

## Confirmed Corrections And Science Coverage

### A5 semantic failure pairing

The eleven failures now emit:

```text
calm/frozen/missing-enthalpy/multiple-class/nonneutral/snow/
snow-terminal/thawing -> LSEB-E-030 / unsupported_domain

singular              -> LSEB-E-034 / singular_pivot
backtracking limit    -> LSEB-E-034 / backtracking_limit
iteration limit       -> LSEB-E-034 / iteration_limit
```

The Draft 2020-12 schema makes both implications conditional and binding:
`unsupported_domain -> E-030`, `E-030 -> unsupported_domain`, numerical kinds
`-> E-034`, and `E-034 ->` a numerical kind. The fixture executes all forty
wrong-pair poisons (ten wrong codes for each of four failure kinds), and each
has `accepted=false`, a null candidate, and a typed schema failure. The other
thirty-two component/protocol/owner poisons also reject, giving 72/72 rejected
poisons overall.

### A4 frozen equality cap

The exact two-rank review probe retains the frozen ground cap
`0.00015581562596770875 kg m^-2 tile-ground s^-1` in both the multirank and
one-rank frozen evaluations. The perturbed unfrozen law is
`0.00015581538846473828`, while both frozen results remain bit-identical to
the cap. The frozen branch reaches the reconstructed first-rank surface row,
centered finite differences, and line search.

### Complete selected stack

The regenerated fixture executes 22 mandatory exact records spanning the V8
two-stream lower boundary, reciprocal arbitrary-rank longwave, covered and open
neutral turbulence, shared canopy-air heat/vapor feedback, bare mineral soil,
dry/wet forest litter, signed evaporation and condensation, surface enthalpy,
arbitrary-layer Crank--Nicolson soil heat, beginning-snapshot water
authorization, fixed-cap rebuild, post-solve liquid advection, active caps,
alternate starts, and natural singular/backtracking/iteration failures.

The fixture independently records one authorization, rebuilding potential and
final passes from the same beginning hash, exact zero post-ingress mass
residual, `-4.656612873077393e-10 J m^-2` post-ingress energy residual, exact
failure rollback, and rejection of every poison. The earlier A, A2, A3, A4,
and A5 radiation, turbulent-transfer, surface-state, numerical, active-set,
failure-diagnostic, and poison defects are corrected on the reviewed bytes,
subject to the material owner-state conservation defect below.

## Material Finding

### `A6-CRITICAL-001` / same root as `OWN6-CRITICAL-001` — Infiltration enthalpy is not consumed by the soil-thermal ending state

`SC-LANDSURFACEENERGY-001@3` requires:

```text
E_soil,1,1 = E_soil,1,pre + sum(Q_infiltration)
```

and requires the soil owner to reconstruct its first-layer temperature from
that credited enthalpy. The reviewed `reconstruct_owner_endings()` validates
the infiltration receipt value but updates every soil temperature using only
the layer's ground-heat receipt. It then stores the infiltration value as a
separate receipt scalar without applying it to node 1.

The frozen vector is nondegenerate:

| Operand | Value |
|---|---:|
| beginning node-1 temperature | `291.5 K` |
| node-1 ground-heat receipt | `4766.427866774984 J m^-2 stand` |
| infiltration-enthalpy receipt | `53529.68923674751 J m^-2 stand` |
| node-1 areal heat capacity | `120000 J m^-2 tile K^-1` |
| tile fraction | `0.62` |
| emitted node-1 temperature | `291.56406489068246 K` |
| required node-1 temperature | `292.28354996106884 K` |

The omitted increment is `0.7194850703863835 K`. The current validator
accepts because it reconstructs the same omission, and the focused Rust test
checks receipt presence rather than receipt consumption. This violates the
selected advected-energy identity, complete owner-state reconstruction, and
atomic five-owner conservation claim.

Required correction is to apply the accepted stand-ground infiltration energy
to the identified first soil node exactly once after the capped ground-heat
update, preserve its typed receipt, and add executed wrong-node/omitted/
duplicated-receipt poisons. Regenerate every invalidated digest and fixture and
obtain fresh independent confirmation on the resulting exact bytes.

## Conclusion

The A5 diagnostic remediation and all forty bidirectional pairing poisons pass,
and the A4 frozen-cap mechanism remains correct. The infiltration-energy
crossing, however, is recorded but not present in the receiving physical state.
That is a material science/conservation defect inside Child 1's authority
release envelope.

**Result: FAIL. Child 1 is not eligible for implementation-authority release
on the reviewed bytes.**
