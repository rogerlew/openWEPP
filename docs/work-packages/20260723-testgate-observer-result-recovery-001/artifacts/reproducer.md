# Reproducer

Ran: at exact HEAD `21ac2fdf`, attempt receipt `64a6f292...26b44` and durable
ATTEMPT record `95398d7a...31fa` sealed FAIL. The driver then raised
`NameError: package_result is not defined` from `observe()` at line 785.
