# Local TESTGATE Receipt

Evidence class: `Ran` prerequisite evidence.

Status: `NO_RECEIPT / BLOCKED-POLICY-DIGEST-DRIFT`.

The local TESTGATE helper was not invoked. Its planner prerequisite already
failed because `gate-policy/v1/impact-map.json` does not bind the current
testing-strategy bytes. Executing farther could not issue a valid terminal plan
or independently verified receipt and would waste compute.

No local receipt exists, no trust promotion is claimed, and no workflow was
dispatched. Repairing or bypassing the mismatch was outside the declared write
set.
