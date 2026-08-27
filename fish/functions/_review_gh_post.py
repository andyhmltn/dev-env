#!/usr/bin/env python3
"""
Parse tuicr's --stdout markdown export and post it as a real GitHub PR review
with inline line comments via `gh api`.

Called from review-gh.fish. See that function for the full workflow.

Tuicr export format (v0.12):

    1. **[ISSUE]** `src/auth.rs:42` - Magic number should be a named constant
    2. **[SUGGESTION]** `src/auth.rs` - Consider adding unit tests
    3. **[NOTE]** `src/auth.rs:50-55` - This block could be refactored

- `path` alone        -> file-level comment
- `path:N`            -> single-line inline at N (RIGHT side)
- `path:N-M`          -> multi-line inline N..M (RIGHT side)

Entries on lines that are NOT part of the PR diff fall back to file-level
comments (GitHub rejects inline comments on non-diff lines with HTTP 422).
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass, field


ENTRY_RE = re.compile(
    r"^\d+\.\s+\*\*\[(?P<type>[A-Z]+)\]\*\*\s+`(?P<ref>[^`]+)`\s*-\s*(?P<body>.+?)(?=^\d+\.\s+\*\*\[|\Z)",
    re.MULTILINE | re.DOTALL,
)
REF_RE = re.compile(r"^(?P<path>.+?)(?::(?P<start>\d+)(?:-(?P<end>\d+))?)?$")
HUNK_RE = re.compile(r"^@@ -\d+(?:,\d+)? \+(?P<start>\d+)(?:,\d+)? @@")


@dataclass
class Comment:
    type: str
    path: str
    line_start: int | None
    line_end: int | None
    body: str


@dataclass
class Review:
    intro: str
    comments: list[Comment] = field(default_factory=list)


def run(*cmd: str, input_text: str | None = None) -> str:
    result = subprocess.run(
        cmd,
        input=input_text,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        sys.stderr.write(f"command failed: {' '.join(cmd)}\n{result.stderr}\n")
        sys.exit(result.returncode)
    return result.stdout


def parse_tuicr(text: str) -> Review:
    matches = list(ENTRY_RE.finditer(text))

    intro = ""
    if matches:
        first_match_start = matches[0].start()
        intro = text[:first_match_start].strip()

    comments: list[Comment] = []
    for m in matches:
        type_ = m.group("type")
        ref = m.group("ref").strip()
        body = m.group("body").strip()

        ref_match = REF_RE.match(ref)
        if not ref_match:
            sys.stderr.write(f"warning: unparseable ref {ref!r}, skipping\n")
            continue

        path = ref_match.group("path")
        start = ref_match.group("start")
        end = ref_match.group("end")

        comments.append(
            Comment(
                type=type_,
                path=path,
                line_start=int(start) if start else None,
                line_end=int(end) if end else None,
                body=body,
            )
        )

    return Review(intro=intro, comments=comments)


def commentable_ranges(owner: str, repo: str, pr_number: int) -> dict[str, set[int]]:
    raw = run(
        "gh", "api", "--paginate",
        f"repos/{owner}/{repo}/pulls/{pr_number}/files",
    )
    files = json.loads(raw)

    by_path: dict[str, set[int]] = {}
    for f in files:
        path = f["filename"]
        patch = f.get("patch") or ""
        lines: set[int] = set()
        right = None
        for line in patch.splitlines():
            hunk = HUNK_RE.match(line)
            if hunk:
                right = int(hunk.group("start"))
                continue
            if right is None:
                continue
            if line.startswith("+"):
                lines.add(right)
                right += 1
            elif line.startswith(" "):
                lines.add(right)
                right += 1
            elif line.startswith("-") or line.startswith("\\"):
                pass
            else:
                right += 1
        by_path[path] = lines
    return by_path


def build_payload(
    review: Review,
    ranges: dict[str, set[int]],
    head_sha: str,
    event: str,
    extra_body: str,
) -> tuple[dict, int, int, int]:
    api_comments: list[dict] = []
    inline = 0
    fallback = 0
    skipped = 0

    for c in review.comments:
        body = f"**[{c.type}]** {c.body}"
        path_lines = ranges.get(c.path)

        if c.line_start is None:
            api_comments.append({"path": c.path, "body": body, "subject_type": "file"})
            fallback += 1
            continue

        if path_lines is None:
            sys.stderr.write(
                f"warning: {c.path} not in PR diff, dropping comment on line {c.line_start}\n"
            )
            skipped += 1
            continue

        end = c.line_end or c.line_start
        if c.line_start not in path_lines or end not in path_lines:
            api_comments.append({"path": c.path, "body": body, "subject_type": "file"})
            fallback += 1
            continue

        comment: dict = {"path": c.path, "side": "RIGHT", "line": end, "body": body}
        if c.line_end and c.line_end != c.line_start:
            comment["start_line"] = c.line_start
            comment["start_side"] = "RIGHT"
        api_comments.append(comment)
        inline += 1

    payload = {
        "commit_id": head_sha,
        "event": event,
        "body": extra_body,
        "comments": api_comments,
    }
    return payload, inline, fallback, skipped


def post_review(owner: str, repo: str, pr_number: int, payload: dict) -> dict:
    out = run(
        "gh", "api",
        f"repos/{owner}/{repo}/pulls/{pr_number}/reviews",
        "--method", "POST",
        "--input", "-",
        input_text=json.dumps(payload),
    )
    return json.loads(out)


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--input", required=True, help="Path to tuicr --stdout export")
    p.add_argument("--owner", required=True)
    p.add_argument("--repo", required=True)
    p.add_argument("--pr", required=True, type=int)
    p.add_argument("--head-sha", required=True)
    p.add_argument(
        "--event",
        default="COMMENT",
        choices=["COMMENT", "REQUEST_CHANGES", "APPROVE"],
    )
    p.add_argument("--body", default="", help="Extra text prepended to review body")
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args()

    with open(args.input) as f:
        text = f.read()

    review = parse_tuicr(text)
    if not review.comments and not review.intro:
        sys.stderr.write("nothing to post (empty export)\n")
        return 1

    ranges = commentable_ranges(args.owner, args.repo, args.pr)
    payload, inline, fallback, skipped = build_payload(
        review, ranges, args.head_sha, args.event, args.body,
    )

    sys.stderr.write(
        f"Parsed {len(review.comments)} comments: "
        f"{inline} inline, {fallback} file-level, {skipped} dropped (file not in PR)\n"
    )

    if args.dry_run:
        json.dump(payload, sys.stdout, indent=2)
        sys.stdout.write("\n")
        return 0

    result = post_review(args.owner, args.repo, args.pr, payload)
    sys.stderr.write(f"posted: {result.get('html_url', '(no url)')}\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
