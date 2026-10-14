# Sei-Desk UI/UX design specification

**Status:** proposed design for review, 14 October 2026. **Deliverables:** written specification and eleven static vector mockups, with PNG previews. Dock behavior revised to auto-hide on 14 October 2026. Nothing in this package implements application behavior. All metrics, application entries, dates and device names in mockups are illustrative.

## 1. Source of truth and design interpretation

The primary reference is [Sei-Desk Pastel Desktop UI Showcase.png](../../assets/Sei-Desk%20Pastel%20Desktop%20UI%20Showcase.png), inspected at its native **1536 × 1024** resolution before selecting the design direction. The folder also contains `icon.png`, inspected separately: a glossy cyan S mark with a Windows-like inset. Its luminous treatment conflicts with this brief; do not use its glow, cyan edge or glossy finish as UI styling. The small mountain emblem in the showcase is the provisional mockup mark. Final brand-mark selection remains a review decision; these plates do not replace either source asset.

The showcase combines a blue/lavender alpine lake wallpaper with blush clouds, dark trees, translucent blue-slate floating surfaces, fine pale edges, mint selection/slider accents, compact application icons, and pale text. The launcher sits beneath a distinct top control dock; the taskbar runs almost edge to edge at the bottom. Lower panels share a left navigation rail, compact metric tiles and rounded fields. Typography visually resembles a Windows system sans serif; the actual font cannot be identified from pixels alone.

**Retain:** airy pastel environment, compact density, floating rounded desktop controls, consistent shell geometry, muted mint emphasis, restrained shadows. **Refine:** use light lavender/blue-gray surfaces for larger application windows, dark slate text for contrast, a single selected-row language, consistent icon geometry and accurate numeric/slider relationships. This is a documented adaptation toward the requested soft pastel direction: the source’s large dashboard panels are darker than the proposed ones. Keep the desktop overlays dark enough to remain readable over wallpaper; they must not turn the whole product into a graphite theme.

**Correct reference inconsistencies:** remove Files, Settings and Web search filters from the application launcher; replace horizontal navigation hints with Up/Down; show the value matching the slider thumb; replace the arbitrary neon color picker with four constrained pastel accents; distinguish running from active rather than using glow; use unambiguous disk capacity versus activity labels. Source panel labels, tiny raster artifacts and window chrome are illustration details, not product requirements.

The mockup wallpaper is an original simplified vector interpretation of the reference landscape and its sampled colors. It is a composition aid, not a new wallpaper feature or a proposal to modify the user’s desktop. App monogram tiles are legible design placeholders; production should use installed application icons without recoloring their brands.

## 2. Product structure and release boundary

Sei-Desk is a Windows desktop customization and basic monitoring application. Mode: **Operate**. The primary user wants quick application access, readily accessible desktop controls and readable resource information without a busy desktop.

| Surface | Purpose | Initial release |
| --- | --- | --- |
| Bottom taskbar | Launch pinned apps; switch running windows; compact status and time | Yes, subject to Windows integration validation |
| Start launcher | Find and launch installed applications from the keyboard | Yes, Windows-key behavior subject to validation |
| Top quick-control dock | Auto-hiding brightness, audio, session/power actions, compact indicators | Yes, individual controls depend on supported hardware/APIs |
| Overview | CPU, memory, disk, network and limited health readings | Yes |
| Applications | Supporting catalog, launch and pin management | Yes, supports launcher/taskbar |
| Settings | Supporting appearance and behavior controls | Yes, bounded to these surfaces |
| Optimization center | Explicit selection, review and recovery concept | Future only; absent from initial navigation |

No window tiling, marketplace, assistant, gaming features, extra system utilities, file search, web search or shell-replacement claims. Initial navigation contains **Overview, Applications, Settings**. Do not carry the reference’s ambiguous “System” page into an extra module.

## 3. Mockup index and reading conventions

