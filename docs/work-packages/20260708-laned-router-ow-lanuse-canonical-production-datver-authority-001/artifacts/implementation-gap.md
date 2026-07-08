# Implementation Gap

Evidence class: Static.
Status: complete.

## Open Runtime Gaps

- openWEPP must enforce the rev-49 native datver rule in the default/active Lane
  D eligibility resolver.
- Native `ow-lanuse-1` scheduled lanes with missing route coefficients must fail
  closed for Lane D production instead of being treated like all-legacy
  no-coefficient fallback.
- Mixed scheduled native/legacy datvers must fail closed for Lane D production.
- Runtime diagnostics should distinguish all-legacy fallback, native
  missing-coefficient failure, mixed datver failure, mixed coefficient failure,
  and explicit-disable rollback.
- Existing protected all-legacy/off identity tests must remain.

## Open Producer Gaps

- WEPPpy Disturbed/native producers must emit `ow-lanuse-1` native management
  files with embedded `routing_coefficients`.
- Producer evidence must prove the five coefficients are materialized in the
  `.man`, not supplied by an optional sidecar.
- Existing legacy datver generation may remain for compatibility/validation, but
  must not be described as new-physics Lane D production authority.

## Next Action

Scaffold an implementation package that updates openWEPP runtime eligibility
guards and coordinates the wepppy producer migration. The implementation package
must include parser/selector tests for:

- all-native complete coefficients: active/default attaches Lane D;
- all-legacy datvers: legacy/off protected output path;
- native missing coefficients: fail closed;
- mixed native/legacy datvers: fail closed;
- optional sidecar attempt: unsupported/fail closed;
- explicit disable on native-complete input: protected legacy/off identity.
