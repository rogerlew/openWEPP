# Gate Results

Terminal evidence is recorded after the exact commands run. No Rust gate is
applicable because the diff is documentation and a package-local audit tool.

| Gate | Result | Evidence |
|---|---|---|
| Four-site/eight-hourly-field manifest plus elevation ancillary | `PASS` | Audit tool validates counts and binds manifest SHA-256. |
| Credential secrecy | `PASS` | Boolean presence only; values were not read or recorded. |
| Result-bearing ERA5 comparison | `PASS` | `RADIATION_FIRST_COMPLETE` binds 8 product/site results, exact source/comparator hashes, corrected interval/plane/peak operators, complete-hour daily metrics, and protected precipitation posture; dual review and dual verification pass. |
| Protected production/test/fixture diff | `PASS` | Terminal diff inventory contains only declared documentation/package paths. |
| Python syntax and fail-closed receipts | `PASS` | `py_compile` passed; validator and comparison tools reject an existing output receipt. |
| JSON parsing | `PASS` | `.venv/bin/python -m json.tool` passed for acquisition, validation, comparison-manifest, and result artifacts. |
| Markdown lint | `PASS` | `markdown-doc` validated the exact-current package and three catalog/roadmap surfaces with zero findings. |
| Diff hygiene | `PASS` | `git diff --check`, declared inventory, protected empty diff, and no-`__pycache__` checks pass. |
| Acquired-data/negative-policy dual review and verification | `PASS` | Both fresh reviews and both exact-current terminal verifications reproduce the cohort identities/numerics and pass after lifecycle/source-framing corrections. |
| Direct content and grid elevation validation | `PASS` | Corrected validator emits `VALIDATED_COMPLETE` for 8 hourly files and 8 elevation records; two fresh independent reviews and two exact-current verifications pass. |
| Radiation figures and sidecars | `PASS` | Generator emits four deterministic accessible SVG/Markdown pairs, exact plotted-data sidecar, and a manifest binding every file; strengthened semantic validator, dual independent visual/data reviews, and dual terminal verifications pass. |
