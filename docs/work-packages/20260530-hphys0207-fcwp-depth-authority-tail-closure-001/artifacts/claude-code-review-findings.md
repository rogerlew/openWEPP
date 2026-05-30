# HPHYS0207 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-30
Scope: FC/WP depth-authority / normalized-tail change in
`crates/openwepp-runner/src/hillslope/mod.rs` (WB13 builder) and
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`,
plus supporting tests, contracts, and the 39-hillslope rerun under
`/tmp/hphys0207_20260530T042607Z/parity/`.

Evidence classes: **Static** (read source / contract / artifact) and **Ran**
(command executed). Runs over package outputs were produced by the package
executor and analyzed (and, where noted, re-counted) by this reviewer.

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## I-1 — Depth-authority objective met; WP recovered, FC improved to its prior residual floor

Static. FC/WP store seeds are now aggregated over the normalized (expanded)
layer set in `compute_wb13_profile_symbols_from_legacy_seed`
(`profile_fc_store_mm += corrected.thetfc * thickness_mm` in the same loop that
yields the exact-matching porosity), and the WB13 builder consumes those
storage seeds directly. The HPHYS0206 parser-grid `ofe_corrected_*` seed source
is removed.

Ran (reviewer re-count over the 39 per-hillslope semantic reports): FC fail
27/39, WP fail 1/39, ProfilePorosityCap fail 0/39 — matching
`hphys0207_disposition.md`. Ran (reviewer, duckdb), H1: ProfileWPStore = 31.4016
(legacy 31.40; HPHYS0206 was 29.40), ProfileFCStore = 111.03 (legacy ~113.6;
HPHYS0206 was 107.14), ProfilePorosityCap = 1092.71, ProfileDepth = 1800.0. The
WP depth-tail deficit (HPHYS0206 I-3) is closed; FC moved toward legacy but H1
remains ~2.3% high and in the FC-fail set.

## I-2 — WB13 publication authority is reversed relative to HPHYS0202

Static. The WB13 builder no longer aggregates per-layer
`thetfc_#### · dg_####`; it reads `wb13_profile_depth_mm`,
`wb13_profile_porosity_cap_mm`, `wb13_profile_fc_store_mm`,
`wb13_profile_wp_store_mm` directly as authoritative. This inverts the
HPHYS0202 direction (per-layer aggregation authoritative; storage seeds
"non-authoritative adapter diagnostics"). `SC-WATBAL-001` now carries both: the
HPHYS0202/0205 sections stating the storage symbols are "non-authoritative …
must not override layer-authoritative" publication, and the HPHYS0207 amendment
(rev 48) "aligned WB13 FC/WP publication authority to normalized-profile runtime
storage symbols." The superseded sections are retained (the authority-section
presence test still asserts the HPHYS0202 layer-aggregation section exists); the
authority statements are reconciled by amendment-history framing rather than
revision of the superseded text.

## I-3 — New profile-level ordering invariant added

Static. The WB13 builder adds
`profile_porosity_cap_mm >= profile_fc_store_mm >= profile_wp_store_mm`
(`SIMOUT-E-001`). This is a profile-aggregate check, not per-layer; a profile
whose aggregates satisfy the ordering while an individual layer violates
`fc <= por` or `wp <= fc` would pass. The rerun did not trigger it.

## I-4 — Two FC/WP representations now coexist on different grids

Static. WB13 publication uses the normalized-grid storage seed (I-1). The
per-layer `thetfc_####`/`thetdr_####` symbols remain emitted via the HPHYS0206
parser-grid remap (`map_corrected_layer_theta_symbols_to_parser_layers`) and
remain the inputs mapped for WB14 (`Wb14SoilThetaFieldCapacity`/`…Residual`),
the WB18 upper-limit derivation (`wb11_seed`), and the MOFE03 path. WB13 FC/WP
and the WB14/WB18 FC/WP inputs are therefore now sourced from different layer
grids (normalized vs parser).

## I-5 — WB13 builder no longer reads per-layer symbols; storage seeds hard-required

Static. The change removes the `nsl` loop, the per-layer
`thetfc/thetdr/dg/theta_s` reads, the per-layer WB13 guards, and the `theta_s`
porosity-aggregation fallback. WB13 now hard-fails if any of
`wb13_profile_depth_mm`/`_porosity_cap_mm`/`_fc_store_mm`/`_wp_store_mm` is
absent (previously porosity could fall back to a `theta_s_####` aggregation).
The direct-probe unit tests and the integration test were renamed
`hphys0202_*` → `hphys0207_*` and flipped to assert the published values equal
the injected storage symbols (e.g., `100.0`/`55.0`) rather than a layer
aggregate; consistent with the implementation.

## I-6 — FC residual is unchanged from HPARITY02 and is now the isolated open item

Ran/Static. FC fail count 27/39 equals the HPARITY02 baseline (27 → 27, "no
change" per disposition); WP is 1/39. The FC residual is present in the direct
normalized-grid aggregation (the same aggregation that makes porosity exact and
WP match on 38/39), independent of the publication-authority and grid changes in
this arc. H1 ProfileFCStore 111.03 vs legacy ~113.6 (~2.3%).

## I-7 — Gates; working tree uncommitted

Ran (reviewer, from `/home/workdir/openWEPP`, HPHYS0207 working tree):
`cargo fmt --check` → exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0;
`cargo test --workspace` → exit 0. Static: HPHYS0207 changes are unstaged and
the package directory is untracked; `git log` HEAD is `a8e7b76` (HPHYS0206
execution); no HPHYS0207 commit exists.

---

## Post-Review Follow-up — Codex (2026-05-30)

Evidence classes: **Static** (source/contract diff inspection) and **Ran**
(targeted tests + format gate).

1. I-3 hardening implemented: per-layer corrected-moisture ordering now fails
   closed during normalized correction (`thetdr <= thetfc <= porosity`, finite
   domain required) in
   `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`.
2. I-3 guard coverage added:
   `hphys0207_corrected_layer_moisture_preserves_per_layer_storage_ordering`
   in
   `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
   asserts per-layer ordering over normalized corrected layers from
   `valid_9002.sol`.
3. I-2 contract-precedence clarity tightened: superseded HPHYS0202/0205/0206
   FC/WP authority sections are explicitly marked historical in
   `SC-WATBAL-001` and `SC-SYSTEM-001`, and HPHYS0207 supersession markers are
   asserted by
   `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`.
4. Ran follow-up checks:
   - `cargo fmt --check` → pass
   - `cargo test -p openwepp-hillslope-orchestrator hphys0207_` → pass
   - `cargo test --test hphys0202_profile_fc_wp_lineage_contract` → pass
   - `cargo test -p openwepp-runner hphys0207_` → pass
5. Remaining open item unchanged: I-6 FC residual posture remains package-level
   `HOLD` follow-on scope (`27/39` FC fail columns, `1/39` WP fail column).
