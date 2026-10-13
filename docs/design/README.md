# Sei-Desk design review package

Start with the [design overview](SEI-DESK-DESIGN-OVERVIEW.png), then read the [UI/UX specification](SEI-DESK-UI-UX-SPEC.md).

| Plate | PNG preview | Editable vector design |
| --- | --- | --- |
| 01 · Complete desktop | [PNG](mockups/01-desktop-composition.png) | [SVG](mockups/01-desktop-composition.svg) |
| 02 · Taskbar states | [PNG](mockups/02-taskbar-states.png) | [SVG](mockups/02-taskbar-states.svg) |
| 03 · Start launcher states | [PNG](mockups/03-start-launcher-states.png) | [SVG](mockups/03-start-launcher-states.svg) |
| 04 · Dock and quick controls | [PNG](mockups/04-quick-control-dock.png) | [SVG](mockups/04-quick-control-dock.svg) |
| 05 · Monitoring dashboard | [PNG](mockups/05-system-overview.png) | [SVG](mockups/05-system-overview.svg) |
| 06 · Applications catalog | [PNG](mockups/06-applications-catalog.png) | [SVG](mockups/06-applications-catalog.svg) |
| 07 · Appearance and behavior | [PNG](mockups/07-appearance-behavior-settings.png) | [SVG](mockups/07-appearance-behavior-settings.svg) |
| 08 · Future optimizer | [PNG](mockups/08-future-optimization-concept.png) | [SVG](mockups/08-future-optimization-concept.svg) |

All deliverables are static design documents. SVGs contain vector artwork, text and presentation-board annotations; they contain no scripts or application components. No app was scaffolded and no application logic or original assets were changed. PNGs are raster previews of those vector plates. Use the specification’s DIP dimensions when reviewing future implementation; board pixels are presentation measurements.

## Reference and intentional adaptations

The original assets folder was inspected and both images were opened. The 1536 × 1024 showcase is the visual authority. The specification records exact sampled patches/pixels and distinguishes proposed derived colors. Desktop overlays retain slate glass; larger windows use pale lavender and blue-gray surfaces with dark text. Reference mint is deliberately muted. The wallpaper is a simplified vector interpretation of the original alpine scene. Monogram tiles stand in for installed-app artwork. The mountain mark is provisional; the existing glossy cyan icon was not modified.

## Design verification

All eight full-resolution PNGs were opened and inspected. SVG files parsed successfully as static XML without scripts. One correction batch moved an overlapping tooltip and desktop caption, aligned shared pin labels/catalog ordering, added chart scale labels and matching current endpoints, and outlined Cancel in the restart confirmation. Final previews were inspected after rendering those changes.

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
