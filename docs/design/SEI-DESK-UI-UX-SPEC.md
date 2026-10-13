# Sei-Desk UI/UX design specification

**Status:** proposed design for review, 14 October 2026. **Deliverables:** written specification and eight static vector mockups, with PNG previews. Nothing in this package implements application behavior. All metrics, application entries, dates and device names in mockups are illustrative.

## 1. Source of truth and design interpretation

The primary reference is [Sei-Desk Pastel Desktop UI Showcase.png](../../assets/Sei-Desk%20Pastel%20Desktop%20UI%20Showcase.png), inspected at its native **1536 × 1024** resolution before selecting the design direction. The folder also contains `icon.png`, inspected separately: a glossy cyan S mark with a Windows-like inset. Its luminous treatment conflicts with this brief; do not use its glow, cyan edge or glossy finish as UI styling. The small mountain emblem in the showcase is the provisional mockup mark. Final brand-mark selection remains a review decision; these plates do not replace either source asset.

The showcase combines a blue/lavender alpine lake wallpaper with blush clouds, dark trees, translucent blue-slate floating surfaces, fine pale edges, mint selection/slider accents, compact application icons, and pale text. The launcher sits beneath a distinct top control dock; the taskbar runs almost edge to edge at the bottom. Lower panels share a left navigation rail, compact metric tiles and rounded fields. Typography visually resembles a Windows system sans serif; the actual font cannot be identified from pixels alone.

**Retain:** airy pastel environment, compact density, floating rounded desktop controls, consistent shell geometry, muted mint emphasis, restrained shadows. **Refine:** use light lavender/blue-gray surfaces for larger application windows, dark slate text for contrast, a single selected-row language, consistent icon geometry and accurate numeric/slider relationships. This is a documented adaptation toward the requested soft pastel direction: the source’s large dashboard panels are darker than the proposed ones. Keep the desktop overlays dark enough to remain readable over wallpaper; they must not turn the whole product into a graphite theme.

**Correct reference inconsistencies:** remove Files, Settings and Web search filters from the application launcher; replace horizontal navigation hints with Up/Down; show the value matching the slider thumb; replace the arbitrary neon color picker with four constrained pastel accents; distinguish running from active rather than using glow; use unambiguous disk capacity versus activity labels. Source panel labels, tiny raster artifacts and window chrome are illustration details, not product requirements.

The mockup wallpaper is an original simplified vector interpretation of the reference landscape and its sampled colors. It is a composition aid, not a new wallpaper feature or a proposal to modify the user’s desktop. App monogram tiles are legible design placeholders; production should use installed application icons without recoloring their brands.

## 2. Product structure and release boundary

Sei-Desk is a Windows desktop customization and basic monitoring application. Mode: **Operate**. The primary user wants quick application access, persistent desktop controls and readable resource information without a busy desktop.

| Surface | Purpose | Initial release |
| --- | --- | --- |
| Bottom taskbar | Launch pinned apps; switch running windows; compact status and time | Yes, subject to Windows integration validation |
| Start launcher | Find and launch installed applications from the keyboard | Yes, Windows-key behavior subject to validation |
| Top quick-control dock | Brightness, audio, session/power actions, compact indicators | Yes, individual controls depend on supported hardware/APIs |
| Overview | CPU, memory, disk, network and limited health readings | Yes |
| Applications | Supporting catalog, launch and pin management | Yes, supports launcher/taskbar |
| Settings | Supporting appearance and behavior controls | Yes, bounded to these surfaces |
| Optimization center | Explicit selection, review and recovery concept | Future only; absent from initial navigation |

No window tiling, marketplace, assistant, gaming features, extra system utilities, file search, web search or shell-replacement claims. Initial navigation contains **Overview, Applications, Settings**. Do not carry the reference’s ambiguous “System” page into an extra module.

## 3. Mockup index and reading conventions

