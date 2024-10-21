// #todo Abbreviate to `buf`.
// #todo consider other names: `Buf`, `Byte-Buffer`, etc.

use std::sync::{Arc, RwLock};

use tan::{context::Context, error::Error, expr::Expr, util::module_util::require_module};

// #insight
// size -> in bytes (maybe size-in-bytes ?)
// length/count -> in items/elements (maybe size ?)

// #todo add support for u32, u64 and maybe even other types.
// #todo add support for little-endian/big-endian.

// #todo use array instead of vec? can we have dynamic array, probably a slice.

// #todo make buffer Iterable/Iterate

pub fn buffer_new(args: &[Expr]) -> Result<Expr, Error> {
    // #todo also support a default-element/fill option.

    let [length] = args else {
        return Err(Error::invalid_arguments("requires `length` argument", None));
    };

    // #todo create a helper.
    let Some(length) = length.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("length=`{length}` is not Int"),
            length.range(),
        ));
    };

    let length = length as usize;
    // #todo allow for custom initial value.
    let buf: Vec<u8> = vec![0; length];

    Ok(Expr::Buffer(length, Arc::new(RwLock::new(buf))))
}

// (put buf index value)
pub fn buffer_put(args: &[Expr]) -> Result<Expr, Error> {
    // #todo enforce bounds!!

    let [buffer, index, value] = args else {
        return Err(Error::invalid_arguments(
            "requires `index` and `value` arguments",
            None,
        ));
    };

    let Some((length, mut buffer)) = buffer.as_buffer_mut() else {
        return Err(Error::invalid_arguments(
            &format!("buffer=`{buffer}` is not a Buffer"),
            buffer.range(),
        ));
    };

    let Some(i) = index.as_int() else {
        return Err(Error::invalid_arguments(
            &format!("index=`{index}` is not Int"),
            index.range(),
        ));
    };

    if i < 0 {
        // #todo use specialized error variant? e.g. invalid_argument_out_of_bounds?
        return Err(Error::invalid_arguments(
            &format!("buffer index=`{i}` cannot be negative"),
            index.range(),
        ));
    }

    let i = i as usize;

    if i >= length {
        return Err(Error::invalid_arguments(
            &format!("buffer index=`{i}` must be less than the buffer length=`{length}`"),
            index.range(),
        ));
    }

    let Some(value) = value.as_u8() else {
        return Err(Error::invalid_arguments(
            &format!("value=`{value}` is not U8"),
            value.range(),
        ));
    };

    buffer[i] = value;

    // #todo what should we return?
    Ok(Expr::None)
}

pub fn setup_lib_buffer(context: &mut Context) {
    // #todo put in 'buffer' path, and import selected functionality to prelude.
    let module = require_module("prelude", context);

    // #todo consider `Buf`.
    module.insert_invocable("Buffer", Expr::foreign_func(&buffer_new));

    // #todo also provide a put$$Int

    module.insert_invocable("put$$Buffer$$Int$$U8", Expr::foreign_func(&buffer_put));
}

// #todo Push with Int, reuse push with U8
// #todo Support 5u8.
// #todo Implement Eq.
