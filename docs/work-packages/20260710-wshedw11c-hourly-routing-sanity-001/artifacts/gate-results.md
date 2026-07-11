# Gate Results

Status: `EXECUTED-HOLD-SANITY-FAIL`

Evidence mode: `Static + Ran`

| Gate | Status | Evidence |
|---|---|---|
| Focused W11C matrix | PASS | Corrected debug run `f695f3db-0627-4c28-8d97-8e5c5d023158`; 35 cases emitted. |
| Physical sanity acceptance | FAIL | Material negative storage, peak amplification, and non-terminal legacy publication. |
| Release W11C matrix | PASS | Final-tree release run `29024159-9f78-4506-9918-09c7f007af0d`; exact binary provenance recorded. |
| Protected W11B test file | PASS | Corrected final-tree run `ace36dab-5980-499c-b510-de33836bed64`; 3/3. |
| Focused clippy | PASS | Runner test target with warnings denied. |
| Formatting | PASS | `cargo fmt --check`. |
| Workspace clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`; exit 0. |
| Erosion profile | PASS | Final-tree run `cf70b31e-7aa4-4dc5-ab77-3d4480b8b540`; 313/313 passed. |
| Full workspace | PASS | Final-tree run `6f4f2479-68f4-4f2f-8e78-e4c24a80d949`; 1,678/1,678 passed. |
| Cargo deny | PASS | Advisories, bans, licenses, and sources passed. |
| Markdown lint | PASS | `markdown-doc lint` on W11C, W11D, catalog, and roadmap: 52 files, zero findings. |
| Diff hygiene | PASS | `git diff --check`; exit 0. |
| Dual review/verification | PASS | Both reviews dispositioned; Verification A `PASS — EXECUTED-HOLD-SANITY-FAIL LEGITIMATE`, Verification B `EXECUTED-PASS`. |

The `FAIL` physical-sanity gate requires an executed-hold disposition despite
all implementation-quality and regression gates passing.
