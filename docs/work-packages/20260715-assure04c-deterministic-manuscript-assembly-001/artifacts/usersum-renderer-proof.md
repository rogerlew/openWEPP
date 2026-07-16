# ASSURE-04C usersum Renderer Proof

Status: PASS

Evidence class: Ran

The production usersum route imports
`cmarkgfm.github_flavored_markdown_to_html` as its Markdown renderer in
`/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/usersum.py`. The same
installed function was invoked against the retained ASSURE-04C staging bytes
after the final focused rebuild.

| Input | Result |
| --- | --- |
| `index.md` | PASS; 18,025 HTML bytes, 5 tables, 6 links |
| `supplement.md` | PASS; 6,691 HTML bytes, 2 tables, 6 links |
| escaped metadata probe | PASS; no generated anchor and no raw `script` element |

The assertions required one H1 and one or more tables, no unresolved assembly
directive, no `javascript:` target, both generated figure image/alternative
records, the DOI reference, the staged `SC-GWBASEFLOW-001.md` link, the main-
report return link, and the retained focused-test object link. The probe passed
the renderer output corresponding to escaped brackets and encoded angle
brackets and asserted that it created neither an anchor nor executable HTML.

This proves parser compatibility with the renderer used by usersum. It does not
vendor the staging tree into wepppy, exercise Flask routing, authorize public
promotion, or replace ASSURE-04D review-lock work.
