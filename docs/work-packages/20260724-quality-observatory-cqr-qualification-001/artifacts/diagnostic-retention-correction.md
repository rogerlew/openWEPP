# Diagnostic Retention Correction

Evidence class: Static and Ran.

The partial-index contract now computes the full SHA-256 for every retained
diagnostic regardless of size and embeds a digest-bound final 32 KiB as
canonical base64. Validation requires:

- an exact closed-list path;
- nonnegative exact size;
- lowercase full-file SHA-256;
- tail length equal to `min(size, 32 KiB)`;
- canonical base64;
- exact decoded tail length; and
- exact tail SHA-256.

The total control ceiling remains 1 MiB and complete publications still reject
partial indexes. Raw logs remain excluded from publication and are cleaned
after bounded finalization.

A deterministic failed-child fixture writes a log larger than 256 KiB and
proves its SHA is non-null and its 32 KiB tail is retained. The complete 18-test
QA/merged-coverage/CQR focused surface and warnings-denied touched-contract
Clippy pass.
