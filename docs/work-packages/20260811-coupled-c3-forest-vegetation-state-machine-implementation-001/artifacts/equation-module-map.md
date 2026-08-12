# Equation Module Map

Status: `FROZEN`

| Families | Production module | Direct evidence |
|---|---|---|
| E01--E03 | `openwepp-vegetation::radiation` | radiation vectors and mixed-stratum poison tests |
| E04--E05 | `openwepp-vegetation::interception` | water/energy reconstruction and phase tests |
| E06--E13 | `openwepp-vegetation::{aerodynamics,photosynthesis,energy}` | FvCB/Medlyn/temperature/energy vectors |
| E14--E15 | `openwepp-vegetation::hydraulics` | layer/cap/redistribution/rollback vectors |
| E16--E22 | `openwepp-vegetation::{carbon_nitrogen,phenology,turnover}` | C/N/DM and trajectory vectors |
| arbitration/receivers | `openwepp-biogeochemistry` and kernel DTOs | competition, receiver, and zero-transformation tests |
| atomic owner set | orchestrator diagnostic module | failure injection and byte-identical rollback |
