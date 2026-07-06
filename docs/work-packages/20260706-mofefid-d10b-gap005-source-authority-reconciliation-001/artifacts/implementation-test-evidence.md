# Implementation / Test Evidence (D10B S4)

Status: executed
Evidence mode: Ran

Production corrections landed (all in `ofe_routing`, per the rev-24
pre-implementation gate; see `contract-implementation-evidence.md` for the
contract-first ordering):

1. `kinematic_wave::phi()` — source-correct limiter branch (Davis 3.20 /
   Mingham 31f).
2. Two-sided FACE-BASED dissipation (Mingham 31a/31g) with zero
   domain-boundary-face flux (exact telescoping), material-interface faces
   carrying zero dissipative flux, and boundary-adjacent limiter-stencil
   mirroring.
3. Prescribed-flux upstream BC in BOTH sweeps (actual injection = `q_up dt`).
4. Donor outflow closure; ledger books the scheme-actual boundary fluxes.
5. Half-weight predictor/corrector stage clamps.
6. TRUE kinematic celerity (`dq/dh` via a perturbed friction fixed-point
   evaluation) for CFL dt selection, CFL evidence, and face `Cf` — fixes a
   latent true-Courant ~1.8 condition on the laminar `k_o/Re` limb.
7. Conservative bin-series handoff (`run_with_upstream_integral` + cascade
   `integrate_bin_series`) and bin-mean boundary-flux hydrograph export
   (`BinRecorder`).
8. Manning limb (`CellParameters::manning`, `f = 8 g n^2 / h^(1/3)`) +
   `dval::run_iwagaki_manning` (rev-24 Case-4 acceptance configuration).
9. Oracle: `ofe_routing::iwagaki_oracle` (monotone FV reference +
   characteristics fan, validation tier).

Test evidence (Ran):

- `cargo test -p openwepp-hillslope-orchestrator --release ofe_routing`:
  **61 passed / 0 failed** (includes the 5 D10B contract-derived tests that
  failed 5/5 pre-correction, the 4 oracle self-tests, and the dispositioned
  behavior-pinned updates).
- Gates: `cargo fmt --check` PASS; `cargo clippy --workspace --all-targets
  -- -D warnings` PASS (0 errors); `cargo deny check` PASS
  ("advisories ok, bans ok, licenses ok, sources ok");
  `cargo nextest run --workspace --profile full` — see
  `gate-results.md`.

Consumer note: the runtime Lane D shadow (`laned_shadow.rs`) consumes
`run_cascade` unchanged (its conservation diagnostics now measure the
corrected path); protected outputs are unaffected by construction — the
subsystem remains opt-in/shadow-only (`INV-OFEROUTE-010`), and the full
workspace suite (incl. runner protected-output and byte-identity gates)
is the enforcement surface. D14's endpoint timing MUST be refreshed
before any D15 activation claim (celerity evaluation doubled per wet cell;
handoff/export changed) — recorded in the worker handoff.
