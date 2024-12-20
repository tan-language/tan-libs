use tan::{
    context::Context,
    error::Error,
    expr::{format_value, Expr},
    util::{
        args::{unpack_int_arg, unpack_stringable_arg},
        module_util::require_module,
    },
};

// #todo string push/append/concat, make similar to array: push one char, append/concat another string, ++ handles both.

// #todo rearrange the functions in some logical order, can be alphabetical.

// #todo (compare str1 str2) ; => Ordering
// #todo (to-lowercase str) or (lowercased str)
// #todo (to-uppercase str) or (uppercased str)

// #idea just use the String constructor: (String "hello " num " guys"), or even (Str "hello " num " guys")
// #todo support: (Str (HTML-Expr (p "This is a nice paragraph!")))
pub fn string_new(args: &[Expr]) -> Result<Expr, Error> {
    let output = args.iter().fold(String::new(), |mut str, x| {
        str.push_str(&format_value(x));
        str
    });

    Ok(Expr::String(output))
}

// #todo better name: `size`?
// #insight `count` is not a good name for length/len, better to be used as verb
pub fn string_get_length(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments(
            "`chars` requires `this` argument",
            None,
        ));
    };

    let Expr::String(s) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    Ok(Expr::Int(s.len() as i64))
}

// #todo Implement in Tan?
pub fn string_is_empty(args: &[Expr]) -> Result<Expr, Error> {
    let s = unpack_stringable_arg(args, 0, "s")?;
    Ok(Expr::Bool(s.is_empty()))
}

// #todo trim-start
// #todo trim-end

pub fn string_trim(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    let Expr::String(s) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    Ok(Expr::string(s.trim()))
}

// #todo how to implement a mutating function?
// #todo return (Maybe Char) or (Maybe Rune), handle case of empty string.
/// Removes the last character from the string buffer and returns it.
pub fn string_pop(_args: &[Expr]) -> Result<Expr, Error> {
    // #todo handle the string mutation!
    // #todo handle empty string case!!

    todo!()
}

// #todo enforce range within string length
// #todo rename to `cut`? (as in 'cut a slice')
// #todo relation with range?
// #todo pass range as argument?
// #todo support negative index: -1 => length - 1
// #insight negative index _may_ be problematic if the index is computed and returns negative by mistake.
/// (slice str 2 5)
/// (slice str 2)
/// (slice str 2 -2) ; -2 is length - 2
pub fn string_slice(args: &[Expr]) -> Result<Expr, Error> {
    let [this, start, ..] = args else {
        return Err(Error::invalid_arguments(
            "`slice` requires `this` and start arguments",
            None,
        ));
    };

    let Some(s) = this.as_stringable() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let Expr::Int(start) = start.unpack() else {
        return Err(Error::invalid_arguments(
            "`start` argument should be an Int",
            this.range(),
        ));
    };

    let start = *start;

    let end = if let Some(end) = args.get(2) {
        let Expr::Int(end) = end.unpack() else {
            return Err(Error::invalid_arguments(
                "`end` argument should be an Int",
                this.range(),
            ));
        };
        *end
    } else {
        s.len() as i64
    };

    let start = start as usize;
    let end = if end < 0 {
        // #todo supporting negative index may hide errors if the index is computed
        // #todo offer a link to only support negative values for constant index
        // If the end argument is negative it indexes from the end of the string.
        (s.len() as i64 + end) as usize
    } else {
        end as usize
    };

    let string_slice = &s[start..end];

    Ok(Expr::string(string_slice))
}

