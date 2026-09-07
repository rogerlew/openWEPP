# Role settings

Static: repository configuration; Ran: local TOML parse and bundled model catalog query.

| Role | Before configured | After configured/requested | Effective session |
| --- | --- | --- | --- |
| Correctness | xhigh | high | UNOBSERVED until runtime metadata |
| QA | xhigh | medium | UNOBSERVED until runtime metadata |
| Verifier | generic/inherited | explicit package_verifier medium; deterministic assignments may request low | UNOBSERVED |
| Runner | gpt-5.3-codex-spark high | same model, low | UNOBSERVED; no heavy command assigned |

Exposed session tool definitions list rust_code_reviewer high, rust_qa_reviewer medium and comparator_suite_runner low with its existing fixed model. These are declared tool settings, not observed model execution telemetry. The bundled CLI model catalog supports low/medium/high/xhigh on listed models but does not list the runner model; this does not prove its remote service is available. Retain it without silent replacement. This session has no package_verifier tool registration yet, so invoke bounded generic verifiers with explicit medium effort and role procedure; future sessions can use the new registered profile. No new model name or price claim.

Ran: codex --strict-config features list exited 1: strict-config unsupported for features. Ran: codex debug models --bundled (no refresh/network), and Python tomllib parsed all five TOML files successfully. Config syntax is established; service availability and profile reload/effective effort remain UNOBSERVED. No unrelated session history inspected. Use non-forked sessions and record their returned IDs; inherited parent transcript is not claimed.
