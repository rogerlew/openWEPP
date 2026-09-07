# Independent review

Read common guide, package acceptance/write set, current handoff, assigned changed files and primary evidence. Read docs/standards/testing-and-gate-strategy.md sections 7-10 and 17-18. For kernel/consumer/conservation claims read applicable science-obligations.md sections and relevant canonical authority; specialized-workflows.md only for the package type. Parent summaries are not primary evidence.

Two reviewers independently inspect one stable substantive cut. A owns authority/correctness/precedence/self-waiver; B owns QA/usability/security/validation. Both challenge impact classifications and acceptance legitimacy. Use docs/prompt_templates/assurance-findings-template.md. Every finding needs severity, location, evidence and explicit disposition. Accepted fixes require focused independent re-review; rejected findings require reasons; deferred/follow-up findings need owner/link and cannot waive current acceptance. Check Rust line-count obligations when Rust changes.

Write only assigned review artifact; no configuration/source/authority edits. Correctness requests high; QA medium. Escalate to xhigh only for a named difficult numerical issue or unresolved correctness finding, justified in the same artifact. Configured/requested and effective runtime settings are separate: UNOBSERVED without session metadata. Parent self-review cannot replace independence.