| Plate | Design artifact | Coverage |
| --- | --- | --- |
| 01 | [Desktop composition](mockups/01-desktop-composition.svg) | Wallpaper, dock, open searched Start, bottom taskbar |
| 02 | [Taskbar states](mockups/02-taskbar-states.svg) | Default, running, active, hover, keyboard focus |
| 03 | [Start states](mockups/03-start-launcher-states.svg) | Empty query with pins, matches, moved selection, no results |
| 04 | [Quick-control dock](mockups/04-quick-control-dock.svg) | Default dock, brightness, volume/mute, power, confirmation, unavailable |
| 05 | [Monitoring](mockups/05-system-overview.svg) | Four metrics, histories, basic health and data-failure rule |
| 06 | [Applications](mockups/06-applications-catalog.svg) | Search, All/Pinned, alphabetical catalog, selected app and actions |
| 07 | [Settings](mockups/07-appearance-behavior-settings.svg) | Pastel theme, accents, translucency, motion, Windows-key status |
| 08 | [Future optimizer](mockups/08-future-optimization-concept.svg) | Selection → review → progress → failure/recovery |

Plates are presentation boards, not pixel-for-pixel runtime screenshots. Most boards are 1600 × 1000; the launcher state board is 1600 × 1080. Their captions and margins are outside application chrome. Four quick-control variants are presented together for review, although only one popover opens in use. Shell windows and widgets are enlarged on boards for legibility; the following DIP measurements govern the proposed runtime layout. SVGs are self-contained static design assets without scripts or browser-dependent application behavior.

## 4. Color tokens and provenance

Sampling uses the original PNG in sRGB. Coordinates are zero-based pixels; patches use median RGB. Screenshot samples are flattened against the wallpaper and **do not reveal the original material opacity**. Raw colors below are evidence; surface mixes and state colors are proposed refinements, not claims of exact extraction.

| Evidence token | Raw sample | Source location | Use |
| --- | --- | --- | --- |
| Reference lavender | `#CBC2DE` | Median patch x1010–1079, y455–484 | Lavender accent, soft backdrop |
| Reference dusty blue | `#7A9DCD` | Median patch x100–199, y8–39 | Blue accent, landscape |
| Reference cloud pink | `#F3B8C8` | Pixel (1090,64) | Restrained pink accent |
| Reference mist blue | `#9FBAE0` | Pixel (800,460) | Pale blue family |
| Reference slate | `#293649` | Median patch x800–949, y270–339 | Primary text; floating panel base |
| Reference bright mint | `#71F9F0` | Median patch x564–589, y132–141 | Provenance only; desaturate for product |
| Reference pale blue text | `#D6DFEA` | Pixel (150,825) | Evidence for pale neutral family |

| Proposed semantic token | Value | Rationale / role |
| --- | --- | --- |
| Canvas | `#EFF1F7` | Near-white lavender tint derived from lavender + pale neutral |
| Panel / inset | `#E3EAF3` | Pale mist-blue neutral, distinct from canvas |
| Floating base | `#293649` | Source slate, proposed opacity 94–98%; 100% fallback |
| Desktop bar base | `#293649` | Source slate, proposed opacity 90–96%; 100% fallback |
| Primary text | `#293649` | Dark slate on pale surfaces |
| Secondary text | `#53647A` | Derived slate-blue, usable metadata contrast |
| On-dark primary | `#F4F7FC` | Pale neutral derived from source light text |
| On-dark secondary | `#CBD7E6` | Pale blue metadata |
| Muted mint | `#C5E8E3` | Lower-saturation tint of source mint; default selected fill |
| Lavender / blue / pink accents | `#CBC2DE` / `#7A9DCD` / `#F3B8C8` | Evidence colors; small areas only |
| Selection on dark | `#526C7E` | Derived slate-blue; mint left rail and pale text |
| Field on dark | `#40536B` | Derived lighter slate; edges `#7188A4` |
| Decorative border | `#B7C6DA` | Fine separation on light panels; not a focus cue |
| Focus / chart stroke | `#44638A` | Darker source blue for meaningful edges and charts |
| Disabled text | `#53647A` | Keep readable; use disabled shape and explanation rather than low opacity |

