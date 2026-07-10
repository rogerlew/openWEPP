# ADR-0021 Coverage Closure

Tier: science. The direct-runtime target emits typed guard failures used by
water-balance and routing contract enforcement, so the science-tier floor is
at least 90% line and 90% region coverage, with no eligible function below 75%
region coverage without a documented exclusion.

Obligation-to-test binding: each of the 20 `DirectRuntimeError` variants is
bound to a direct `Display` rendering assertion. This includes the typed hard
failure families required by `SC-WATBAL-001` and `SC-OFEROUTE-001`: missing
upstream, non-finite, negative, domain, kernel-guard, publication, closure, and
direct-day execution failures.

Ran: final delegated isolated coverage records `417/426` production-only lines
(97.8873%) and `528/542` production-only regions (97.4170%), above both
science-tier floors. The 41 target CRAP function observations contain no
coverage value below 75%; no `COVERAGE-EXCLUDE` is used or needed.

The direct consumer is the actual `Display` implementation invoked by
`error.to_string()`, not a producer-only helper. The pre-decomposition
characterization command and its 20 complete-output assertions are recorded in
`characterization.md`; the private representation decomposition followed that
successful oracle.
