# Protected Surface Manifest

Ran: PASS. Each digest is SHA-256 over the byte-sorted `sha256sum` manifest of
tracked files in the named scope. Intake, pre-review, and terminal values are
identical.

| Protected scope | Files | Intake digest | Terminal digest |
| --- | ---: | --- | --- |
| `assurance/v2/reports` | 34 | `951ad12b6fb6f1f36b9758fa7b627d222a6c97904efcb238fd547106711d40db` | `951ad12b6fb6f1f36b9758fa7b627d222a6c97904efcb238fd547106711d40db` |
| `assurance/v2/identity.lock.json` | 1 | `faa5086acbc771966acd2579e1832113e2f7a84bb392c959e05ce418d3128bd4` | `faa5086acbc771966acd2579e1832113e2f7a84bb392c959e05ce418d3128bd4` |
| `assurance/v2/principals.yaml` | 1 | `e86da20fad381be86899b3b95583fd1a33728c1e600eda1679890576d3430c05` | `e86da20fad381be86899b3b95583fd1a33728c1e600eda1679890576d3430c05` |
| `assurance/v2/transactions` | 17 | `ffe1c9a07ce033c2f2d4ff2b616ba8b7a43095c83a74dbedf16837436829c557` | `ffe1c9a07ce033c2f2d4ff2b616ba8b7a43095c83a74dbedf16837436829c557` |
| `usersum` | 11 | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

Static: `git status --short` also shows no write under any protected scope.
