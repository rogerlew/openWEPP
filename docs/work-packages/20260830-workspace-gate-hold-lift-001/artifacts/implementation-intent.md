# Implementation intent

Status: `DECLARED — PRE-EDIT`

Evidence mode: `Static`

Intent is behavior-preserving source-quality correction plus defect closure for
classified workspace test failures. Initial authorized code edits are limited
to the two exact Clippy diagnostics. No test, kernel, numerical, contract,
serialization, public-output, or failure-policy change is authorized until its
failure is prospectively entered in `failure-inventory.md` with governing
instructions and focused validation.

Risk: `CRITICAL`, because terminal acceptance is workspace-wide correctness.
The comparator runner owns heavy commands; the parent and bounded workers own
focused diagnosis/correction only.
