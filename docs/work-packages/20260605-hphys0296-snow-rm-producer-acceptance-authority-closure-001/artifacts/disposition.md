# Disposition

Status: executed-hold
Evidence mode: Static + Ran

Ran:
- Contract gate passed.
- Full H1..H39 suite ran.
- H1/H7/H39 snow/`RM` diagnostics ran.
- Full workspace, clippy, deny, authority anti-evasion, and doc gates passed.

Decision:
- Do not patch WB17, WB18, WB19, or WB13 in HPHYS0296.
- Carry the six first-2013/spring-2014 windows as unresolved
  corrected-negative-melt candidates. They are not accepted, excluded, or
  re-tiered because they lack per-window reconstruction and independent
  defective-model verdicts.
- Keep the three spring-2016 windows in `HOLD` as snow/winter producer
  magnitude/timing residuals.

Hold reason:
- Full semantic parity remains `0/39`.
- Spring-2016 H1/H7/H39 windows are not explained by material negative raw
  melt.
- Claude Code review found the initial HPHYS0296 acceptance gate too
  correlational; contract authority has been tightened, but no candidate window
  has the required reconstruction and independent correctness adjudication.
- Dual independent review and verification artifacts are not completed.

Recommended continuation:
- Scaffold HPHYS0297 as a defect-ledger package. First, produce per-window
  root-cause/reconstruction/correctness verdicts for the six
  corrected-negative-melt candidates before any re-tiering. Then continue
  spring-2016 snow/winter producer magnitude/timing diagnosis for windows where
  negative raw melt is immaterial.
