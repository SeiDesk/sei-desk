# Stage 2L — Bounded upstream provider capability review

2026-10-17. **BLOCKERS REMAIN — no stable published path demonstrated for one or both provider requirements.** Both reviewed blockers remain: UIA LabeledBy mapping and container ScrollPattern. The Stage 2K experiment remains **NO-GO**; production accessibility remains **BLOCKED**. This is source/release research, not native runtime acceptance or adoption approval.

## Scope and verified baseline

Read [K owner decisions](STAGE-2K-OWNER-DECISIONS.md), [qualification plan](STAGE-2K-NONEDITABLE-QUALIFICATION-PLAN.md)/[evidence](evidence-stage2k/README.md), [J policy](STAGE-2J-UPSTREAM-REVIEW-POLICY.md)/[baseline](evidence-stage2j/baseline.json), and [I requirements](STAGE-2I-ACCESSIBILITY-ARCHITECTURE-DECISION.md). Actual HEAD is `d14d4bede2096d4bea2561de22977d181376b640`, branch main. At entry there were no tracked changes; existing untracked G–K reports/evidence and the G observer were preserved. [Current baseline](evidence-stage2l/baseline.json) records status, pins and root/spike manifest/lock hashes collected before this research.

Only the two provider blockers and their semantic/consumer dependencies were investigated. No new iced integration, editor, focus architecture, product design or general accessibility qualification was attempted. O01–O07 remain unresolved exactly as [K records](evidence-stage2k/owner-decision-record.json); no role, approval, budget or support matrix was filled in.

## Fresh official release and source facts

| Cooperating component | Exact pinned / current newest stable | Published VCS revision |
| --- | --- | --- |
| accesskit | 0.25.1 / 0.25.1 | `ce8164ba92995cfa86005b6259115e08c8244253`, path accesskit |
| accesskit_consumer | 0.39.1 / 0.39.1 | Same revision, path accesskit_consumer |
| accesskit_windows | 0.35.1 / 0.35.1 | Same revision, path adapters/windows |

