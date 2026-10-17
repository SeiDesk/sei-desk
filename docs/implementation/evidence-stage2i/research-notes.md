# Stage 2I authoritative research notes

2026-10-17. Conclusions are scoped to exact pinned artifacts and the recorded official registry snapshot. No upstream message or implementation was made.

Official sources consulted through the web tool:

- [iced releases](https://github.com/iced-rs/iced/releases): accessible list labels 0.14.0 latest. Release-note input-method support is not a coherent text-accessibility API promise.
- [AccessKit releases](https://github.com/AccessKit/accesskit/releases): accessible release list consulted; exact version conclusions use the fresh registry responses and cached source.
- [Edit control requirements](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-supporteditcontroltype): Text and string Value obligations, associated static label relationship; informs existing gates.
- [Text/TextRange contract](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-implementingtextandtextrange): insertion point may be a degenerate selection; visible ranges and hit testing are separate methods; ITextProvider2 adds caret operations. No general GetCaretRange support inferred from GetSelection.
- [Standard-control support](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-controlsupport): Edit/RichEdit entries and version caveats. Documentation motivates a native-control feasibility hypothesis; does not prove a mixed iced host or all specific patterns pass.
- [Client-side proxies](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-clientsideprovider): standard-control proxies exist; client-specific proxy configuration would not qualify all accessibility clients.
- [Server-side providers](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-serversideprovider): HWND/fragment navigation, host interaction and lifecycle obligations; mixed trees need actual qualification.
- [Native edit controls](https://learn.microsoft.com/en-us/windows/win32/controls/about-edit-controls): OS edit boundary reference, not a new code or test result.

Web opening crates.io API pages and an attempted clients-and-servers Microsoft URL failed. Successful supported Microsoft pages above were used instead. Local read-only requests initially failed DNS; a separately permitted read-only official registry request succeeded for iced, iced_widget, accesskit, accesskit_windows and accesskit_consumer. Both attempts are recorded without overwriting failures. Network sandbox permission is not Windows RunAs/elevation and did not change system settings.

Exact API URLs and UTC capture timestamps are in registry-recheck.json. For each package, current newest/stable equals the version already pinned and its selected current release is not yanked. Existing cached crate archive hashes match the returned checksums. No latest-version API support was inferred without inspecting its matching source; no new version was fetched, resolved, compiled or run.

The source evidence is local exact published package data, not remote commit pages assumed readable. Immutable source URLs in the decision identify corresponding upstream locations; source findings use actual cached bytes and F's historical archive-to-source verification. All 11 rechecked file hashes match F. Public State exposes cursor; private retained paragraphs/composition and actual IME selection-minimum computation remain unchanged. Provider property dispatch and GetVisibleRanges implementation still have the documented limits.

Exploratory reads used nonexistent crates/sei-desktop/Cargo.toml and a singular pattern-observation/observation.json path. The actual workspace member crates/sei-desk/Cargo.toml and the discovered plural observations.json were subsequently read. A stray irrelevant nonexistent-file read also returned no content. These reads made no changes. No source/build test failed in this research stage because none was run. No environment/read failure is relabeled as runtime acceptance.