| Plate | Design artifact | Coverage |
| --- | --- | --- |
| 01 | [Desktop — dock hidden](mockups/01-desktop-composition.svg) | Default desktop, no top strip, unchanged Start and taskbar |
| 01B | [Desktop — dock revealed](mockups/01b-desktop-dock-revealed.svg) | Same composition after non-activating top-edge reveal |
| 02 | [Taskbar states](mockups/02-taskbar-states.svg) | Default, running, active, hover, keyboard focus |
| 03 | [Start states](mockups/03-start-launcher-states.svg) | Empty query with pins, matches, moved selection, no results |
| 04 | [Quick-control dock](mockups/04-quick-control-dock.svg) | Revealed dock, brightness, volume/mute, power, confirmation, unavailable |
| 05 | [Monitoring](mockups/05-system-overview.svg) | Four metrics, histories, basic health and data-failure rule |
| 06 | [Applications](mockups/06-applications-catalog.svg) | Search, All/Pinned, alphabetical catalog, selected app and actions |
| 07 | [Settings](mockups/07-appearance-behavior-settings.svg) | Pastel theme, accents, translucency, motion, Windows-key status |
| 08 | [Future optimizer](mockups/08-future-optimization-concept.svg) | Selection → review → progress → failure/recovery |
| 09 | [Auto-hide states](mockups/09-auto-hide-dock-states.svg) | Hidden, revealing, visible, expanded, pending, hiding |
| 10 | [Expanded dock interaction](mockups/10-expanded-dock-interaction.svg) | Anchored volume, focus, pointer bridge and confirmation protection |

Plates are presentation boards, not pixel-for-pixel runtime screenshots. Most boards are 1600 × 1000; the launcher state board is 1600 × 1080 and the dock state board is 1600 × 1260. Their captions and margins are outside application chrome. Four quick-control variants are presented together for review, although only one popover opens in use. Shell windows and widgets are enlarged on boards for legibility; the following DIP measurements govern the proposed runtime layout. SVGs are self-contained static design assets without scripts or browser-dependent application behavior.

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

Target composition at 1920 × 1080 physical pixels / 100% scale: bottom taskbar inset 12 DIP horizontally and 10 from bottom, height 58; top dock hidden by default, revealed centered at y12, width about 760, height 52; launcher centered horizontally, width 580, top edge 80. The reference uses approximately 40% screen width for its dock and 30% for its launcher; treat these as proportional evidence, not a requirement to stretch on ultrawide displays. Keep intrinsic sizes with max-width clamping.

Launcher height follows content, nominal 300–440 DIP with at most five visible result rows and an internal scrollbar for more. Keep a minimum 12 DIP gap below the dock; never cover it. Anchor brightness/volume/power popovers 8 DIP below their owning control; clamp within the monitor with 12 DIP safe margins. Popovers are 280–320 DIP wide. Management window nominal size 1040 × 680, minimum 800 × 560 DIP. Use normal resizable window behavior; never cover the taskbar deliberately.

Layer order within the design: desktop wallpaper → ordinary applications → desktop bars where OS policy permits → active launcher or dock popover → local menu/tooltip → confirmation dialog. This expresses visual intent only. Actual topmost behavior, appbars, exclusive fullscreen, focus restrictions and secure-desktop boundaries require Windows validation. Do not place bars above lock screen, UAC or exclusive fullscreen. A passive pointer reveal of the dock may coexist with Start without changing its focus. Deliberately activating a dock control closes Start; invoking Start closes dock popovers and hides the dock unless a power confirmation is unresolved. Launcher and dock popovers never compete for focus; confirmation ownership rules in section 9 take precedence. Tooltips do not receive keyboard focus.

The taskbar stays bottom-mounted; the auto-hiding dock retains a fixed top-center revealed position. Reserve bottom workspace if supported. The top dock reserves **no permanent workspace**: hidden means no visible strip or layout gap; deliberate reveal temporarily overlays the application without resizing it. Maximized-window, topmost and input-routing behavior still require Windows validation; visual positioning alone cannot guarantee non-interference. Primary-monitor-only remains the proposed first-release default. A launcher invoked from a different monitor uses that monitor’s available work area, pending validation. Do not replicate docks or activate other monitors implicitly. Recompute the dock’s physical-edge sensor after scaling, display movement/disconnection or primary-display change, including negative desktop coordinates. See section 9 for suppression and focus rules.

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

