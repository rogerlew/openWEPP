# Coupled C3 Forest Vegetation State-Machine Implementation

Status: `queued / implementation authority released`

Package ID: `20260811-coupled-c3-forest-vegetation-state-machine-implementation-001`

Authority predecessor: `20260811-coupled-c3-forest-vegetation-model-stack-authority-001`,
`SC-VEGETATION-001` v5, and `SC-BIOGEOCHEM-001` v1.

## Objective

Implement the complete `OPENWEPP_C3_WOODY_V1` coupled state machine. Delivery
may use stable internal increments, but no increment is an independent
water-only, phenology-only, immutable-N, diagnostic-photosynthesis, or final-LAI
authority. Existing production behavior remains unchanged until the complete
default-off shadow state machine and real consumers pass.

## Required Scope

- strict digest-bound caller configuration and complete initial state;
- multistratum two-stream radiation and liquid interception;
- coupled FvCB--Medlyn--leaf-energy--hydraulic solver and typed rollback;
- hydrology-owned layer water arbitration;
- persistent vegetation C/N, respiration, allocation, storage,
  retranslocation, turnover, mortality and leaf-C/SLA-owned LAI;
- `SC-BIOGEOCHEM-001` mineral-N and litter/CWD receiving owner;
- exact water, energy, C, N and dry-material ledgers and atomic commit;
- typed rejection of every unsupported branch; and
- independent A1/A3 vectors plus real downstream consumer proof before cutover.

No runtime activation, agricultural PMET donation, hidden default, fallback
solver, direct soil mutation, unlimited nutrient source, mixed-profile average,
or partial endpoint is authorized. Calibration and transferability remain
separate claims. Canopy snow and full soil transformations remain named
dependencies, not silent branches.
