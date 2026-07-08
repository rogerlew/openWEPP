# Worker Handoff

Status: scaffolded.

First execution actions:

1. Finalize whether YAML gets `SC-INFILE-MANAGEMENT-YAML-001` or an explicit
   amendment to `SC-INFILE-MANAGEMENT-001`.
2. Confirm crate ownership using `artifacts/crate-ownership-assessment.md`.
3. Promote the YAML spec from draft outline to normative schema, including the
   producer/consumer extension policy.
4. Add the shared Rust schema/parser/validator crate or module.
5. Wire the real runtime consumer to read YAML.
6. Add route-coefficient projection and eligibility tests from YAML.
7. Add producer tests for lowercase `.yaml` output and default `.man.yaml`
   naming, plus consumer tests for `.yaml`, `.YAML`, `.yml`, and `.YML`.
8. Record consumer-path proof and run closure gates.

Do not add sidecars, legacy-field coefficient inference, or a native flat
management writer. Do not emit `.yml`, `.YML`, or `.YAML` from openWEPP
producer tools.
