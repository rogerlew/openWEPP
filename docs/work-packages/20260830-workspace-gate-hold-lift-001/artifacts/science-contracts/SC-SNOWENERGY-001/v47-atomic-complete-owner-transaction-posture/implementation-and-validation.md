# V47 atomic complete-owner transaction posture implementation and validation

Status: `IMPLEMENTED; DUAL REVIEWS APPROVE; CANONICAL R122 PENDING`

Evidence mode: `Static + Ran`

## Correction

`SC-SNOWENERGY-001@47` preserves the ordinary/public atomic-install rule that
vegetation, LSE, BGC, accepted soil state, and prepared soil target must carry
one identical transaction. A distinct soil target is available only to the
authenticated unpublished-continuation install. That path must carry an
explicit native-V2 `PhysicalSoilEnergyTransactionAuthorityV2` and reconstruct
the exact source transaction, soil target transaction, soil predecessor,
support, accepted owner, receipt, and state/layer seals before staging any
install.

The split posture accepts only the exact composed successor chain observed in
r121: mutually equal vegetation/LSE/BGC source transaction 42, authenticated
soil target/state transaction 43, and exact authenticated predecessor 42. It
does not infer numerical adjacency. Foreign, swapped, absent, legacy-V1,
source-owner-disagreeing, and owner/substitution cases fail before mutation.
The install remains clone-then-validate-then-atomic-swap; refusal leaves all
authoritative owners unchanged and publishes no unpublished child.

The composed continuation call sites now construct the explicit authority from
the authenticated continuation and prepared target before install. The former
temporary source-owner rewrite to the soil target was removed, so source owners
retain their exact outer transaction while only the soil owner advances.

## Tests

The focused behavior vectors prove:

- ordinary same-source/same-soil-target admission;
- exact authenticated soil-successor admission;
- foreign, swapped, missing, and source-owner-disagreeing refusal;
- a genuine accepted first child followed by a composed second-child
  source/target/predecessor transaction chain;
- rollback and no publication after generic split-identity refusal; and
- retained V39 transaction separation and V46 budget-preflight behavior.

The restored retained V32 oracle proves that same-sign vapor images refuse the
V32 opposite-side active-set interface and continue through the authoritative
V31 exact `W/H` dispatch with closure and no publication.

## Validation

Ran focused V47 behaviors:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v47_/)'
```

Result: Nextest run `9b22ce56-9041-4cfd-9391-fb1c4d32d998`, `15 passed; 0
failed`.

Ran focused V47 authority/source obligations:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v47_/)'
```

Result: Nextest run `9de0d9f6-b7ba-4ea8-b69c-a81dae3b30cd`, `2 passed; 0
failed`.

Ran the complete unfiltered authority/source target after rebinding four stale
retained textual assertions to canonical contract wording and restoring the
retained V32/V31 dispatch oracle:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract
```

Result: Nextest run `ade914e1-fb90-4ca8-8771-b7bf967602cb`, `38 passed; 0
failed`.

Ran retained V39, V46, and V47 behavior regressions:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v39_/) | test(/v46_/) | test(/v47_/)'
```

Result: Nextest run `1a67966c-661f-492a-b51f-2eb4199aaabf`, `29 passed; 0
failed`.

Ran the retained persisted-restart crate after the V47 transaction-chain
correction:

```text
nix develop -c cargo nextest run -p openwepp-persisted-restart-v1
```

Result: Nextest run `0d91a3f0-7599-426b-9967-697a0c93f8d3`, `40 passed; 0
failed`.

Ran the authority-suite anti-evasion script and required-suite obligation
guard after changing source-bound authority assertions. The script reports
`PASS`; Nextest run `69bf853b-aa8e-42d2-9cca-161c464f99bd` reports `3 passed; 0
failed`.

Ran the orchestrator all-target/all-feature check, workspace format check, and
`git diff --check`; all pass at this checkpoint. Exact production scans find
no V47/R121 temporary diagnostic seam. Touched Rust files remain below 3,000
lines; `open_snow_convergence_tests.rs` is 2,997 lines after the retained-oracle
repair.

The warnings-denied all-target Clippy probe confirms the V47 duplicated-allow
finding is removed. The broad command remains blocked by separately owned
package-wide `too_many_lines`, `similar_names`, and `unreadable_literal`
findings recorded under the open WGHL Clippy disposition; none is introduced
by the bounded V47 production seam.

Independent `rust_code_reviewer` correctness disposition: `APPROVE`, with
fresh retained V39/V46/V47 `29/29` and V47 source `2/2` runs. Independent
`rust_qa_reviewer` disposition after correction: `APPROVE`, with fresh V47
runtime `15/15`, V47 source `2/2`, retained V39/V46/V47 `29/29`, format,
diff, and targeted duplicate-attribute lint evidence. See `correctness-review.md`
and `qa-review.md` beside this artifact.

## Remaining qualification

The implementation agent did not run the canonical one-day fixture. Parent-owned
r122 must verify that the corrected composed second-child atomic install lets
the canonical execution advance while retaining exact receipt, conservation,
rollback, and publication closure.
