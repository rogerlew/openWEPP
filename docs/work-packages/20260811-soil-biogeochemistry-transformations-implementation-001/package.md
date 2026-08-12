# Soil Biogeochemistry Transformations Implementation

Status: `queued / dependency authority bounded`

Package ID: `20260811-soil-biogeochemistry-transformations-implementation-001`

Authority predecessors: `SC-BIOGEOCHEM-001` v1 and
`20260811-coupled-c3-forest-vegetation-model-stack-authority-001`.

## Objective

Admit and implement the soil/litter/CWD decomposition, immobilization,
mineralization, and mineral-N transformation state machine downstream of the
already authoritative `SC-BIOGEOCHEM-001` receiving and arbitration boundary.

## Bounded Scope

- select exact litter and CWD decay, soil-organic-matter transfer, microbial
  demand, immobilization, mineralization, nitrification, denitrification, and
  leaching equations;
- define persistent receiver pools, layer/area/time bases, caller parameters,
  initial state, numerical guards, and atomic transaction ordering;
- conserve C, N, and dry matter with independent donor/receiver reconstruction;
- provide mineral `NH4-N` and `NO3-N` availability to the existing proportional
  request/authorization/finalized-use boundary; and
- add contract-derived poison, competition, failure, and rollback vectors.

Until this package is executed, transformation deltas are exactly zero and any
run requiring decomposition, mineralization, or an endogenous replenishment
source fails `BGC-E-040`. It may not introduce an unlimited nutrient source,
site defaults, surrogate kinetics, or mutate vegetation state. Execution
requires a new authorized kickoff and its own declared write set and gates.

## Terminal Criteria

Complete only after a canonical contract amendment admits every selected
equation, field, state owner, solver, guard, and independent C/N/DM vector, and
after a real soil-biogeochemistry consumer proves the new path. Otherwise hold
on the exact unresolved authority or verification boundary.
