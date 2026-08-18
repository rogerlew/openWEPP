#!/usr/bin/env python3
"""Independent non-Rust OPENWEPP_C3_WOODY_V9 oracle.

V9 imports the exact V8 equation implementation and changes only the
reproducible numerical-runtime, serialization, definition, and vector identity.
The V8 module is checksum-bound before import.  Generation writes only stdout;
repository authority bytes are never opened for writing.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import struct
import sys
from typing import Any

HERE = Path(__file__).resolve().parent
V8_CALCULATOR = (
    HERE.parents[2]
    / "20260814-snow-free-land-surface-energy-authority-001"
    / "artifacts/reference_lse_v8_joint_canopy_core.py"
)
V8_CALCULATOR_SHA256 = "525538f32c91e2377f5d58f72fa4cfff2e81d46d5e12555e79792d92e1e81d6f"
V8_DEFINITION_SHA256 = "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b"
V9_DEFINITION = HERE / "openwepp_c3_woody_v9_definition.json"
RUNTIME_DESCRIPTOR = HERE / "runtime_descriptor.json"
DYNAMIC_OBJECT_PATHS = {
    "ld-linux-x86-64.so.2": Path("/lib64/ld-linux-x86-64.so.2"),
    "libc.so.6": Path("/usr/lib/x86_64-linux-gnu/libc.so.6"),
    "libcrypto.so.3": Path("/usr/lib/x86_64-linux-gnu/libcrypto.so.3"),
    "libexpat.so.1": Path("/usr/lib/x86_64-linux-gnu/libexpat.so.1"),
    "libm.so.6": Path("/usr/lib/x86_64-linux-gnu/libm.so.6"),
    "libz.so.1": Path("/usr/lib/x86_64-linux-gnu/libz.so.1"),
    "libzstd.so.1": Path("/usr/lib/x86_64-linux-gnu/libzstd.so.1"),
}
RUNTIME_FILE_PATHS = {
    "LC_CTYPE": Path("/usr/lib/locale/C.utf8/LC_CTYPE"),
    "gconv-modules.cache": Path("/usr/lib/x86_64-linux-gnu/gconv/gconv-modules.cache"),
    "locale-archive": Path("/usr/lib/locale/locale-archive"),
    "openssl.cnf": Path("/usr/lib/ssl/openssl.cnf"),
}


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _directory_closure_sha256(root: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(item for item in root.rglob("*") if item.is_file()):
        relative = path.relative_to(root).as_posix().encode("utf-8")
        content = _sha256(path).encode("ascii")
        digest.update(relative + b"\0" + content + b"\n")
    return digest.hexdigest()


def _host_cpu_identity() -> dict[str, Any]:
    first = Path("/proc/cpuinfo").read_text(encoding="ascii").split("\n\n", 1)[0]
    fields = {}
    for line in first.splitlines():
        if ":" in line:
            key, value = line.split(":", 1)
            fields[key.strip()] = value.strip()
    word = struct.calcsize("P")
    fmt = "=QQ" if word == 8 else "=II"
    auxv = Path("/proc/self/auxv").read_bytes()
    auxiliary = {}
    for offset in range(0, len(auxv), word * 2):
        kind, value = struct.unpack_from(fmt, auxv, offset)
        if kind == 16:
            auxiliary["AT_HWCAP"] = f"0x{value:x}"
        elif kind == 26:
            auxiliary["AT_HWCAP2"] = f"0x{value:x}"
    return {
        "vendor_id": fields["vendor_id"],
        "cpu_family": fields["cpu family"],
        "model": fields["model"],
        "stepping": fields["stepping"],
        "flags": fields["flags"],
        **auxiliary,
    }


def _load_v8() -> Any:
    actual = _sha256(V8_CALCULATOR)
    if actual != V8_CALCULATOR_SHA256:
        raise RuntimeError(f"V8 calculator checksum mismatch: {actual}")
    spec = importlib.util.spec_from_file_location("openwepp_v8_exact_import", V8_CALCULATOR)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load exact V8 calculator")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _verify_runtime(descriptor: dict[str, Any]) -> None:
    executable = Path(sys.executable).resolve()
    if descriptor["model_version"] != "OPENWEPP_C3_WOODY_V9":
        raise RuntimeError("runtime descriptor model identity mismatch")
    if descriptor["architecture"] != platform.machine():
        raise RuntimeError("runtime architecture mismatch")
    if descriptor["cpu_identity"] != _host_cpu_identity():
        raise RuntimeError("CPU/HWCAP identity mismatch")
    expected_os = f"{platform.system()} {platform.release()}"
    if descriptor["operating_system"] != expected_os:
        raise RuntimeError("runtime operating-system mismatch")
    if descriptor["python"]["executable"] != str(executable):
        raise RuntimeError("Python executable path mismatch")
    if descriptor["python"]["implementation"] != platform.python_implementation():
        raise RuntimeError("Python implementation mismatch")
    if descriptor["python"]["version"] != platform.python_version():
        raise RuntimeError("Python version mismatch")
    expected_executable = descriptor["python"]["executable_sha256"].removeprefix(
        "sha256:")
    if _sha256(executable) != expected_executable:
        raise RuntimeError("Python executable checksum mismatch")
    if set(descriptor["dynamic_objects"]) != set(DYNAMIC_OBJECT_PATHS):
        raise RuntimeError("dynamic object inventory mismatch")
    for name, path in DYNAMIC_OBJECT_PATHS.items():
        record = descriptor["dynamic_objects"][name]
        if Path(record["path"]) != path:
            raise RuntimeError(f"dynamic object path mismatch: {name}")
        expected = record["sha256"].removeprefix("sha256:")
        if _sha256(path) != expected:
            raise RuntimeError(f"dynamic object checksum mismatch: {name}")
    if set(descriptor["runtime_files"]) != set(RUNTIME_FILE_PATHS):
        raise RuntimeError("runtime file inventory mismatch")
    for name, path in RUNTIME_FILE_PATHS.items():
        record = descriptor["runtime_files"][name]
        if Path(record["path"]) != path or _sha256(path) != record["sha256"]:
            raise RuntimeError(f"runtime file closure mismatch: {name}")
    closure = descriptor["directory_closures"]["python_stdlib"]
    stdlib = Path(closure["path"])
    if _directory_closure_sha256(stdlib) != closure["manifest_sha256"]:
        raise RuntimeError("Python standard-library closure mismatch")
    expected_command = [str(executable), "-I", "-S", "-B", Path(__file__).name]
    if descriptor["generation_command"] != expected_command:
        raise RuntimeError("generation command descriptor mismatch")
    if not sys.dont_write_bytecode:
        raise RuntimeError("generator requires Python -B; bytecode writes forbidden")
    if sys.orig_argv[1:4] != ["-I", "-S", "-B"] or len(sys.orig_argv) != 5:
        raise RuntimeError("executed generation command flags are not exact")
    if Path(sys.orig_argv[0]).resolve() != executable:
        raise RuntimeError("executed generation command uses wrong interpreter")
    if Path(sys.orig_argv[4]).resolve() != Path(__file__).resolve():
        raise RuntimeError("executed generation command uses wrong calculator")
    if not sys.flags.isolated or not sys.flags.no_site:
        raise RuntimeError("generator requires isolated no-site execution")
    if len(sys.argv) != 1 or Path(sys.argv[0]).resolve() != Path(__file__).resolve():
        raise RuntimeError("generator must execute directly without arguments")
    expected_serialization = {
        "allow_nan": False,
        "encoding": "UTF-8",
        "ensure_ascii": True,
        "json_separators": [",", ":"],
        "newline": "LF",
        "sort_keys": True,
    }
    if descriptor["serialization"] != expected_serialization:
        raise RuntimeError("serializer descriptor mismatch")
    if os.environ.get("LC_ALL") != descriptor["locale"]:
        raise RuntimeError("LC_ALL does not match runtime descriptor")
    if os.environ.get("SOURCE_DATE_EPOCH") != descriptor["source_date_epoch"]:
        raise RuntimeError("SOURCE_DATE_EPOCH does not match runtime descriptor")


def _replace_v8_identity(value: Any, v9_definition_sha256: str) -> Any:
    if isinstance(value, dict):
        return {key: _replace_v8_identity(item, v9_definition_sha256)
                for key, item in value.items()}
    if isinstance(value, list):
        return [_replace_v8_identity(item, v9_definition_sha256) for item in value]
    if value == V8_DEFINITION_SHA256:
        return v9_definition_sha256
    if value == "OPENWEPP_C3_WOODY_V8":
        return "OPENWEPP_C3_WOODY_V9"
    return value


def build_vectors() -> dict[str, Any]:
    definition = json.loads(V9_DEFINITION.read_text(encoding="utf-8"))
    descriptor = json.loads(RUNTIME_DESCRIPTOR.read_text(encoding="utf-8"))
    _verify_runtime(descriptor)
    if definition["model_version"] != "OPENWEPP_C3_WOODY_V9":
        raise RuntimeError("wrong V9 definition model identity")
    if definition["canonical_contract"] != "SC-VEGETATION-001@13":
        raise RuntimeError("wrong V9 canonical contract binding")
    if definition["oracle_identity"]["calculator_sha256"] != _sha256(Path(__file__)):
        raise RuntimeError("V9 calculator checksum mismatch")
    if definition["base_model_definition_sha256"] != V8_DEFINITION_SHA256:
        raise RuntimeError("V9 does not import exact V8 definition")
    descriptor_sha256 = _sha256(RUNTIME_DESCRIPTOR)
    if definition["oracle_identity"]["runtime_descriptor_sha256"] != descriptor_sha256:
        raise RuntimeError("runtime descriptor checksum mismatch")
    v9_definition_sha256 = _sha256(V9_DEFINITION)
    # Hash exact V8 execution bytes without rewriting embedded V8 identity.
    # V9 identity is carried by the separate authority envelope, avoiding a
    # definition/vector digest cycle while proving an unchanged V8 import.
    imported_vectors = _load_v8().build_expanded_joint_vectors()
    canonical = lambda value: json.dumps(
        value, sort_keys=True, separators=(",", ":"), ensure_ascii=True,
        allow_nan=False).encode("utf-8")
    scenario_sha256 = {
        key: hashlib.sha256(canonical(value)).hexdigest()
        for key, value in sorted(imported_vectors.items())
    }
    imported_execution = {
        "canonical_byte_length": len(canonical(imported_vectors)),
        "canonical_sha256": hashlib.sha256(canonical(imported_vectors)).hexdigest(),
        "scenario_sha256": scenario_sha256,
    }
    payload_sha256 = hashlib.sha256(canonical(imported_execution)).hexdigest()
    if definition["oracle_identity"]["vector_payload_sha256"] != payload_sha256:
        raise RuntimeError("V9 vector payload checksum mismatch")
    return {
        "authority": {
            "base_model_definition_sha256": V8_DEFINITION_SHA256,
            "model_definition_sha256": v9_definition_sha256,
            "model_version": "OPENWEPP_C3_WOODY_V9",
            "runtime_descriptor_sha256": descriptor_sha256,
            "supersession_scope": "numerical-runtime-and-serialization-identity-only",
        },
        "imported_v8_execution": imported_execution,
    }


if __name__ == "__main__":
    print(json.dumps(build_vectors(), sort_keys=True, separators=(",", ":"),
                     ensure_ascii=True, allow_nan=False))
