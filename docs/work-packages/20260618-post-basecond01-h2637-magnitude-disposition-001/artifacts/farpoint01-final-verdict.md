# FARPOINT01 Final Verdict

Evidence class: Static + Ran

Status: complete.

## Verdict

`CORRECT-BY-CONSTRUCTION` for the internal openWEPP H2637 lateral-magnitude
lineage.

`NO DEFECT` for the FARPOINT01 71% openWEPP magnitude flag.

`CONTRACT-GAP` for the absolute physical magnitude question.

## Meaning

The H2637 `71.0036550031206%` PASS `runvol` result is the output of a verified
lateral conductivity -> WB19 lateral-flow -> routed outlet lineage:

- the H2637 routing/export/accounting path closes;
- WB19 `latqcc` reconstructs from the governed equation and operands;
- source-intent defects found during the investigation were fixed;
- those fixes were byte-inert or aggregate-inert on H2637;
- the byte-live base lateral conductivity path is source-intent correct for
  H2637 under the current contracts.

This verdict is intentionally limited. It does not claim that `71%` has been
validated against field data or an independent forest-hydrology authority. It
claims that current openWEPP authority has no remaining internal defect to close
for this flag.

## Comparator Disposition

Under ADR-0017, legacy remains a comparator flag, not a target:

- legacy without UI at `55.5%` is a bounded magnitude contrast;
- legacy with UI at `127.7%` violates the runoff <= precipitation bound and is
  disqualified as an authority target;
- no production edit is authorized to move openWEPP toward either legacy
  magnitude.

## Residual Gap

The remaining gap is external: openWEPP does not yet have an authority suite
that defines an acceptable absolute lateral-flow magnitude envelope for wet
forest H2637-like hillslopes. That work is recorded as a backlog concept note,
not as an active blocker for the FARPOINT01 defect investigation.
