# Tuit (pronounced To-It, or Toy-t)

# Note: Tuit is a W.I.P.

- While the flesh of the library is there and documented well-enough, what Tuit is really
  missing is some more widgets!
- And some more testing, too!
  
- Currently, several layout widgets exist, like `Stacked`, `Centered`, and `Margin`. See the [docs](https://docs.rs/tuit/latest) for more information.
- There is a `Buttons` widget and a `Text` widget. See the [docs](https://docs.rs/tuit/latest) for more information.


- The licence is currently GPL-3.0+ — this is because it is s a lot easier to go from GPL-3 to a 
  more permissive licence, but not the other way around. For the foreseeable future, I will be releasing
  future releases under the GPL-3.0+ licence, but I am open to suggestions.


- If you have any concerns, you can open an [issue](https://github.com/T-O-R-U-S/tuit/issues/new).

## The TUI lib for the no_std, no_alloc hard-mode Rust user!

Tuit does not provide a complete suite of utilities to handle terminals. It can be
considered the 'view' component of the MVC architecture, since it only deals with the
display of terminal objects. However, it will not handle things like input, draw order, or other
application state for you — mostly because these things are far too
implementation-specific.

This means that, while the terminal objects may have nice features to handle their own
state, you are the one ultimately in control of program logic.

### Virtual terminals

At the base of Tuit is an abstraction on the terminal; the virtual terminal.

Virtual terminals are anything that implements the `tuit::terminal::Terminal`, and there
are a few options that both avoid and use allocation (note that the `alloc` feature is
required for certain dynamically resizing terminals, but there is also a no-alloc `MaxSize` which can
finitely scale until the specified parameters).

### Idea Backburner

- The way `tuit` is made, you can compose `BoundingBox` widgets together to create a full layout.
- ...so what if you could create a TUI application that chains them together?
- ...in a Unix-style fashion, allowing you to create a TUI application that is a composition of
  widgets, and then you can run it as a command-line program.

- I formally dub it "wedge"

```bash
wedge text "Hello, world!"
| wedge above
$(wedge button "Button 1")
| wedge center
```

Surely no one has thought of this before, right?

The revelation came to me and I just had to write it down somewhere.

### MSRV Policy

Current minimum required version: Rust Stable 1.80

My personal recommendation is to stick to the latest stable release. Up until recently, the library was based on nightly
because some needed features were missing -- however, as of writing on Rust 1.80, they are all available, so I can move to stable.