Accent choice changes selected backgrounds and small indicators, never text contrast or focus-ring contrast. No neon outlines, glow, black page backgrounds or ornamental panel gradients. Wallpaper may contain natural color transitions. Use text, icons and state labels alongside color for warning, success and error. Pink is an accent, not an automatic error meaning.

Contrast target: at least 4.5:1 for ordinary text, 3:1 for meaningful graphical controls/large text. Check composited surfaces against both light and dark wallpaper patches; opacity is not an accessibility guarantee. Enable opaque surfaces when contrast cannot hold. Meaningful focus/selection borders use the focus token; decorative hairlines are intentionally quieter. High-contrast/forced-color behavior and screen-reader support must be verified with the selected iced accessibility stack before release.

## 5. Typography and geometry

**Proposed typeface:** Segoe UI on Windows, based on the reference’s visual character and platform context. Do not claim the screenshot proves its exact family. Use regular 400 and semibold 600; avoid extra weights. Provide a system sans-serif fallback. Use tabular numeric figures where available; otherwise reserve stable widths so metrics do not move.

| Role | Size / line height in DIP | Weight |
| --- | --- | --- |
| Window heading | 24 / 30 | 600 |
| Section heading | 18 / 24 | 600 |
| Main metric | 24 / 30 | 600 |
| Application result / body | 14 / 20 | 400; selected name 600 |
| Control label | 13 / 18 | 400 or 600 |
| Metadata / keyboard hint | 12 / 16 | 400 |
| Clock | 13 / 16 | 600; date 12 / 16 |

Never shrink below 12 DIP to solve overflow. Ellipsize long app names, expose the full name in a tooltip and accessible label. Labels and keyboard hints must tolerate localization.

Spacing scale: **4, 8, 12, 16, 24, 32 DIP**. Icon-to-label gap 8; row padding 12; card padding 16; window content padding 24; major sections 24 apart. Compact density means disciplined spacing, not tiny input targets. Desktop icon buttons have 40 × 40 DIP hit areas; general controls at least 36 DIP high, preferably 40. Thin sliders have a 32 DIP interactive lane and a 20 DIP or larger thumb hit region.

Radii: fields/rows 8, tiles 10, dock/taskbar/popovers 12, launcher/window 16, chips half-height. Borders: 1 DIP decorative, 2 DIP focus with 2 DIP offset where space permits. Draw straight edges on the physical-pixel grid after scaling; never distort icon strokes to achieve snapping.

Shadow proposals: floating surfaces offset (0,6), blur 20, slate at 13%; main windows offset (0,8), blur 24, slate at 12%; inset controls no shadow. Avoid stacking multiple shadows. Shadow and translucency are optional visual polish; an opaque, unblurred rendering must preserve hierarchy. Mockups illustrate tint/opacity, not verified live backdrop blur.

Icons: 20 DIP controls, 24 DIP application/taskbar glyphs, 32 DIP launcher/catalog app icons. Use a single line-icon family with 1.5–1.75 DIP strokes, round joins, consistent optical size. Use installed app artwork as contained images with preserved proportions. Do not use emoji, glow or full-color illustrations for system actions. Clock/date are passive status in the initial design; no calendar is added.

## 6. Desktop composition, positioning and layering

Target composition at 1920 × 1080 physical pixels / 100% scale: bottom taskbar inset 12 DIP horizontally and 10 from bottom, height 58; top dock centered at y12, width about 760, height 52; launcher centered horizontally, width 580, top edge 80. The reference uses approximately 40% screen width for its dock and 30% for its launcher; treat these as proportional evidence, not a requirement to stretch on ultrawide displays. Keep intrinsic sizes with max-width clamping.

Launcher height follows content, nominal 300–440 DIP with at most five visible result rows and an internal scrollbar for more. Keep a minimum 12 DIP gap below the dock; never cover it. Anchor brightness/volume/power popovers 8 DIP below their owning control; clamp within the monitor with 12 DIP safe margins. Popovers are 280–320 DIP wide. Management window nominal size 1040 × 680, minimum 800 × 560 DIP. Use normal resizable window behavior; never cover the taskbar deliberately.

