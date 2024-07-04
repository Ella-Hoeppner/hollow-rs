AST visualizer/editor for general-purpose visual programming. Follow up on [vlojure](https://github.com/Ella-Hoeppner/Vlojure). Early WIP.

## to-do
### High priority, necessary for mobile prototype
* when zoomed in, have forms that are too big to be fully on screen become part of the "background". You can't drag out of or into these forms, and dragging on them acts like dragging in the background (i.e. it scrolls). However, if you tap on them, it still zooms to their view like normal
* support different form shapes, determined in FormStyle
  * should be represented as a star: https://www.shadertoy.com/view/3tSGDy
    * This can act as an n-gon, or a circle (i.e. n-gon with ultra-high n), or a spikey thing somewhere in between. This alone gives plenty of variety, but it would also be easy to parameterize a path along the outside, such that I can eventually add spikes or dots or whatever to decorate forms further
* let the window be split into two rectangles, where the main app only shows up on one, allowing the other portion to be used for graphical apps
  * get a little proof-of-concept WGSL shader running in the other part of the window
* create a little wgsl domain warping DSL, to have something to play around with on mobile and see how the editing experience feels when working on a real project
* on-screen buttons for undo/redo, I guess?
  * I guess if we're gunna have onscreen buttons, could have one for toggling `InputMode::Delete`
* figure out a principled scaling factor for the 2-touch drag scrolling
  * currently it's at 500 and that feels about right. I don't get why it needs to be so high though? The movement should be in screenspace units... I guess maybe it would need to be converted from screenspace units to workspace units or something? But a factor of 500 doesn't seem to match up with that
    * I guess with 2 fingers it's kinda doubling up... might make sense to calculate with the diff of the center rather than the individual touch position
* consider emulating momentum when scrolling with dragging on the background, or the 2-touch gesture

### Mid priority, necessary for real programming on desktop (probably with clojure at first)
* BUG: drag-and-deleting a subform into a different top-level form seems to break buffer data
  * Forms disappear, though when you hover over them they reappear, though even then the text data seems fucked up, you get the invalid-character boxes
    * usually seems to happen on the right of where the form was dragged to, tho sometimes it happen on the left as well
* refine rendering
  * Make UI elements be rendered similarly to forms in Fancy mode
  * some characters, like "0" and "!", seem to be cut off at the bottom, having a flat edge where they should be rounded. Not sure why this is happening.
  * add character color, determined by a field in FormStyle
  * in fancy render mode, compute sdfs for each character, and render text as an engraving in forms
  * maybe have special logic for changing the appearance of specific strings?
    * the form "*" looks weird right now, the * is all the way at the top of the circle. Maybe that's just the font?
* text editing
  * use [egui](https://github.com/emilk/egui)
  * App will need to accept a `ast_reader(String) -> Option<AST<I,L>>` on intialization. When the user finishes editting the text in a leaf, that function will be used to process it. If `None` is returned, the leaf won't be changed (either it won't let the user exit the text box or it will just revert it to what the leaf was before editing, not sure yet), but if `Some` is returned, the leaf will be replaced with the returned AST. The returned AST isn't required to be a leaf, so the user could write a full heirarchical sub-tree in text to replace the leaf, if they wanted to.
* Make evaluation asynchronous
  * https://github.com/rust-windowing/winit/issues/1199#issuecomment-1685122253
  * make the babashka evaluator handle errors properly using this, as a proof of concept
* support de/serializaing edn expressions, use this in the babashka repl
* Implement an evaluator that connects to a clojure nrepl server
  * And then clojurescript nrepls, too. Ideally it should be easy to connect to a shadowcljs repl
* keyboard shortcuts:
  * escape should fully zoom out
* Have a menu for reorganizing the screen layout
  * this should be overlayed over the workspace rather than just a set of sliders in the settings menu like vlojure had, to make it more intuitive and directly obvious what is being affected
  * when you enter this screen, it will automatically zoom to be focused on the first top-level form
    * it will remember your location when you entered this screen, and put you back where you were when you leave
    * you won't be able to scroll or zoom around the workspace in this view
  * features:
    * moving, adding, and deleting formbars
      * vlojure's UI for this is pretty good, can probably just do that again
    * resizing formbars
      * should each be able to have its own size? or just have one universal size control?
    * resizing trash zone, repl zone
    * maybe move trash zone, repl zone to different corners?
      * eventually I might have a ton of different optional elements like these, so letting these be elements that can be customizable will be good in the long run
      * also, a way to control the direction in which they expand
    * angle of workspace
      * Can have a little draggable circle with an icon at the edge of the first top-level form, pointing in the angle of the current workspace alignment. This will be similar to what vlojure had, but more intuitive since it's placed directly on the workspace rather than a simulacrum of the workspace in the settings menu
    * zoom level of focused forms
      * not quite sure what the best UI for this would be...
      * could just make it so the normal zooming control affects this, which would be cool if you knew about that, but it wouldn't be very discoverable...
        * could have some other UI for controlling it, but allow this as an option
      * maybe just an explicit slider, or plus/minus buttons?
  * not sure how this screen should be accessible...
    * I guess there could just be something in the options menu for this
      * once I implement an options menu, at least
    * I guess for development it could just be something you can toggle into with a key, but it eventually needs to be accessible through some on-screen button for mobile

### Low priority
* BUG: On desktop, you can drop things into corner zones even when you drag the mouse outside the window
* I think it might be possible to simplyify the rendering order and do fewer render passes if I use depth testing for various elements
* The built-in mac scrolling momentum along with the soft-clamping systems sometimes leads to jitteriness. Need to resolve this somehow.
  * Maybe integrate the scroll deltas over time, then tween in that direction of multiple frames, rather than applying the deltas instantaneously as offsets
    * Maybe even have an explicit handler for both the integrated scroll delta along with the soft clamp force as a single equation, rather than trying to handle them separately and summing the results
* have some subtle visual indication of when a form is focused, i.e. when it is pointed to by `workspace_view.ast_key_and_path`, as opposed to the camera just being near it
  * so when you click on a form, this visual indicator would show up, but if you scroll or magnify even a little bit, it would disappear
  * this would telegraph the fact that the view will be automatically adjusted when the form is deleted, whereas it won't be if you've scrolled/magnified since clicking on a form
* when the mouse enters a form, rather than just highlighting, add a movement that attracts towards the default position, but starts with a small velocity in the direction of the mouse movement
  * should give a really nice jiggle effect
  * would be nice to make initial speed proportional to mouse speed
    * probably want this calculated as like a rolling average, not instantaneous
* more advanced/flexible formbar features:
  * a type of formbar that has a fixed size, and can contain more elements than would really fit within it, but that you can scroll inside of
  * logic for ensuring formbars never overlap
  * maybe support multiple formbars on the same "row" of a side, like vlojure had
  * add a kind of formbar intended for showing all bindings available in the current lexical scope
    * of course, this depends on the semantics of the langauge, so the logic for determining this in a given location definitely needs to be abstracted out to whatever instantiates the app
    * maybe this shouldn't really be a formbar tho... maybe some other kind of UI element?

* support multiple workspaces being open at once, as a split screen
  * it would be nice if these could be like minimized too... I guess I kinda want a tab + split system like a normal IDE window manager...
  * should formbars be attached to each workspace, or to the whole window?
    * I think whole window... or maybe both? but that could be difficult, and confusing for new users

## Long-term plan
* explore new window types
  * current ideas:
    * a view of whatever form is hovered over 
      * should show the relative position of the mouse inside the form
    * a view of each form along the currently focused path
      * should be able to click to zoom out
      * zooming out will leave the previously-focused subforms in the view, until the user focuses into something not along the currently displayed path
