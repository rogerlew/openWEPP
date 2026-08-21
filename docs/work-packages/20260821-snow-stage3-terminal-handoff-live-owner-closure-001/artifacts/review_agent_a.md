Status: complete
Evidence mode: Static local review; delegation unavailable

Review focus: consumer path and ordinary API. Findings:

1. Old request/executor methods remain in `03_executor.rs` only under
   `#[cfg(test)]`. Disposition: accepted; they are retained as internal test
   expectations and are not part of the ordinary compiled API.
2. The ordinary runner uses the persistent attachment scheduler hook and the
   immutable batch accessor. Disposition: accepted.
3. The returned batch exposes rows only through an immutable slice accessor.
   Disposition: accepted.

No unresolved consumer-path finding.
