# CLIM05 Coupled Water-Balance Effect Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Authority Equation Paths

From implemented CLIM05 kernel behavior:
- Runoff reconciliation uses snow-coupled liquid input:
  - `Q = (hyetograph_rainfall + S) + runon - infiltration - depression_storage_delta`
- Storage reconciliation includes signed snow term:
  - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - Q - ET - D - Qd`

## Ran Coupling Check

Command:
```bash
awk 'BEGIN{rain=3.0;runon=0.4;inf=2.909931093255933;dep=0.2;S=0.333333333333333;storage_initial=12;precip=3;losses=4.75;q_no=rain+runon-inf-dep;q_yes=(rain+S)+runon-inf-dep;st_no=storage_initial+precip+0-q_no-losses;st_yes=storage_initial+precip+S-q_yes-losses;printf("q_no_s=%.15f\nq_with_s=%.15f\ndelta_q=%.15f\nstorage_no_s=%.15f\nstorage_with_s=%.15f\ndelta_storage=%.15f\n",q_no,q_yes,q_yes-q_no,st_no,st_yes,st_yes-st_no)}'
```

Observed output:
- `q_no_s=0.290068906744067`
- `q_with_s=0.623402240077400`
- `delta_q=0.333333333333333`
- `storage_no_s=9.959931093255934`
- `storage_with_s=9.959931093255932`
- `delta_storage=-0.000000000000002` (numerical roundoff)

## Contract-Test Confirmation (Ran)

Command:
```bash
cargo test --test clim05_snow_runtime_kernel_contract
```

Observed result:
- pass (`4 passed`)
- coupled conformance vector confirms:
  - signed snow flux `S = 0.333333333333333`
  - runoff `Q = 0.6234022400774`
  - reconciled storage `wb12_storage_reconciled = 9.959931093255934`

## Effect Summary

- CLIM05 coupling increases runoff exactly by signed `S` relative to no-snow-coupling liquid input.
- WB12 storage reconciliation remains mass-consistent when the same signed `S` term is included explicitly in storage closure.