## 9. Auto-hiding top dock and control panels

This revision changes dock visibility and its coordination only. It preserves the existing palette, typography, compact controls, 760 × 52 DIP revealed geometry, 12 DIP top inset, 12 DIP radius, icon family, fine border and slate translucent surface. Taskbar, Start, monitoring, catalog, settings artwork and future optimizer remain visually unchanged. The desktop compositions differ only in dock visibility and review captions. An always-visible top dock is superseded by this section.

### 9.1 Default, activation zone and geometry

Default state is **Hidden**, including at startup and after dismissal. The entire dock, border and shadow are outside the monitor viewport or fully non-rendered and non-interactive. No handle, thin background strip, persistent glow or activation-zone graphic remains. Plate 09’s arrows, labels and diagram geometry are reviewer annotations, not product chrome.

Proposed reveal sensor: **one physical pixel high** at the supported monitor’s physical top edge, horizontally centered and spanning only the dock’s revealed width (about 760 DIP converted at that monitor’s scale, clamped to screen width minus 24 DIP). It is not a full-screen-width band. Entry is detected without a visible surface and without consuming mouse clicks, wheel events or window-drag input; the reliable sensing mechanism must be validated. Trigger on actual entry, without dwell, only when no pointer button is held and no suppression condition applies. A pointer parked on the edge after dismissal must leave and re-enter before revealing again. This avoids an immediate reopen after Escape.

The physical edge is the monitor’s top coordinate, not desktop y=0 and not the maximized app’s client edge. A stacked monitor boundary needs explicit validation; v1 does not create sensors on unsupported monitors or promise a hard pointer stop at every monitor edge. The sensor must remain one physical pixel at 125–200% scaling, rather than accidentally become a large DIP strip. Re-evaluate its width/position with each monitor/DPI change; disable stale geometry until recalculated.

Reveal slides down from fully outside the monitor (including shadow) to the established y12 DIP position. It must be clipped at that monitor boundary, with no animation spilling onto a monitor above it. Shadow disappears with the dock. While visible, a logical pointer corridor spanning the dock width covers the 12 DIP gap from edge to dock, so the user can travel from the sensor to its controls. The corridor is used only to decide visibility and does not intercept clicks. Expanded panels retain their 8 DIP gap; a corridor only as wide as the overlap between the dock and associated panel bridges that gap. No large invisible rectangle surrounds the desktop. Corridor membership is active only while the dock is revealing/visible, never an extra hidden-state activation area.

### 9.2 Timing and six states

| Parameter | Proposed value | Rule |
| --- | --- | --- |
| Reveal trigger | Immediate on eligible edge entry | No dwell or focus request |
| Reveal animation | 180 ms | Subtle slide down, decelerating; acceptable tuned range 150–220 ms |
| Dismissal delay | 450 ms | Starts only after the last interaction hold ends |
| Hide animation | 180 ms | Subtle slide up, gently accelerating; acceptable range 150–220 ms |
| Return before delay expires | Cancel immediately | No queued hide or countdown UI |
| Reverse during animation | Current position → new endpoint | No jump or restart from an endpoint; duration proportional to remaining travel, capped at 180 ms |
| Reduced motion | 0 ms positional animation | Immediate reveal/hide; retain 450 ms dismissal delay and all protections |
| Tooltip | Existing 500 ms delay | Never extends the dock’s lifetime by itself |

