# Plan

1. Add a `direct_runtime::growth` module with typed annual/perennial growth inputs, direct compute, state mutation, downstream operands, shadow projection, span reports, and active-context/action enums.
2. Extend `DirectDayFrame` with annual/perennial growth fields and wire R5D spans between R5C residue partitioning and R4 hydrology.
3. Update R4N direct evapotranspiration inputs so the direct executor can require growth context and fail closed when required context is absent.
4. Add focused tests for phase identity, annual/perennial active updates, fallow/pre-plant/reset/cut/grazing paths, alias-sensitive plant inputs, upstream R5C requirement, R4N growth-context requirement, and failure domains.
5. Update package/catalog/roadmap artifacts and run the required validation gates.

