# DRAFT Assurance Impact

Ran: typed source adoption and report validation.

The package changed canonical `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001`, both
identified sources of the DRAFT snow-and-frozen-soil process report. Typed
`adopt-report-source` operations created transactions
`94e8663ca2a94589f16d5383d6e1385a3ef69dfffbc8d8518dcb892567924f65`
and `5e52e938eb1833de40ab10fb1367fbf56d7e7b9ff988c89fa714360a478b38be`.
Generation advanced from `7d1a3ba111cb4e4f4b9f72b0efb72e6486087c011eb360121b86598961198fb8`
through `1e0b1accfcc2f3d46984ecbd537fa7bc2be6eff857c0f72ee40dbb927d5ff148`
to `dbfccf2420c3de654e12f0d185dc0fcca3ebaecb8b71c4fcf23e3fff6e2bc6fb`.

`openwepp-assurance validate --report snow-and-frozen-soil-process-evaluation`
passes. The plan reports the selected report `current` with source root
`0bf8b46611580bda182f8c906306f11ae31aa116b1458a192a0d1282cf4e2e76`.
Lifecycle remains `DRAFT`; public report count remains zero. No review,
approval, release, lifecycle promotion, or publication was performed.

The canonical review-draft renderer applied and then passed its exact check
over 98 governed files. Only the review-draft README, snow build manifest, and
the projected `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001` research objects
changed.
