# V46 complete-step budget-preflight implementation and validation

Status: `IMPLEMENTED; R121 CLEARED DIRECT RECEIPT BLOCKER; V47 SUCCESSOR REQUIRED`

Evidence mode: `Static + Ran`

## Correction

`SC-SNOWENERGY-001@46` adds one exact capacity preflight before the first
finite-difference column of every safeguarded step. The validated residual
dimension `d`, one mandatory full-physics trust trial, and the unchanged role
reserve `r` must fit as `d+1+r` unconsumed charges. Above tolerance, failure is
typed `EvaluationBudget`; during finite side-valid sub-tolerance polishing it
returns `ReceiptEntryReserve` with the complete carried best bundle and zero
new physical calls.

The per-map preserve checks remain in place for reverse perturbations and
rejected trust trials. Every column and trust trial still executes the same
one-charge finalization-equivalent physical map. The generalized Jacobian,
Newton direction, trust/backtrack behavior, coordinates, residuals, strict
merit descent, tolerances, maximum 96, exact receipt stabilization, oscillation
refusal, independent same-input replay, finalization, rollback, and
no-publication guards are unchanged. V46 adds no chord, secant, Broyden,
quasi-Newton, CN heat, receipt, digest, or map-difference residual.

The focused authentic receipt vector starts at shared used 89 and proves that
six exact CN receipt probes followed by the mandatory independent replay can
end exactly at used 96. This is capacity and chronology evidence, not a claim
that the canonical r120 receipt tail stabilizes in six probes. Parent-owned r121
must decide that sufficiency question without loosening exact equality or the
cap.

## Validation

Ran focused V46 behaviors:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v46_/)'
```

Result: Nextest run `f543e85d-d882-4baf-b1ff-b4037a099c65`, `8 passed; 0
failed`.

Ran V46 authority/source obligations:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v46_/)'
```

Result: Nextest run `cd517189-c7e9-43bc-a98f-914e6c2f0380`, `2 passed; 0
failed`.

Ran the retained V35--V46 source obligations after rebinding the historical
V38 finalization-input assertion to the V45 direct complete-bundle seam:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(/v3[5-9]_/) | test(/v4[0-6]_/)'
```

Result: Nextest run `3d9a168c-514f-49bc-9207-8c43b1f93562`, `24 passed; 0
failed`.

Ran retained V35--V46 and phase-consistent solver regressions:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v3[5-9]_/) | test(/v4[0-6]_/) | test(/phase_consistent/)'
```

Result: Nextest run `2728b9b0-ddd7-49ee-8cb6-3ff7ec752b0c`, `77 passed; 0
failed`.

Ran retained persisted-restart regression:

```text
nix develop -c cargo nextest run -p openwepp-persisted-restart-v1
```

Result: Nextest run `b88da76b-6080-483b-aba4-b0282483f9af`, `40 passed; 0
failed`.

Ran the orchestrator all-target/all-feature check, workspace format check, and
`git diff --check`; all pass at this checkpoint. Exact scans find no V46/R120
temporary diagnostic or receipt-repair seam. Source line counts remain below
3,000 with a binding split disposition recorded in line-count governance.

## R121 disposition

Parent-owned r121 cleared the prior direct `60 s` authentic receipt
stabilization blocker. The run later failed on composed support `1800..1980 s`
at the exact V9 native-V2 soil atomic complete-owner transaction join. Retained
log `/tmp/wghl_001d_v46_64m_r121.log` has SHA-256
`bf703a976e5852a17b1a922d2086a9b2ce7786c4f459aa3cb79d2a346d3cca47`.
That later custody defect is governed by V47 and does not invalidate V46's
complete-step budget preflight or focused capacity evidence.
