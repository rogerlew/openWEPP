# Coverage Closure

Ran: the attempted refactor preserved the six existing public real-binary tests
unchanged and produced a like-for-like line result of `719/1048` (`68.607%`)
versus `667/992` (`67.238%`) at the scaffold. This evidence is insufficient for
the normative cover-first precondition: the glue module remains below `85%`,
the LCOV population contains no region counts, and no per-function region-floor
map exists.

Three uncovered low-cyclomatic rows are dispositioned rather than covered:

- `Totalwatsed3Error::code` and `fmt` are typed error/observability mapping
  arms. Adding exhaustive variant tests would activate the whole glue-module
  `85%` line/region and per-function floor gate; the live module is below that
  precondition.
- `date_ofe_key_from_columns` is reached only by optional soil/element inputs,
  absent from the authorized characterization surface. Adding that new fixture
  family would likewise be a material test enhancement.

Independent review rejected the proposed disposition for `code` and
`date_ofe_key_from_columns`: both are eligible production behavior, not
closed-list observability exclusions. `fmt` may be reviewed as observability
formatting only after the module coverage prerequisite exists. The attempted
implementation is therefore rolled back and the package closes locally held.
