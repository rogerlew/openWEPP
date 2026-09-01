# V45 authentic receipt root-polishing pre-implementation red

Status: `PASS — expected production red retained`

Evidence mode: `Ran + Static`

Retained canonical evidence:

- r117: `/tmp/wghl_001d_v44_64m_r117.log`, SHA-256
  `33ac890a9dbe05962363a0a5838b992d7ca2ad3c13e9fe2912f5555968748c5e`;
- r118: `/tmp/wghl_001d_v44_64m_r118.log`, SHA-256
  `fb65dfbfd53d4f587a416ffc97d6e1aca9a4d4a8cfd0ac2b49e925c265edc858`;
- exact failing support `1860..1920 s`; r118 typed error
  `EvaluationBudget`, `used=96`, `maximum=96`.

Static charge trace: the support dispatches after 13 already-charged raw
Picard maps. Every private initial/Jacobian/fallback/trust/rejected map and
every authentic receipt probe/replay charges exactly once at
`covered_phase_consistent_finalization_equivalent_map_v1`. The existing
`minimum_solver_reserve` is eligibility-only and is never enforced by the
solver or receipt loop. Exact receipt oscillation did not fire; the retained
tail is a nonrepeating binary64 CN receipt contraction after tolerance closure.

Ran:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(v45_contract_binds_authentic_receipt_root_polishing_and_replay_reserve) | test(v45_authentic_receipt_root_polishing_production_seams_are_required)'
```

Nextest run `b84a1144-cea2-420d-ad54-cfa6fe37d3be`: authority 1/1 PASS;
production obligation 1/1 expected FAIL. Missing seams are the typed complete
polished-root carrier, unchanged-residual polish function, purpose-specific
three/two/one shared-budget reservations, and six exact behavior vectors.

No production implementation or tolerance/cap/receipt change preceded this
red. Temporary r118 diagnostics were removed before V45 authoring and the
production diagnostic scan was clean.
