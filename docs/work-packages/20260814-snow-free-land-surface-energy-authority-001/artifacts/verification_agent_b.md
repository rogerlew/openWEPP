# Terminal Verification B: Ownership And Release Boundary

Evidence class: `Static + targeted Ran` against the exact current worktree.

Verdict: **FAIL / terminal evidence reconciliation required**.

The admitted hydrology, surface-liquid, soil-thermal, routed-energy, owner-
candidate, rollback, reference-custody, and protected-production boundaries
passed this verification. One material package-governance finding prevents a
terminal PASS: several current evidence artifacts still describe the exact
final hashes and accepted findings as awaiting remediation or confirmation.

## Exact worktree inspected

- branch: local `main`, one commit ahead of `origin/main`;
- parent commit: `0db1960129ad4f8fc4e292b20574dfe7229d5fe1`;
- LSE definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- C3 woody V8 definition:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- independent calculator:
  `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859`;
- joint core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- vectors:
  `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- water-protocol schema:
  `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07`.

Static diff inspection found nine tracked documentation/contract/test changes,
the campaign and child-package trees, and four rights-reviewed references.
There is no modified or untracked file below `crates/`, no Cargo manifest or
lockfile change, and no runner, selector, default, production dispatch, state
mutation, or output-publication change.

## Passing ownership assessment

### Water and surface-liquid custody

Hydrology is the sole water-mass owner for ponded, litter-held, and soil-layer
water. LSE owns one surface enthalpy state and receives immutable source-keyed
mass views; it does not retain a duplicate mutable liquid amount or per-store
liquid temperature. Condensation is a positive hydrology credit, separate from
nonnegative evaporation withdrawal.

The exact protocol preserves transaction, owner, component, OFE, requesting
tile/occupancy, source tile or layer, surface class, resource type, interval,
and amount basis through `D/A/F`. The authority is frozen once from immutable
beginning stores, `0 <= F <= A <= D`, and finalized use alone is debited.
Current precipitation, runon, and canopy releases are post-final-solve ingress;
they do not inflate same-interval ET supply. There is no reauthorization,
canopy-to-ground demand donation, or unused-authorization credit.

### Advected energy and routed OFE identity

Precipitation, canopy release, routed runon, infiltration, runoff, and outlet
records retain source temperature/enthalpy and paired receiver custody. The
nondegenerate routed case converts `0.6 kg m^-2` over `120 m^2` to
`0.36 kg m^-2` over `200 m^2`, preserving `72 kg` and the same extensive
enthalpy. Tile fractions remain OFE-local and are applied exactly once; the
route converts via upstream and downstream OFE areas rather than reusing an
upstream tile fraction.

### Soil-thermal receipt consumption

The corrected first soil node consumes both the surface ground-heat receipt
and infiltration enthalpy. The independent reconstruction yields exactly
`292.28354996106884 K`. Omission, duplication, wrong-node, and wrong-basis
poisons are bound. Receipt presence is therefore joined to actual receiving-
owner state mutation rather than treated as sufficient evidence by itself.

### Five-owner envelope and rollback

The strict authority transaction has nonempty physical candidate bodies for
vegetation, hydrology, LSE, soil thermal, and biogeochemistry. Receipts are
independently reconstructed from primitive candidate/beginning operands; the
material path is nonempty and does not rely on a producer `validated` flag or
equal empty hashes. All eleven typed failure records expose no candidate and
retain exact before/after hashes for the five owners plus transaction envelope
under one attempted transaction identity.

This is Child-1 authority evidence only. It does not misclassify the Python
arbiter as the actual production hydrology owner; Child 2 retains that separate
real-owner obligation.

## References, gates, and production exclusion

The four committed Copernicus sources are recorded as CC-BY-3.0 or CC-BY-4.0,
and their bytes match the acquisition ledger:

- R-155: `e63ec3dd516dae8984739046613ae72300c5b4a46e3d63019575088718fd464e`;
- R-156: `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`;
- R-157: `31e9d99f9e1102cb67028479cab4ec08fb73490da4f3fdb9bdee067860075c5c`;
- R-158: `52df13b80758e2fd250e200335b5e91d024029e0123d7b795e8d247b90c89fab`.

The restricted CLM5 note remains under gitignored `references/copyrighted/`.
The bibliography supplies citation, DOI, date, path, checksum, license,
locators, and selected/rejected process mapping.

Ran during this verification:

```text
cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick
7 passed / 0 skipped
```

The retained current documentation scan has findings byte-identical to the
frozen baseline: 15 errors and zero warnings. The baseline scanned 20,074
files; the retained current scan scanned 20,144. The package gate records a
later 20,152-file scan with the same 15 findings and zero warnings. The
acceptance claim is correctly baseline-relative and makes no zero-link-error
claim.

Heavy-gate evidence belongs to this child and is not borrowed from another
child: full workspace nextest records 2,674/2,674 passing, strict workspace
Clippy passed after a preserved test-only failure, doctest invocation and
dependency policy passed, and formatting/diff hygiene passed. The two failed
delegated comparator attempts are truthfully classified as infrastructure
failures; the parent records running only the unfinished heavy commands.

The active kickoff prompt remains present under `prompts/active/` and has not
been archived early. No runtime implementation or production activation claim
is present in the diff.

## Material finding

### `TVB-HIGH-001`: current evidence surfaces contradict terminal closure

Disposition: **accepted verification finding; correction required before
terminal PASS**.

`terminal-diff-reconciliation.md`, `pre-implementation-authority-gate.md`, and
the terminal dual reviews correctly state that accepted findings are closed,
heavy gates passed, and only terminal verification remains. However, the
following current, non-historical evidence still describes the same final
hashes or current authority as pending remediation/confirmation:

- `contract-amendment-evidence.md`: invalidated admission rerun pending and
  exact-byte unit/admission reruns still required;
- `contract-test-evidence.md`: confirmation pending and exact-hash focused
  gates/review still required;
- `oracle-fixture-manifest.md`: release confirmation and fresh review pending;
- `control-volume-and-owner-selection.md`, `surface-liquid-custody.md`,
  `advected-energy-convention.md`, and `equation-authority-ledger.md`: accepted
  remediation still in progress;
- `state-and-configuration-schema.md`: accepted findings remain unremediated
  and schema digest still awaits stabilization;
- the concluding sentence of `review-finding-disposition.md`: heavy comparator
  gates are pending, despite `gate-results.md` row 74 recording heavy-gate
  PASS.

Historical failed review reports and checkpoint-specific table cells must stay
unchanged. The defect is limited to current status prose that is supposed to
describe the final release candidate. Reconcile those surfaces to the already
recorded exact-hash review and gate results, rerun Markdown/diff hygiene, then
request fresh terminal verification against the resulting exact bytes.

## Conclusion

No material scientific, hydrology-custody, owner-identity, routed-area,
rollback, rights, or production-boundary defect was found. Child 1 nevertheless
cannot receive terminal verifier B PASS while its current evidence set gives
contradictory lifecycle states.

**Result: FAIL solely on `TVB-HIGH-001`; terminal release is not yet
authorized.**
