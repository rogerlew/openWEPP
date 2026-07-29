# Gate Evidence

Status: `complete`

Evidence class: `Ran + Static`

| Requirement | Command/evidence | Result |
| --- | --- | --- |
| Exact runner build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS; current release binary built in 1m15s. |
| Complete matrix | `.venv/bin/python .../tools/execute.py --jobs 10` | PASS; 261/261 runs, 259 forest/member and two open controls; full rerun retained 24,012 per-run period-operand rows. |
| Analysis | `.venv/bin/python .../tools/analyze.py` | PASS; nine lane summaries, 13 admissible observation summaries, 13 verdict cells. |
| Figures | `.venv/bin/python .../tools/plot_results.py` | PASS; six accessible plot-only SVGs with six Markdown caption/ancillary-information sidecars. |
| Terminal validator | `.venv/bin/python .../tools/validate.py` | PASS; independently reconstructed all 261 run summaries, nine ensemble summaries, and 13 observation summaries; checked 95,526 climatology rows, 783 score rows, 13 cells, six source-digest-bound figures, and six paired sidecars. |
| Rebuild determinism | repeat analysis/renderer with before/after SHA-256 lists | PASS; no digest difference. |
| Visual inspection | all six canonical SVGs converted with `rsvg-convert`, assembled into a contact sheet, and inspected | PASS; SVGs contain plots only, while captions, units, peak/melt-out context, source-null semantics, and advancement boundaries remain legible in paired sidecars. |
| Focused contracts | `cargo nextest run --test cancov_stratified_observations --test canopy_litter_external_boundary_contract` | PASS; 19/19. |
| Runner component | `cargo nextest run -p openwepp-runner` | PASS; 221/221 in 108.010s. |
| Package Markdown | `markdown-doc lint --path <CAL-06-package>` | PASS; 30 files, zero errors/warnings. |
| Roadmap Markdown | `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md` | PASS; zero errors/warnings. |
| Figure-contract Markdown | `markdown-doc lint --path docs/planning/canopy-cal-06-figure-contract.md` | PASS; zero errors/warnings. |
| Work-package catalog Markdown | `markdown-doc lint --path docs/work-packages/README.md` | PASS; zero errors/warnings. |
| Diff hygiene | `git diff --check` | PASS. |
| Independent review | `review-agent-a.md`; `review-agent-b.md` | PASS/PASS; all seven named findings corrected and dispositioned. |
| Independent verification | `verification-agent-a.md`; `verification-agent-b.md` | PASS/PASS; inventories, reconstructions, hashes, source semantics, figure bindings, write set, and final disposition reproduced. |
| Plot/sidecar follow-on review | appended sections in `review-agent-a.md`; `review-agent-b.md` | PASS/PASS; plot-only visible text, sidecar semantics, accessibility, exact source bindings, deterministic regeneration, and unchanged scientific results confirmed. |
| Plot/sidecar follow-on verification | appended sections in `verification-agent-a.md`; `verification-agent-b.md` | PASS/PASS; six paired artifacts, 22 resolvable sidecar links, plot-only XML text, byte-identical regeneration, exact scientific-object identities, and final disposition reproduced. |
| Harvard downstream legend correction | targeted appendices in both reviews and both verifications | PASS/PASS/PASS/PASS; visible/data strata are exactly open, deciduous, and mixed; conifer is absent, fault-injection guards pass, rendering is byte-identical, and scientific hashes are unchanged. |

The first execution attempt failed before result publication because WAT
simulation year was misread as calendar year. `execution-incident-001.md`
records the correction and discarded attempt.

Harvard SWE is excluded rather than corrected: official centimeter metadata
contradicts the raw depth-density-SWE identity by approximately tenfold.
`harvard-swe-authority-contradiction.md` retains the evidence.

Harvard density scoring uses only daily bulk density. Vertical profile layers
are retained as `NOT_EVALUATED_SCALE_MISMATCH`; see
`observation-operator-disposition.md`.

The one-time 38-run Harvard rescore confirmed exact trace/WAT digest identity
with the original executions before the complete 261-run matrix was rerun from
the corrected operator. The terminal result is therefore direct full-matrix
output, not a mixed stale-summary patch.
