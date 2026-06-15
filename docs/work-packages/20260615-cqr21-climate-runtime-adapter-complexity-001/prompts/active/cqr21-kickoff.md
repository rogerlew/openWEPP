# CQR21 Kickoff Prompt

Execute CQR21 under
`docs/work-packages/20260615-cqr21-climate-runtime-adapter-complexity-001/`.

Target:
`crates/openwepp-climate-runtime-adapter/src/lib.rs`.

Preserve public runtime request/error APIs, typed guards, stable error IDs,
parser compatibility, formulas, units, float expression order, and
science-contract behavior while reducing the live target and extracted helpers
to CRAP `<= 30`.