Layer order within the design: desktop wallpaper → ordinary applications → desktop bars where OS policy permits → active launcher or dock popover → local menu/tooltip → confirmation dialog. This expresses visual intent only. Actual topmost behavior, appbars, exclusive fullscreen, focus restrictions and secure-desktop boundaries require Windows validation. Do not place bars above lock screen, UAC or exclusive fullscreen. Close the active transient surface when opening another; launcher and dock popovers never compete for focus. Tooltips do not receive keyboard focus.

The taskbar and top dock remain at fixed edges in v1. Reserve bottom workspace if supported; separately validate whether top dock requires reserved space or must yield to maximized windows. Never imply overlay positioning alone prevents overlap. Primary-monitor-only is the proposed first-release default; a launcher invoked from a different monitor uses that monitor’s available work area, pending validation. Additional replicated taskbars/docks are not assumed. Explicitly test moved/disconnected displays and negative desktop coordinates.

## 7. Taskbar behavior and states

Start button opens/closes the application launcher. Search button opens the same launcher focused on input; it does not introduce a different search scope. Left section: Start, search, pinned app slots, then unpinned running apps. Right section: compact supported status, an overflow affordance if needed, clock/date. Keep a separator and flexible spacer so launch and status areas remain legible.

| State | Visual / behavior |
| --- | --- |
| Pinned, not running | App icon; no underline |
| Running, inactive | Small centered pale dot below icon |
| Active | Short mint underline plus tinted tile; no glow |
| Hover | Slightly lighter slate tile; app-name tooltip after 500 ms |
| Pressed | Stronger local tint until release; no layout shift |
| Keyboard focus | 2 DIP light outline on dark surface; independent of active underline |
| Multiple windows | Group count in tooltip/menu; open a compact text window list |
| Launching | Small progress mark; preserve slot position, then running indicator |
| Launch failure | Anchored message with app name and Retry; no endless spinner |
| Missing pinned app | Neutral placeholder with label and Remove pin action |

Click a non-running app to launch; click a running app to request activation; click a group with multiple windows to choose a window. Do not assume clicking an active app minimizes it; v1 requests activation consistently. Right-click / Shift+F10 exposes Launch and Pin/Unpin, plus supported window selection. No jump-list, thumbnail-preview or task termination feature is introduced. Pins persist in user-defined order; manage ordering through a bounded “Move left / Move right” action in the catalog/taskbar context menu, avoiding a required drag implementation.

Focus cycling uses Tab/Shift+Tab between groups, Left/Right within app slots, Enter/Space activate, Escape close a menu. When app slots overflow, preserve Start/status and offer a “More apps” menu rather than shrink icons. Third-party tray hosting is a separate engineering problem: initial status icons may be Sei-Desk-owned network/audio/battery indicators. Do not claim full Explorer notification-area parity until validated; preserve access to the Windows tray if it cannot be replicated safely.

## 8. Start launcher behavior

Launcher opens near top-center with search focused. Empty query shows six pinned apps in stable order; “All applications” opens the catalog and closes Start. If no pins exist, show “No pinned applications” with “Browse applications”; search remains usable. A pinned icon receives an outline when keyboard focused.

Typing filters installed application names only. Proposed matching: case-insensitive prefix first, then word/substring; use alphabetical ordering for ties, without introducing recommendation or recency algorithms. Query updates are immediate from the user’s perspective. During app-index loading, display “Loading applications…”; if indexing fails, explain it and offer Retry while keeping already known entries usable.

| Input / event | Result |
| --- | --- |
| Windows key | Desired open/toggle behavior; Windows-specific interception is an unverified requirement |
| Windows + other key | Preserve OS shortcuts such as Win+L, Win+D and Win+E; do not trigger launcher |
| Typing / IME composition | Edit query; do not launch or move selection while composition is active |
| Up / Down with results | Move active result without moving text caret; scroll it into view; clamp at ends |
| First results for new query | Select first match; selection changes announced when accessibility permits |
| Enter | Launch selected result; close only on accepted launch request |
| Enter with no results | No action; retain useful no-results message |
| Escape | Close launcher and restore prior window focus if permitted |
| Windows key while open | Desired close behavior, subject to shortcut handling validation |
| Tab / Shift+Tab | Move among search, pins/results and All applications; maintain visible focus |
| Left / Right in search | Normal text-caret behavior |
| Mouse hover on result | Hover tint only; does not silently replace the keyboard-selected result |
| Mouse click on result | Select and launch that result |
| Click outside / genuine focus loss | Dismiss launcher; never cancel a pointer interaction inside its own popup |

