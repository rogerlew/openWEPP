# Historical Generator Command and Environment

Evidence class: `Static`

The historical V3 command is repeatedly recorded as:

` .venv/bin/python docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/reference_calculator.py `

The historical V5 heavy log records:

` .venv/bin/python docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/reference_calculator_v5.py `

at `2026-08-13T13:04:38Z`, exit zero, duration 24,431 ms, followed by the
authority verifier. Package evidence says two consecutive regenerations
matched the frozen V5 bytes. Both calculators are standard-library-only and do
not call Rust.

No preserved log records the historical Python version/build, OS/libm, locale,
`PYTHONHASHSEED`, compiler, CPU math path, or complete environment. The current
`.venv` description is not evidence of the release environment. Exact command
recovery therefore does not recover exact numerical-runtime provenance.
