# Codex Review - MOFEFID-C01 Lateral-Flow Envelope Promotion

Date: 2026-07-02
Reviewed head: `33c0a3ef`
Scope: review only. No production code, tests, or contracts were modified.

## Evidence Classes

Static:

- Read `package.md`, `artifacts/extraction.md`, `artifacts/envelope-derivation.md`.
- Read `SC-SUBHYD-001.md` rev 13, especially `REF-SUBHYD-OBSERVED-LATERAL-ENVELOPE` and `INV-SUBHYD-033`.
- Read `docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`.
- Read `tests/fixtures/forest_lateral_flow_authority/README.md` and fixture inventory.
- Spot-checked source/reference anchors by targeted `pdftotext`/`rg` reads against WS10, Panola, Weiler, Maimai, Mosley, McGlynn, and Woods source files/transcriptions.

Ran:

- `bash tools/release/check_authority_suite_antievasion.sh` - PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` - PASS, 2 tests.
- `pdftotext references/copyrighted/McGuire_McDonnell2010_ws10_connectivity.pdf - | rg -n "2220|30 mm|18\\.8|37|0\\.31|stormflow|runoff" -i`
- `pdftotext references/copyrighted/TrompvanMeerveld_McDonnell2006_panola_threshold_1.pdf - | rg -n "1240|13|0\\.63|55 mm|40|60 mm|147|RC|1%|80%|threshold" -i`
- `pdftotext references/copyrighted/Weiler2005_subsurface_stormflow_encyclopedia.pdf - | rg -n "15|35|threshold|Panola|Maimai|stormflow|runoff coefficient" -i`
- `pdftotext references/copyrighted/mcglynn2002.pdf - | rg -n "1550|2600|34|376|runoff|annual" -i`
- `rg -n "1320|54%|2450|2600|mainly above 35|25 and 130|60 cm|5573|23 mm|commencement|melted snow|74%|annual" references/copyrighted/*.md -S`

Not run:

- I did not run a C03 judgment harness or regenerate H2637 event classifications. C01 states that C02/C03 own the harness and formal judgment.

## Findings

### C01-CX-001 - Accepted Candidate - H2637 "inside all four tiers" is over-claimed before event-tier evidence exists

Evidence class: Static.

`package.md:3` says H2637 post-DC01 "sits inside all four tiers", while `package.md:14-16` says C01 delivers only the envelope and that the harness/judgment are C02/C03 with no C01 judgment. `artifacts/envelope-derivation.md:47-50` and `:82-85` provide annual ENV-Y and ENV-ET H2637 readings, but the ENV-T and ENV-E sections at `artifacts/envelope-derivation.md:52-73` define observed event laws only; they do not provide an H2637 event commencement threshold, above-threshold event ratio distribution, or ascending storm-size/antecedent-wetness classification. The closing claim at `artifacts/envelope-derivation.md:109-114` therefore consumes the event tiers as if C03 had already run.

This conflicts with `SC-SUBHYD-001.md:298`, where ENV-T and ENV-E are explicit event-shape tiers judged only after closure/routing/export prerequisites hold. Annual Y and ET can be called in-band on the presented evidence; event tiers cannot.

Disposition recommendation: accept and narrow the package claim before close. Either revise C01 wording to "ENV-Y and ENV-ET are in-band; ENV-T and ENV-E are defined but unjudged pending C03", or attach C03-equivalent event evidence showing H2637's event threshold, conditioned event ratio band, and ascending shape.

### C01-CX-002 - Deferred Candidate - Verification-debt wording is ambiguous for a ratified contract

Evidence class: Static + Ran.

`package.md:62-63` says research-agent numbers enter the contract only after page-cited verification. `artifacts/extraction.md:3-5` says the C01 extraction is page-cited and verified against PDFs/transcriptions. But `artifacts/envelope-derivation.md:8-10` says individual anchor numbers still carry "verification debt" and that C03 re-verifies any number before a verdict consumes it. At the same time, `SC-SUBHYD-001.md:70` and `:298` have already promoted the numeric envelope into canonical contract text.

My source spot checks did not find a contradictory anchor. They supported the major annual and event-shape inputs I checked, including WS10 annual precipitation/runoff ratio, Panola class/threshold limits, Weiler's cross-site threshold band, and Maimai annual-yield anchors. The issue is governance wording, not a proven bad number.

Disposition recommendation: clarify the debt boundary. If C01 verification is complete and C03 is only independent re-verification before a verdict, change the artifact language accordingly. If the numbers are not yet verified enough for contract authority, then the contract promotion should remain provisional until that verification is complete.

### C01-CX-003 - Accepted Candidate - Package gate log did not record required external-authority source guards

Evidence class: Static + Ran.

The C01 artifact set has no gate log, and I found no package-local record of the required external-authority anti-evasion/source guards. Because this package changes external-authority suite posture, those guards are load-bearing review evidence.

I ran both independently and they passed:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

Disposition recommendation: record these review results as satisfying the gate, or rerun and add an implementer gate note before package closure.

## Accepted Checks

- The promoted invariant is correctly framed as an ADR-0011/ADR-0017 investigation flag, not legacy parity and not a hard-fail: `SC-SUBHYD-001.md:298`.
- The metric choice sums surface export and lateral export for annual stream-yield comparison, avoiding the single-channel category error: `package.md:24-33`, `artifacts/envelope-derivation.md:24-27`.
- Panola is limited to threshold shape and excluded from annual magnitude; Coweeta remains context-only with no direct `latqcc` verdict; per-trough coefficients and single-channel/two-channel comparisons are excluded as bounds: `artifacts/extraction.md:17-20`, `:70-76`; `artifacts/envelope-derivation.md:97-105`; `SC-SUBHYD-001.md:298`.
- Source spot checks supported the major anchors used to derive ENV-Y, ENV-T, and ENV-E. I did not find a source-number contradiction in the checked corpus.
- The backlog promotion itself is appropriately scoped: it says the envelope is ratified and that the judgment run is C03, not C01 (`docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md:3`).

## Review Outcome

Do not close C01 as written until C01-CX-001 is dispositioned. The envelope contract looks reasonable as a promoted external-authority frame, but the package should not claim H2637 is inside all four tiers without event-tier judgment evidence. C01-CX-002 and C01-CX-003 are smaller governance/evidence cleanup items.
