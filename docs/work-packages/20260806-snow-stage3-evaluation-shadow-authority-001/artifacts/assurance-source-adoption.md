# Assurance Source Adoption

Evidence class: `Static + Ran`

The exact typed transaction chain is:

| Receipt | Transition | Purpose |
| --- | --- | --- |
| `095ab87a` | `910ab3d3 -> 4fead9cc` | initial v127 contract-source adoption |
| `b2e9d32d` | `4fead9cc -> 3ff23817` | accepted-review contract re-adoption |
| `e30ab158` | `3ff23817 -> 34f2f80e` | v127 DRAFT report-subject adoption |

All three are `scientific-full`, changed transactions with no invalidated
authority. The final snow report roots are science `fb2fe5f9`, communication
`165b881c`, review governance `d1975e59`, and content-review subject
`52566c28`.

Ran: `openwepp-assurance validate --all` passes for three DRAFT reports and
zero public reports. The snow report has empty active events and null finding,
pre-steward approval, approval, realization, and release-transfer roots. No
review, approval, release, or publication event was created.
