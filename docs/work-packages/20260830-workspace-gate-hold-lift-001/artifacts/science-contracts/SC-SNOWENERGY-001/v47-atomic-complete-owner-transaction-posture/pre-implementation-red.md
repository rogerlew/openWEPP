# V47 atomic complete-owner transaction posture pre-implementation red

Status: `EXPECTED RED CONFIRMED`

Evidence mode: `Static + Ran`

## Trigger

Retained canonical r121 at `/tmp/wghl_001d_v46_64m_r121.log`, SHA-256
`bf703a976e5852a17b1a922d2086a9b2ce7786c4f459aa3cb79d2a346d3cca47`,
cleared the prior direct `60 s` authentic-receipt blocker and then failed on
composed support `1800..1980 s` with `V2 soil atomic complete-owner
transaction join`. Static owner tracing establishes mutually equal outer
vegetation/LSE/BGC source transaction 42, authenticated soil target/state
transaction 43, and exact authenticated soil expected predecessor 42.

The pre-V47 generic atomic guard requires the accepted soil state transaction
to equal the outer source and therefore rejects this lawful composed second
child. Merely accepting any different or numerically adjacent soil target
would weaken custody. V47 instead reserves the split posture for the
authenticated unpublished-continuation install and requires an explicit
native-V2 `PhysicalSoilEnergyTransactionAuthorityV2` joined to the
continuation/prepared target and exact predecessor.

## Expected-red execution

Ran:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(/v47_/)'
```

Result: Nextest run `27db3007-eda2-417f-ba66-f869a0a9fbdf`, `0 passed; 2
failed`. The production-source obligation reports the absent typed posture,
explicit source/target/predecessor carrier, both exact posture variants, and
all six required behavior vectors. The contract-text assertion also exposed a
line-wrapped phrase and is narrowed to its semantic substring before the green
run; that textual mismatch is not production evidence.

No V47 production source had been edited when this evidence was captured.