// #todo search `recognize_range`.
// #todo this should reuse the plain string_slice method.
/// Cuts a slice out fo a string, defined by a range.
pub fn string_slice_range(args: &[Expr]) -> Result<Expr, Error> {
    let [this, start, ..] = args else {
        return Err(Error::invalid_arguments(
            "`slice` requires `this` and range arguments",
            None,
        ));
    };

    let Expr::String(s) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let Expr::IntRange(start, end, ..) = start.unpack() else {
        return Err(Error::invalid_arguments(
            "`range` argument should be a Range",
            this.range(),
        ));
    };

    // #todo support open-ended ranges.
    // #todo extract the following.

    let start = *start;
    let end = *end;

    let start = start as usize;
    let end = if end < 0 {
        // #todo supporting negative index may hide errors if the index is computed
        // #todo offer a link to only support negative values for constant index
        // If the end argument is negative it indexes from the end of the string.
        (s.len() as i64 + end) as usize
    } else {
        end as usize
    };

    let string_slice = &s[start..end];

    Ok(Expr::string(string_slice))
}

/// Returns a char iterable for the chars in the string.
pub fn string_chars(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments(
            "`chars` requires `this` argument",
            None,
        ));
    };

    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let mut exprs: Vec<Expr> = Vec::new();

    for char in this.chars() {
        exprs.push(Expr::Char(char));
    }

    Ok(Expr::array(exprs))
}

pub fn string_constructor_from_chars(args: &[Expr]) -> Result<Expr, Error> {
    let [chars] = args else {
        return Err(Error::invalid_arguments("requires `chars` argument", None));
    };

    let Some(exprs) = chars.as_array() else {
        return Err(Error::invalid_arguments(
            "`chars` argument should be a (Array Char)",
            chars.range(),
        ));
    };

    // #todo verify Array item type!

    let mut chars: Vec<char> = Vec::new();

    for expr in exprs.iter() {
        if let Some(c) = expr.as_char() {
            chars.push(c);
        }
    }

    let string = String::from_iter(chars);

    Ok(Expr::String(string))
}

// #todo overload for string and char!

pub fn char_to_upper_case(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments(
            "`to-upper-case` requires `this` argument",
            None,
        ));
    };

    let Expr::Char(this) = this.unpack() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a Char",
            this.range(),
        ));
    };

    // #todo omg...
    let uppercased = this.to_uppercase().next().unwrap();

    Ok(Expr::Char(uppercased))
}

// #insight
// Originally the Swift-like `lowercased` was considered, `to-lower-case` was
// preferred, as it's more consistent, allows for (let lowercased (to-lower-case str)),
// and scales to other cases, e.g. `to-kebab-case`, `to-snake-case`, etc)
pub fn string_to_lower_case(args: &[Expr]) -> Result<Expr, Error> {
    let [this] = args else {
        return Err(Error::invalid_arguments("requires `this` argument", None));
    };

    // #todo consider as_stringable?
    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let lowercased = this.to_lowercase();

    Ok(Expr::String(lowercased))
}

// #todo make this a String constructor?
// #todo 'join' and 'format' versions?

// #todo use (to-string ..) instead of format-value
// #todo find another name, this is too common: `fmt`? `stringf`?
// (format-string "hello {} {:.5}" name price)
pub fn string_format(args: &[Expr]) -> Result<Expr, Error> {
    let output = args.iter().fold(String::new(), |mut str, x| {
        str.push_str(&format_value(x));
        str
    });

    Ok(Expr::String(output))
}

// name: split
// type: (Func (String String) String)
// macro annotation: (this: String, separator: String) -> String
// (Func (this separator) ..)
// (Func (#String this #String separator) String)
pub fn string_split(args: &[Expr]) -> Result<Expr, Error> {
    // #todo Consider different order of arguments.
    // #todo Don't use `this` name.
    let string = unpack_stringable_arg(args, 0, "this")?;
    let separator = unpack_stringable_arg(args, 1, "separator")?;

    // #todo should return iterator

    let parts: Vec<Expr> = string.split(separator).map(Expr::string).collect();

    Ok(Expr::array(parts))
}

// #todo string_is_matching

pub fn string_contains(args: &[Expr]) -> Result<Expr, Error> {
    let [this, string] = args else {
        return Err(Error::invalid_arguments(
            "`contains` requires `this` and `string` arguments",
            None,
        ));
    };

    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let Some(string) = string.as_string() else {
        return Err(Error::invalid_arguments(
            "`string` argument should be a String",
            string.range(),
        ));
    };

    Ok(Expr::Bool(this.contains(string)))
}

