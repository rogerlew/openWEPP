# Terminal Verification A

Evidence mode: `Static + Ran`.

Disposition: `PASS`.

Scope: verify the amendments resolving independent review A. Write access was
limited to this artifact. No model subprocess was launched.

## Checks Run

- `.venv/bin/python -m py_compile tools/analyze_failures.py` — `PASS`.
- `.venv/bin/python tools/analyze_failures.py --self-check` — `PASS`.
- Independent Python reconstruction/assertions over
  `artifacts/failure-attribution.json` — `PASS`.
- SHA-256 verification of all six frozen inputs — `PASS`; every current byte
  identity matches the JSON binding.
- XML parse of all three SVG figures — `PASS`.
- `git diff --check` — `PASS`.

The temporary Python bytecode created by the syntax check was removed after the
run; it is not package evidence or worktree scope.

## Amendment Verification

### Open-control versus canopy-longwave identification — PASS

The machine-readable attribution now identifies exactly five
`sublimation_sensitive_open_control` timing failures and zero
canopy-longwave-identifying failures. Independent reconstruction confirms all
five timing rows have `stratum == open`, all 16 selected primary metrics have
`L - B == 0`, and the row roles state that the timing evidence cannot identify
canopy longwave.

The synthesis, criterion-fitness report, process-attribution table, timing
figure/sidecar, package outcome, roadmap, and catalog consistently state the
bounded inference: the five timing failures test sublimation/open-control
behavior and block promotion of the combined LS bundle, but neither demonstrate
nor refute canopy-longwave efficacy. EB-04S nonpromotion remains unchanged.

### Factorial interaction and Niwot cases — PASS

The JSON summary and independent calculation agree on 13 nonzero interactions:
12 mitigate additive primary error, one amplifies it, and three are zero. The
error-response figure now includes an explicit L×S-interaction column.

Both Niwot cases reconstruct exactly and are explained in the synthesis,
process attribution, and timing sidecar:

- peak depth: S main error `-4.0 d`, interaction `+4.0 d`, LS combined error
  `0.0 d`;
- peak SWE: S main error `+2.5 d`, interaction `-2.0 d`, LS combined error
  `+0.5 d`.

The prose correctly treats these as factorial associations and avoids unique
causal assignment, particularly for open controls.

### Full metric versus primary-scalar invariance — PASS

Independent checks confirm zero full metric objects are identical between B and
LS, while exactly one selected primary error is unchanged. Generated JSON,
gate evidence, synthesis, package outcome, and figure sidecar now preserve that
distinction; the former “none is numerically invariant” wording is absent from
the current conclusion.

### Mixed depth-SWE ownership — PASS

Both depth-SWE rows now carry `target_sensitivity == mixed_ambiguous`, and the
human artifacts identify mixed density/layer-geometry and canopy-interception
ownership. The reported 11 density/geometry rows explicitly include two
ambiguous guards rather than presenting all 11 as clean adjacent-process
authority.

### No-materiality boundary — PASS

The package, synthesis, criterion report, process attribution, all three figure
sidecars, root roadmap, campaign roadmap, and catalog state that the `15 away / 1
unchanged` classification uses exact sign only and does not establish practical
or scientific materiality. No threshold was introduced and no unchanged
ordinal label was reinterpreted as a regression.

## Final Assessment

All findings from review A are resolved. The amended evidence supports the
qualified diagnostic conclusion and remains within the retained-only,
no-retroactive-promotion boundary. No science, governance, provenance, or
validation blocker from this review remains.

## Final Closure Addendum

Evidence mode: `Static + Ran`.

Final closure disposition: `PASS`.

An exact-terminal audit after closure-artifact and prompt reconciliation found:

- `package.md` is `complete / DIAGNOSTIC_COMPLETE`; every progress item is
  checked, including dual review/disposition, dual verification, roadmap and
  catalog reconciliation, exact diff, documentation, figures, and final
  disposition.
- `gate-results.md` marks every declared current-scope gate `PASS` and closes
  `DIAGNOSTIC_COMPLETE`; no `FAIL`, `BLOCKED`, or unjustified `NOT RUN` remains.
- `review-disposition.md` accepts and resolves every finding from reviewers A
  and B, including the renamed `eb04t-open-control-timing` figure/sidecar.
- `verification-agent-a.md` and `verification-agent-b.md` independently return
  `PASS` on the amended scientific claims and regenerated evidence.
- `exact-diff-reconciliation.md` matches the current worktree: the only tracked
  modifications are `docs/ROADMAP.md`, the snow-surface campaign roadmap, and
  the work-package catalog; the only untracked scope is this new EB-04T package
  tree.
- The protected diff is empty for `crates/`, `tests/`, science contracts,
  EB-04R, and EB-04S. No production, test, authority, prior-package, retained
  runtime output, fixture, or observation edit is present.
- `worker-handoff.md` carries the four required EB-05 claim boundaries and
  assigns any future promotion study to prospective, mechanism-identifying,
  independent evidence.
- `final-disposition.md` accurately reports the retained-only result, mixed
  criterion alignment, interaction limitation, unchanged EB-04S nonpromotion,
  default-off state, and EB-05 handoff.
- The execution prompt is present only under `prompts/archived/`; no active
  execution prompt remains, and `prompts/README.md` reports the archived state.
- Root roadmap, campaign roadmap, and catalog agree that EB-04T is diagnostic
  complete and EB-05 assurance is next, with no materiality, retroactive-
  promotion, or canopy-longwave-identification overclaim.
- Ran `markdown-doc lint` over the package plus all three roadmap/catalog files:
  27 Markdown files validated with zero errors and zero warnings.
- Ran `git diff --check`: `PASS`; no package `__pycache__` remains.

The exact terminal state satisfies the package acceptance criteria and gate
non-deferral rule. No closure blocker remains.
