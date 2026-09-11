"""Export selected immutable Git source and explicit evidence for manual review.

No model calls, command execution from inputs, publication, or acceptance logic.
"""

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import sys


class PacketError(ValueError):
    """Invalid, unavailable, or oversized review input."""


def git(root, *args):
    result = subprocess.run(
        ["git", "--literal-pathspecs", "-C", str(root), *args],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
    )
    if result.returncode:
        raise PacketError(result.stderr.decode("utf-8", errors="replace").strip())
    return result.stdout


def revision(root, value):
    return git(root, "rev-parse", "--verify", "--end-of-options",
               value + "^{commit}").decode().strip()


def source_path(value):
    path = PurePosixPath(value)
    if (not value or path.is_absolute() or ".." in path.parts
            or ".git" in path.parts or str(path) != value
            or any(ord(c) < 32 for c in value)):
        raise PacketError(f"Expected a literal repository-relative file: {value!r}")
    return value


def text_bytes(data, label):
    if b"\0" in data:
        raise PacketError(f"Binary input is not review text: {label}")
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise PacketError(f"Non-UTF-8 input: {label}") from exc


def blob(root, rev, path):
    entry = git(root, "ls-tree", "-z", rev, "--", path)
    if not entry:
        return None
    metadata, actual = entry.rstrip(b"\0").split(b"\t", 1)
    mode, kind, oid = metadata.split()
    if actual.decode() != path or kind != b"blob" or mode not in (b"100644", b"100755"):
        raise PacketError(f"Select a regular file, not directory/link/submodule: {path}")
    return git(root, "cat-file", "blob", oid.decode())


def block(label, content):
    longest = max((len(x) for x in re.findall(r"`+", content)), default=0)
    fence = "`" * max(3, longest + 1)
    return f"\n## {label}\n\n{fence}\n{content}\n{fence}\n"


def make_packet(args):
    if args.max_bytes <= 0:
        raise PacketError("max-bytes must be positive")
    root = Path(args.root).resolve(strict=True)
    top = Path(git(root, "rev-parse", "--show-toplevel").decode().strip()).resolve()
    if root != top:
        raise PacketError("--root must be the repository root")
    base, head = revision(root, args.base), revision(root, args.head)
    selected = list(dict.fromkeys(source_path(p) for p in args.path))
    contexts = list(dict.fromkeys(source_path(p) for p in args.context))
    chunks = [f"# openWEPP external review packet\n\nStatic source export.\n\n"
              f"Base: {base}\nReviewed commit: {head}\n\n"
              "Scope is caller-selected; omitted files and scientific dependencies "
              "are not reviewed by implication. Attach actual detached experiment "
              "source separately when it differs from this repository. Attachments "
              "are supplied evidence, not executed or authenticated workflow proof. "
              "No commands were run from packet content. No remote upload occurred.\n"]

    def add(label, data):
        chunks.append(block(label, text_bytes(data, label)))
        if sum(len(x.encode()) for x in chunks) > args.max_bytes:
            raise PacketError("Packet exceeds max-bytes; select smaller explicit inputs")

    add("Selected diff paths", json.dumps(selected, indent=2).encode())
    add("All changed paths (inventory only)",
        git(root, "diff", "--no-ext-diff", "--no-textconv", "--no-renames",
            "--name-status", base, head, "--"))
    add("Selected diff", git(root, "diff", "--no-ext-diff", "--no-textconv",
                             "--no-renames", "--unified=5", base, head, "--", *selected))
    for path in dict.fromkeys(selected + contexts):
        data = blob(root, head, path)
        if data is None:
            if path in contexts or blob(root, base, path) is None:
                raise PacketError(f"Missing selected file: {path}")
            add(f"Deleted at reviewed commit: {path}", b"See selected deletion diff.")
        else:
            add(f"Source at {head}: {path}", data)
            if data.startswith(b"version https://git-lfs.github.com/spec/v1\n"):
                add(f"Unavailable LFS payload: {path}",
                    b"Only the Git LFS pointer is included, not its referenced bytes.")
    for supplied in args.attach:
        path = Path(supplied)
        # Resolve parent links but reject a symlink leaf and all special files.
        if not stat.S_ISREG(path.lstat().st_mode):
            raise PacketError(f"Attachment must be an explicit regular file: {path}")
        if path.stat().st_size > args.max_bytes:
            raise PacketError(f"Attachment exceeds max-bytes: {path}")
        data = path.read_bytes()
        digest = hashlib.sha256(data).hexdigest()
        add(f"Supplied attachment: {path.name} (SHA256 {digest})", data)
    chunks.append("\n## Review request\n\nIdentify your independent role and reviewed "
                  "source; distinguish Static from Ran. Return stable finding IDs, "
                  "locations, reasons, missing primary evidence, and a scoped verdict. "
                  "An author or design adviser cannot supply either required independent review of their own solution. "
                  "Do not infer acceptance from this packet's existence.\n")
    result = "".join(chunks)
    if len(result.encode()) > args.max_bytes:
        raise PacketError("Packet exceeds max-bytes")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    parser.add_argument("--base", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--path", action="append", required=True,
                        help="Literal diff/source file, repeat as needed")
    parser.add_argument("--context", action="append", default=[],
                        help="Additional committed instruction/authority file")
    parser.add_argument("--attach", action="append", default=[],
                        help="Explicit local UTF-8 evidence file (review for secrets)")
    parser.add_argument("--max-bytes", type=int, default=1048576)
    parser.add_argument("--output", type=Path, help="New output file; never overwrite")
    args = parser.parse_args()
    try:
        result = make_packet(args)
        if args.output:
            with args.output.open("x", encoding="utf-8") as stream:
                stream.write(result)
        else:
            sys.stdout.write(result)
    except (PacketError, OSError) as exc:
        parser.exit(1, f"review-packet: {exc}\n")


if __name__ == "__main__":
    main()
