# Attempt 06: Execution-Snapshot Preflight

Evidence class: Ran.

Durable root:
`/home/workdir/openWEPP-quality-history/20260724-order3-attempt6-preflight-WoUCDS`.

Executed head:
`5dd608df8d011daf246b71a95f81a01d9e7b050a`.

Admission ID:
`df14e6124c909e8de391f8e5053c8aee8b5ee777b85224d24991ef9c447ce9d6`.

## Admission

- `full`: 2,279;
  `7b206070ff04230f834bcc03d512b66bae35b03aa80a2593adf85f3a9537b995`
- `science-manual`: 36;
  `6dee4feea00b245d22bda1250fd2cb0a53741c77ea0dedbead517e068159b902`
- `workspace`: 2,315;
  `552eba5b04293b30458060102f404cf51e63c7c20ed2e60ef5d22c3c48937b11`

Source and execution snapshot were exact at the named head with empty porcelain
status. Snapshot `.git/info/exclude` was exactly seven bytes:
`2f 2e 76 65 6e 76 0a`, or `/.venv\n`.

## Exact Attempt-5 Failures

The three coverage-configured exact-checkout identities ran inside the admitted
execution snapshot: 3 passed, 3 slow, 177 skipped, `1265.185s`, exit 0.

Durable sibling evidence:

- `<root>.admit.{log,exit-code}`
- `<root>.exact-three.{log,exit-code}`
- `<root>.exact-target`

No collection, publication, retry, or repository edit occurred. The correction
is authorized for a fresh complete transition.