pub fn string_starts_with(args: &[Expr]) -> Result<Expr, Error> {
    let [this, prefix] = args else {
        return Err(Error::invalid_arguments(
            "`starts-with` requires `this` and `prefix` arguments",
            None,
        ));
    };

    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let Some(prefix) = prefix.as_string() else {
        return Err(Error::invalid_arguments(
            "`prefix` argument should be a String",
            prefix.range(),
        ));
    };

    Ok(Expr::Bool(this.starts_with(prefix)))
}

pub fn string_ends_with(args: &[Expr]) -> Result<Expr, Error> {
    // #todo consider `suffix` instead of `postfix`.
    let [this, postfix] = args else {
        return Err(Error::invalid_arguments(
            "`ends-with` requires `this` and `postfix` arguments",
            None,
        ));
    };

    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let Some(postfix) = postfix.as_string() else {
        return Err(Error::invalid_arguments(
            "`postfix` argument should be a String",
            postfix.range(),
        ));
    };

    Ok(Expr::Bool(this.ends_with(postfix)))
}

// #todo implement `replace-once`.

// #todo support replace with array of rules or just use array spread.
// #todo consider a separate function called `replace*` to support multiple arguments?
// #todo or better consider compiler-optimization statically if there is only one replacement.
// #todo IDE hint if a compiler-optimization is performed.
// #todo could allow for multiple replacements (i.e. pairs of rules)
// #todo different name? e.g. rewrite?
pub fn string_replace(args: &[Expr]) -> Result<Expr, Error> {
    // #insight _from, _to are only used to verify that there is at least one
    let [this, _from, _to, ..] = args else {
        return Err(Error::invalid_arguments(
            "`replace` requires `this`, `from`, and `to` arguments",
            None,
        ));
    };

    let Some(this) = this.as_string() else {
        return Err(Error::invalid_arguments(
            "`this` argument should be a String",
            this.range(),
        ));
    };

    let mut output: String = this.to_string();

    let mut i = 1;
    while i < args.len() {
        let from = &args[i];
        let Some(from) = from.as_string() else {
            return Err(Error::invalid_arguments(
                "`from` argument should be a String",
                from.range(),
            ));
        };

        let to = &args[i + 1];
        let Some(to) = to.as_string() else {
            return Err(Error::invalid_arguments(
                "`to` argument should be a String",
                to.range(),
            ));
        };

        output = output.replace(from, to);

        i += 2;
    }

    Ok(Expr::String(output))

    // let Some(from) = from.as_string() else {
    //     return Err(Error::invalid_arguments(
    //         "`from` argument should be a String",
    //         from.range(),
    //     ));
    // };

    // let Some(to) = to.as_string() else {
    //     return Err(Error::invalid_arguments(
    //         "`to` argument should be a String",
    //         to.range(),
    //     ));
    // };

    // Ok(Expr::String(this.replace(from, to)))
}

// #todo move to cmp.rs?
// #todo should this get renamed to `stringable_compare`?
// #todo should be associated with `Ordering` and `Comparable`.
pub fn string_compare(args: &[Expr]) -> Result<Expr, Error> {
    // #todo support multiple arguments.
    let [a, b] = args else {
        return Err(Error::invalid_arguments(
            "requires at least two arguments",
            None,
        ));
    };

    // #todo is this check required if we perform type inference before calling
    // this function?

    let Some(a) = a.as_stringable() else {
        return Err(Error::invalid_arguments(
            &format!("{a} is not a String"),
            a.range(),
        ));
    };

    let Some(b) = b.as_stringable() else {
        return Err(Error::invalid_arguments(
            &format!("{b} is not a String"),
            b.range(),
        ));
    };

    // #todo temp hack until Tan has enums?
    let ordering = match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    Ok(Expr::Int(ordering))
}

// #todo Add repeat, justl, justr.

