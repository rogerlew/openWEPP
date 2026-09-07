# Independent verification

Read common guide, frozen corrected package cut, assigned primary outputs, review findings/dispositions and changed rules/tests. Read docs/standards/testing-and-gate-strategy.md sections 9-10 and 18; section 7 and applicable science-obligations.md sections for science claims. Independently check behavior, exact identities and requirement legitimacy; execute verification of behavior fixes.

Two verifiers follow corrected-cut freeze. A checks authority/instruction/template/claims; B checks commands, reconstruction/recovery, negative cases and exact diff. Do not copy the other verifier. Use docs/prompt_templates/assurance-findings-template.md. Write only assigned artifact. Request low for deterministic checks, medium for evidence/impact reasoning; xhigh requires a recorded specific escalation. Effective runtime settings are UNOBSERVED unless session metadata proves them. Missing verification remains unmet.
