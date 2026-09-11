"""Behavioral tests using disposable Git repositories and explicit evidence."""

import argparse
from pathlib import Path
import subprocess
import tempfile
import unittest

import review_packet as packet


class ReviewPacketTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "-q")
        self.git("config", "user.name", "Packet test")
        self.git("config", "user.email", "packet@example.invalid")
        (self.root / "a file.md").write_text("old\n")
        (self.root / "deleted.txt").write_text("removed\n")
        self.commit()
        self.base = self.git("rev-parse", "HEAD").strip()
        (self.root / "a file.md").write_text("new\n```\nfenced source\n```\n")
        (self.root / "deleted.txt").unlink()
        (self.root / "omitted.txt").write_text("not selected body\n")
        self.commit()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.root), *args], text=True)

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "fixture")

    def args(self, **overrides):
        values = dict(root=str(self.root), base=self.base, head="HEAD",
                      path=["a file.md", "deleted.txt"], context=[], attach=[],
                      max_bytes=100000)
        values.update(overrides)
        return argparse.Namespace(**values)

    def test_immutable_scope_deletion_and_fences(self):
        (self.root / "a file.md").write_text("dirty bytes must not leak\n")
        result = packet.make_packet(self.args())
        self.assertIn(self.git("rev-parse", "HEAD").strip(), result)
        self.assertIn("-old", result)
        self.assertIn("+new", result)
        self.assertIn("Deleted at reviewed commit", result)
        self.assertIn("omitted.txt", result)
        self.assertNotIn("not selected body", result)
        self.assertNotIn("dirty bytes must not leak", result)
        self.assertIn("````", result)

    def test_missing_traversal_glob_and_invalid_revision_fail(self):
        for path in ("missing", "../a file.md", "/etc/passwd", "*.md", ".git/config"):
            with self.subTest(path=path), self.assertRaises(packet.PacketError):
                packet.make_packet(self.args(path=[path]))
        with self.assertRaises(packet.PacketError):
            packet.make_packet(self.args(head="--help"))

    def test_attachment_literal_and_hash(self):
        evidence = self.root / "$(touch bad).txt"
        evidence.write_text("Ran: supplied log only\n")
        result = packet.make_packet(self.args(attach=[str(evidence)]))
        self.assertIn("SHA256", result)
        self.assertIn("Ran: supplied log only", result)
        self.assertFalse((self.root / "bad").exists())

    def test_binary_symlink_directory_and_size_fail(self):
        binary = self.root / "binary"
        binary.write_bytes(b"a\0b")
        link = self.root / "link"
        link.symlink_to("a file.md")
        self.commit()
        for path in ("binary", "link"):
            with self.subTest(path=path), self.assertRaises(packet.PacketError):
                packet.make_packet(self.args(path=[path]))
        for path in (binary, link, self.root):
            with self.subTest(attach=path), self.assertRaises(packet.PacketError):
                packet.make_packet(self.args(attach=[str(path)]))
        with self.assertRaises(packet.PacketError):
            packet.make_packet(self.args(max_bytes=100))

    def test_cli_never_overwrites_output_or_writes_failed_packet(self):
        output = self.root / "packet.md"
        command = [str(Path(__import__('sys').executable)), str(Path(packet.__file__).resolve()),
                   "--root", str(self.root), "--base", self.base, "--head", "HEAD",
                   "--path", "a file.md", "--output", str(output)]
        self.assertEqual(subprocess.run(command, capture_output=True).returncode, 0)
        saved = output.read_bytes()
        self.assertNotEqual(subprocess.run(command, capture_output=True).returncode, 0)
        self.assertEqual(output.read_bytes(), saved)
        output.unlink()
        self.assertNotEqual(subprocess.run(command + ["--max-bytes", "10"],
                                           capture_output=True).returncode, 0)
        self.assertFalse(output.exists())


if __name__ == "__main__":
    unittest.main()