Each result row: 32 DIP application icon, name, “Application” subtitle and Enter hint only on the active row. Selected fill uses slate-blue, mint rail and semibold name; selection is not communicated by color alone. Empty-search means empty query with pins; no-results means a non-empty query with zero matches. No-results copy: **“No applications found”**, followed by **“Try a different application name or check spelling.”** Launch failure appears inline, retaining query and selection for retry.

Windows-key interception must be designed around press/release timing, left/right Windows keys, repeat suppression, modifier combinations, focus policy and recovery from failed hooks. Validate it in an isolated future technical investigation. Provide clickable Start/search access regardless; show the settings row as unavailable until this capability is validated. These are requirements, not a claim that iced can capture global keys.

## 9. Top dock and control panels

Default dock groups: small brand mark/name; brightness with compact slider; volume with compact slider; network/battery status; power/session button. Battery appears only when a battery exists; no-battery devices omit it without a broken icon. Network is status only in v1. The brand button opens the Overview window. Dock controls have accessible labels and numeric values in expanded panels; default sliders retain full hit lanes.

Brightness: 0–100%, proposed default example 80%; label the controlled display. Click its button opens the brightness panel; drag changes the requested value. Keyboard arrows adjust by 1, PageUp/PageDown by 10, Home/End min/max. Only report success after the OS acknowledges the change. Unsupported display: disable slider and explain “Brightness unavailable for this display”; do not imply every external monitor supports brightness changes. A failed write returns to the last confirmed value and shows Retry.

Volume: 0–100%, example 60%; current device name, slider and Mute button. Space activates focused mute button; arrows control focused slider. Mute preserves the prior volume value; use a crossed-speaker icon plus “Muted” text. Unmuting restores that value. No output device shows “No audio output device” and a disabled slider; do not introduce a device-management utility. The panel has no unexplained settings cog.

Power menu order: Lock, Sleep, divider, Restart, Shut down. Lock acts immediately; Sleep acts immediately when supported. Restart/Shut down open a confirmation with explicit action name, “Save your work…” and Cancel initially focused. Escape cancels. Do not auto-confirm on focus loss; dismiss without executing. Disabled actions explain OS/hardware limitations. Errors remain visible and restore menu usability. No timed power action or hidden forced shutdown is proposed.

Opening a dock panel closes Start and any other dock panel. Escape dismisses and returns focus to its dock button; outside click dismisses. Sliders should not steal focus merely from pointer hover. Tooltip delay 500 ms; tooltip dismissal immediate on pointer exit or key input. Proposed motion: 120–160 ms opacity/tint transitions, maximum 4 DIP vertical popover movement, no spring/zoom. Reduce motion makes transitions immediate. None of these animations is required for first implementation feasibility.

## 10. Monitoring dashboard

Overview is the default management-window destination. A 176–200 DIP left rail contains the three navigation entries; a content header precedes four compact cards. At widths with insufficient space for four readable cards, use 2 × 2; never force tiny metrics. Sidebar current location uses tinted fill, a short leading rail and semibold label.

| Metric | Example | Meaning |
| --- | --- | --- |
| CPU | 18% | Aggregate utilization; 60-second simple line history |
| Memory | 7.8 / 24 GB; 32.5% | Physical memory used / total; capacity bar |
| Disk C: | 318 / 1,000 GB; 31.8% | Capacity used / total, explicitly decimal GB |
| Disk activity | 4%; 2.1 MB/s | Separate sampled busy time and transfer rate; do not substitute capacity |
| Network | ↓12.4 Mbps; ↑3.1 Mbps | Receive/transmit on named active interface; 60-second history |
| Basic health | Resource pressure normal; 682 GB free; uptime 2 h 34 m | Summary of measured resources only |

