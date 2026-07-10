# Obligation-to-Test Map

| Contract guard / parser surface | Public test binding | Status |
|---|---|---|
| G-IRD-001/010 datver strict/compat policy | strict legacy reject, compat no-datver/legacy-furrow cases | PASS |
| G-IRD-002 header and default options | header/token/itemp tests and default-options assertion | PASS |
| G-IRD-004/005 sprinkler/furrow grammar | strict sprinkler/furrow arity/nozzle/rate/depsrg cases | PASS |
| G-IRD-006 date/value/element domain | token, day/year, endpln/topology cases | PASS |
| G-IRD-007/008 initialization/continuation | existing initialization and nonmonotone continuation cases | PASS |
| G-IRD-009/012 cross-file/furrow context | count/system/run-option and strict/compat disallowed-furrow cases | PASS |
| G-IRD-011 zero-start transition | compatibility zero-start transition/warning case | PASS |
| IRD-E-000 through IRD-E-009 | exact complete Display/contract-ID tests | PASS |

The parser-owned guard map is directly applicable. Runtime irrigation-trigger
physics is owned by `SC-IRRIG-001` and is not changed by this parser-only CQR
package. No coverage exclusion is used.
