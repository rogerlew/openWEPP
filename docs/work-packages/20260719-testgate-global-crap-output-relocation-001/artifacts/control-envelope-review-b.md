# Control-Envelope Review B

Evidence class: Static and narrow checks.

Initial verdict: FAIL, not HOLD.

- HIGH: control envelope lacked path/type/symlink confinement. Accepted and
  patched through the confined artifact reader.
- HIGH: validated report bytes could differ from receipt publication bytes.
  Accepted and patched by returning the freshly bound buffer directly to
  atomic publication.
- MEDIUM: focused evidence omitted missing/symlink and publication-race paths.
  Accepted; symlink and post-validation tamper regressions now cover the
  material gaps. Existing strict parsing covers missing, float, duplicate,
  non-PASS, nonzero, and digest-mismatch failures.
- MEDIUM: touched `executor.rs` crossed the line-count WARN without truthful
  disposition. Accepted; exact 2,611-line evidence and follow-on split intent
  are recorded.

Final verdict: PASS, no remaining finding.

Reviewer B confirmed numeric report preservation, strict control semantics,
exact adapter bindings, and diff hygiene. No broad suite ran.
