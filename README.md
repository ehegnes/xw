# xw
A safe, high-level builder-pattern X11 wrapper.

For now, this is a fun learning exercise for me. It is not intended for public
use, although interest is welcomed.

## Examples
Check the tests in each source file for usage examples.

## Faculties
### Implemented
- Basic server connection with reasonable defaults
- Windowing
  - Naming
  - Positioning
  - Border decoration
- Coloring
  - Easy color management with `XBuilder::add_color()`
  - Foreground and background color setting
- Drawing
  - Points
  - Segments

### TODO
- Event handling
- Proper error handling
- Some way to enforce builder method calling order?
- Build out drawing module and collect for redrawing

## Attributions
Thanks to the folk at `#rust` and `#xorg-devel`.
