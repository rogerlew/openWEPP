# R151 disposition and workspace-gate suspension

Status: `UNRESOLVED — PACKAGE SUSPENDED`

Evidence mode: `Static + Ran`

## Canonical r151

Ran against a freshly rebuilt 64 MiB-stack canonical DFF binary:

`env RUST_MIN_STACK=67108864 /usr/bin/time -v nix develop -c cargo test --test dff_ws2_ksatadj_direct_runtime dff_ws2_forest_high_severity_loam_runs_with_live_direct_ksatadj_effect -- --exact --nocapture`

Retained log: `/tmp/wghl_001d_v57_64m_r151.log`.

SHA-256:
`d4a26e0194a769c1303cc7500ea254d2a9dbcdaa08e05f65188e4ba07ea27252`.

Result: `FAIL`, exit `101`; wall `5:09.55`, user `309.20 s`, system
`0.28 s`, peak RSS `442368 KiB`.

The canonical path now reaches the V56/V57 specialization on exact-floor
support `1800..1860 s` and fails with:

`V11 adaptive candidate requires refinement: frozen temperature-primary safeguarded physical solve`

This is materially different from r148/r149, which remained
`EnthalpyPrimaryWithCnHeat` at `2100..2160 s`, reached shared budget `63/96`,
and failed V55 with `PrivateQLatticeNoWitness`. R150 proved the remaining
eligibility exclusion was an implementation error that confused the intended
terminal one-volume domain with occurrence of a terminal event. Removing that
error makes V57 dispatch, but r151 demonstrates that the temperature-primary
solver itself remains unresolved.

Retained predecessor evidence:

- r148 SHA-256
  `3665414efca21b49677c7eedc1c9d3ee21cb3973afb6cae2b7a1c88674e65e7c`,
  wall `6:46.40`, RSS `442596 KiB`;
- r149 SHA-256
  `79e4fa39bf51b248a8b41de2ffdc61f2668f9c1ed4332773b973c03d70f98bf3`,
  wall `6:44.41`, RSS `442920 KiB`;
- r150 SHA-256
  `524ec476c3f4fe38e54b906b3733711b7df076a78b8137e784d1fb72240c09bc`,
  wall `6:43.93`, RSS `443252 KiB`.

## Truthful qualification disposition

V56 and V57 focused tests and source contracts pass, but neither has canonical
end-to-end acceptance evidence. R151 did not finish the one-day fixture, so it
provides no accepted/rejected microstep count, completed step-width
distribution, one-day runtime qualification, or final mass/energy ledger
closure. Those claims remain unavailable. The workspace gate is not lifted and
the package is not complete.

## Owner-directed stop boundary

Stop after dispositioning r151. Do not authorize or implement V58 or any
further exact-witness, receipt-lattice, carry, or eligibility successor. The
current worktree is preserved. Temporary r149/r150 audit diagnostics were
removed before suspension; no new solver diagnostic is persisted.

Further solver work is transferred to the separately authorized replacement
`20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001`.
That package must first establish representative runtime budgets and physically
justified convergence tolerances, and it must retain:

- Lane D MOFE;
- RHESSys-derived native-vegetation ET;
- the new non-CoE Stage 3 snow.

All conservation, custody, phase, topology, receipt, rollback, event, and
fail-closed requirements remain binding.
