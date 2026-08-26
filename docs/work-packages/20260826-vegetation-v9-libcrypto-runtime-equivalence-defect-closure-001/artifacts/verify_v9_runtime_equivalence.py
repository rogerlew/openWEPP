#!/usr/bin/env python3
"""Verify frozen V9 authority under exact-host or bounded SHA-256 equivalence."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
from typing import Any, Callable


REPO = Path(__file__).resolve().parents[4]
V9 = (
    REPO
    / "docs/work-packages/20260817-c3-woody-v3-v5-oracle-reconciliation-001"
    / "artifacts/v9"
)
CALCULATOR = V9 / "reference_calculator_v9.py"
DESCRIPTOR = V9 / "runtime_descriptor.json"
DEFINITION = V9 / "openwepp_c3_woody_v9_definition.json"
VECTORS = V9 / "openwepp_c3_woody_v9_vectors.json"
V8_CALCULATOR = (
    REPO
    / "docs/work-packages/20260814-snow-free-land-surface-energy-authority-001"
    / "artifacts/reference_lse_v8_joint_canopy_core.py"
)
LIBCRYPTO = Path("/usr/lib/x86_64-linux-gnu/libcrypto.so.3")

PROTECTED_SHA256 = {
    CALCULATOR: "05cee9082a2595fe3692c4e0ad69dd9d190d2b0577e3c9a71d53d8494156ad5a",
    DESCRIPTOR: "e0d05e49eabe43340e9fc7e251b319bcd08305d59af522298001b3c4f6bf951f",
    DEFINITION: "f388aa883631d935e89368d8ca6e0275db4f6c00292ff0a6adf1936d7b71bcd0",
    VECTORS: "f86770cce11235ba282b47e81de2fa5dc9af19c29dc3bd91c62256957c590633",
    V8_CALCULATOR: "525538f32c91e2377f5d58f72fa4cfff2e81d46d5e12555e79792d92e1e81d6f",
}

KNOWN_ANSWERS = (
    (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
    (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
    (
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
    ),
)


class VerificationError(RuntimeError):
    """Typed local failure for V9 verification evidence."""


def _sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def _sha256_path(path: Path) -> str:
    return _sha256_bytes(path.read_bytes())


def _sha256_fd(file_descriptor: int) -> str:
    digest = hashlib.sha256()
    offset = 0
    while chunk := os.pread(file_descriptor, 1024 * 1024, offset):
        digest.update(chunk)
        offset += len(chunk)
    return digest.hexdigest()


def _verify_protected(
    byte_overrides: dict[Path, bytes] | None = None,
) -> dict[Path, bytes]:
    overrides = byte_overrides or {}
    retained = {}
    for path, expected in PROTECTED_SHA256.items():
        value = overrides.get(path, path.read_bytes())
        actual = _sha256_bytes(value)
        if actual != expected:
            raise VerificationError(
                f"VEG-E-133: protected V9/V8 bytes changed: {path}: {actual}"
            )
        retained[path] = value
    return retained


def _verify_sha256_capability(
    factory: Callable[[bytes], Any] = hashlib.sha256,
) -> None:
    for value, expected in KNOWN_ANSWERS:
        if factory(value).hexdigest() != expected:
            raise VerificationError("VEG-E-133: SHA-256 known-answer mismatch")

    streamed = factory(b"")
    for chunk in (b"a", b"b", b"c"):
        streamed.update(chunk)
    if streamed.hexdigest() != KNOWN_ANSWERS[1][1]:
        raise VerificationError("VEG-E-133: SHA-256 streaming mismatch")


ProviderIdentity = tuple[str, int, int, int]


def _provider_file_identity(path: Path) -> ProviderIdentity:
    status = path.stat()
    return (
        str(path.resolve()),
        os.major(status.st_dev),
        os.minor(status.st_dev),
        status.st_ino,
    )


def _provider_fd_identity(file_descriptor: int) -> ProviderIdentity:
    status = os.fstat(file_descriptor)
    return (
        str(Path(f"/proc/self/fd/{file_descriptor}").resolve()),
        os.major(status.st_dev),
        os.minor(status.st_dev),
        status.st_ino,
    )


def _loaded_libcrypto_identities() -> set[ProviderIdentity]:
    # Importing hashlib above loads CPython's _hashlib provider before this map
    # is inspected. Device/inode identity prevents a same-path replacement from
    # being mistaken for the object that supplied the exercised SHA-256 code.
    identities = set()
    for line in Path("/proc/self/maps").read_text(encoding="ascii").splitlines():
        fields = line.split(maxsplit=5)
        if len(fields) != 6 or "libcrypto.so.3" not in fields[5]:
            continue
        if fields[5].endswith(" (deleted)"):
            raise VerificationError("VEG-E-133: loaded libcrypto mapping is deleted")
        major, minor = (int(value, 16) for value in fields[3].split(":", 1))
        identities.add((str(Path(fields[5]).resolve()), major, minor, int(fields[4])))
    return identities


def _load_calculator() -> Any:
    spec = importlib.util.spec_from_file_location("openwepp_v9_exact", CALCULATOR)
    if spec is None or spec.loader is None:
        raise VerificationError("VEG-E-133: cannot load exact V9 calculator")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _with_calculator_argv(function: Callable[[], Any]) -> Any:
    original_argv = sys.argv
    original_orig_argv = sys.orig_argv
    executable = str(Path(sys.executable).resolve())
    try:
        sys.argv = [str(CALCULATOR)]
        sys.orig_argv = [executable, "-I", "-S", "-B", str(CALCULATOR)]
        return function()
    finally:
        sys.argv = original_argv
        sys.orig_argv = original_orig_argv


def _verify_runtime(
    module: Any,
    descriptor: dict[str, Any],
    provider_fd: int,
    sha256_factory: Callable[[bytes], Any] = hashlib.sha256,
    loaded_identity_override: set[ProviderIdentity] | None = None,
) -> tuple[str, str, ProviderIdentity]:
    expected = descriptor["dynamic_objects"]["libcrypto.so.3"]["sha256"].removeprefix(
        "sha256:"
    )
    actual = _sha256_fd(provider_fd)
    mismatches = []
    for name, record in descriptor["dynamic_objects"].items():
        path = Path(record["path"])
        observed = _sha256_path(path)
        wanted = record["sha256"].removeprefix("sha256:")
        if observed != wanted:
            mismatches.append(name)
    if mismatches not in ([], ["libcrypto.so.3"]):
        raise VerificationError(
            "VEG-E-133: runtime mismatch is not libcrypto-only: "
            + ",".join(mismatches)
        )

    expected_loaded_identity = _provider_fd_identity(provider_fd)
    if _provider_file_identity(LIBCRYPTO) != expected_loaded_identity:
        raise VerificationError(
            "VEG-E-133: provider pathname does not identify retained object"
        )
    loaded_identities = (
        loaded_identity_override
        if loaded_identity_override is not None
        else _loaded_libcrypto_identities()
    )
    if loaded_identities != {expected_loaded_identity}:
        raise VerificationError(
            "VEG-E-133: CPython loaded unexpected libcrypto object identity: "
            + repr(sorted(loaded_identities))
        )
    if (
        hashlib.sha256.__module__ != "_hashlib"
        or hashlib.sha256.__name__ != "openssl_sha256"
    ):
        raise VerificationError("VEG-E-133: hashlib.sha256 is not the OpenSSL provider")

    route = "exact-host"
    original_sha256 = module._sha256
    if mismatches:
        route = "sha256-provider-equivalent"
        _verify_sha256_capability(sha256_factory)

    def retained_provider_sha256(path: Path) -> str:
        if Path(path) == LIBCRYPTO:
            if _sha256_fd(provider_fd) != actual:
                raise VerificationError("VEG-E-133: retained provider bytes changed in-run")
            return expected
        return original_sha256(path)

    module._sha256 = retained_provider_sha256

    try:
        _with_calculator_argv(lambda: module._verify_runtime(descriptor))
    except VerificationError:
        raise
    except Exception as error:
        raise VerificationError(f"VEG-E-133: runtime verification failed: {error}") from error
    return route, actual, expected_loaded_identity


def _verify_output(output: bytes) -> None:
    expected = VECTORS.read_bytes()
    if output != expected:
        raise VerificationError(
            "VEG-E-133: complete V9 output mismatch: " + _sha256_bytes(output)
        )


def verify(
    *,
    descriptor_override: dict[str, Any] | None = None,
    protected_overrides: dict[Path, bytes] | None = None,
    sha256_factory: Callable[[bytes], Any] = hashlib.sha256,
    loaded_identity_override: set[ProviderIdentity] | None = None,
    output_transform: Callable[[bytes], bytes] | None = None,
) -> tuple[bytes, str, str]:
    before = _verify_protected(protected_overrides)
    descriptor = descriptor_override or json.loads(DESCRIPTOR.read_text(encoding="utf-8"))
    try:
        provider_fd = os.open(LIBCRYPTO, os.O_RDONLY | os.O_CLOEXEC)
    except OSError as error:
        raise VerificationError(f"VEG-E-133: cannot retain provider object: {error}") from error
    try:
        module = _load_calculator()
        route, provider_sha256, loaded_identity = _verify_runtime(
            module,
            descriptor,
            provider_fd,
            sha256_factory=sha256_factory,
            loaded_identity_override=loaded_identity_override,
        )
        try:
            value = _with_calculator_argv(module.build_vectors)
        except VerificationError:
            raise
        except Exception as error:
            raise VerificationError(f"VEG-E-133: exact calculator failed: {error}") from error
        output = (
            json.dumps(
                value,
                sort_keys=True,
                separators=(",", ":"),
                ensure_ascii=True,
                allow_nan=False,
            ).encode("utf-8")
            + b"\n"
        )
        if output_transform is not None:
            output = output_transform(output)
        _verify_output(output)
        after = _verify_protected(protected_overrides)
        if after != before:
            raise VerificationError("VEG-E-133: protected bytes changed during execution")
        if _provider_fd_identity(provider_fd) != loaded_identity:
            raise VerificationError("VEG-E-133: retained provider identity changed in-run")
        if _provider_file_identity(LIBCRYPTO) != loaded_identity:
            raise VerificationError("VEG-E-133: provider pathname changed in-run")
        if _sha256_fd(provider_fd) != provider_sha256:
            raise VerificationError("VEG-E-133: retained provider bytes changed in-run")
        return output, route, provider_sha256
    finally:
        os.close(provider_fd)


class _WrongSha256:
    def __init__(self, _value: bytes = b"") -> None:
        self._value = bytearray()

    def update(self, value: bytes) -> None:
        self._value.extend(value)

    def hexdigest(self) -> str:
        return "0" * 64


def _expect_rejection(
    name: str, expected_message: str, function: Callable[[], Any]
) -> str:
    try:
        function()
    except VerificationError as error:
        if expected_message in str(error):
            return name
        raise VerificationError(
            f"VEG-E-133: poison {name} rejected at wrong stage: {error}"
        ) from error
    raise VerificationError(f"VEG-E-133: poison was accepted: {name}")


def self_test_poisons() -> dict[str, list[str]]:
    verify()
    descriptor = json.loads(DESCRIPTOR.read_text(encoding="utf-8"))
    wrong_runtime = copy.deepcopy(descriptor)
    wrong_runtime["dynamic_objects"]["libc.so.6"]["sha256"] = "sha256:" + "0" * 64
    return {
        "rejected": [
            _expect_rejection(
                "wrong_sha256_provider_result",
                "SHA-256 known-answer mismatch",
                lambda: verify(sha256_factory=_WrongSha256),
            ),
            _expect_rejection(
                "mapped_provider_identity_mismatch",
                "loaded unexpected libcrypto object identity",
                lambda: verify(loaded_identity_override={("wrong", 0, 0, 0)}),
            ),
            _expect_rejection(
                "second_runtime_mismatch",
                "runtime mismatch is not libcrypto-only",
                lambda: verify(descriptor_override=wrong_runtime),
            ),
            _expect_rejection(
                "changed_protected_bytes",
                "protected V9/V8 bytes changed",
                lambda: verify(protected_overrides={CALCULATOR: b"changed"}),
            ),
            _expect_rejection(
                "changed_generated_output",
                "complete V9 output mismatch",
                lambda: verify(output_transform=lambda output: output + b" "),
            ),
        ]
    }


def main() -> int:
    if os.environ.get("LC_ALL") != "C.UTF-8" or os.environ.get(
        "SOURCE_DATE_EPOCH"
    ) != "0":
        raise VerificationError("VEG-E-133: canonical locale/epoch required")
    if sys.argv[1:] == ["--self-test-poisons"]:
        print(json.dumps(self_test_poisons(), sort_keys=True, separators=(",", ":")))
        return 0
    if sys.argv[1:]:
        raise VerificationError("VEG-E-133: unexpected verifier arguments")
    output, route, provider_sha256 = verify()
    print(
        f"V9_RUNTIME route={route} libcrypto_sha256={provider_sha256}",
        file=sys.stderr,
    )
    sys.stdout.buffer.write(output)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except VerificationError as error:
        print(str(error), file=sys.stderr)
        raise SystemExit(1) from None
