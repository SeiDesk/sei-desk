# Stage 2G observational evidence

2026-10-17. [Report](../STAGE-2G-IME-OBSERVATION-REPORT.md). Observational investigation complete; full coherence, actual OS composition and native reorder remain unverified/unavailable. Production gate stays BLOCKED.

- `preservation-before.json`: hashes of baseline d14d4be tracked/untracked input before adding harness files. Historical accessibility and consumer evidence unchanged.
- `build-01/`: retained compiler failure (incorrect public import and deprecated counter method); not a passing build.
- `build-02/`: first successful four-test/strict-Clippy/optimized build.
- `build-final/`: retained nonexistent get_mode API failure; actual supported mode API used subsequently. Directory name is historical, not a success claim.
- `build-04/`: successful source used for run-01; exact commands, four tests, strict compiler/Clippy and binary/rlib/lock hashes.
- `build-05/`: final source adding long-value fixture and payload deduplication; all gates passed again.
- `run-01/`: 2048 bounded JSONL samples plus app commands/lifecycle. Reached observation cap; cited sample IDs precede it. Raw before/after Shell data, deliberate replay, normal cleanup. main.rs.txt and observe.rs.txt are small exact pre-final source snapshots, retained because source/logging changed before run-02.
- `run-02/`: 56 changed-payload samples on final binary, long text/width/Home/reverse-selection/collapsed endpoint and fresh reopen/normal close. No duplicate large prior-run capture.
- `driver-notes.md`: actual synthetic Computer Use action chronology, public-operation/model-fixture distinction, failed type_text delivery and rejected refresh option. Screenshots observed in tool results, not duplicated as historical images.
- `key-observations.json`, `value-transition-samples.json`: extracted raw sample references for reported findings, not generated geometry or mock UIA assertions.
- `runtime-summary.json`: counts, exact process/binary identities, preedit-null observations, mode and stale-replay records and cleanup.
- `source-identities.json`: current harness/tool hashes, baseline pins/lock hashes and run-01 source snapshot hashes. Published source identities referenced in Stage 2F remain unchanged.
- `input-method-configuration.json`: initial read-only query yielded null fields. Inconclusive; not proof that no IME exists.
- `final-review.json`: baseline preservation, source/document/JSON/parser checks and no remaining owned process.

No AccessKit provider, TextPattern, private-field access, independent shaping, synthetic IME event, real OS IME session, physical keyboard or Narrator test was added. Four tests prove harness helper logic only. Every runtime sample comes from stock widget callbacks; caller epochs are not native layout generations, and phase after_child_redraw is not a paint acknowledgement. A positive caller-epoch check does not certify coherence.

Build/run instructions and exact F-key fixture semantics are in [harness README](../../../experiments/iced-ime-observer/README.md). Run.ps1 owns one process with a ten-minute bound; both observed runs exited normally with no child. Rust binaries/debug artifacts are ignored under the new harness target directory. No existing accepted spike or lockfile changed, no commit or push.
