# Focused Results

Status: `SUPPORTING-PASS`

Evidence class: **Ran** through the archived full-workspace release log. The
interactive focused-run output was not archived and is supporting only.

The interactive focused checks reported the source guard 1/1 and these three
H2637 cases 3/3 in process-isolated execution:

- `h2637_active_fails_closed_without_routing_coefficients`;
- `h2637_active_and_disable_are_mutually_exclusive`; and
- `h2637_active_and_shadow_are_mutually_exclusive`.

Their focused raw output was not archived. Terminal correction evidence instead
comes from `logs/01-release-candidate.log`: the exact release command passed all
1,945 selected full-profile workspace tests, including the source guard and
nonignored H2637 selector cases, confirming that the collision did not recur.
