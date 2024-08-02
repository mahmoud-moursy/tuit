# Tuit (pronounced To-It, or Toy-t)

# Note: Tuit is a W.I.P.

- While the flesh of the library is there and documented well-enough, what Tuit is really
  missing is some more widgets!
- And some more testing, too!
  
- Currently, only about two functional widgets exist, `CenteredText` and`CenteredPrompt`.
  They demonstrate what is possible with Tuit, but without more widgets,
  and a proper book, I cannot really call this a true TUI library.

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

### MSRV Policy

Current minimum required version: Rust Stable 1.80

My personal recommendation is to stick to the latest stable release. Up until recently, the library was based on nightly
because some needed features were missing -- however, as of writing on Rust 1.80, they are all available, so I can move to stable.