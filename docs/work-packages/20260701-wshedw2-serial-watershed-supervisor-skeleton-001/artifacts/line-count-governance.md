# Line-Count Governance

Status: `QUEUED`

W2 execution must record touched `.rs` line counts before closure.

Governance:

- 2000+ lines is `WARN`;
- 3000+ non-exempt files require refactor or an explicit hold before closure;
- do not grow monolithic runner files without recording a split/debt
  disposition.
