# Codex Re-Confirmation

Verdict: `REOPEN`

Blocking defect: `WSHED-W11A-RECONFIRM-001` — v52 still aliases the
wave-routing lateral-discharge series and the Chapter-13 per-unit-length
erosion-profile lateral inflow under `qlat(it)`. The conversion and unit binding
are absent, so H1 does not yet provide a unique executable operand map.

## Evidence header

- `Static`: reviewed closing commit
  `a9e4637c61c53a789efc8ce13a1c4f70661708b3`, `SC-ROUTE-001` v52,
  `codex-review-disposition.md`, the amended W11 handoff, and the reconciled
  package artifacts. Checked each amendment against the original findings in
  `codex_posthoc_review.md`.
- `Ran`: `git show`/line-numbered v51-to-v52 inspection; confirmed the pinned
  baseline remains `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; inspected
  `wshchr.for`, `chnrt.for`, the routed-wave Rust publication surface, and the
  WS20 flow partition; ran `git show --check a9e4637c` and the binding-exposure
  checker (PASS, seven rows). No build, simulation, comparator, or production
  test was run; this is a contract re-confirmation.

## Closure status

### 1. H1 hydraulic-profile operand map — `still-open`

V52 now makes the storage posture and intended profile anchors explicit:
`qe(it) := q1(it)`, `qt(it) := qin(it)`, and `qlat(it) :=` the wave-routing
lateral series (`SC-ROUTE-001.md:147,631-641`). It also rejects event-peak
fractions and adds vector 11 (`:768-775`). Those changes close the original
choice among peak-fraction, inlet-only, outlet-only, and storage-reconciled
profiles.

The binding is nevertheless not executable uniquely because `qlat(it)` names
two dimensionally different quantities:

- The wave-routing series is assembled as total lateral discharge in `m^3/s`
  (`/workdir/wepp-forest_260430_baseline/src/chrqin.for:1-5,65-70` and
  `wshchr.for:123-191`). Baseline divides it by channel length only later
  (`wshchr.for:329-331`). The current Rust public surface likewise exposes
  `RoutedChannelWaveState::qlat_m3_s`
  (`crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs:264-270`;
  publication at `kernel/direct.rs:618-622`).
- The Chapter-13/`chnrt` erosion-profile operand is effective lateral inflow
  **per unit length**, explicitly `ft^3 s^-1 ft^-1`
  (`/workdir/wepp-forest_260430_baseline/src/chnrt.for:217-242`). The current
  Rust segment core also consumes `qlat_cfs_per_ft`
  (`kernel/routing/02_ws20_segment_routing.rs:52-99,1023-1051`).

The contract's Variables and Units row instead groups `qlat` with discharge
symbols as `ft^3 s^-1` (`SC-ROUTE-001.md:110-111`), while v52's storage
expression and vector 11 multiply `qlat(it) * lc` (`:147,774-775`), which
assumes the per-unit-length meaning. Conversely, the disposition and handoff
call it an already-published WS11 state symbol, which is the total-`m^3/s`
meaning. An executor must still decide whether to pass the published total,
divide by `lc`, or introduce a distinct effective-lateral construction.

Required closure: define separate symbols and units, for example
`qlat_wave_total(it)` (`m^3/s`) and `qlat_erosion(it)`
(`ft^3 s^-1 ft^-1`), bind the SI-to-English and `/lc` conversion explicitly,
and update INV-ROUTE-016, the operand table, vector 1, vector 11, the storage
expression, and the W11 handoff to use the appropriate symbol. The anti-alias
fixture must distinguish raw total lateral discharge from the normalized
per-length operand.

### 2. M1 erosion exposure/normalization split — `closed`

Variables and Units defines `t_exp(it)` and `t_norm(it)` with distinct roles
(`SC-ROUTE-001.md:125`). `INV-ROUTE-018` binds `t_exp` to every former
`timsh` slot, `t_norm := dtchr` to every former `tb` denominator slot, retires
the triangular factor two, and states the flux/geometry reconstruction
equations (`:149`). The operand table repeats the split (`:640-641`), and
vector 1 pins both values (`:717-727`). No erosion-clock or normalization
choice remains.

### 3. M2 `d_i` and `rho_soil` definitions — `closed`

The symbol table now binds `rho_soil` as in-place bulk mass density in
`lbm/ft^3`, with baseline `wtdsoi` provenance, and binds `d_i` to baseline
`di = excess * Kch * (tau - taucr)` in `lbm ft^-2 s^-1`
(`SC-ROUTE-001.md:123-126`). `INV-ROUTE-018` uses the same definition in the
`timpot` rule (`:149`), and `INV-ROUTE-019` uses the mass-density convention
for constructive geometry-mass closure (`:150`). The earlier alias and
lbm/lbf choices are closed.

### 4. H2 pinned-`dcap.for` realization and terminals — `closed`

`INV-ROUTE-018` now makes pinned `dcap.for` the authority and the migrated
lanes the implementation target, not the reverse (`SC-ROUTE-001.md:149`). It
names both divergent terminals and requires correction before interval reuse.
`GAP-ROUTE-014` records the open implementation defect and W11 Phase-B
obligation (`:793`); vector 10(b)/(c) requires capped-geometry and
post-contact/subcritical terminal behavior plus independent geometry-mass
reconstruction (`:753-767`). This closes the authority contradiction without
pretending the Rust correction has already landed.

### 5. M3 source narrowings / L1 record reconciliation — `still-open`

M3 is `closed`:

- CREAMS's “not a physics claim” interpretation is mixed-graded.
- KINEROS is narrowed to zero upper-boundary-capacity deposition mode, with
  whole-reach dry deposition labeled inference.
- HEC-RAS is narrowed to bed-change state carry with threshold-gated
  cross-section refresh, and geometry-update authority is returned to Chapter
  13/pinned lineage (`SC-ROUTE-001.md:77-80`).
- `authority-matrix.md:430-437` records the HEC correction.

L1 remains `still-open`, although non-blocking by itself:

- `final-disposition.md:21` still says ten vectors while its reopen addendum
  says eleven at `:95-98`.
- `final-disposition.md:23,49-50` still says all verification notes were
  addressed, while `:64-65` and amended `gate-results.md:20` say note 4 was
  deferred.
- `w11-handoff.md:5-10` still identifies v51 as the verified/current authority
  even though its later rows rely on v52.

`contract-disposition.md` and `gate-results.md` otherwise carry the corrected
anchor/vector counts and fallback-only storage wording.

## Final verdict

`REOPEN` for `WSHED-W11A-RECONFIRM-001`.

M1, M2, H2, and M3 are closed. H1 remains a science/units binding defect, and
L1 retains minor record inconsistencies. `WSHED-W11-HOLD-001` may not yet stand
lifted, and W11 should not resume Phase B until the total-versus-per-length
`qlat` mapping is made explicit and the confirmation artifact is rechecked.

No contract or production file was edited in this pass.
