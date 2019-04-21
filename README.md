# xw
A safe, high-level builder-pattern X11 wrapper.

# Example
```rust
extern xw;

use xw::XBuilder;
use xw::visualinfo::VisualInfo;
use xw::window::Window;

fn main() {
  xw::XBuilder::default()
    .visual(VisualInfo::default())
    .colormap()
    .attributes()
    .with_window(Window::default()
                 .name("Test Window")
                 .x(100)
                 .y(100)
                 .width(200)
                 .height(200))
    .flush();
}
```
