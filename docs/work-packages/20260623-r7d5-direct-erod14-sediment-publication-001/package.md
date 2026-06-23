# R7D5 Direct EROD14 Sediment Publication

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT` and continue
`R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY` until HBP/PASS sediment publication
parity passes or a narrower contract-authority hold is proven.

Rationale: R7D4 closed the direct MOFE dynamic water-transfer blocker. Focused
H2637 default/direct evidence now shows byte-identical WAT and PASS hydrology
outputs, and loss/plot differ only by run name. The remaining HBP delta is
sediment-family payload: default HBP contains nonzero event sediment
concentration, total detachment, and total deposition payload bytes, while
direct publication still emits zero erosion authority.

Included scope:

- Promote direct EROD14/EROD15 sediment publication operands into
  `DirectPublicationErosionOperands` with explicit producer authority.
- Implement direct publication of `total_detachment_kg`,
  `total_deposition_kg`, `hbp_total_detachment_kg`,
  `hbp_total_deposition_kg`, `hbp_sediment_concentration_kg_m3`, and
  `sediment_concentration_kg_m3[0..5]` only from contract-authorized direct
  producers or fail closed with a named narrower hold.
- Preserve R7D4 WAT/PASS byte identity and direct
  `compatibility_edge_invocations = 0`.
- Add anti-alias fixtures that distinguish HBP aggregate sediment fields,
  PASS `tdet`/`tdep`, and class sediment concentrations from zero authority,
  water-transfer fields, and stale runtime aliases.
- Re-run focused H2637 direct/default parity. If HBP improves but another
  in-envelope direct publication blocker remains, continue iterating until
  R7D closes or a named out-of-envelope blocker is proven.

Excluded scope:

- Default activation; compatibility remains default.
- Broad sediment-kernelization physics beyond the minimum R7D direct
  publication authority needed to close the HBP/PASS payload blocker.
- Wrapping compatibility public-output builders, compatibility WB13 rows, or
  stale logical/runtime aliases as direct authority.
- Claiming sediment-coupled MOFE01 M-G `qin/qout` closure without SED-owned
  prior-OFE `qout` plus particle/class-fraction handoff evidence.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime*.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260623-r7d5-direct-erod14-sediment-publication-001/**`

Dependencies:

- R7D4 executed hold:
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`.
- `SC-SYSTEM-001` HBP routing-boundary payload authority, especially
  `total_detachment_kg`, `total_deposition_kg`,
  `sediment_concentration_kg_m3,k`, `INV-SYSTEM-032`, and the EROD14/EROD15
  active boundary-carry addenda.
- `docs/architecture/array-native-runtime-specification.md` publication
  operand ledger rows for HBP/PASS erosion fields.

Correction authority envelope:

- Defect: `R7D-DIRECT-PRODUCTION-PUBLICATION-PARITY`.
- Observed failure: H2637 WAT/PASS are byte-identical after R7D4, but
  `H2637.hbp` still differs. Both files are `5254` bytes; `34` bytes differ.
  Decoded payload slots at offsets `928`, `936`, and `944` are nonzero in
  default and zero in direct, matching sediment concentration plus total
  detachment/deposition payload authority.
- In-scope corrections: direct erosion publication operand production,
  direct publication frame row population, HBP/PASS writer consumption,
  manifest provenance for direct erosion operands, anti-alias tests, and H2637
  parity evidence.
- Protected boundaries: do not read compatibility scheduler results,
  compatibility WB13 rows, compatibility HBP bytes, compatibility public-output
  builders, or aggregate runtime aliases as direct production authority.

Phase plan:

1. Read R7D4 handoff, `SC-SYSTEM-001` HBP/sediment payload authority, the
   array-native publication operand ledger, and current HBP/PASS writer code.
2. Identify the currently available erosion/sediment producer surfaces in
   direct production mode. Classify each candidate as direct-authoritative,
   compatibility-derived, diagnostic-only, or absent.
3. Add focused anti-alias tests for direct erosion operands and HBP/PASS writer
   consumption before production edits.
4. Implement the narrowest contract-authorized direct sediment publication
   producer bridge. If no direct-authoritative producer exists, fail closed
   with a narrower hold naming the missing EROD14/EROD15 producer and first
   implementation action.
5. Re-run focused tests and H2637 direct/default parity. Continue iterating
   through in-envelope direct publication blockers until HBP/PASS/loss/WAT
   parity passes or a named out-of-envelope blocker is proven.
6. Complete review, verification, line-count, docs, and final
   complete-or-hold disposition.

Anti-premature-stop rule:

- Do not stop after documenting that direct erosion is zero-authority.
- Do not stop after adding a bridge that only copies one aggregate scalar while
  HBP/PASS sediment class payloads remain mismatched and in envelope.
- Do not claim R7D closure while H2637 HBP differs on sediment payload bytes,
  WAT/PASS regress, loss/plot develop non-run-name residuals, or direct
  compatibility edge counters are nonzero.
- A hold is allowed only for a proven missing canonical sediment producer,
  contradictory canonical authority, invalid upstream input that correctly
  fails closed, or broader SED-owned process physics beyond this direct
  publication envelope. The hold must name exact fields, residual bytes or
  output columns, and the first code action.

Acceptance gates:

- Focused anti-alias fixture proves direct HBP/PASS sediment publication reads
  `DirectPublicationErosionOperands`, not water-transfer operands, zero
  defaults, compatibility WB13 rows, or runtime aliases.
- Direct HBP event payload publishes nonzero sediment concentration,
  detachment, and deposition when direct producer authority is nonzero.
- H2637 direct production exits 0 with
  `compatibility_edge_invocations = 0`.
- H2637 HBP/WAT/PASS/loss/manifest parity passes, or the package closes in a
  named hold with exact residual fields and an out-of-envelope blocker.
- R7D4 WAT/PASS byte identity is preserved.
- Rust closure gates pass before `complete`: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`, unless the package closes
  in a named hold before full R7D closure.
- Scoped Markdown lint and `git diff --check` pass.

Security-impact gate:

- No secrets, tokens, credentials, or machine-local absolute paths are
  committed as normative config.
- Direct production remains explicit opt-in and fail-closed.

Review requirements:

- Dual local reviews with explicit finding disposition.
- Verification artifact labels `Static:` and `Ran:` evidence.
- Conservation/publication anti-tautology review before any HBP/PASS parity
  claim.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  production files block closure.

Final disposition:
`HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
