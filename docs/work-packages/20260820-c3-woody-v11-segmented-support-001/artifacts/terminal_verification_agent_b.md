# Terminal Verification B — `b052158d03668dadcc592d539a1d960f152c6440`

Verdict: **FAIL / lifecycle reconciliation incomplete**.

Independent technical verification passes:

- exact HEAD and branch checkpoint: PASS;
- LSE support authority oracle: `15/15` PASS;
- Draft 2020-12 baseline schema and independent domain-preimage KAT/reframing
  poison: PASS;
- focused LSE support tests: `2/2` PASS;
- vegetation support-receipt replay/no-parent-mutation poison: PASS;
- actual V11 minimum-support and one-tick-below rollback paths: PASS;
- vegetation plus persisted-restart package nextest: `298/298` PASS, run ID
  `ed6c38af-69e0-4f84-9480-956a1102fafb`;
- persisted-restart library: `26/26` PASS;
- focused LSE, vegetation, and persisted-restart Clippy with warnings denied:
  PASS;
- `git diff --check 464cd506 b052158d0`: PASS;
- protected boundaries: PASS. The implementation range changes no
  coupled-time source. Persisted-restart changes are confined to additive
  `vegetation_v11_v3.rs`; DirectV10 Restart V1 and V11 Restart V2 sources are
  unchanged. V10 vegetation behavior is not edited.
- Restart V3 retains the exact ordered receipt envelopes by projecting the
  independently admitted complete vegetation checkpoint. The vegetation
  restore authority authenticates receipt bytes, digest, owner/support joins,
  uniqueness, and replay before the V3 projection is returned.
- Implementation Reviews A, B, and C terminate PASS with all enumerated
  support-receipt findings closed without waiver.

Terminal promotion nevertheless fails on the exact committed evidence tree:

1. `line-count-governance.md` remains `Status: queued` and contains no touched
   Rust counts or threshold disposition.
2. `gate-results.md` records `297/297`, while the exact-tree independent run is
   `298/298`; it also still says heavy closure and terminal verification are
   pending.
3. `final-disposition.md` still states that heavy gates, exact diff/line-count
   reconciliation, and dual terminal verification remain pending.
4. At `b052158d0`, the finding-disposition and exact-diff artifacts were not
   terminally reconciled. Uncommitted concurrent corrections do not alter the
   exact checkpoint under verification and require a new evidence-only
   checkpoint before they can be admitted.

These are lifecycle and evidence defects only. Reconcile the counts, line
governance, finding disposition, exact diff, gate status, and final disposition
in one evidence-only checkpoint, then rerun both terminal verifiers against
that exact checkpoint.

## Superseding verification — `f9eeecf8ff4c4075cb7a5d6e2cacf5fe82057b3d`

Verdict: **PASS**.

The evidence-only checkpoint closes the preceding B findings without changing
production Rust:

- LSE support authority oracle: `15/15` PASS;
- land-surface-energy, vegetation, and persisted-restart nextest: `361/361`
  PASS, run ID `76d9fd85-8159-4e0c-a138-7fff3fcb2e23`;
- focused LSE, vegetation, and persisted-restart Clippy with warnings denied:
  PASS;
- package finding disposition: complete, no open finding and no waiver;
- exact implementation diff reconciliation: PASS at `b052158d0`;
- line-count governance: PASS with the sole >3,000-line file classified as
  pre-existing and the package delta bounded to 44 additive lines;
- gate evidence now records the complete `361/361` population and classifies
  the full-workspace quick-profile snow identity drift and all-target test
  `float_cmp` lint as pre-existing workspace debt, not package failures;
- `git diff --check b052158d0 f9eeecf8f`: PASS;
- `git diff --name-only b052158d0 f9eeecf8f -- crates`: empty, proving the
  reconciliation checkpoint is evidence-only;
- coupled-time source, immutable V10 behavior, DirectV10 Restart V1, and V11
  Restart V2 remain protected. Restart V3 receipt custody and replay
  authentication remain technically unchanged from the independently verified
  implementation checkpoint.

The pending terminal/final-disposition language is truthful pre-transition
state and may now be advanced after the independent terminal-verifier pair is
complete. Terminal Verification B finds no remaining authority,
implementation, wire, evidence, or lifecycle blocker.
