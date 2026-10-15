# Nybbles Clickthrough

A Godot 4.2+ GDExtension that lets mouse clicks fall through a transparent window to the apps behind it,
for overlays and desktop pets.

## Install

Download `nybbles_clickthrough.zip` from the [latest release](https://github.com/ChristofferEnne/nybbles-clickthrough/releases/latest)
and unzip it into your project, so you get `addons/nybbles_clickthrough/`. There is nothing to enable in Project Settings:
Godot loads the extension on start.

## Use

```gdscript
var clickthrough := ClickThroughManager.new()
add_child(clickthrough)
clickthrough.set_clickthrough(true)  # clicks go to whatever is under the window
clickthrough.set_clickthrough(false) # the window takes clicks again
```

Turn it off while the mouse is over your own UI, and on everywhere else.

| Platform | How |
|---|---|
| Windows (x86_64) | Sets `WS_EX_TRANSPARENT` and `WS_EX_LAYERED` on the window |
| Linux (x86_64), macOS (universal) | Godot's own `WINDOW_FLAG_MOUSE_PASSTHROUGH` |

For a see-through window, also turn on `display/window/size/transparent`, `display/window/size/borderless` and
`display/window/per_pixel_transparency/allowed`, and set `get_viewport().transparent_bg = true`.

## Build from source

```
cargo build --release
```

Copy the library from `target/release/` into `addons/nybbles_clickthrough/bin/` with the name the
`.gdextension` file expects.

## Releasing

Push a tag such as `v0.2.0`. The Build workflow compiles Windows, Linux and macOS, zips the addon with the
binaries in `bin/`, and attaches `nybbles_clickthrough.zip` to a GitHub release. That zip is what you upload to
the Godot Asset Store. Every push to `master` also keeps the zip as a workflow artifact.

## Licence

MIT, see `LICENSE`.
