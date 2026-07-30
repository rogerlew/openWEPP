# Terminal Verification B

Status: complete / pass

Verifier: fresh independent agent `assure_maint02_terminal_b`

Evidence class: Static + Ran

Verified:

- both unrelated staging roots and the stable preview contain the same 91-file
  inventory;
- path inventory SHA-256 is
  `9c2dc4858f0628e0067ceb49a626786fe5a7ed09c7583bb93819a1ae7527990a`;
- content inventory SHA-256 is
  `cf5337c1d5764071db42e80a85707cf7b45952a4176471c88c5b8c31207679d4`;
- `check --all` passes in all three roots;
- `cmark-gfm` parses all six generated report/supplement documents;
- each report has its index, supplement, and build manifest;
- CAL-09 has eight retained and one generated accessible staged SVG;
- generation verification passes at 22 transitions and generation
  `30db4a7e6a691601426428b7772e28143ff9fa1bf10dd9d1ae80062d7f0002a2`;
- the governing workspace log is intact at 2,161 passed and 5 skipped, with no
  implementation or test source newer than the log;
- tracked usersum/public/generated assurance, kernel/runner, CAL evidence, and
  existing authored report surfaces remain unchanged;
- roadmap, catalog, documentation lint, and `git diff --check` pass.

The verifier identified that `src/lib.rs` was authorized API wiring but omitted
from the prospective path list. Package and exact-diff evidence now record that
late inventory correction as a process deviation. Recheck accepted the
disposition.

Verdict: PASS. No unresolved findings.
