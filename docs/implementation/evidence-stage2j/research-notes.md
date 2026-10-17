# Stage 2J read-only research notes

2026-10-17. Exact API/issue/version fields and capture timestamps are in review-00-metadata.json. Current registry newest/stable equals the five already-pinned components; no newer released candidate identified. Initial sandbox DNS failures are retained. Separately permitted network reads succeeded; no Windows elevation, settings change, installation, dependency resolution or runtime test occurred.

Official pages consulted:

- [iced 0.14.0 release](https://github.com/iced-rs/iced/releases/tag/0.14.0) and [Windows adapter 0.35.1 release](https://github.com/AccessKit/accesskit/releases/tag/accesskit_windows-v0.35.1): released provenance, no inference of a coherent snapshot from notes.
- [Issue 552](https://github.com/iced-rs/iced/issues/552), [PR 3111](https://github.com/iced-rs/iced/pull/3111), [PR 1849](https://github.com/iced-rs/iced/pull/1849), [PR 3281](https://github.com/iced-rs/iced/pull/3281): development status only. PR 1849 web opening failed, but official GitHub API succeeded. Drafts remain distinct from stable published code; no branch was installed or exhaustively source-audited as a candidate.
- [List requirements](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supportlistcontroltype): selectable/scrollable list contract and associated-label relationship. These obligations motivate distinct list qualification.
- [ListItem requirements](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supportlistitemcontroltype): selection, optional invocation and scroll-into-view are different patterns. No list runtime was exercised.

Authoritative source findings rely on exact cached published bytes, not draft PR claims. Prior eleven source hashes match F/I. Additional exact consumer node/provider node/changelog members match the current cached published archives. Current five archive checksums match official registry metadata. The adapter's CHANGELOG starts with older unheaded dependency changes; its opening lines do not establish today's version or any missing API. Exact package metadata/releases/source were used instead.

Pinned Windows source conditionally exposes Selection/SelectionItem and ScrollItem; set_selected can emit Click. Mapping such a row action to launching would be unsafe. No container ScrollPattern implementation was found in all 12 Rust source files searched. No new list bridge, control, selection handler, provider or regression test was implemented; these are requirements for a separately approved experiment, not tests already passed.

Source URLs are immutable corresponding revision references; they identify provenance even when a remote page is not opened. Package VCS data uses accesskit_consumer as the consumer repository path; no alternative guessed path is relied upon. No copied historical runtime logs or unperformed manual tests are presented as new evidence.
