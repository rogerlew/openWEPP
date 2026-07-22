# CRAP After

Ran: same-LCOV CRAP processing at exact `dc935c7a` exited zero in 0.17 seconds.
The production inventory contains 65 rows, zero above 30, and a maximum of
exactly 30. All four original rows and every extracted helper close without an
exception.

| Original function | Before CRAP | After CC / coverage / CRAP |
| --- | ---: | ---: |
| `reject_unknown_options` | 31.0382 | 2 / 80% / 2.032 |
| `trusted_transition_command` | 132 | 4 / 0% / 20 |
| `trusted_heavy_run` | 42 | 2 / 0% / 6 |
| `pre_heavy_audit_command` | 306 | 5 / 0% / 30 |

The 17 extracted helpers have CRAP scores from 2 through 30. The maximum-score
owned helpers are `prepare_transition` and `pre_heavy_audit_inputs`, both 30.

| Artifact | SHA-256 |
| --- | --- |
| LCOV | `0cbbc0a1198a17d9886bb3ff5a5adbdce0165dd6eeb2da81b67687d9eea3810f` |
| CRAP JSON | `0a5684b182387edee0871d14d0d074b45de5b4776437a034b68a11c1f7bb7f83` |
| coverage JSON | `26bcae8981a79dcdfffd074d5ae2be73f16c8699b2cf75c68dd4c81c6d01e026` |
| package function record | `9c2d5161ff5de794c63d477aa319941ef937d62fc65725bfea8ab333c5152c07` |
