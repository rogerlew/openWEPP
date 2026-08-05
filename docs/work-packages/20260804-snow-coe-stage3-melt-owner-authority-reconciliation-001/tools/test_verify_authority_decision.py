from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("verify_authority_decision.py")
SPEC = importlib.util.spec_from_file_location("verify_authority_decision", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def test_sha256_matches_known_vector() -> None:
    assert MODULE.sha256(b"openWEPP") == hashlib.sha256(b"openWEPP").hexdigest()


def test_git_blob_reads_scaffold_base() -> None:
    root = Path(__file__).resolve().parents[4]
    data = MODULE.git_blob(
        root,
        "4c205c3c4f84a1f900710caefe3334dd69797ec3",
        "tests/integration/snow_surface_eb03_contract.rs",
    )
    assert hashlib.sha256(data).hexdigest() == "a30d4fe9f810b212850b55255d404b3d7b2883289446e4deab297bcc50261e25"
