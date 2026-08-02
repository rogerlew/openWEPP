# Behavior-Neutrality Evidence

Evidence mode: **Ran**.

The exact EB-04W v3 outputs were decoded and compared with the retained exact
EB-04V v2 outputs for every B/L/S/LS cell:

- all `245,456` WAT rows and `736,368` numeric WAT values matched exactly;
- all `245,456` snow-trace rows and `72,093,744` numeric values across every one
  of the `111` prior v2 top-level fields matched exactly;
- maximum WAT and prior-v2 trace numeric differences were both `0`;
- EB-04W accumulation/melt fields are additive to the retained v2 schema; and
- Parquet container bytes were not used as a behavioral oracle because writer
  metadata may differ while decoded rows are identical.

The comparison binds current binary SHA-256
`b50dd71cb00f24806193b98d73fc5444e836efac84ad5a4e0465d1e67c81fec9`,
current execution receipt SHA-256
`6f6bfe361c5b0aa155de1cfba61306e6d20fd570e68f67521eed12a3154dfbf7`,
and retained-reference receipt SHA-256
`f2cc806de485cdbc00bc4c5b9e0e778ccb62fd6e1582d511fe5ea2b47f7fb1be`.
The comparator tool SHA-256 is
`1b3f331d3c298d1a6b7f9a220efccc7dc788fecf13ca77aa7217a502a9ce7b35`;
the observed-harness SHA-256 is
`c91f01780c871a3ecc28f0ef4833fb13ef1e552e56eb582bfd256b58a0785fa9`.
`behavior-neutrality.json` has SHA-256
`b896b53ecb3787dd85fe46732a7154b22788c942af3f6c8093a96113859e7d0e`
and retains the exact command, working directory, per-cell input hashes,
counts, and receipt identities. The terminal comparison passed in `254.85 s`
with exit `0`. This establishes exact retained-output neutrality, not efficacy.
