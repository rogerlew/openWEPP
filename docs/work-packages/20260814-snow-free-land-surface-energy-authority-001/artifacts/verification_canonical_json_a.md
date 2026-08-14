# Canonical-JSON Correction Terminal Verification A

Evidence class: `Static + Ran`

Verdict: `PASS`

This verification is limited to the reopened Child-1 canonical-JSON
evidence-portability correction. It does not verify or release the concurrent
Child-3 constitutive runtime. The final successful checks ran with repository
HEAD `f3e9ed64172de583a9b854839339c70a7cb949d7`; the comparison baseline is the
released Child-1 commit `3f1cf8ee3`.

## Exact correction bytes

- canonical-JSON/runtime identity code:
  `f0a15da155a6ae8b6e2a5191dd6f5e44549279e0f4efecc82ae5f7e54d10d981`;
- corrected generator:
  `9278be79b1a74d4d609ab5857d00071b1e5717e036cc7323cbfcbf970795666c`;
- corrected vectors:
  `9f171b0fd0e9a9a2e40d6ea8773d120b961c343e2aad6ad951ae705c8d683f3b`;
- LSE definition, unchanged:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- V8 definition, unchanged:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- joint canopy-ground core, unchanged:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`.

All six schema hashes equal their `3f1cf8ee3` bytes:

- configuration `6499b98c...`;
- coupled transaction `02dfa522...`;
- diagnostics `41fb7909...`;
- forcing `f1fb785e...`;
- state `91243e40...`;
- water protocol `2e5ade75...`.

## Independent regeneration and structural comparison

Ran:

```text
.venv/bin/python artifacts/reference_calculator.py \
  --write /tmp/child1_va.json
cmp /tmp/child1_va.json artifacts/openwepp_snow_free_lse_v1_vectors.json
```

Result: `PASS`; regenerated SHA-256 is exactly `9f171b0f...` and the bytes
compare equal.

An independent recursive parsed-value comparison against the released
`3f1cf8ee3` fixture found exactly four changed leaves:

1. `strict_schema_instances.state.state_sha256`;
2. `strict_schema_instances.coupled_transaction.beginning_lse_state.state_sha256`;
3. `strict_schema_validation.state.instance_sha256`;
4. `strict_schema_validation.coupled_transaction.instance_sha256`.

The corrected embedded state digest is
`6ff22f0d72b6c4fdad3c0d8a0b2947571191e48213635609af8f3b951c07abf1`.
No other parsed fixture leaf changed. Therefore definitions, schemas, model
reductions, joint-core results, accepted physical values, requests,
authorizations, finalized uses, owner candidates, residuals, failure branches,
poisons, and rollback values remain unchanged.

Static inspection confirms that `canonical_digest()` applies the exponent
adapter only after typed `serde_json` serialization. The scanner tracks quoted
and escaped-string state and pads only signed one-digit lowercase numeric
exponents outside strings. The poison covers a numeric token plus ordinary and
escaped exponent-like strings. The adapter is called only by identity digest
construction; it does not alter typed binary64 values or participate in
physics, branch selection, convergence, authorization, closure, or solver
acceptance.

## Focused gates

The first focused attempt was retained as a failed concurrent-worktree check:
both commands could not compile while `lib.rs` declared `transaction` before
the concurrently authored `transaction.rs` existed. This was not treated as a
correction PASS.

After that module became present, the exact same commands were rerun:

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-land-surface-energy --profile quick` | `PASS`, 21/21, 0 skipped |
| `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick` | `PASS`, 7/7, 0 skipped |
| `git diff --check -- <Child-1 package, LSE crate, authority test>` | `PASS` |

The crate tests deserialize and validate the complete frozen configuration and
state, independently recompute their canonical digests, and poison unknown
fields and one-bit scientific mutations. The authority tests consume the
corrected committed fixture.

## Lifecycle and prompt check

`package.md` remains `executing / canonical JSON evidence-portability
correction active`; `final-disposition.md` remains an in-progress placeholder
that distinguishes historical hashes from correction-candidate hashes. The
active correction prompt remains under `prompts/active/`. Only the historical
original kickoff prompt is archived. No active correction prompt was archived
before dual verification.

## Conclusion

The correction is a bounded canonical representation and stale embedded-digest
repair. It changes exactly four dependent fixture leaves and changes no model
definition, schema, joint core, physical result, numerical method, tolerance,
or accepted branch. No unresolved finding remains in this verification scope.
