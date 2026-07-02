# Review Disposition

Review: `review-codex.md` (Codex, `d36d052a`, hold — ADR-0033 not ratified).
D3 friction kernels accepted as shadow-only; three ratification blockers,
all **accepted and fixed**:

| # | Blocker | Action |
|---|---|---|
| D01-CX-001 | Case 3 fixture `plot_m [6.1, 3.6]` vs docx `6.1 x 1.8` | **Fixed.** Re-read the authoritative `3.1_Validation_Input.docx` (all 4 cases): Case 3 corrected to `[6.1, 1.8]`, added missing `soil: Miami silt loam` and `k_o: 500`; the paper-body `3.6` and body-only strip-length `2.45 m` recorded under explicit discrepancy/attribution keys. Cases 1/2/4 verified against the docx (Case 2 soil added); no other mismatch. |
| D01-CX-002 | D2 provenance not package-manifested (source files are gitignored local-cache) | **Fixed.** Added `source-manifest.md` with sha256 of the two source files (`3.1_Validation_Input.docx` `0aee1455...`, `Figure_4.xlsx` `2bf68787...`) under the R-63 local path, so the derivation is verifiable against the operator's cache without vendoring copyrighted material. |
| D01-CX-003 | ADR-0033 over-authorizes (ratification -> solver/cascade before SC-OFEROUTE-001 exists) | **Fixed.** Added a **Scope of ratification** section narrowing ratification to the representation + activation decision + retention of the shadow-first D3 kernels ONLY; D4 solver / D5 cascade are **gated on SC-OFEROUTE-001 being authored + ratified first** (top-down contract order, ADR-0011). Consequences + package next-steps updated to match. |

Post-fix: friction tests 6/6, clippy `-D warnings` 0, suite 154/154 (no
code change — CX fixes are fixture/manifest/ADR text). ADR-0033 remains
**Proposed**; the narrowed scope is what a ratification decision now grants.
