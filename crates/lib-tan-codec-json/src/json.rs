use std::collections::HashMap;

use serde_json::{json, Map, Value};

use tan::{
    context::Context,
    error::Error,
    expr::Expr,
    util::{args::unpack_arg, expect_lock_read, module_util::require_module},
};

// #todo Extract as standalone library.

// #todo text/json or codec/json?
// #todo support json with comments.
// #todo functions should not panic on invalid arguments!!

/// Converts a JSON Value to a symbolic Expr.
fn json_value_to_expr(json: Value) -> Expr {
    match json {
        Value::Array(items) => {
            let mut arr = Vec::new();
            for item in items {
                arr.push(json_value_to_expr(item));
            }
            // #todo should generate base AST
            Expr::array(arr)
        }
        Value::Object(obj) => {
            let mut map: HashMap<String, Expr> = HashMap::new();
            for (key, value) in obj {
                // #todo should support more key types.
                // #todo should convert k from camelCase, PascalCase, snake_case, etc.
                let key = key.replace('_', "-");
                // let key = Expr::KeySymbol(key);
                map.insert(key, json_value_to_expr(value));
            }
            Expr::map(map)
        }
        Value::String(s) => Expr::String(s),
        Value::Number(n) => Expr::Float(n.as_f64().unwrap()), // #todo handle Int, Float, remove unwrap!
        Value::Bool(b) => Expr::Bool(b),
        Value::Null => Expr::None, // #todo is Unit the correct mapping?
    }
}

// #todo try to use ExprIter / fold -> maybe not the best use-case.
// #todo add comprehensive unit tests.
/// Converts a symbolic Expr to a JSON Value.
fn expr_to_json_value(expr: impl AsRef<Expr>) -> Value {
    let expr = expr.as_ref();

    // #todo support multi-line strings
    // #todo support Null
    // #todo somehow encode annotations.
    // #todo strip comments!
    // #todo write unit test!
    match expr {
        Expr::Array(exprs) => {
            let mut arr = Vec::new();
            // #todo should use try_lock_read?
            let exprs = expect_lock_read(exprs);
            for x in exprs.iter() {
                arr.push(expr_to_json_value(x));
            }
            Value::Array(arr)
        }
        Expr::Map(map) => {
            let mut obj = Map::new();
            // #todo should use try_lock_read?
            let map = expect_lock_read(map);
            for (k, v) in map.iter() {
                obj.insert(k.to_string(), expr_to_json_value(v.unpack()));
            }
            Value::Object(obj)
        }
        Expr::String(s) => Value::String(s.clone()),
        Expr::Symbol(s) => Value::String(s.clone()),
        Expr::KeySymbol(s) => Value::String(s.clone()),
        Expr::Int(n) => json!(n),
        Expr::Float(n) => json!(n),
        Expr::Bool(b) => Value::Bool(*b),
        _ => {
            dbg!(&expr);
            Value::String("Unsupported".to_string()) // #todo remove!
        }
    }
}

// #todo implement write/encode.

// #todo find a better name.
pub fn json_read_string(args: &[Expr]) -> Result<Expr, Error> {
    // #todo try to coerce to String.

    let [this] = args else {
        return Err(Error::invalid_arguments(
            "`read` requires `json` argument",
            None,
        ));
    };

    let Expr::String(json) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`json` argument should be a String",
            this.range(),
        ));
    };

    let Ok(value) = serde_json::from_str::<Value>(json) else {
        // #todo what is the correct error type?
        return Err(Error::invalid_arguments(
            "`json` string is not valid JSON",
            this.range(),
        ));
    };

    Ok(json_value_to_expr(value))
}

// #todo support (Str #JSON "{...}")

pub fn expr_to_json_string(args: &[Expr]) -> Result<Expr, Error> {
    let expr = unpack_arg(args, 0, "expr")?;
    let json_value = expr_to_json_value(expr);
    Ok(Expr::string(json_value.to_string()))
}

pub fn import_lib_codec_json(context: &mut Context) {
    // #todo find a good path and name.
    // #todo codec or serder? codec is more general.
    // #todo use `encode`/`decode` instead of `write`/`read`

    // #todo BAD module path.
    let module = require_module("codec/json-codec", context);

    // #todo find better name.
    // (use codec/json-codec)
    // (let value (json-codec/read json))
    module.insert_invocable("read", Expr::foreign_func(&json_read_string));
    // #todo find better name
    module.insert_invocable("to-string", Expr::foreign_func(&expr_to_json_string));
}

// #todo consider separate namespace for module names and paths? then we could have json-codec -> json
