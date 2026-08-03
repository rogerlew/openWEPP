# Assurance Source Adoption Evidence

Status: terminal contract source adopted

Evidence mode: **Ran**

The first checked adoption moved the assurance generation from
`4d83e2a92a154e8b4dedcaf94a34714494cfda8282c802f3f8558f6f7b1c4a4d`
to `a26a0352c6237c821021d7d566be824c639a1a576d200e3eab66705ca6d553bf`;
its retained transaction is
`assurance/v2/transactions/9ad8f1702af92cefa4e1874b899c7bb07c7948f3fb8d2ca6d7c81bba14717c69.json`.

Review-required contract clarification then changed the canonical source again.
A second `--check` classified the impact as `scientific-full`, after which the
same checked `adopt-report-source` operation was applied. That generation was
`f2b8a335d55361dfa97eaf4c70df3061a4d332c206de37d0345725771de91a93`;
the retained transaction is
`assurance/v2/transactions/d18e660207161df7f388ccb10661d3e9fe120c645358664a57ded454b8dd884d.json`.

Terminal review clarified that the precipitation threshold is independently
sufficient, not exclusive of retained cold/snow/frost triggers. A third checked
adoption moved the terminal generation to
`9e64c4c70ed9a5e77d1d9f1de373ef1ad11b27058d23ff030ec140ecdff36cea`;
its retained transaction is
`assurance/v2/transactions/cb88bf7b9f6d04cd65ca23f421fbf14391f8f20ea23c7dec113dbde7c0fe51ba.json`.

All three transactions report no invalidated authority. The package-local JSON
receipt records the first adoption; canonical transactions and the terminal
identity lock govern the final source chain.
