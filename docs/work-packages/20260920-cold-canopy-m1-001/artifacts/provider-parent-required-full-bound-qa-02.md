# Required full-validation recorder QA — 02

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static recorder diff and inspection of the supplied non-launching boundary controls. This reviewer did not execute the recorder or a full-workspace command.

## Findings

### HOLD — the second pre-launch timeout guard still rejects the sole allowlisted command

`artifacts/run_recorded.py:189-193` retains the former unconditional check:

```python
if args.physical and args.timeout > 180:
    raise SystemExit('Physical command declared bound exceeds 180 seconds')
```

The new `validate_declared_bound(...)` runs before custody preflight at lines 98-100, but is not used at this second, immediately-prelaunch check. Consequently, the exact flagged physical 900-second full-workspace command passes the first allowlist and support-record checks, then fails before output reservation and launch. This does not implement the authorized exception.

Replace the second unconditional pair with the same `validate_declared_bound(command, args.timeout, args.physical, args.required_full_validation, remaining_for_command)` call used at first preflight. That preserves the live reserve calculation while applying the identical strict allowlist at both decision points.

## Verified controls

The diff otherwise is suitably narrow: it adds the exact Nix/environment/full-workspace argv constant, the `--required-full-validation` flag, `physical`/flag receipt fields, and a support-record equality binding. `validate_declared_bound` correctly rejects altered argv, nonphysical full validation, wrong bounds, unflagged physical 900-second full validation, and unrelated physical commands; it preserves the reserve test.

`provider-parent-full-bound-controls-01.json` reports all 14 helper-level boundary cases passed without launching a command. Those tests do not reach the second `main()` guard, so they cannot detect this wiring defect.

## QA disposition

**HOLD — do not release the recorder correction or full-workspace run.** Apply the one-call pre-launch fix and add a non-launching control that exercises both guard positions for the exact 900-second allowed case and a rejected variant. Existing recorder reserve, custody, no-retry, and ordinary 180-second physical-cap evidence remain accepted.
