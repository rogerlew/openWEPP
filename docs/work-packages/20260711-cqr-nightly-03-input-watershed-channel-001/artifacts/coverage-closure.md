# Coverage Closure

Ran: glue tier applies to this input parser. Cover-first chronology is proven by
the tests-only artifacts created before any production edit:

- LCOV: `/tmp/openwepp-cqr-20260711-t03-characterized.lcov`, SHA-256
  `6bda2705a1bd90291f9dc8cc1510c1755aa6f62e50aeac316fb82d092dfccefc`;
- JSON: `/tmp/openwepp-cqr-20260711-t03-characterized.json`, SHA-256
  `c2b867b23c1c40f0369998934c4408fbcba59652571b18fd204c3c11514f4fa2`;
- CRAP: `/tmp/openwepp-cqr-20260711-t03-characterized-crap.json`, SHA-256
  `550dafd2b1d8c56dbab13e491d17a43d59ce09c1dacf8f0117531972cbfe48bb`.

Tests-only coverage was `466/520` lines (`89.615%`) and `592/651` regions
(`90.937%`), above both `85%` glue module floors. The required per-logical-
function cover-first floor did not pass: `parse_single_i32` was `20/25`, but its
same-function parse-error closure was `0/5`, yielding `20/30` (`66.667%`). The
package therefore decomposed too early. Review later added a malformed integer
case and lifted the provisional logical function above `75%`, but that cannot
retroactively satisfy cover-first sequencing. This is attempt/hold evidence,
not closure.

Neither the module percentages nor later floor repair waived obligation mapping.
Review found family H /
contract guard `G-CHN-013` cannot close: `SC-INFILE-WATERSHED-CHANNEL-001`
requires missing/extra conditional rating records to emit `CHN-E-006`, but an
extra row when `icntrl != 4` currently emits `CHN-E-002`. Therefore the A-H map
cannot truthfully close and the target is held despite passing percentage and
CRAP attempts. G conservation is not applicable because the parser performs no
conservation math.
