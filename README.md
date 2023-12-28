# Tuit (pronounced To-It, or Toy-t)

## The TUI lib for the no_std, no_alloc hard-mode Rust user!

Tuit does not provide a complete suite of utilities to handle terminals. It can be considered the 'view' component of
the
MVC architecture, since it only deals with the display of terminal objects.

### Virtual terminals

At the basis of Tuit is an abstraction on the terminal; the virtual terminal.

Virtual terminals are anything that implements the `tuit::terminal::Terminal`, and there are
a few options that both avoid and utilize allocation (note that the `alloc` feature is required for
certain dynamically resizing terminals, but there is also a no-alloc `MaxSizeTerminal` which can finitely scale until 
the specified parameters).

