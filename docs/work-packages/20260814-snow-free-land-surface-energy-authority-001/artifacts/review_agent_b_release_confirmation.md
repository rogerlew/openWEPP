# Release Ownership Confirmation Review

Evidence class: `Static + targeted Ran`, fresh independent exact-worktree
confirmation review.

Verdict: **NO-GO / FAIL**.

The `OWN5-CRITICAL-001` remediation is substantive: the strict coupled
transaction now contains the same five nonempty physical candidate bodies as
the executed post-ingress owner transaction; receipts are issued only after
the primitive reconstruction validator accepts; receipts contain no producer
`validated` boolean; and the body, copied-join, empty-material, and truncated-
node poisons reject. One new load-bearing ownership defect remains in that
primitive reconstruction: infiltration enthalpy is recorded as a soil-thermal
receipt but is not applied to the receiving first-layer temperature.

## Exact Bytes Reviewed

- LSE definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- top-level calculator:
  `9c1dc79631af4840d69f04a619e0e5d6bcbd9210b70a0e669e58f57ec89cae79`;
- frozen vectors:
  `ad2e3e7e39a594e297eaf9f4c5d4549fa27f531f9edb7e9bba1c5197d43282da`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- water-protocol schema:
  `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07`.

I read the applicable repository, work-package, science-contract, and test
instructions; the complete current package; the canonical LSE and vegetation-
transaction ownership text; all earlier ownership reviews and dispositions;
the strict schemas; calculator; frozen vectors; and focused authority test.
Per the review assignment I did not regenerate the oracle.

Targeted execution:

- imported the checksum-bound calculator without invoking generation;
- validated the frozen strict coupled transaction through its registered
  Draft 2020-12 schema;
- reran `validate_owner_candidates` against the frozen physical transaction;
- independently checked body/receipt hashes, D/A/F, condensation, rollback,
  and routed mass/energy lineage;
- independently reconstructed the post-ingress soil-node temperature from the
  canonical energy operands;
- ran
  `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick`:
  **7 passed, 0 skipped**.

## Material Finding

### `OWN6-CRITICAL-001` — Infiltration enthalpy is received but omitted from the soil-thermal ending state

`SC-LANDSURFACEENERGY-001@3` requires, after accepted ingress partition,

```text
E_soil,1,1 = E_soil,1,pre + sum(Q_infiltration)
```

and explicitly requires the soil owner to reconstruct its first-layer
temperature from the credited enthalpy.

`reconstruct_owner_endings()` validates that
`soil_thermal_operands.infiltration_enthalpy_receipt_j_m2_stand_ground` equals
the independently reconstructed post-ingress receipt, but its temperature loop
uses only `ground_heat_receipt_j_m2_stand_ground` for every layer. It then
stores the infiltration receipt as a separate scalar in the candidate without
adding it to node 1 energy.

The frozen physical vector makes the omission nondegenerate:

| Operand | Frozen value |
|---|---:|
| beginning node-1 temperature | `291.5 K` |
| node-1 ground-heat receipt | `4766.427866774984 J m^-2 stand` |
| node-1 infiltration-enthalpy receipt | `53529.68923674751 J m^-2 stand` |
| node-1 areal heat capacity | `120000 J m^-2 tile K^-1` |
| tile fraction | `0.62` |
| emitted candidate node-1 temperature | `291.56406489068246 K` |
| contract-required candidate node-1 temperature | `292.28354996106884 K` |
| omitted temperature increment | `0.7194850703863835 K` |

The current validator accepts this candidate because it reconstructs the same
omission. The focused Rust authority test also passes because it checks receipt
presence and owner-body non-emptiness, not the canonical soil-node energy
update. Thus the receipt is not independently consumed by the mutable owner
state, and the claimed five-owner reconstruction is not yet complete.

Required correction:

1. Add the accepted infiltration enthalpy to the identified first soil-thermal
   node after the capped ground-heat update, using the frozen OFE/tile area
   basis exactly once.
2. Preserve the separate typed infiltration receipt, but reconstruct the
   ending node energy/temperature from ground heat plus infiltration rather
   than treating receipt presence as consumption.
3. Add a poison that omits, duplicates, or applies the receipt to the wrong
   soil node and require independent rejection.
4. Flow the corrected calculator through vectors, bound digests, focused
   tests, and a fresh confirmation review.

## Confirmed Ownership Surfaces

The following assigned surfaces passed on the exact reviewed bytes and should
be retained through remediation:

- `candidate_owner_bodies` in the strict envelope is byte-identical to the
  executed five-owner physical candidate set, and every ending body is
  nonempty;
- every strict receipt hashes the candidate body present in the same envelope,
  shares its beginning lineage, and contains no `validated` boolean;
- `owner_receipts()` calls the complete candidate validator before issuing any
  receipt;
- vegetation ending occupancies derive from final component temperature and
  hydraulic operands;
- hydrology ending stores derive from finalized uses and typed condensation
  credits;
- LSE ending enthalpy/temperature derives from accepted advection operands;
- BGC material receipts are reconstructed from one nonempty vegetation
  proposal rather than two empty-list hashes;
- all five owner-body mutations, copied-candidate join, both empty-material
  tautologies, and soil-node truncation reject with null candidates;
- shared competition retains 19 unique request/authorization/finalized-use
  identities with `F <= A <= D` and six source-store ledgers;
- the positive condensation protocol validates against the normative water
  schema and carries five receipts;
- all eleven failures retain null candidates and exact six-member owner-plus-
  envelope rollback;
- routed runoff/runon preserves accepted transaction/state lineage and closes
  `72 kg` plus identical extensive energy across the `120 m2 -> 200 m2`
  conversion.

## Conclusion

The final `OWN5` schema/body/copy remediation is real, but its independent
soil-thermal reconstruction drops a required accepted energy crossing from the
ending state. This is a conservation and owner-state defect inside Child 1's
release envelope, not a later runtime-only obligation.

**Result: FAIL. Do not release Child 1 until `OWN6-CRITICAL-001` is corrected,
the invalidated identity chain and evidence are refreshed, and a fresh
independent ownership confirmation passes the exact resulting bytes.**
