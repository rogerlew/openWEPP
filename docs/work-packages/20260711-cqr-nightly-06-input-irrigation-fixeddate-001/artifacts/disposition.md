# Finding Disposition

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-CONTRACT-MISMATCH`.

Accepted finding `FDIR-FINITE-VALUE-GUARD-001`: the input specification requires
finite `irint`, `irdept`, `qspply`, `tstart`, and `tend`, while `parse::<f64>()`
accepts non-finite values and the parser's inequality checks allow `NaN` into
typed output. Fixing this fail-closed defect changes accepted-input behavior.

Accepted: formatter-only CRAP is excluded under ADR-0021. Follow-up: a
contract-first defect/test-closure package must ratify typed `FDIR-E-005`
non-finite rejection, test `NaN`/positive and negative infinity for required
fields, audit `nozzle`/`tdepl`, implement guards, close obligation/coverage
preconditions, then rerun CQR. No findings remain undispositioned.
