# Verification Agent A

Evidence: Ran
Date: 2026-06-10

## Commands

```console
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Result: `PASS-DEFERRED`, 27 rows, 11 follow-ons, exit `0`.

```console
python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Result: `PASS-DEFERRED`, 27 rows, 11 follow-ons, strict exit `1`.

```console
git diff --check
```

Result: passed.

## Verdict

Verified. Binding exposure is structurally valid and safely deferred where
authority remains unresolved.
