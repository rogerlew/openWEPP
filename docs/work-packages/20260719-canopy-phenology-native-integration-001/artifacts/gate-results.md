# Gate Results

Evidence mode: `Ran + pending terminal`

Status: `focused gates pass; exact terminal plan not yet executed`

Focused package tests, integration tests, gate-policy schema checks, formatting,
and selected-package Clippy pass. An earlier six-package nextest attempt exposed
the intentional 32-byte `DirectDayFrame` growth from two exact consumer
observations; the size guard was documented and updated, and its focused test
then passed. That interrupted run is diagnostic only and is not closure
evidence.

Generation 7 gate policy now binds every changed science path to exactly one of
the three amended contracts and to a package-level A1 gate. A fresh exact-diff
critical plan and receipt remain mandatory after contract promotion.
