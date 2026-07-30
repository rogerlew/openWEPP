# Full-Catalog Render Evidence

Status: complete / pass

Evidence class: Ran

Final generated identity: `30db4a7e6a691601426428b7772e28143ff9fa1bf10dd9d1ae80062d7f0002a2`.

- `validate --all`: PASS; three selected DRAFT reports and zero public reports.
- `plan --all --format json`: PASS; all three reports current.
- Unrelated staging roots:
  `/tmp/openwepp-assure-terminal-e.pVjtRz` and
  `/tmp/openwepp-assure-terminal-f.BvJPTY`.
- `build --all` and `check --all`: PASS in both roots.
- Complete output inventories: 91 files in each root; sorted relative-path and
  SHA-256 inventories compare byte-for-byte equal.
- Stable preview: `target/assurance-preview/usersum/assurance/reports/`;
  `build --all` and `check --all` PASS.
- Consumer parse: `cmark-gfm` parsed all six generated top-level report and
  supplement Markdown files.
- CAL-09 contains eight sanitized retained figures plus one generated transfer
  figure. Every staged SVG has a title, description, and `role="img"`.

The stable preview is disposable and ignored. No tracked public report was
created.
