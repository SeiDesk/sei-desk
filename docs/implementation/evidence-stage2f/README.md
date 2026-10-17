# Stage 2F research and documentation evidence

2026-10-15. [Decision brief](../STAGE-2F-UPSTREAM-COMPATIBILITY-DECISION.md). Headless acceptance unchanged; production accessibility remains BLOCKED. No source, dependency, lockfile, design or historical evidence changes.

- `preservation-before.json`: all pre-existing tracked/untracked input file hashes.
- `source-hashes.json`, `source-review.txt`: exact pinned source inspected, including the newly considered Shell input-method output. These are source-level findings, not new API/runtime tests.
- `published-vcs.json`: released package VCS metadata and repository-relative paths from cached artifacts.
- `registry-recheck.json`: successful direct read-only official crates.io queries outside restricted networking. Latest stable AND newest versions equal current pins for iced, iced_widget, accesskit and accesskit_windows. Includes exact version numbers, timestamps, yanked fields and source URLs. No package installation or dependency resolution.
- `release-research.txt`: chronological release-page observations and initial fetch failures, followed by successful registry recheck. Main-branch/milestone data not treated as release/API support proof.
- `published-file-verification.json`: initial optional checksum-metadata check unavailable (no .cargo-checksum.json); not a failed source hash comparison.
- `archive-source-verification.json`: all 11 inspected files match bytes in the existing published crate archives; all archive hashes match Cargo.lock. Archives read into memory only, no source extraction/install.
- `final-review.json`: preservation, local documentation links, whitespace and git diff --check.

Primary release sources consulted: https://github.com/iced-rs/iced/releases , https://github.com/iced-rs/iced/tags , https://github.com/AccessKit/accesskit/releases . Exact source links and versions are in the brief; full local artifact hashes provide evidence where web source fetches failed.

Only source/release research and documentation consistency checks were performed. No cargo/build/test execution, GUI, UIA, keyboard, Narrator, prototype, fork, dependency upgrade or external upstream message. Existing Stage 2D/2E captures are historical and never labeled new. Nothing committed or pushed.
