# Sei-Desk design review package

For the latest revision, start with the [auto-hiding dock overview](AUTO-HIDING-DOCK-OVERVIEW.png), then read [section 9 of the UI/UX specification](SEI-DESK-UI-UX-SPEC.md#9-auto-hiding-top-dock-and-control-panels). The [complete design overview](SEI-DESK-DESIGN-OVERVIEW.png) includes all eleven plates.

| Plate | PNG preview | Editable vector design |
| --- | --- | --- |
| 01 · Desktop, dock hidden | [PNG](mockups/01-desktop-composition.png) | [SVG](mockups/01-desktop-composition.svg) |
| 01B · Desktop, dock revealed | [PNG](mockups/01b-desktop-dock-revealed.png) | [SVG](mockups/01b-desktop-dock-revealed.svg) |
| 02 · Taskbar states | [PNG](mockups/02-taskbar-states.png) | [SVG](mockups/02-taskbar-states.svg) |
| 03 · Start launcher states | [PNG](mockups/03-start-launcher-states.png) | [SVG](mockups/03-start-launcher-states.svg) |
| 04 · Dock and quick controls | [PNG](mockups/04-quick-control-dock.png) | [SVG](mockups/04-quick-control-dock.svg) |
| 05 · Monitoring dashboard | [PNG](mockups/05-system-overview.png) | [SVG](mockups/05-system-overview.svg) |
| 06 · Applications catalog | [PNG](mockups/06-applications-catalog.png) | [SVG](mockups/06-applications-catalog.svg) |
| 07 · Appearance and behavior | [PNG](mockups/07-appearance-behavior-settings.png) | [SVG](mockups/07-appearance-behavior-settings.svg) |
| 08 · Future optimizer | [PNG](mockups/08-future-optimization-concept.png) | [SVG](mockups/08-future-optimization-concept.svg) |
| 09 · Six auto-hide states | [PNG](mockups/09-auto-hide-dock-states.png) | [SVG](mockups/09-auto-hide-dock-states.svg) |
| 10 · Expanded volume interaction | [PNG](mockups/10-expanded-dock-interaction.png) | [SVG](mockups/10-expanded-dock-interaction.svg) |

All deliverables are static design documents. SVGs contain vector artwork, text and presentation-board annotations; they contain no scripts or application components. No app was scaffolded and no application logic or original assets were changed. PNGs are raster previews of those vector plates. Use the specification’s DIP dimensions when reviewing future implementation; board pixels are presentation measurements.

## Launcher review — 14 October 2026

The existing launcher plate now shows its fifth state: a failed launch with query and selection retained, a polite inline error and an explicit Retry action. Section 8 defines fullscreen suppression, native key fallback, safe focus restoration, launch success/failure and accessibility behavior. No duplicate launcher artifact was added. The launcher’s colors and component style continue to match the pastel reference and adjacent desktop surfaces.

## Taskbar audit — 14 October 2026

The existing taskbar plate already covers pinned/running, active, hover and keyboard-focus visuals. Section 7 now clarifies minimized-window restoration and the explicit Minimize menu action, keyboard-accessible tray overflow, read-only taskbar status with volume routed to the dock’s single control panel, and full-screen suppression. It also states the primary-monitor mockup and secondary-monitor fallback. The existing plate remains unchanged; no duplicate mockup was needed. Shell coexistence, appbar reservation, window activation/minimize requests, tray hosting, full-screen detection and mixed-DPI behavior remain Windows engineering validation items in section 15.

## Auto-hide revision — 14 October 2026

The dock is hidden by default with no visible strip. Proposed behavior: one-physical-pixel top-edge sensor over the centered dock width, 180 ms reveal/hide, 450 ms dismissal grace, cancellation on return. Pointer travel through defined gaps is protected; slider capture, keyboard interaction and pending power confirmations hold visibility. Hover does not steal app focus. Reduced motion removes the slide while preserving the grace period. The expanded-control plate uses a pale focus ring on the slider and a muted-mint expanded cue on its owner.

Section 9 is authoritative for dock states, physical-edge geometry, timer rules, keyboard/menu access, focus restoration, full-screen suppression, maximized windows and mixed-DPI monitors. Ctrl+Alt+Q is a proposed keyboard shortcut requiring validation, with an existing control-menu access route as fallback. No permanent top workspace is reserved. Full-screen/secure-desktop suppression must not discard an unresolved confirmation.

Only dock-related compositions, annotations, specification sections and overview images changed. Existing taskbar, Start-state, monitoring, catalog, settings and optimizer plates retain their committed content (SVG comparisons normalize Git line endings); their PNGs are byte-for-byte unchanged. No application code was added. The task remains design-only.

## Reference and intentional adaptations

The original assets folder was inspected and both images were opened. The 1536 × 1024 showcase is the visual authority. The specification records exact sampled patches/pixels and distinguishes proposed derived colors. Desktop overlays retain slate glass; larger windows use pale lavender and blue-gray surfaces with dark text. Reference mint is deliberately muted. The wallpaper is a simplified vector interpretation of the original alpine scene. Monogram tiles stand in for installed-app artwork. The mountain mark is provisional; the existing glossy cyan icon was not modified.

## Design verification

The original eight full-resolution PNGs were inspected during the initial design pass. The launcher and full desktop reference assets were opened and reviewed for this Start design audit. In this revision, all five new/updated plates were rendered and opened at full resolution. One correction batch improved the intermediate slide framing and distinguished expanded-owner styling from actual keyboard focus. Final previews were inspected after rendering. Static SVG/XML and local-document links were checked, and unchanged module plates were compared against the initial commit. For this launcher review, the existing launcher PNG and combined overview were refreshed; the updated launcher PNG was rendered and inspected. No second launcher mockup was created. These are artifact checks, not runtime interaction tests.

Calculated opaque sRGB contrast ratios for representative token pairs:

| Pair | Ratio |
| --- | --- |
| Primary slate / light canvas | 10.82:1 |
| Secondary slate / pale panel | 4.99:1 |
| Pale primary text / dark selected row | 5.14:1 |
| Pale secondary text / slate base | 8.38:1 |
| Focus blue / pale panel | 5.10:1 |
| Focus blue / selected mint | 4.72:1 |

These calculations verify flat token pairs, not every future wallpaper composite. Runtime contrast, live focus handling, Narrator support, DPI behavior and OS integration were not tested because there is no application implementation in this task. Small-screen layouts are specified, not supplied as additional rendered plates. Live blur is an optional future validation item; opaque fallbacks are defined.

Review focus: pastel fidelity, the lighter management-window interpretation, taskbar/Start/dock separation, provisional brand mark, and the initial-release boundary. Windows behavior remains subject to technical validation. The optimizer is a concept only and is excluded from initial navigation. Design approval does not authorize implementation without a later instruction.