| State | Visual specification | Entry, exit and interaction |
| --- | --- | --- |
| **Hidden** | No visible dock, shadow or strip; no focusable off-screen controls | Eligible edge entry or explicit keyboard/menu request → Revealing. Suppressed entry does nothing. |
| **Revealing** | Cropped dock translates down; pastel/slate tokens unchanged | On arrival → Visible. If pointer leaves early without any hold, finish the short reveal and enter Pending; do not flicker. Keyboard invocation requests focus only after dock is available. |
| **Visible** | Complete dock at normal position | Pointer within dock/entry corridor holds it. Open control → Expanded. No hold → Pending. |
| **Control expanded** | Full dock plus anchored panel; active owner gets expanded cue | Pointer within dock, panel or bridge holds it. Slider capture, keyboard session or confirmation suspends dismissal. Losing all holds → Pending with panel still visible during grace. |
| **Dismissal pending** | Same full surface; no fade, dimming, timer badge or geometry change | Wait 450 ms. Return to dock/panel/bridge or acquire any hold → Visible/Expanded. Expiry with no hold → close transient panel and Hiding. |
| **Hiding** | Dock translates up; panel already closed; no lingering shadow | Completed → Hidden. Eligible edge or dock re-entry before completion reverses smoothly → Revealing. Once fully hidden, only sensor/explicit access can reopen it. |

Written transition path: **Hidden → Revealing → Visible ↔ Expanded → Pending → Hiding → Hidden**. Re-entry cancels Pending. Any valid visibility hold blocks Pending/Hiding. Re-entry during Hiding reverses from the current position. These states do not authorize any implementation in this task.

A visibility hold exists when at least one condition is true: pointer is within the dock/panel/defined bridges; a slider or pressed control owns a pointer interaction; a keyboard session owns focus within the dock/panel; or a dock-owned power confirmation is unresolved. A panel merely being open, or a control retaining incidental focus from a pointer click, is not an indefinite keyboard hold. Pointer motion alone does not cancel a keyboard session. Clear that session on explicit close, deliberate external interaction or genuine focus transfer outside the dock and its owned windows. Track owned popovers as one interaction, so changing between separate native windows does not look like leaving the application.

### 9.3 Retained quick controls and dismissal ownership

Control groups remain: brand/name, brightness, volume/mute, compact network/battery status, and power/session. Battery is omitted on devices without a battery. Network is status only; brand opens Overview. Default inline sliders keep full hit lanes. No additional system utility is added.

Brightness remains 0–100%, example 80%, labeled by controlled display. Arrow keys change by 1, PageUp/PageDown by 10, Home/End to endpoints. Unsupported display shows the existing disabled slider/explanation. Failed writes revert to the last confirmed value with Retry. Volume remains 0–100%, example 60%, with current audio device and a Mute button. Space activates the focused Mute button; muting preserves volume and adds the crossed-speaker icon and “Muted” text. No output device retains the disabled state. Percentages and thumb positions match.

Opening brightness/volume/power closes any other dock panel. Clicking outside a transient panel closes it immediately; if no other hold exists, start the 450 ms dock delay. Do not hide a dock that is still being hovered. Slider press/drag owns a hold until release or explicit cancellation, even if the pointer leaves the panel, dock, monitor or screen. Release outside starts a fresh 450 ms delay; release inside resumes normal visibility. Genuine pointer-capture loss must end editing safely at the last confirmed value, not leave the dock stuck open. Never dismiss halfway through an active drag.

Power menu remains Lock, Sleep, divider, Restart, Shut down, with supported/disabled states unchanged. Restart/Shut down open an owned confirmation with Cancel initially focused. **A confirmation never auto-dismisses:** pointer exit, the dismissal timer, ordinary focus loss or app switching cannot close it. Outside click is ignored for confirmation dismissal and never confirms an action. Escape and explicit Cancel cancel; the explicit action button alone accepts. Dock and dialog retain a visibility hold until resolution. A background confirmation remains recoverable; do not force it above a full-screen app or secure desktop. Returning to Sei-Desk restores access to it. Actual Windows shutdown/UAC/session behavior remains unverified.