Fresh official [core registry](https://crates.io/api/v1/crates/accesskit), [consumer registry](https://crates.io/api/v1/crates/accesskit_consumer) and [Windows registry](https://crates.io/api/v1/crates/accesskit_windows) requests succeeded. For all three, newest and max-stable equal the pins and selected current versions are not yanked. No newer stable candidate was identified in this observed snapshot. The [official 0.35.1 release](https://github.com/AccessKit/accesskit/releases/tag/accesskit_windows-v0.35.1) is neither draft nor prerelease; its released changelog describes MSRV/dependency updates, not these missing mappings. Exact UTC API retrieval times, URLs, selected metadata and response/stored-file hashes are in [retrieval-ledger.json](evidence-stage2l/retrieval-ledger.json).

All three existing cached `.crate` archive SHA256 values match fresh official registry checksums. **23/23** inspected members match their archive bytes, including semantic lib.rs, consumer node/tree/iterators, all **12** Windows Rust source files, manifests/VCS metadata and provider changelog. Archives were read in memory; nothing was downloaded into or installed/patched in the dependency cache. [Archive verification](evidence-stage2l/archive-verification.json), [published identities](evidence-stage2l/published-identities.json) and [source inventory](evidence-stage2l/source-inventory.json) record exact paths, immutable source URLs, revisions, file/member hashes and line counts.

The current baseline remains iced/core 0.14.0, iced_widget 0.14.2, iced_winit 0.14.1 and winit 0.30.13; those packages were not checked for new releases because this review did not find a provider candidate requiring them. Provider manifest requirements are accesskit 0.25.1 and consumer 0.39.1; the existing lock resolves those exact versions and Windows bindings 0.62.2 under the provider's 0.62.0 requirement. These are existing cooperating versions, not newly resolved compatibility results.

The three inspected AccessKit manifests declare MIT OR Apache-2.0 and Rust 1.87; existing workspace/spike minimum is 1.88. Prior Chromium-derived notices remain applicable; no full license/security audit or minimum-compiler build was run here. No plausible stable candidate exists, so `compatibility-audit.json` is not created and no new transitive/Windows compatibility qualification is claimed. A future candidate still needs the complete K/J approval and compatibility audit before any isolated dependency change.

## LabeledBy: semantic relation stops at Windows property mapping

Exact source paths/ranges/hashes are expanded in [labeledby-audit.json](evidence-stage2l/labeledby-audit.json) and [line-numbered excerpts](evidence-stage2l/source-excerpts.txt). References below are to the verified published revision, not a development branch.

| Step | Published source inspected | Finding and limit |
| --- | --- | --- |
| Authored visible label identity | accesskit/src/lib.rs:701–717, 2053–2059, 3208–3232 | NodeId/TreeUpdate represent nodes; set_labelled_by/push/clear retain explicit target IDs. A catalog author must include the actual visible label node in the tree and reference its stable identity. Semantic storage exists. |
| Consumer preservation/resolution | accesskit_consumer/src/node.rs:719–741, 775–797; iterators.rs:537–583 | Explicit IDs resolve in the same tree. Consumer label text may derive from referenced labels. An accessible Name string is not a UIA element relationship. ListBox does not gain a native HWND label association automatically. |
| Updates/removal | accesskit_consumer/src/tree.rs:270–293, 353–381 | Changed Node data is replaced and unreachable nodes removed. Explicit label iterator uses unwrap after target lookup: valid target references are a precondition. No automatic repair of dangling label IDs was established in the examined update/removal path. Update/clear the relation when removing its target; potential failure is source inference, not an executed test. |
| Windows property dispatch | adapters/windows/src/node.rs:1017–1072, 1183–1242, 1306–1330 | Full property registry lacks UIA_LabeledByPropertyId. Generated property dispatch defaults to an empty Variant; GetPropertyValue fallback handles other root/toolkit/ControllerFor cases, not this relationship. Root host-provider fallback does not map a virtual catalog list to its authored label. |
| Generic node lifetime | context.rs:80–97; adapter.rs:347–352; node.rs:847–895; util.rs:360–362, 426–430 | Platform nodes are cached by ID; removal drops the cache entry and retained objects resolve the current tree/context, reporting element-unavailable when missing. This generic machinery does not produce a missing LabeledBy target/event, prevent author ID reuse, or qualify label removal/replacement races. |

The [published Windows property list](https://github.com/AccessKit/accesskit/blob/ce8164ba92995cfa86005b6259115e08c8244253/adapters/windows/src/node.rs#L1306-L1330) and its fallback were reviewed directly, beyond a token search. NodeWrapper.name at 404–414 routes consumer text to Name. The property path never constructs the required provider reference to the visible associated label. No attribute alone repairs that missing Windows dispatch.

[PR 78](https://github.com/AccessKit/accesskit/pull/78) is merged naming work, according to fresh official metadata. Its title/merge is not proof of UIA LabeledBy; actual published consumer source establishes name derivation, while the stable Windows mapping remains absent. No branch ancestry or release inclusion was inferred just from that PR.

Historical [E observation](evidence-stage2e/pattern-observation/observations.json) found LabeledBy=false on the spike's Edit input. It is not a fresh catalog-list query. This review's list conclusion is source-level because the common provider property dispatch is unmapped. Correct target identity, changed/removed/replaced label, retained element behavior and relationship events for a future supported implementation still require owner-approved native Q01/lifecycle qualification.

**Disposition:** stable semantic/consumer support is present; a complete stable semantic-to-Windows LabeledBy path is missing. Keep the visible associated label. Do not rename/remove it, substitute ControllerFor/Name or introduce a native duplicate/override.

## Container ScrollPattern: offsets and ScrollItem do not implement the container

[scrollpattern-audit.json](evidence-stage2l/scrollpattern-audit.json) separates source foundations from every missing provider member. [provider-search-inventory.json](evidence-stage2l/provider-search-inventory.json) retains all 12 searched file identities, matches and all ten registered pattern families. Search corroborates manual review of the COM interface declaration, generated dispatch/default, complete pattern registry and ScrollItem body; absence is not inferred from grep alone.

- **Semantic/consumer foundations:** accesskit/src/lib.rs:315–338 and 2207–2220 contain directional/point/offset actions and optional current/min/max X/Y scroll values. Consumer node.rs:510–532 reads those fields. They are not authored UIA percentages/view sizes, and no Windows conversion or unavailable-axis policy is inferred from their f64 types.
- **Role/container requirements:** consumer node.rs:938–957 includes ListBox among selectable containers; Windows maps ListBox/ListBoxOption to List/ListItem. ScrollView maps to Pane and ScrollBar to ScrollBar. Roles and selection predicates do not advertise ScrollPattern. A genuine catalog viewport still needs the complete container interface plus item behavior.
- **Provider break:** node.rs:786–810 declares the implemented COM interfaces and includes IScrollItemProvider, not IScrollProvider. The pattern macro at 1245–1270 dispatches only registered families and otherwise returns an empty error. The registry beginning at 1332 contains ten families, including ScrollItem and Selection, without UIA_ScrollPatternId. No other implementation was found in the complete twelve-file Windows source inventory.
- **Actual item path:** node.rs:644–646 conditionally supports ScrollItem when ScrollIntoView is available; 1370–1381 dispatches Action::ScrollIntoView. This supplies neither container methods nor scroll percentages/view sizes. Whether a future catalog's item actually scrolls is also a native test, not a source pass.

| Container requirement | Stable 0.35.1 finding |
| --- | --- |
| HorizontallyScrollable / VerticallyScrollable | No container property implementation or axis availability contract |
| HorizontalScrollPercent / VerticalScrollPercent | No offset-to-percent mapping or proved unavailable-axis/sentinel behavior |
| HorizontalViewSize / VerticalViewSize | No viewport/content percentage behavior |
| Scroll(horizontalAmount, verticalAmount) | No method/increment/unsupported-axis/error mapping |
| SetScrollPercent(horizontalPercent, verticalPercent) | No method, checked percentage conversion, no-scroll/invalid/out-of-range/zero-extent policy |
| Container scroll property-change events | No container registry mapping from which to claim corresponding native events |
| ScrollItem.ScrollIntoView | Released conditional item path only; not equivalent to container scrolling |

Microsoft's [IScrollProvider contract](https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcore/nn-uiautomationcore-iscrollprovider) lists those container properties/methods. [Scroll](https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcore/nf-uiautomationcore-iscrollprovider-scroll) takes independent axis amounts; [SetScrollPercent](https://learn.microsoft.com/en-us/windows/win32/api/uiautomationcore/nf-uiautomationcore-iscrollprovider-setscrollpercent) describes percentages and the no-scroll sentinel. These SDK APIs do not implement AccessKit behavior. Two specific Microsoft getter-page fetches failed; their detailed sentinel/view-size semantics are not claimed verified from those pages. All axis/percentage/view-size/error behavior remains absent from the stable provider and requires exact source plus native tests in any future candidate. No conversion algorithm or workaround was implemented.

[PR 816](https://github.com/AccessKit/accesskit/pull/816) is directly relevant but **open, not draft, merged=false**, head `29f2ec503e98f1afcbbd1a26d11a1986b5198d5e`, per fresh official API metadata. Its page fetch failed and was retained; the distinct API endpoint succeeded on its first request. No PR code was adopted, built or audited as a stable implementation. It is an open proposal, not merged-but-unreleased, published support or runtime proof. No directly relevant new merged-unreleased fix was established by this bounded search; this is not an exhaustive claim about all upstream work.

**Disposition:** no stable container ScrollPattern path demonstrated. Do not substitute ScrollItem, a scrollbar RangeValue pattern, a renamed role or silent pagination for K's required scrollable list.

## Retrieval bounds, evidence classes and next permissible action

One first attempt per API/page URL: three registry requests, one pinned-release record, two searches limited to ten items/one page each, and first metadata requests for the two directly relevant resulting PRs. Six official web page requests provide proposal/SDK context. No failed URL was retried, polled or scheduled. J's single later-day retry within seven days remains merely permitted, not scheduled or performed. Direct API requests used read-only network permission initially because restricted-DNS failure was already established historically; no Windows RunAs or configuration change occurred.

The ledger retains **three failed web fetches**, independently of successful official API/source evidence. API responses have precise retrieval UTC timestamps and hashes of received UTF-8 text; selected stored fields have separate file hashes. Full raw API/web responses are not retained. Web tooling did not expose exact HTTP retrieval timestamps or original response text hashes; recording time and an explicit UTC session window are retained instead, never fabricated precise timestamps. Failures of ancillary pages do not conceal a stable-source uncertainty: the current registry/checksum/provenance and both missing dispatch paths were verified independently.

Evidence classes: stable published source verifies foundations and missing mappings; merged PR metadata is history/context; open PR metadata is a proposal; lifecycle implications are labelled source inference; historical UIA belongs to its original run; **no runtime behavior was tested in L**. No new stable adoption candidate or source-only workaround was elevated to acceptance.

Next permissible action: owner records O01–O07 and may request another bounded read-only check under J, particularly if PR 816 later merges **and** appears in an exact stable artifact. A future candidate must independently satisfy both label and container contracts, compatible versions/license/MSRV/transitives/Windows review, then obtain explicit isolated experiment approval before any dependency change or native Q01/Q17 testing. Current experiment remains NO-GO; no label removal, pagination decision, provider implementation, architecture switch or production work is authorized.

## Validation and changed files

Added only this report and new files under `docs/implementation/evidence-stage2l/`, enumerated with hashes in its [README](evidence-stage2l/README.md)/[checksums](evidence-stage2l/checksums.json). [Final checks](evidence-stage2l/final-checks.json) record exact preservation, source/archive/ledger/reference/JSON/link/whitespace/checksum and `git diff --check` outcomes. Existing source, pins, manifests, lockfiles, cached sources, designs, owner records and historical evidence remain unchanged. No build, GUI, UIA, IME, physical keyboard, Narrator, settings/scheduling change, external communication, commit or push.

**BLOCKERS REMAIN — no stable published path demonstrated for one or both provider requirements.**