Display units consistently: decimal GB, MB/s and Mbps; labels are explicit to prevent byte/bit confusion. Percentages carry a % sign. Data refresh proposal: once per second, only redraw changed readings; suspend histories while window is hidden if practical. Charts show “60 s history,” provide labeled endpoints and expose current values in text. Hover/focus may reveal time/value; do not make precise values dependent on a tooltip. Mark gaps, not invented zeros.

Health thresholds are a proposed rule requiring later product validation: “High resource use” when CPU or memory exceeds 90% continuously for 30 s; “Storage low” when C: has under 10% free; otherwise “Resource pressure normal.” Include which threshold was crossed. This is not antivirus, thermal, drive-health or security certification. Do not show temperatures or invented hardware health data.

First sample: “Collecting readings…” with stable card skeleton. Failure: “Unavailable,” explanation on demand, last successful update timestamp. If no fresh sample arrives for 5 s, show “Data paused” and retain labeled last values; never show a stale “normal” health judgment. Retry/reconnect updates without moving cards. Limit the dashboard to read-only monitoring.

## 11. Applications catalog

Search field, All/Pinned filter, sort control and responsive grid. Default sort is Name A–Z; “Pinned order” groups pins in their shared order, with remaining applications alphabetical. Plate 06 demonstrates Pinned order. Four to six columns as space allows; card width at least 112 DIP, icon 32, readable label. Scroll content while navigation/header remain stable. Catalog search follows launcher matching, but Enter launches only the focused/selected item. Arrow keys navigate grid spatially, Tab reaches filters/actions, Shift+F10 opens context menu; mouse click selects, double-click or explicit Launch activates. This distinction prevents accidental launches while managing pins.

Selected item gets mint fill and a contrasting outline. Footer action area shows selected name, Launch and Pin/Unpin. Pin order is shared with Start/taskbar; empty pins state offers “Browse all applications.” Unpinning never uninstalls an application. Search with no results uses the launcher’s copy. Loading, scan failure, missing icons and launch errors use the same component states. Do not add uninstall, app installation, marketplace, or “Recent” tracking in v1.

## 12. Appearance and behavior settings

Two sections, Appearance and Behavior. Proposed controls: single Pastel theme; four pastel accents with names in tooltips/accessibility labels; translucent desktop panels toggle; Reduce motion toggle; Windows-key launcher toggle only when supported. No dark graphite theme is proposed. Transparency changes only desktop overlays; main content stays readable and nearly opaque. Use the system high-contrast preference to request the opaque fallback, subject to validation.

Settings save immediately with unobtrusive “Saved” feedback; no Apply button. A failed save retains the last persisted value and shows “Could not save. Try again.” Display disabled controls with a reason instead of hiding them. For Windows-key behavior, “Needs validation” is design-review annotation; the eventual product should show a user-facing explanation such as “Unavailable on this setup” or a working toggle, never engineering jargon. Initial geometry is fixed, avoiding a new taskbar-layout editor. Clock/date respect Windows locale and 12/24-hour preference.

## 13. Shared component inventory and states

| Component | Required states / rules |
| --- | --- |
| Icon/text button | Rest, hover, pressed, focus, disabled with reason; tooltip for icon-only |
| Search input | Empty, focused, typing/composition, loading, results, no results, index error; accessible label |
| App tile/result | Rest, hover, selected, focus, launching, missing icon, failure; fallback monogram |
| Taskbar slot | Pinned idle, running, active, grouped, hover, focus, launch failure |
| Navigation item / filter chip | Rest, current/selected, hover, keyboard focus; state label |
| Slider | Rest, hover, drag, focus, muted where relevant, disabled, rejected write; numeric text |
| Toggle | On, off, hover, focus, disabled, persistence error; label and state |
| Menu/popover | Open, focused item, disabled action, viewport clamped, dismissed |
| Metric/chart | Loading, live, high use, unavailable, paused, empty history |
| Feedback message | Success, caution, error, retry action; message and symbol, not color only |
| Confirmation | Title, consequence, Cancel, explicit action; initial focus Cancel |
| Tooltip | Delayed pointer reveal and keyboard equivalent; does not receive focus |

