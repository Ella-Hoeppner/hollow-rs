# hollow-rs (working name)
WIP low-level graphics library for rust. Wraps [wgpu](https://github.com/gfx-rs/wgpu) with more convenient abstractions, assuming sane defaults for most settings but allowing be overriding when necessary, primarily through "builder" structs.

## to-do
* purefrag shader abstraction
* writing to textures
  * demo for this, maybe stigmergy?
* optional event handler functions on Sketch trait
  * resize
  * mouse move
  * mouse click

* support multiple windows
  * demo for this
