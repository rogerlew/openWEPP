# M-G erosion qin/sediment coupling decision evidence

Status: executed; contract-pinned follow-on

Evidence mode: Static + Ran

## Decision

M-G does not implement new erosion-routing math. The evidence says MOFE01's
water seam exposes the transfer operands that EROD14 needs, but does not own
full downstream erosion `qin`/sediment coupling inseparably.

The accepted boundary is:

- MOFE01 water-routing closure owns `TransferInput`, `TransferOutput`,
  `UpStrmQ`, `SubRIn`, hourly carry arrays, public `Q`, and public `QOFE`.
- Accepted downstream erosion `erod14_qin` for OFE `i > 1` requires prior-OFE
  erosion `qout` plus incoming particle/class-fraction handoff lineage.
- Current EROD14 `qin` seeding from water-transfer provenance is compatibility
  continuity only. It must remain operator-visible and must not be treated as
  sediment-coupled closure.

## Static

- `mofe-routing-port-scope.md` identified the scope seam: current runner
  `erod14_qin` reads `UpStrmQ`, while pinned legacy `xinflo.for:130-151` and
  `route.for:139-154` couple prior-OFE `qout`, current `qin`, and incoming
  sediment fractions.
- `mofe-per-ofe-state-architecture.md` explicitly left erosion/sediment
  `qin/qout` to M-G and required M-E to expose the water transfer state without
  implementing sediment routing.
- Runtime inspection confirmed `seed_mofe03_wave2_runtime_surface_inputs`
  seeds `erod14_qin` from `UpStrmQ` on active Wave-2 paths, and EROD14 consumes
  `qin/qout/qostar` inside deposition/enrichment math.
- Canonical contracts now pin the boundary:
  - `SC-RUNOFFPART-001` version 44 adds `INV-RUNOFFPART-030`.
  - `SC-WATBAL-001` version 160 adds `INV-WATBAL-099`.
  - `SC-SED-001` version 41 adds `INV-SED-012` and the legacy `xinflo`/`route`
    authority anchor.
  - `SC-SYSTEM-001` version 83 adds `INV-SYSTEM-032`.
- Operator visibility is implemented in run manifests:
  - `erod14_qin_source_policy`
  - `erod14_qin_sediment_coupled`
  Active multi-OFE Wave-2 runs currently report
  `water-transfer-only-mofe01-mg-sediment-coupling-follow-on` and `false`.

## Ran

- `cargo fmt --check`: PASS.
- `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`: PASS.
- `cargo test --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`:
  PASS.

No semantic comparator comparison was required for M-G because this increment is
a contract/manifest boundary decision, not a WAT value-acceptance increment. No
comparator subagent was used.

## Follow-On

Create a sediment/erosion coupling package before any closure narrative claims
MOFE erosion coupling complete. That package must:

- source downstream `erod14_qin` from prior-OFE erosion `qout`, not from public
  WAT rows or aggregate runoff;
- carry incoming particle/class-fraction lineage through the same OFE handoff;
- add a two-OFE vector where the upstream `qout` source can diverge from public
  `UpStrmQ`;
- flip `erod14_qin_sediment_coupled` only after the above closes under
  `SC-SED-001#INV-SED-012`.

## Claude review (2026-06-13) — M-G ACCEPTED (boundary decision, no scope creep)

Evidence mode: Ran (git scope check + contract/evidence read).

Correct decision, matching the M-D pre-authorization ("implement only if the
water seam owns it inseparably; otherwise contract-pin and emit the
follow-on"):

- **Sound and legacy-cited**: `erod14_qin` couples prior-OFE erosion `qout`
  and incoming sediment class-fractions (`xinflo.for:130-151`,
  `route.for:139-154`) — not derivable from the water-routing transfer state
  (`UpStrmQ`/`SubRIn`). Boundary pin is correct.
- **No scope creep**: zero production erosion-kernel edits (git diff: only a
  contract test). No erosion math implemented under a "decision" increment.
- **Boundary explicit/auditable**: `SC-SED-001` v41 `INV-SED-012` +
  manifest `erod14_qin_source_policy` /
  `erod14_qin_sediment_coupled=false` — the un-coupled state is published,
  not silent.

Carry-forward for M-H closure: the sediment-coupling follow-on is currently a
policy string + evidence note; M-H must **formalize it as a named
backlog/handoff item**, and the closure narrative must state plainly that this
rung delivers **water-routing closure** with inter-OFE **sediment coupling as
a deliberate named follow-on** — not "MOFE complete." Accepted; M-H is the
final increment.
