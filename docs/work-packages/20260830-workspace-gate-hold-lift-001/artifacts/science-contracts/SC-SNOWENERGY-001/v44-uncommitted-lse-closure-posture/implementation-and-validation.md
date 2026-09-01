# V44 uncommitted LSE closure-posture implementation and validation

Status: `IMPLEMENTED; CANONICAL QUALIFICATION PENDING`

Evidence mode: `Ran + Static`

## Correction

`SC-SNOWENERGY-001@44` now assigns closure posture from the sealed coupled
evaluation kind. A charged `PrivateTrial` alone uses the existing uncommitted
provisional LSE posture so corrected reciprocal-longwave, shortwave, sensible,
and vapor exchange can be rebuilt before one Stage 3 image is evaluated.
Receipt-stabilization probes, the independent same-input replay, and authentic
finalization retain strict non-provisional weighted-OFE closure. The private
result remains ineligible for authentic admission and publication.

The private root's corrected post-LSE boundary exchange is sealed as the only
strict replay input. Receipt probes and replay refuse if that corrected input
is absent; they cannot fall back to the stale endpoint seed. Each successful
strict map carries its corrected exchange into the independent finalization
inputs, so the unchanged final weighted-OFE closure runs before admission.

The projected top-soil temperature coordinate is read by the snow--soil CN
path exactly once. The Stage3-covered V8 posture retains the admitted soil
beginning with zero ground-heat/storage contribution and rejects double use.
No weighted-OFE operand, tolerance, conservation equation, evaluation charge,
shared cap, receipt, custody, rollback, event, topology, or publication rule
changed.

## Validation

Ran:

```text
nix develop -c cargo test -p openwepp-hillslope-orchestrator v44_ -- --nocapture
```

Result: terminal `6 passed; 0 failed`: the five contract-named V44 behaviors
plus the real DirectV9 resident-V8 selector/double-use poison.

Ran:

```text
nix develop -c cargo test --test snow_terminal_enthalpy_event_numerics_contract v44_ -- --nocapture
```

Result: `2 passed; 0 failed` for V44 authority and production/source binding.

Ran:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v(38|39|40|41|42|43|44)_/)'
```

Result: terminal run ID `0f7c515a-a7a8-42fb-b035-aec6379e5725`,
`37 passed; 0 failed`.

Ran:

```text
nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(/v(38|39|40|41|42|43|44)_/)'
```

Result: terminal run ID `09955e18-1df9-42e3-834a-084252970677`,
`14 passed; 0 failed`.

Ran retained coupled-solver regression:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v(31|32|33|34|35|36|37|38|39|40|41|42|43|44)_/)'
```

Result: terminal run ID `650452c0-83e0-4fce-99ee-e1ac0b3f2503`,
`72 passed; 0 failed`.

Ran:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator --all-targets
nix develop -c cargo fmt --all -- --check
git diff --check -- <V44 owned write set>
```

Result: all `PASS`.

The six focused V44 behaviors now include the captured r116 weighted-OFE poison
(`423.500682899798 J m^-2 tile`) and a zero-residual corrected strict closure;
exact strict selection of the corrected boundary plus missing-seed refusal;
one charged Stage 3 map; independent reconstruction of the real snow--soil CN
heat from the projected temperature coordinate and sealed receipt; changed,
nonfinite, and missing-coordinate poisons; strict receipt stabilization and
one independent replay; rollback artifact byte lock; and publication
exclusion. The real V9/V8 physical-beginning selector is also exercised with a
numerical V2 projection whose physical state differs from the resident: it
selects the resident admitted read view for V8, selects the candidate only for
the ordinary unpublished-physical posture, and rejects projected-as-V8,
missing-candidate, and mixed-posture substitutions. These are runtime
assertions over production helpers used by the charged evaluator and V8
consumer, not source-string-only evidence.

The unfiltered source-contract binary retains four pre-existing V32/V33/index
failures already recorded by the V43 evidence. The V38--V44 source-bound
selection is green and V44 neither owns nor changes those stale requirements.

## Independent review

- Rust correctness review: `APPROVE`; no blocking terminal finding.
- Rust QA review: `APPROVE`; no blocking terminal finding.

Both preliminary HOLDs were resolved before approval by carrying corrected
post-LSE exchange into strict replay/finalization, adding the real resident-V8
selector and substitution poisons, replacing selector-only tests with runtime
closure/CN/replay/rollback evidence, and restoring the source line limit by an
exact test split. The full dispositions are recorded beside this artifact.

## Hygiene and remaining qualification

Ran exact diagnostic scans for `DFF_R115`, `DFF_R116`, the R112--R116 family,
`eprintln!`, and `dbg!` across the owned production/test paths; no temporary
V44/R115/R116 diagnostic remains. Line-count disposition is recorded in
`artifacts/line-count-governance.md`; both warning files remain below 3,000
lines with an exact-move split intent and no exception.

The canonical one-day/60-second-support consumer has not been run by this
implementation agent. Parent-owned canonical qualification must establish
that the real charged private coordinate reaches corrected exchange, strict
authentic replay/finalization closes the unchanged ledgers, and the prior
1860..1920-second safeguarded-solve refusal is cleared.
