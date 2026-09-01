# Version-16 retained-surface exact-carry post-implementation evidence

Status: `IMPLEMENTED — FOCUSED CORRECTNESS GATES PASS — FINAL REVIEWS APPROVE`

Evidence mode: `Ran + Static`

Date: 2026-08-31

## Outcome

The frozen-litter successor now retains authoritative surface enthalpy as
`U = exact(U_hi) + R_U`. Existing V3/V2 high-state bytes remain bit-identical,
nonauthoritative mirrors. Accepted OFE-to-tile credits are aggregated in exact
dyadic arithmetic in their contract order; the high term is rounded once to
nearest-even and the exact residual is retained. No tolerance, forced ULP,
snap, discarded term, carry-driven physics branch, or production diagnostic
was added.

`SurfaceLiquidCompleteOwnerProjectionV4`, V11 terminal complete-owner identity,
receipts, rollback, publication, checkpoint/reload, and the real frozen-litter
consumer all bind the exact owner. Receipt validation independently replays the
beginning owner and operands, enforces source-owner/source-receipt lineage and
canonical nested receipt bytes, and recomputes the ending high/carry and receipt
digest. Operand omission, duplication, reorder, substitution, and stale lineage
fail closed without mutating resident state.

V4 projection admission also requires an independently supplied digest of the
historical beginning LSE V3 state. The projection retains those canonical bytes
for replay, but cannot use its own nested copy as the authority anchor. A
temperature-only nested-parent substitution, even with every dependent inner
and outer digest resealed, is therefore refused against the real predecessor.
Legacy and V3-only execution retain the historical refusal of a nonzero
retained credit that disappears below binary64 spacing; unchanged-high
acceptance is enabled only while the authoritative exact V4 owner is present.

## Restart and compatibility evidence

The production tagged checkpoint transaction selects the V2 exact-hydrology
parent whenever a direct or nested V4 resident is present. A V1 projection
refuses such omission. Exact-parent restore independently validates the
unchanged nested V3 bytes plus V4 supplement and installs the restored frame
atomically; abort retains the original committed parent bytes. Existing V1
wire behavior remains unchanged for V1 inputs.

The canonical V8 tagged-parent codec retains JSON numbers through
`u64::MAX` and uses a canonical decimal string above that boundary. It rejects
negative, fractional, noncanonical, and overflowing forms. The pre-existing
accepted fixture bytes remain unchanged with SHA-256
`497b8b6e833ea99cdd3e80f7b598e99e0df1249188a7bddb8c95d6d9ca11c4e4`.

The authentic two-support split fixture advances transactions `703 -> 704`.
Its first accepted support retains the nonzero carry
`sign=1, coefficient_hex=1a1, exponent2=-59`. Checkpoint/reload followed by the
real production execution of the second support is exact-owner-, restart-,
checkpoint-, and projection-identical to uninterrupted execution, with two
physical V3 and two exact V4 publications. Receipt-free downgrade and a
poisoned successor are refused atomically.

## Ran: focused implementation suites

The following chained focused suites passed on the merged working tree:

- `cargo test -p openwepp-vegetation v8_state::tests` — 15/15;
- `cargo test -p openwepp-hillslope-orchestrator --all-features exact_surface`
  — 10/10;
- the exact-V4-only retained-high custody selector — 1/1;
- `cargo test -p openwepp-hillslope-orchestrator --all-features v4_restart_evidence::tests`
  — 2/2;
- the noncanonical nested-receipt and independently anchored beginning-parent
  projection poison selectors — 2/2;
- `cargo test -p openwepp-persisted-restart-v1 --all-features projection::tests`
  — 2/2;
- `cargo test -p openwepp-persisted-restart-v1 --all-features frozen_litter_v3_tests`
  — 7/7;
- `cargo test -p openwepp-persisted-restart-v1 --all-features transaction::tests`
  — 2/2.

The exact-surface vectors include positive and negative authentic retained
sub-ULP credits, nearest-even ties, exact cancellation to canonical zero carry,
largest-finite acceptance, overflow refusal, authentic multi-parcel/multi-tile
fusion and retained credits, retained-credit omission/source/order/OFE-basis
formula poisons, exhaustive operand/source/identity poison cases, V2/V3 mirror
poisons, rollback, and stale restart/checkpoint refusal. Signed-zero no-op bits
are preserved.

## Ran: authority and source gates

- `cargo nextest run --test land_surface_energy_balance_authority_contract`
  — 16/16 PASS, run `0e688414-a3f7-437d-a597-09a6b0ea538c`;
- `cargo nextest run --test surface_liquid_hydrology_custody_authority_contract`
  — 17/17 PASS, run `df36743f-2988-4505-913d-a644319c3c6d`;
- `bash tools/release/check_authority_suite_antievasion.sh` — PASS;
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
  — 3/3 PASS, run `bce8b83f-0278-45e2-ba72-66dda1ce5b02`.

All-target checks passed for `openwepp-vegetation`,
`openwepp-hillslope-orchestrator --all-features`,
`openwepp-persisted-restart-v1 --all-features`, and `openwepp-runner`.
The real integration compile
`cargo check -p openwepp --test dff_ws2_ksatadj_direct_runtime` passed.
`cargo fmt --all -- --check` and `git diff --check` passed.

An additional non-required full `openwepp-persisted-restart-v1 --all-features`
run passed 63/71 tests. Eight legacy Stage3 restart integration fixtures failed
before checkpoint comparison at the concurrent V39 guard
`V2 soil target transaction authority` (`snow_stage3_v11_tests.rs:1016` and
`:1411`). All V16-focused projection, exact restart, split continuity, and
production transaction selectors passed in the same source state. This is
retained as an external concurrent-lane blocker rather than represented as a
V16 full-crate pass.

## Clippy disposition

`cargo clippy --all-targets --all-features --no-deps -- -D warnings` passed for
`openwepp-vegetation` and `openwepp-persisted-restart-v1`. The corresponding
whole orchestrator invocation remains red on unrelated pre-existing/concurrent
warnings. The previously reported unreachable-public diagnostic for the
feature-gated restart executor was corrected by its feature-gated crate
re-export; all-feature/all-target compilation and the focused calls prove that
the API is reachable. A filtered audit found no remaining unreachable-public
diagnostic. This evidence therefore does not claim a whole-orchestrator Clippy
pass.

## Scope boundary

No canonical `p61` performance run was launched by this V16 lane after the
review corrections. Canonical solver captures remain owned by the concurrent
solver lane; this implementation changes exact custody and restart behavior,
not solver tolerances or temporal resolution.

## Final independent reviews

- Correctness re-review: `APPROVE` (`Ran + Static`). It verified the
  independently anchored beginning-LSE chain, temperature-only reseal poison,
  exact-V4-only unchanged-high admission, and unchanged legacy/V3 refusal.
- QA re-review: `APPROVE` (`Ran + Static`). It verified real support-704
  execution after support-703 reload, authentic signed retained sub-ULP
  vectors and poisons, canonical nested receipts, rollback, signed zero, and
  production diagnostic hygiene.
