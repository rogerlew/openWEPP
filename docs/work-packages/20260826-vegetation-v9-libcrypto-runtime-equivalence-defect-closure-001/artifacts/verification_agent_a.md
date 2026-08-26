# Terminal verification A

Status: initial FAIL; corrected candidate pending rereview. Evidence class:
Static + Ran.

At pushed HEAD `1c8c2eff3`, terminal verifier A independently confirmed clean
HEAD equal to `origin/main`; exact verifier SHA-256 `71ccef3c...d6148d`;
unchanged protected V9/V8 hashes; current `.4` frozen output; all five
end-to-end poison rejections; exact implementation-candidate full-profile
evidence (3,337 run, 3,326 passed, exact historical eleven failed, 34 skips);
and no production, Assurance, or terminal-posture change. Docs-only closure
evidence reuse is valid.

Finding: both terminal-verifier files still said queued/not-run while the
package claimed COMPLETE. Verdict: FAIL until the records are reconciled and
rereviewed. This candidate records that result without claiming completion.
