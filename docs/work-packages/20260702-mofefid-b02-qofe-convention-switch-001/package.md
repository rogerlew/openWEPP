# MOFEFID-B02 — QOFE = Q Convention Switch

Status: **EXECUTED — REVIEW-READY** (2026-07-02). QOFE=Q landed as default
(`INV-RUNOFFPART-032`). Gates green: single-OFE **byte-identical** (all 5
outputs, clean same-run_name comparison); H2637 `H.pass.parquet`
**byte-invariant** (runvol/peak preserved); WAT `QOFE==Q` on all rows (53,298
multi-OFE values changed); Q/latqcc/storage unchanged; suites 148+101.
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane B
(B01 finding B7 disposition). Operator-authorized 2026-07-02 ("switching
QOFE = Q is the right call"). Owner: Claude Code. Worktree: `mofefid-b02-qofe`.

## Objective

Close B01's B7 contract decision: openWEPP published the pre-`wepp_260516`
`QOFE` convention (per-OFE local-length denominator) while the ecosystem
standardized on `QOFE = Q`. Adopt `QOFE = Q` on all WB13 rows.

## Design (runvol/peak invariance is the hard constraint)

`runvol_m3` and the peak-runoff threshold both derived from the published
`qofe_publication_mm`. The switch **decouples** them: the old per-OFE
local-length expression is retained as an internal `runvol_basis_mm`
(feeds `runvol_m3` and the peak near-zero threshold, unchanged), and the
published `qofe_mm := q_publication_mm` (= Q, cumulative-length normalized).
So `H.pass.runvol` and peak are byte-invariant by construction; only the
published `QOFE` column value changes, and only on multi-OFE lanes.
Single-OFE: `cumulative_length == ofe_length` and `q_ofe == q_runoff`, so
`QOFE` already equalled `Q` — no-op, byte-identical.

Site: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
`direct_publication_runoff_operands`.

## Acceptance gates

1. Single-OFE runs **byte-identical** (all five outputs) vs pre-B02.
2. Multi-OFE (H2637): `H.pass.runvol` **byte-invariant**; peak operands
   invariant; the WAT `QOFE` column now equals `Q` on every row
   (`INV-RUNOFFPART-032`); `Q`, `latqcc`, storage columns unchanged.
3. Full suite green; fmt/clippy/deny clean.
4. Contract `INV-RUNOFFPART-032` + WB13 rule generalization landed.

## Downstream (wepppy) note

Consumers recovering canonical hillslope runoff volume from `H.wat` must use
`Q(outlet) x A_total` or `H.pass.runvol` directly — **not**
`QOFE x per_OFE_area`, which under `QOFE = Q` no longer carries the legacy
`n x` / `1/n` cancellation. `H.pass.runvol` is unchanged, so channel routing
and watershed reports that consume it are unaffected. Operator owns wepppy
release sequencing (the affected reports are documented in the wepp-forest
brief §"Audit and consumer formulas").


## Execution record

- Single-OFE byte-identity (Ran): initial run showed loss.json/plot diffs
  from a **run_name confound** in the reference (loss.json/plot embed
  run metadata); a clean same-run_name old-vs-new comparison
  (marcell_conifer_mn) is **byte-identical on all five outputs**.
- H2637 (Ran): `H.pass.parquet` sha256 byte-invariant vs pre-B02 (runvol
  and peak preserved by the retained per-OFE basis); WAT `QOFE==Q` True on
  every row; 53,298 multi-OFE QOFE values changed; `Q`, `latqcc`,
  `Total-Soil`, `Ep`, `Dp`, `Snow-Water` all unchanged.
- Test migration: `r7d4_publication_q_uses_runoff_geometry_scale_independently_from_qofe`
  encoded the pre-B02 convention (asserted `Q != QOFE`); migrated to
  `r7d4_publication_qofe_equals_q_with_independent_runvol_basis` (asserts
  `QOFE == Q` bitwise AND `runvol != QOFE x area`, proving the runvol basis
  stays independent).
- Suites 148 (orchestrator) + 101 (runner) green; fmt/clippy `-D warnings`
  clean.