// #todo #decision Using pad works better with pad character.
// #insight The name is inspired from Python, an alternative could be `padr``.
// Justify Left.
pub fn string_justl(args: &[Expr]) -> Result<Expr, Error> {
    // #insight The s arg comes first to work better with the optional parameter.
    let s = unpack_stringable_arg(args, 0, "str")?;
    let width = unpack_int_arg(args, 1, "width")? as usize;
    let pad_char = if args.len() > 2 {
        unpack_stringable_arg(args, 2, "pad-char")?
    } else {
        " "
    };

    let length = s.chars().count();

    let just_s = if length >= width {
        s.to_string()
    } else {
        format!("{}{}", s, pad_char.repeat(width - length))
    };

    Ok(Expr::String(just_s))
}

pub fn setup_lib_string(context: &mut Context) {
    let module = require_module("prelude", context);

    module.insert_invocable("String", Expr::foreign_func(&string_new));

    // #insight it's OK if it's not short, it's not often used.
    // #todo consider a shorter name, e.g. `Str/from`, `Str/from-chars`
    // #todo implement as String constructor method/override?, e.g. `(Str [(Char "h")(Char "i")])`
    module.insert_invocable(
        "String/from-chars",
        Expr::foreign_func(&string_constructor_from_chars),
    );
    // env.insert("String$$Array", Expr::foreign_func(&string_constructor_from_chars)));

    module.insert_invocable("chars", Expr::foreign_func(&string_chars));
    module.insert_invocable("chars$$String", Expr::foreign_func(&string_chars));

    module.insert_invocable("is-empty?", Expr::foreign_func(&string_is_empty));
    module.insert_invocable("is-empty?$$String", Expr::foreign_func(&string_is_empty));

    // #todo rename to `to-uppercase`, more consistent?
    module.insert_invocable("to-upper-case", Expr::foreign_func(&char_to_upper_case));
    module.insert_invocable(
        "to-upper-case$$Char",
        Expr::foreign_func(&char_to_upper_case),
    );

    module.insert_invocable("to-lower-case", Expr::foreign_func(&string_to_lower_case));
    module.insert_invocable(
        "to-lower-case$$String",
        Expr::foreign_func(&string_to_lower_case),
    );

    module.insert_invocable("format-string", Expr::foreign_func(&string_format));

    module.insert_invocable("split", Expr::foreign_func(&string_split));

    module.insert_invocable("replace", Expr::foreign_func(&string_replace));

    // #todo slice is to general works both as noun and verb, try to find an explicit verb? e.g. `cut` or `carve`
    // #todo alternatively use something like `get-slice` or `cut-slice` or `carve-slice`.
    module.insert_invocable("slice", Expr::foreign_func(&string_slice));
    module.insert_invocable("slice$$String$$Int$$Int", Expr::foreign_func(&string_slice));
    module.insert_invocable(
        "slice$$String$$(Range Int)",
        Expr::foreign_func(&string_slice_range),
    );

    // #todo find a better name, `size`?
    // #insight `count` is _not_ a good name, reserve it for verb/action.
    // #todo What about count-of?
    module.insert_invocable("get-length", Expr::foreign_func(&string_get_length));
    module.insert_invocable("get-length$$String", Expr::foreign_func(&string_get_length));

    // #todo write tan unit test
    module.insert_invocable("trim", Expr::foreign_func(&string_trim));

    // module.insert_invocable("contains?", Expr::foreign_func(&string_contains)));
    module.insert_invocable(
        "contains?$$String$$String",
        Expr::foreign_func(&string_contains),
    );

    module.insert_invocable("starts-with?", Expr::foreign_func(&string_starts_with));

    /*
    (if (ends-with filename ".png")
    (if (ends-with? filename ".png")
        (handle-image filename)
        (handle filename)
    )
     */
    // #todo: consider 'ends-with' without '?'.
    module.insert_invocable("ends-with?", Expr::foreign_func(&string_ends_with));

    // Non-prelude string methods go to the /str namespace.

    let module = require_module("str", context);
    // #todo Consider other names, e.g. padr.
    module.insert_invocable("justl", Expr::foreign_func(&string_justl));
}
