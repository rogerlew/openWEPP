# WS10 Routing/Impoundment Contract Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Routing Contract Evidence (`SC-ROUTE-001`)
- WS10 addendum implemented:
  - runtime boundary symbols for channel lane and dependencies
  - typed routing guard family `WKERNEL-WS10-CHANNEL-E-001..003`
  - WS10 routing test-vector obligations
- Production code evidence:
  - channel lane implemented in `run_channel_node`
  - dependency payload parsing and hard-fail validation implemented
  - channel outputs published as `ws10_channel_{id}_qpo`,
    `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff`.
- Test evidence:
  - nominal routing vector passes
  - missing/non-finite/domain vectors pass with expected guard IDs.

## Impoundment Contract Evidence (`SC-IMPOUND-001`)
- WS10 addendum implemented:
  - runtime boundary symbols for impoundment lane and dependencies
  - typed impoundment guard family `WKERNEL-WS10-IMPOUNDMENT-E-001..003`
  - WS10 impoundment test-vector obligations
- Production code evidence:
  - impoundment lane implemented in `run_impoundment_node`
  - explicit headroom/overflow branch and dependency-payload validation
  - impoundment outputs published as `ws10_impoundment_{id}_qo`,
    `ws10_impoundment_{id}_durout`, `ws10_impoundment_{id}_hnext`,
    `ws10_impoundment_{id}_outflow_volume`.
- Test evidence:
  - nominal impoundment vector passes
  - domain violation vector (`h > hfull`) passes with expected `-E-003` code.

## Cross-Contract Integration Evidence
- `SC-HYDRAULICS-001` WS10 consumer-coupling addendum implemented.
- `SC-SYSTEM-001` WS10 integration addendum implemented.
- Registry notes updated in `docs/specifications/science-contracts/index.md`.
