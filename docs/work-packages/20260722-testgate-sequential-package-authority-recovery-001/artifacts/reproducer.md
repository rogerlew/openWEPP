# RTR-044 Reproducer

Ran: at exact HEAD `59557953d7fbb6a972ea9a42a84c0098baf9f75a`, singular
package validation from aggregate scaffold
`ddd0e4aae924b7d9d8eca91b377106676c4d4dcf` returned
`INVALID/UNDECLARED_CHANGED_PATH`. Audit ID:
`8def403640c88bff45f2b33ab8ef70b9f50ebd6cdfd92ab84d7f60871a78631e`.
No TESTGATE attempt or gate started.

Static: the aggregate prospectively authorized both CQR modules. RTR-043 was
then independently opened and prospectively scaffolded before its correction.
The final diff is therefore covered by valid sequential authorities, but the
singular base-to-head validator can select only one package.
