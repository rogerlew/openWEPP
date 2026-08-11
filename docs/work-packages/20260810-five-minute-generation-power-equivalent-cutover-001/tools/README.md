# Package Tools

Status: `active — prospective feasibility only`

Place only package-local diagnostic or cohort-harness tooling here. Large
result-bearing outputs belong in the external immutable evidence root named by
`package.md`.

`feasibility_study.py` mirrors a bounded subset of the production rill-width,
Chezy/shear, Shields/Yalin, and normalized driver equations for prospective
one-sided exponent rejection. It omits full continuity, end-slope/`ktrato`,
critical-shear state, and persistent width chronology, so it cannot admit a
candidate or serve as a production parity oracle. It consumes no Topanga
mutation outcomes and cannot be imported by production crates. Run its tests
and emit the compact package artifact with:

    .venv/bin/python -m pytest \
      docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools

    .venv/bin/python \
      docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools/feasibility_study.py \
      --output docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/constitutive-response-study.json
