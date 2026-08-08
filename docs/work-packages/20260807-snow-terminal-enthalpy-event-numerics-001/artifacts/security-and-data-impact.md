# Security And Data Impact

Status: reviewed / no new security or external-data surface

Evidence mode: Static

- No network, credential, observation, deployment, or external message path was
  added.
- The only new selector is an opt-in environment value parsed fail-closed.
- Invalid model/operator combinations fail before evaluation.
- The independent schema-v8 consumer validates the row before opening the
  trace file, preserving atomic failure.
- Terminal liquid, energy, and remaining time remain censored internal values.
