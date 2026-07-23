# Execute TESTGATE Runner GitHub CLI Recovery

Execution mode: package-end-to-end.

Correct RTR-046 by installing the official checksum-pinned GitHub CLI in the
trusted runner image, enforcing its exact preflight version, rebuilding and
rebinding the image, and proving the live runner contract before one
changed-head push.

This prompt explicitly authorizes two independent read-only reviewers and two
terminal verifiers. Do not manually dispatch TESTGATE or rerun unchanged
expensive gates.

