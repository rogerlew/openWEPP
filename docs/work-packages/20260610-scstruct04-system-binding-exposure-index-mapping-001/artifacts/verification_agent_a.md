# Verification Agent A

Evidence: Ran
Date: 2026-06-10
Scope: Binding exposure lint gate.

## Commands

```console
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Result:

```console
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md: 27 binding exposure row(s), 27 science-review-follow-on row(s) not yet consolidated
```

Exit code: `0`

```console
python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Result:

```console
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md: 27 binding exposure row(s), 27 science-review-follow-on row(s) not yet consolidated
strict mode: deferred rows are not-consolidated; failing completion gate
```

Exit code: `1`

## Verdict

Verified. Default mode is binding-safe `PASS-DEFERRED`; strict mode correctly
blocks full-consolidation claims while SCSTRUCT05 remains open.
