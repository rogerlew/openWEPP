# Worker Handoff

Status: `ready / successor implementation package released`

Evidence mode: `Static + Ran`

Implement only through
`20260811-coupled-c3-forest-vegetation-state-machine-implementation-001` and
the digest-bound `OPENWEPP_C3_WOODY_V1` definition. Every delivery phase must
preserve the whole coupled photosynthesis--stomata--energy--hydraulic--C/N
state machine; no water-only, phenology-only, immutable-N, or diagnostic-GPP
endpoint has independent authority. Runtime selection and cutover remain
separately unauthorized. Caller supplies topology, every parameter, and a
complete compatible initial state. The package oracle and contract test are
the pre-production authority surface; calibration and independent validation
remain `NOT_CALIBRATION_READY` / `NOT_VALIDATED`.
