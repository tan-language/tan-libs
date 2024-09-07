// #note #WARNING Not used yet!

// #todo Move all the code from tan/library.

// #todo helper function or macro for arithmetic operations!
// #todo also eval 'if', 'do', 'for' and other keywords here!
// #todo use macros to monomorphise functions? or can we leverage Rust's generics? per viariant? maybe with cost generics?
// #todo support overloading,
// #todo make equality a method of Expr?
// #todo support non-Int types
// #todo support multiple arguments.
// #todo helper function or macro for arithmetic operations!

// #todo primitives, builtins
// #todo cannot be implemented with Expr::ForeignFunc as the args are pre-evaluated.

// #insight special-form != primitive ? special form requires special case in the interpreter?

// #todo consider removing the `std` prefix from module paths, like haskell.
// #todo find a better prefix than setup_
// #todo use Rc/Arc consistently
// #todo some helpers are needed here, to streamline the code.

// #todo only setup non-prelude libs on demand!pub mod http_server;

// #todo call the foreign setup from the actual tan module file.

// #todo find a good name for this.
// pub fn setup_lib(context: &mut Context) {
//     setup_lib_uuid(context);
// }
