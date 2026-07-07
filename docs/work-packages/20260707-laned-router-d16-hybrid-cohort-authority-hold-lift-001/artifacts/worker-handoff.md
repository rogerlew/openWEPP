# Worker Handoff

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## First Actionable Follow-On

Scaffold `D16-HYB-ROUTE-COEFF-AUTHORING-BRIDGE`.

Objective: produce source-authorized native route-coefficient inputs for the
selected D16 hybrid promotion cohort before building the executable active
plain-vs-hybrid suite.

Required first implementation action:

- Choose one authority path:
  - import operator/source-authored `ow-lanuse-1` managements with explicit
    `routing_coefficients` for the selected cohort; or
  - author a contract-first legacy-to-native bridge that maps named legacy
    input surfaces to the five route-coefficient operands with provenance,
    tests, and anti-evasion guard coverage.

Required gates:

- Native management parse/projection tests for every selected coefficient
  source class.
- Active plain and active explicit hybrid preflight for each selected cohort
  member.
- Executable owcmp suite manifest replacing or complementing the current
  `cohort-inventory` manifests.
- `tools/owcmp/owcmp env --manifest` and `manifest run` on the new executable
  suite.
- Active output/timing surfaces required by the prior tolerance-surface design.
- Anti-evasion guards if any suite posture, cohort fixture, or required-case
  binding changes.

Do not return to D16 default-promotion selector changes until the route
coefficient input-authority package passes.
