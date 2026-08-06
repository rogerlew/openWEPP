# Contract V8 Assurance Source Adoption

Evidence class: Ran, canonical typed transaction on 2026-08-06.

The first clean closeout candidate passed formatting, warnings-denied Clippy,
and doctests, then failed quick because the governed identity still bound the
pre-v8 `SC-SNOWENERGY-001` bytes. All `31` quick failures reduced to that one
identity drift or its downstream assembly/planner effects.

The snow/frost report was already governed `DRAFT`, so the canonical operation
was an exact external-source adoption:

```text
target/release/openwepp-assurance amend adopt-report-source \
  --report snow-and-frozen-soil-process-evaluation \
  --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md \
  --if-generation f9884c0556bea183c9df5d084298d28a4b9243c75208c59591ab6c0f338de0ea \
  --apply
```

Receipt:
`assurance/v2/transactions/ae69c6d2af715b5b24fd08aa3b75375671a7e50afb85f10599b0f6cdaee5a99c.json`.

- old generation: `f9884c0556bea183c9df5d084298d28a4b9243c75208c59591ab6c0f338de0ea`;
- new generation: `910ab3d3de81d78aedfb8c900188c3bd67a8f574613289af1371890fefdb394e`;
- lifecycle: `DRAFT`;
- active review events: zero;
- approval lock root: absent;
- public reports: zero; and
- prior events: retained under invalidated custody.

Ran after apply:

- `openwepp-assurance validate --all` — PASS, `3/3` reports, public count zero;
- focused source, snow contract, and snow runtime tests — PASS, `44/44`;
- strict science-contract binding exposure — PASS; and
- science-contract unit compliance — PASS.

The operation updates source custody. It does not assert that the DRAFT report
scientifically evaluates version 8, create review authority, or approve the
Stage 3 shadow.
