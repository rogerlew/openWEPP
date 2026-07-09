# Worker Handoff

Status: `SCAFFOLDED`

Next action: execute this package end-to-end once the operator starts it.

First implementation actions:

1. Resolve the M-T2R closing-test caveat or record that it is not a blocker.
2. Complete the source map and operand-lineage table before production edits.
3. Add contract-derived tests for `TV-GWBASEFLOW-001` through
   `TV-GWBASEFLOW-008`.
4. Thread `GwcoeffFile` state into Lane D/direct runtime inputs.
5. Implement the daily linear-reservoir recurrence with explicit storage carry.
6. Prove generated groundwater baseflow/deep seepage are not active surface
   source terms.
7. Move the real HBP/pass or watershed consumer, or hold with exact boundary
   evidence if that consumer path is outside package envelope.

Do not implement nonlinear baseflow, coefficient defaults, sidecars, route
coefficient projection, or non-Lane-D production paths.
