# Release and Current Blob Freeze

Evidence class: `Static + Ran`

Starting commit: `448c767b0024bfc63cfdb41b85885194b36a2603`

## V3

Release commit: `94a4c99dc1228aa0399c01f4cc9590742960028f`.

| Artifact | Git blob | SHA-256 | Bytes | Release vs current |
|---|---|---:|---:|---|
| `reference_calculator.py` | `bf938111...` | `7b137c1aa9ed0912caf4d14c779eca1819014b4217156d36f98619f06daabd1a` | 80,157 | byte-identical |
| `openwepp_c3_woody_v3_vectors.json` | `465d2376...` | `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109` | 49,915 | byte-identical |
| `openwepp_c3_woody_v3_definition.json` | `87c8df5e...` | `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852` | 8,205 | byte-identical |

The full SC-VEGETATION contract grew by later amendments, but the frozen V3
definition and its bound canonical section identities did not drift.

The full `SC-VEGETATION-001.md` blob at the V3 release commit is Git blob
`9140b46cdd27d9be263c45d448ff810acb2a769b`, SHA-256
`018e06bba77adc85717715f12271d002070e2339dcf2151da1cd27ff19a06ad4`,
and 134,422 bytes.

## V5

Release commit: `b7e6f08b655452c5c59a498ac9becd1439dd21ef`.

| Artifact | Git blob | SHA-256 | Bytes | Release vs current |
|---|---|---:|---:|---|
| `reference_calculator_v5.py` | `7398f35e...` | `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775` | 58,004 | byte-identical |
| `openwepp_c3_woody_v5_vectors.json` | `f6bfb38f...` | `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d` | 97,724 | byte-identical |
| `openwepp_c3_woody_v5_definition.json` | `630b79b4...` | `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3` | 2,590 | byte-identical |

The full `SC-VEGETATION-001.md` blob at the V5 release commit is Git blob
`4580850cec56bfd08becf7fd1264132add85d483`, SHA-256
`edfd8583b5a4b62f266fb52849a5e28248245ef91ecbb887c9237130113c19b2`,
and 167,857 bytes. At the package starting/current commit
`448c767b0024bfc63cfdb41b85885194b36a2603`, it is Git blob
`29044ce5d3111baba44c11d5199969ade90dbfac`, SHA-256
`1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`,
and 194,464 bytes. These full-contract blobs intentionally differ as later
reviewed amendments accumulated; the immutable V3/V5 definition-bound section
identities and protected artifacts above did not change.

## Isolated exact-release execution

Release trees were exported with `git archive` to isolated temporary roots and
executed with current `.venv/bin/python` (CPython 3.12.14, GCC 15.2,
Linux/glibc 2.42). Repository bytes were not writable targets.

- V3 generated `7e64d63729b538ff5721ded768eb62be4be195a7903464a2ac7a3ab2083bff00`
  (49,913 bytes), not frozen `1210e41f...`; all eight calculator self-checks
  were true.
- V5 generated `327b349cac6dcb4793c61f2d211f20c0140bd27cbc45f180b0f49816accc1eb2`,
  not frozen `6f5e9554...`; its generated definition was `51e72707...`.

The release and current source blobs generate the same divergent outputs in the
same current environment. Source drift is excluded; historical environment
provenance remains missing.
