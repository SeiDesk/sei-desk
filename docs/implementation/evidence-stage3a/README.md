# Stage 3A execution evidence

15 October 2026. New Windows runtime and quality evidence; no earlier stage evidence was overwritten. Reports: [catalog](../STAGE-3A-APPLICATION-CATALOG.md), [launch service](../STAGE-3A-LAUNCH-SERVICE.md), [test matrix](../STAGE-3A-TEST-MATRIX.md).

- [verified-run/quality-gates.txt](verified-run/quality-gates.txt): final UTF-8 command/exit log for workspace fmt, check, ordinary tests, strict Clippy and release builds. Ordinary workspace tests: 25 passed, two launch-fixture tests deliberately ignored at this step.
- [verified-run/windows-tests.txt](verified-run/windows-tests.txt): separate opt-in Windows run; 15 library tests plus one integration test passed, including actual COM and fixture process operations. Injected policy/error tests remain distinct from those native tests.
- [verified-run/catalog-stdout.txt](verified-run/catalog-stdout.txt): count-only actual Start Menu discovery, timing, equality across two explicit refreshes, exclusion-operation counts. No discovered names/targets/arguments persisted.
- [verified-run/catalog-stderr.txt](verified-run/catalog-stderr.txt): actual probe stderr (empty on success).
- [verified-run/resource-baseline.json](verified-run/resource-baseline.json): actual Windows build, toolchain, release binary hashes, observation duration, counters, normal exit and no remaining owned child.
- [initial-tests.txt](initial-tests.txt): first failing display-sort expectation; corrected to the actual required alphabetical ordering.
- [initial-windows-tests.txt](initial-windows-tests.txt): first opt-in run's invalid-PE assertion assumed only error 193; Windows returned 216. Both map to Invalid; corrected assertion then passed.
- [windows-tests.txt](windows-tests.txt), [release-examples.txt](release-examples.txt): intermediate successful fixture/build results, not replacements for the verified run.
- [final-run/resource-baseline.json](final-run/resource-baseline.json), [final-run/catalog-stdout.txt](final-run/catalog-stdout.txt): earlier full-run sample. Its quality log mixed Windows PowerShell Tee-Object UTF-16 with Add-Content output; retain as raw history, use verified-run's consistent UTF-8 log for final quality results. No benchmark result was fabricated from that logging defect.
- [preservation-before.json](preservation-before.json), [preservation-after.json](preservation-after.json): pre-existing file hash comparison; only root workspace membership and lockfile intentionally change. All prior designs, code, reports and evidence remain unchanged.
- [final-review.txt](final-review.txt): final local link/whitespace, dependency and Git scope checks.
- [dependency-tree.txt](dependency-tree.txt): exact sei-apps dependency graph; no iced or AccessKit dependency.

The release probe's two samples are explicit developer measurements, not a service timer or production background process. No arbitrary discovered application, UAC elevation, shell interpreter, Explorer/taskbar modification, global key hook, or production UI was used in these tests. Actual ACL-denial behavior, supported-build coverage, hostile-file races and long-duration leaks remain unqualified.
