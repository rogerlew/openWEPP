# V45 authentic receipt root polishing implementation and validation

Status: `IMPLEMENTED AND DUAL-REVIEW APPROVED; CANONICAL QUALIFICATION PENDING`

Evidence mode: `Static + Ran`

## Correction

Static charge tracing of retained r117/r118 established that the
`1860..1920 s` support entered the coupled solver after 13 already-charged raw
maps, reached an unchanged tolerance-closed physical root, and then exhausted
the single 96-evaluation account while its exact Crank--Nicolson receipt image
was still contracting. The former solver path neither protected an
independent replay slot nor continued the lawful physical residual map after
tolerance closure.

`SC-SNOWENERGY-001@45` retains the existing `R_W/R_H/R_rho/R_E/R_T` plus
derived `R_z` equations, canonical merit, generalized Jacobian, Newton/trust
steps, physical evaluator, tolerances, and maximum 96. The ordinary private
solver now refuses a map that would leave fewer than three charges. It carries
the exact complete ordinary-root bundle without another physical evaluation,
then private root polishing admits only strict descent in the unchanged
canonical merit while every polishing map preserves two charges. Exact-zero
residual bits may stop the private polishing loop; finite side-valid
sub-tolerance stagnation or twelve-trial non-descent carries the best matching
bundle to receipt stabilization without creating acceptance authority.

The ordinary solve and polishing loop now share one safeguarded
Jacobian/Newton/trust step, and the ordinary solve's exact trust radius is
carried across tolerance closure. Every charged physical map returns one
complete residual/artifact/finalization-input bundle directly. The bundle is
stamped with its shared-budget ordinal and exact canonical phase/density
branch identity; coordinate, ordinal, branch, stale-bundle, or malformed
tolerance shape fails typed. No mutable `latest_*` value can substitute a
rejected trial's artifacts. Finalization inputs also participate in the
mandatory exact probe/replay comparison.

Each receipt probe preserves the final independent replay charge. A nonstable
probe at used 94 may charge to 95 but cannot consume charge 96; an exactly
stable probe at used 94 may consume charge 95 and its mandatory same-input
exact replay consumes charge 96. Exact immutable `R_n -> R_(n+1)` receipt
chaining, oscillation refusal, exact residual/artifact/reconstructed-receipt
replay, authentic finalization, rollback, and publication exclusion remain
unchanged. No receipt distance, digest repair, `F(x)-x`, extra budget, or
diagnostic seam was introduced.

## Validation

Ran focused V45 runtime behaviors:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v45_/)'
```

Result: Nextest run `db436802-7b7d-4702-b9c3-4f6fc71efe3e`, `10 passed; 0
failed`.

Ran V45 authority/source obligations:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(/v45_/)'
```

Result: Nextest run `33934bf6-d821-420a-8134-a92bccae7db2`, `2 passed; 0
failed`.

Ran retained V35--V45 and phase-consistent solver regressions:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v3[5-9]_/) | test(/v4[0-5]_/) | test(/phase_consistent/)'
```

Result: Nextest run `e8d8d772-23f2-4913-b7e0-219bb032fdaa`, `69 passed; 0
failed`.

Ran retained persisted-restart regression:

```text
nix develop -c cargo nextest run -p openwepp-persisted-restart-v1
```

Result: Nextest run `bfa9e84d-fcae-4b2b-b751-95a548c41f51`, `40 passed; 0
failed`.

Ran:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator \
  --all-targets --all-features
nix develop -c cargo fmt --all -- --check
git diff --check
```

Result: all `PASS` at the recorded implementation checkpoint. The V45 source
files remain below 3,000 lines. Exact scans find no `DFF_V45`, R117/R118
temporary probe, `eprintln!`, or receipt-repair diagnostic in the production
path.

Ran the affected-package warnings-denied Clippy command with `--no-deps`; it
remains blocked by broad pre-existing crate lint debt (1,286 diagnostics in
unrelated hydrology/terminal modules). V45 introduces no warning in the
all-target check, and this blocked command is not reported as a pass.

## Remaining qualification

Independent Rust correctness and QA reviews both dispositioned `APPROVE` with
no blocking terminal finding. Their exact dispositions are recorded beside
this artifact. The implementation agent did not run the canonical one-day
fixture. Parent-owned canonical
qualification must show whether the retained r117 receipt contraction now
stabilizes and independently replays inside the unchanged cap, then report the
canonical accepted/rejected counts, width distribution, runtime, limiting
reasons, and ledger closure.
