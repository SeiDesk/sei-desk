# Stage 3B worker-lifecycle repair evidence

Fresh Windows executions, 2026-10-15. Final disposition: [PASS for the headless consumer](../STAGE-3B-ACCEPTANCE-REPORT.md), not production UI/accessibility approval. No commit or push.

- `preservation-before.json`: SHA256 baseline of tracked and untracked input work.
- `acceptance-report-before.md`: verbatim archival copy of the independent FAIL report. Its relative links refer to its original location; historical failure logs remain in `../evidence-stage3b-acceptance/` unchanged.
- `acceptance-regressions.txt`: first nine focused repair tests, exit 0.
- `windows-run/`: initial sandboxed run. All workspace gates passed; a pre-existing native shortcut fixture rename failed with Access denied. Preserved as a failed run.
- `windows-run-unrestricted/`: same source/fixture outside the filesystem sandbox, non-admin token. All 48 then-current enabled service tests passed. Intermediate source, before the final two regression additions.
- `acceptance-regressions-final.txt`: all eleven focused tests, exit 0; includes original failure test now reporting completion present. Panic-hook output is expected and retained.
- `windows-final/`: final functional-source run. Full quality gate command/exit logs, 56 ordinary workspace tests (5 opt-in ignored there), then 50 enabled service tests (47 library + 3 actual Windows integration). Native discovery probes and resource metadata include executable hashes and non-admin token status. Overall runner exit recorded in `windows-final-exit.txt`.
- `release-consumer-tests.txt`: all 31 consumer tests passed in release mode, exit 0.
- `post-documentation-gates.txt`: final formatting/all-target check/strict Clippy/diff check after documentation and a comment-only clarification in consumer.rs; records current source hashes in `tested-tree.json`.
- `tested-tree.json`: HEAD, branch, working-tree status and hashes of Rust sources/manifests and test scripts. Tested code is uncommitted; HEAD alone is not its identity.
- `final-review.json`: baseline comparison, changed-file allowlist, documentation link/whitespace checks and absence of named owned probe/fixture processes.

The final comment-only clarification describes the already tested fallback join path; it changes no executable behavior. Both final test output and current source hashes are retained so this distinction is visible.

No previous source/evidence was erased. New nested-panic/exit-path tests use controlled Rust services or worker fixtures, not Windows crash simulation. Windows launch fixture tests use real process creation with injected catalog data; discovery probes separately use actual native enumeration. No physical-keyboard, Narrator, UIA, process-abort or permanently stuck OS-call test is claimed here. Short resource observations include two refreshes and a two-second hold; no pure-idle guarantee or cross-build qualification is inferred.