After resolving a confirmation, return focus to its initiating power button only if Sei-Desk still owns focus; otherwise preserve the app the user switched to. Re-evaluate holds and start a fresh delay if none remains. Do not resume an old expired timer. Pending confirmation also prevents Start invocation from replacing/closing it; preserve the confirmation and offer it when Sei-Desk next receives deliberate interaction. Do not intercept unrelated Windows shortcuts to force attention.

### 9.4 Keyboard, focus and accessibility

Proposed explicit shortcut: **Ctrl+Alt+Q**, subject to registration/conflict and assistive-technology validation. It reveals the dock and focuses its first enabled control. Do not enable an unavailable shortcut silently; provide a keyboard-reachable **Open quick controls** action in the existing Sei-Desk control menu, with no new taskbar visual. Document shortcut availability in user help. This is a required access route, not an implemented global shortcut.

Pure edge hover reveals a non-activating dock without stealing focus from the active application or Start. Clicking a control deliberately may transfer focus. Pressing Tab or an adjustment key within a focused dock begins a keyboard session, which prevents auto-hide without any inactivity timeout. Tab/Shift+Tab cycles the enabled dock controls and, when expanded, the associated panel controls; disabled items are explained and skipped as actions. Enter/Space opens or activates the focused control. Slider arrows retain the existing adjustment behavior. All focus indicators are independent of hover.

Keyboard order: brand/Overview → Brightness → its inline slider if present → Volume → its inline slider if present → Power; passive status icons do not become tab stops unless they expose useful accessible details. Opening a panel moves focus to its primary slider or first enabled menu action. While a transient panel is open, its local Tab sequence stays within that panel; Escape closes it and returns to the owning dock button. A second Escape closes the dock and requests restoration to the previously active application if permitted. With no panel, Escape closes the dock directly. Never hide focused controls or leave focus on an off-screen element. If all controls are disabled, focus a readable explanation and allow Escape.

On deliberate focus transfer outside the dock and owned panels, end the keyboard hold and close transient panels; if the pointer is also outside, start a fresh 450 ms dismissal delay. Confirmation dialogs are the explicit exception. When hiding due to pointer-only departure, do not issue a foreground/focus request. Ordinary app switching should retain the newly active application.

Visible keyboard focus is a 2 DIP pale outline on slate; a panel’s owner also has a muted-mint expanded indicator and an expanded accessibility state. Provide names, roles, slider values, mute state and panel relationships. Announce explicit keyboard reveal and expanded/collapsed changes once; do not announce passive pointer hover or timer ticks. Hidden controls are absent from keyboard traversal/accessibility navigation until deliberately revealed. An accessible named control-menu route is essential because an invisible edge has no discoverable screen-reader target. Validate actual iced/backend UI Automation and Narrator behavior; the artwork is not proof of accessibility support. Respect reduced motion from system or existing setting, using the more restrictive preference; opaque fallback remains available.

### 9.5 Windows desktop constraints and surface coordination