Use one selection fill per surface, one focus treatment per contrast context and no decorative nested cards. Main window uses platform close/minimize/maximize conventions; the drawn titlebar is a proposal requiring native hit-test/drag validation if made custom. Closing the management window should leave taskbar/dock active; full Quit must be available through the Sei-Desk control menu with a clear consequence. Do not conceal a way to return to the Windows desktop controls.

## 14. Focus, accessibility, scaling and overflow

Maintain visible focus independent of selection and pointer hover. Preserve last focused control per management page. Navigation changes move focus to the page heading, while ordinary metric updates do not announce every second. Provide names/roles/states for icon-only controls, sliders, toggles and selected results. Announce launch failures and capability errors without stealing keyboard focus. Keyboard routes must work with Sticky Keys, IME and high-contrast settings; actual assistive-technology support is a technical validation gate.

All runtime sizes are **DIP**, not physical pixels. Test 100%, 125%, 150%, 175% and 200%; also mixed-DPI monitors, reconnects and scaling changes while windows are open. Recalculate monitor bounds and anchored positions on DPI change. Example: a 58 DIP taskbar becomes 87 physical pixels at 150%. Prefer vector system icons and suitably sized raster app icons. Test long names, translated labels, 12/24-hour clock and fonts with different metrics.

Test logical work areas at 1920 × 1080, 1366 × 768, 1280 × 720 and 960 × 540. At narrow widths the dock hides inline slider tracks but keeps labeled quick-control buttons; its popovers retain full controls. Taskbar uses app overflow and condenses optional status while preserving Start, active apps and clock. Launcher width clamps to available width minus 24 DIP and scrolls results; pinned apps wrap. Main window reduces navigation to an icon rail with labels on focus/hover, uses 2-column metrics, and scrolls vertically if the available work area is below the normal minimum. No clipped confirmation buttons or controls beyond monitor bounds. Mockups illustrate the full-width layout; these reduced-size behaviors are specified rather than claimed as rendered evidence.

## 15. Rust + iced feasibility and Windows validation register

Visual plan only: rounded containers, text, buttons, scrollable app lists, slider tracks, custom line icons, simple line charts, opaque fallback and consistent focus states are plausible iced compositions. This is a feasibility assessment, not a tested implementation or dependency/version selection. No CSS, React, JavaScript UI, WebView2 or WinUI 3 is required by this plan.

Current official iced window settings expose decoration, positioning, transparency and window-level concepts. Transparent windows alone do not establish wallpaper blur or Acrylic/Mica. Validate the chosen iced/winit/backend versions, native handle access and Windows visual effects separately. [iced window Settings](https://docs.rs/iced/latest/iced/window/struct.Settings.html), [iced window Action](https://docs.rs/iced/latest/iced/window/enum.Action.html).

| Design decision | OS behavior requiring validation | Fallback / release condition |
| --- | --- | --- |
| Windows key opens Start | Global key interception, modifier preservation, hook timing and Windows Start suppression | Clickable Start/search; do not mark keyboard interception complete without testing |
| Persistent bottom taskbar | Appbar reservation, native taskbar coexistence/hiding and restoration, Explorer restart | Keep native taskbar accessible until coexistence/recovery is proven |
| Running app indicators / activate | Window enumeration, ownership/grouping, foreground restrictions, elevated windows | Show supported windows; explain unavailable activation rather than claim success |
| Tray icons | Explorer notification-area behavior / third-party tray hosting | Sei-Desk-owned status indicators; retain Windows tray access |
| Slate glass | Compositor backdrop blur, transparency, reduced transparency preferences, performance | Opaque slate with same borders and hierarchy |
| Top dock placement | Work area reservation, maximized windows, topmost/fullscreen behavior | Clamp/yield appropriately; no permanent obstruction |
| Brightness | Built-in display versus external monitor interfaces and capability detection | Disabled control with display-specific explanation |
| Volume / mute | Audio endpoint discovery, volume writes, device changes | No-device/unavailable state; last confirmed value |
| Session / power | Lock/sleep/restart/shutdown support, permission, cancellation and failure | Supported actions only, explicit confirmation for destructive session actions |
| Apps catalog / launch | Start menu/installed app discovery, packaged apps, icons, launch mechanism | Known entries usable during partial discovery; retry on failure |
| Monitoring | CPU/memory/disk/network counters, sampling costs, interface selection | Unavailable/paused state with last-update time |
| Multiple DPI / displays | Native DPI awareness, coordinate conversion, monitor/work-area changes | Primary-monitor default and clamped placement |
| Accessible controls | iced/backend UI Automation roles, focus and announcements | Validate with keyboard and Narrator; block release of inaccessible essential routes |
| Custom window chrome | Native drag/resize/hit testing and standard window commands | Standard OS titlebar if custom chrome is unreliable |

Microsoft documents low-level keyboard hooks through `WH_KEYBOARD_LL` and `SetWindowsHookEx`; that is evidence of a Windows integration mechanism, **not proof that this product can reliably replace bare-Windows-key behavior**. Hook callbacks have timing and lifecycle constraints. [Microsoft LowLevelKeyboardProc](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)).

