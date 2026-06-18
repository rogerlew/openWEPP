# FARPOINT01 Flag Resolution

Evidence class: Static + Ran

Status: resolved.

## Resolution

The FARPOINT01 H2637 71% magnitude flag is closed as:

```text
NO DEFECT / EXPECTED STAGE-2 DIVERGENCE
```

Reason: after MAGPARITY01, STAGE2-LATQCC, REFINTENT001,
STAGE2-BASE-CONDUCTIVITY, and BASECOND01, every in-envelope candidate term is
either verified correct or fixed and shown not to drive the H2637 aggregate. The
remaining byte-live driver is the lateral base-conductivity lineage, and that
lineage is source-intent correct for H2637.

## Closure Record

- FARPOINT01 conservation and >10-OFE routing closure remain accepted.
- The 71% magnitude flag no longer routes to a defect-closure ExecPlan.
- No `SC-*` contract change is made in this package.
- No production Rust change is made in this package.
- Absolute magnitude validation is deferred to
  [`docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`](../../../backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md).

## What Would Reopen It

The flag should only be reopened by new evidence that changes the authority
state, such as:

- a failed conservation, routing, export, or operand invariant in current
  openWEPP outputs;
- source-intent evidence contradicting the verified H2637 lateral conductivity
  path;
- a ratified external-authority suite that defines an absolute magnitude
  envelope and classifies H2637 as out of bounds.

The flag should not be reopened merely because:

- legacy without UI reports `55.5%`;
- the fixed vertical `ssc` defect was aggregate-inert on H2637;
- the fixed `ksatadj` defect was byte-inert on H2637;
- making hourly `ui_ssh` harmonic would move the result toward the comparator.
