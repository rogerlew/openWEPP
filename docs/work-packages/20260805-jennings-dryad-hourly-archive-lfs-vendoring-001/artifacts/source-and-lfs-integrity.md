# Source And Git LFS Integrity

Ran: 2026-08-05 at implementation `HEAD`.

- Working-tree file size: `1,206,721,342` bytes.
- Working-tree SHA-256:
  `0cc82fbc5211c2c24b19653c4711d63a88fc4ed7bd90fc39cce84913d071f3a1`.
- Line count: `17,810,806`, including the header.
- Header: `Station_ID,Date,Hour,Air_Temp,Dewpoint,RH,gridded_data_pres,Prec_Type,Snow_Phase,Rain_Phase`.
- `sha256sum -c SHA256SUMS`: all ten source-native Dryad files passed.
- `git check-attr`: `filter=lfs`, `diff=lfs`, `merge=lfs`, and `text=unset`
  on the exact hourly path.
- Staged pointer: LFS specification v1, the frozen SHA-256 OID, and size
  `1206721342`.
- Local LFS object: present under `.git/lfs/objects/0c/c8/`, with the same
  size and SHA-256.
- `git lfs status`: the hourly CSV is staged as LFS object `0cc82fb`.

Disposition: pass. The index stores a small pointer while the working tree and
local LFS store retain byte-identical Dryad content. No LFS upload was made.