Windows appbars and DPI-aware desktop windows have their own lifecycle/positioning requirements. Review these in the future engineering investigation rather than equating a borderless iced window with a shell taskbar. [Microsoft application desktop toolbars](https://learn.microsoft.com/en-us/windows/win32/shell/application-desktop-toolbars), [Microsoft high-DPI desktop development](https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows).

## 16. Future optimization center — concept only

Inspired by WinUtil’s category-based accessibility, not its branding, scripts or tweak catalog. This package does not validate or endorse any actual system change. No initial navigation entry or working optimization action.

Future flow: category browsing → individual unchecked proposals → explicit selection → review summary → permission where necessary → apply with itemized progress → outcome and recovery guidance. Proposed categories are Privacy, Startup and Services, solely to demonstrate organization. Each real proposal must later describe current setting, proposed value, reason, tradeoff, affected subsystem, required privilege, restart need, detection confidence and recovery procedure. Unsupported proposals remain disabled with reasons. No bulk selection by default.

Review must list precisely the selected changes, conflicts and restart requirement; Back preserves selection. Cancel before apply makes no changes. Progress distinguishes queued/running/completed/failed/skipped; cancellation stops at a safe boundary rather than promising mid-operation rollback. On partial failure, list completed and untouched settings separately, show actual error details, and offer recovery only for changes with a supported reverse operation. Never claim every tweak can be undone or a restore point guarantees recovery. Recovery guidance must cover when user action is needed. Warn about restarting only when an actual selected change requires it.

Plate 08 uses “Example change A” deliberately; it illustrates the interaction rather than recommending an unvalidated tweak. The four stages share the same type, radii, pale surfaces, selection and focus language as the initial product.

## 17. Design review checklist and approval boundary

- [x] Folder inspected without assuming reference filename; primary image opened at full resolution.
- [x] Sampled palette recorded with coordinates; derived colors distinguished from evidence.
- [x] Pastel wallpaper and light application surfaces remain coherent with slate floating overlays.
- [x] Taskbar switches/launches; Start searches apps; top dock controls hardware/session actions.
- [x] Eight deliverables include launcher and taskbar states, control panels, supporting pages and future concept.
- [x] Component geometry, focus, pointer/keyboard routes, layers, fallback behavior and scaling documented.
- [x] Rust + iced constraints respected; unverified Windows behaviors explicitly separated.
- [x] Optimizer clearly outside initial release; no unrelated features introduced.
- [x] Static mockups rendered and inspected for legibility, missing content and overlaps; any final QA notes are recorded in README.
- [ ] User reviews and approves design direction, source-to-light-surface interpretation and provisional mark.
- [ ] Future engineering validates OS capabilities and accessibility before implementation/release claims.

Approval applies to the design only. Implementation requires a later instruction; do not begin application scaffolding or component work based on this document alone.
