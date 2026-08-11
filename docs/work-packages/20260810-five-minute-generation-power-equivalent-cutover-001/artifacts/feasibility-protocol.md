# Feasibility Protocol

Status: `executed — prospective diagnostic`

Evidence mode: `Static + Ran`

The package-local `tools/feasibility_study.py` screens the preregistered
`p={1,4/3,3/2,2}` candidates without reading Topanga mutation outcomes. It
mirrors the current production equations for a bounded subset: Gilley
rill-width growth, Chezy-depth iteration, hydraulic-radius shear,
Shields/Yalin capacity, the normalized detachment factor `shear/tcend`, and
the positive-flow reciprocal deposition factor `1/qout`. Source anchors are
`erosion_operands.rs:497-643,710-857` and
`erosion_continuity.rs:2189-2241`.

The deterministic library contains 12 rate shapes: constant, one pulse, two
separated pulses, rising, falling, symmetric triangle, early and late
ponding, a bin-spanning pulse, an hour-edge pulse, saturation background, and
a near-material-floor case. Each uses twelve 300-second intervals and is
integrated, never centre-sampled.

The state cross product contains burned/unburned cover bookends, low/Topanga/
high slopes, and new/developed rill widths. The Topanga texture
`sand=0.25, clay=0.30, silt=0.45`, 242.1 m length, and 0.3267 representative
slope come from the frozen input files. Cover bookends and non-Topanga slope/
width cases are `ASSUMED_FOR_EXECUTION` domain probes, not observations,
physiological bounds, calibration, or independent validation.

For every state/shape/candidate, the tool compares time-integrated pure
constitutive responses with the candidate rectangle. Adoption thresholds are
median <=5%, p95 <=15%, and no material error >30%. It separately verifies
volume and selected-power moments and records threshold branch changes.

Claim boundary: this is a one-sided rejection screen, not a complete Milestone
3 candidate-admission implementation. It omits end-slope shear, `kt2/ktrato`,
critical-shear/erodibility/`qin` states, full detachment/deposition continuity,
invalid-domain census, detailed threshold proximity, persistent width
chronology, and Rust/Python parity. Its transport flag is only a coarse any-bin
versus rectangle comparison. These omissions can hide additional failures and
therefore cannot rescue an already rejected exponent; they prohibit using the
tool to admit one.

Ran:

    .venv/bin/python -m pytest \
      docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools

Result: 5 passed. The first collection run failed because the dynamic test
import was not registered in `sys.modules`; that test-harness defect was fixed
before the accepted run.

    .venv/bin/python \
      docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools/feasibility_study.py \
      --output docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/artifacts/constitutive-response-study.json

Result: 1,008 deterministic records; `NO_FIXED_EXPONENT_ADMITTED`.
Tool SHA-256:
`77535b6c4a3333f9c671ec0cfecac41bf426e446bc95393e8d1b95709085ce4c`.