| Situation | Intended behavior | Engineering validation required |
| --- | --- | --- |
| Normal desktop / windowed application | Eligible sensor entry reveals; no foreground focus change | Physical-coordinate pointer sensing and non-activating native window behavior |
| Maximized application | No top workspace reserved, no resize/reflow; one-pixel centered sensor passes underlying input through; revealed dock temporarily overlays only its own bounds | Input pass-through, browser tabs/titlebars, top-edge window drags and native taskbar coexistence; a transparent window alone is not proof |
| Pointer button held / window being dragged | Do not trigger a hidden dock; active dock slider capture remains protected | Correct distinction between external drag and owned control capture |
| Exclusive or borderless full-screen on supported monitor | Suppress automatic reveal and clear pending edge activation; hide ordinary dock UI at a safe interaction boundary | Reliable full-screen detection, presentation/game behavior, topmost changes and capture cleanup |
| Explicit keyboard/menu invocation in full-screen | Do not force an overlay or steal focus; wait for exit, then require a fresh deliberate invocation | Shortcut delivery/conflicts and safe user feedback; no queued surprise reveal after exit |
| Full-screen while dialog unresolved | Keep confirmation state; suspend overlay presentation until user returns, without discarding dialog or applying anything | Owned dialog persistence, Z-order and safe re-presentation; secure desktop always takes precedence |
| Multiple monitors / scaling | Primary-monitor dock only in v1; derive sensor from that monitor’s physical edge, convert DIP geometry separately, clamp panel to same monitor | Negative coordinates, stacked edges, mixed DPI, primary change, disconnect/reconnect and monitor-bound clipping |
| Custom Start opens | Close dock transient panel and hide dock; Start keeps its existing keyboard semantics, subject to unresolved confirmation exception | Shared surface ownership and focus restoration |
| Dock hovered while Start is open | Passive reveal only; Start retains focus. Activating a dock control closes Start first | Avoid focus-loss dismissal during passive reveal |
| Windows taskbar / custom taskbar | Bottom design unchanged; no new top appbar reservation; control-menu access opens dock deliberately | Shell Z-order, Explorer restarts, desktop switch and taskbar menu routing |
| Lock screen / UAC / secure desktop | Suppress sensor and overlays; never present above secure desktop | Session changes, cancellation and restore without stale topmost state |

Topmost intent is scoped to deliberate visible use on the normal desktop. It is not an instruction to keep a hidden always-on-top window intercepting input or to override foreground policy. All sensor windows, capture, Z-order, full-screen detection and focus changes require future Rust/Windows integration validation. Use iced for native UI composition/animation; an iced transparent window or ordinary mouse subscription alone does not establish global physical-edge sensing. No WebView2, WinUI 3 or web frontend is introduced.

### 9.6 Future acceptance scenarios — specified, not executed

1. Startup shows no dock strip/shadow; exact sensor entry reveals within the 180 ms animation without losing editor/Start focus.
2. Moving through the 12 DIP entry corridor and 8 DIP panel bridge never dismisses the dock; leaving for 449 ms then returning cancels hiding.
3. Leaving all holds for 450 ms closes the transient panel and starts a 180 ms hide; return mid-hide reverses without a jump.
4. Drag a slider outside its panel for longer than 450 ms: dock remains; releasing outside starts a new full delay.
5. Reveal by keyboard, navigate and pause for several seconds: dock remains visible. Escape closes panel then dock and restores focus appropriately.
6. Open Restart confirmation, leave the dock, switch apps and wait: it remains unresolved/recoverable; outside click never confirms or dismisses it.
7. Repeat with reduced motion: no positional animation; delay, focus and confirmation protections unchanged.
8. Test maximized browsers, window drags, borderless/exclusive full-screen, UAC, stacked monitors and mixed 100–200% scaling; no intercepted edge click, stale sensor, permanent workspace gap or unsolicited overlay.

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
| Auto-hiding top dock | Physical-edge sensing, non-activating/topmost window, monitor clipping, owned-popover focus, capture, full-screen suppression | No permanent top reservation; suppress unsupported automatic reveal, retain explicit keyboard/menu access; validate section 9 scenarios |
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
- [x] Eleven plates include the original module designs plus dock-hidden/revealed compositions, six auto-hide states and expanded-control interaction.
- [x] Dock timing, activation zone, keyboard ownership, slider capture and non-dismissing confirmations documented.
- [x] Component geometry, focus, pointer/keyboard routes, layers, fallback behavior and scaling documented.
- [x] Rust + iced constraints respected; unverified Windows behaviors explicitly separated.
- [x] Optimizer clearly outside initial release; no unrelated features introduced.
- [x] Static mockups rendered and inspected for legibility, missing content and overlaps; any final QA notes are recorded in README.
- [ ] User reviews and approves design direction, source-to-light-surface interpretation and provisional mark.
- [ ] Future engineering validates OS capabilities and accessibility before implementation/release claims.

Approval applies to the design only. Implementation requires a later instruction; do not begin application scaffolding or component work based on this document alone.
