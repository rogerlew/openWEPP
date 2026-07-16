# ASSURE-04D usersum Renderer Proof

Status: PASS

Evidence class: Ran

The WEPPcloud usersum route imports
`cmarkgfm.github_flavored_markdown_to_html` in
`/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/usersum.py`. The same
installed function rendered the retained ASSURE-04D public README, report, and
supplement after final focused publication.

| Input | Result |
| --- | --- |
| Public `README.md` | PASS; 1,017 Markdown bytes; 1,193 HTML bytes; 3 links |
| Public `index.md` | PASS; 15,545 Markdown bytes; 18,111 HTML bytes; 5 tables; 6 links; 2 images |
| Public `supplement.md` | PASS; 5,649 Markdown bytes; 6,777 HTML bytes; 2 tables; 6 links |

Every rendered surface retained `TEST ONLY`; neither unresolved assembly
braces nor a raw `script` element appeared. This proves compatibility with the
current usersum Markdown consumer. It does not prove WEPPcloud manifest,
navigation, search, routing, vendoring, scientific approval, or release
acceptance.

## Adversarial Link Semantics

The same cmarkgfm function rendered a multiline raw-HTML-block target with zero
links and replaced the block with `raw HTML omitted`. It rendered a target
following `` ```not-a-close `` inside a four-backtick fence as code with zero
links. The production recognizer now consumes `pulldown-cmark` link events
rather than target text; unit and publication negatives reject both cases.
