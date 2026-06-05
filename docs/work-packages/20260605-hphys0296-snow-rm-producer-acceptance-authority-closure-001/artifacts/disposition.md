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
- Carry the six first-2013/spring-2014 windows as corrected-negative-melt
  candidates pending review.
- Keep the three spring-2016 windows in `HOLD` as snow/winter producer
  magnitude/timing residuals.

Hold reason:
- Full semantic parity remains `0/39`.
- Spring-2016 H1/H7/H39 windows are not explained by material negative raw
  melt.
- Dual independent review and verification artifacts are not completed.

Recommended continuation:
- Scaffold HPHYS0297 for spring-2016 snow/winter producer magnitude/timing:
  isolate why candidate `RM` and `Snow-Water` are lower than baseline when
  negative raw melt is immaterial, with focus on accumulation/carry-state
  timing, rain-on-snow release, and comparator-authority alignment.
