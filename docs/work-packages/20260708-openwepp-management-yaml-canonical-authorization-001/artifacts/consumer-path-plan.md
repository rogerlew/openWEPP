# Consumer Path Plan

Status: scaffolded.

The implementation must prove the real downstream consumer reads management
YAML directly before production-readiness closure.

Required proof chain:

- producer or fixture emits a schema-valid YAML document;
- shared Rust schema/parser accepts the document;
- runfile or runner binding passes the YAML path to hillslope runtime intake;
- hillslope runtime management intake reads the YAML path;
- PL schedule projection receives all five route-coefficient symbols from YAML;
- Lane D eligibility uses the YAML-projected coefficients;
- source checks show the original `.man`, optional migration report, and
  sidecars are not carrying the production-readiness claim.

Producer-only YAML emission cannot close this package.
