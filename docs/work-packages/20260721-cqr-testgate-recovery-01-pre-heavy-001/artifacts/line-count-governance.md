# Line-Count Governance

Static: the target is 2,574 lines. This triggers the 2,000-line WARN but remains
below the 3,000-line closure blocker. The increase is attributable to private
helper decomposition and package-required characterization. Follow-on owner:
gate-planner maintainers; a future readability split may move test-only code to
a dedicated test module without changing production behavior.

Disposition: WARN accepted; no 3,000-line blocker exists.
