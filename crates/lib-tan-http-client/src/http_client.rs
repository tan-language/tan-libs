// #todo find good module path: network/http?
// network/http
// network/http/ws
// network/smtp

// #todo Implement a Client
// #todo Support default headers in Client.

// #todo no need for the `network/prefix`?
// #todo use `net` instead of `network`?

// #todo use non-blocking io.
// #todo add support for streaming responses!

// #todo separate server/client?

// #todo should introduce a `client` 'object'.

// #insight network/http is better than protocol/http, more specific.
// #insight use https://httpbin.org/ for testing.

// #ref https://tokio.rs/tokio/topics/bridging
// #ref https://crates.io/crates/reqwest

// #todo in the future consider using the lower-level hyper library.
// #todo in the future consider an async implementation, bring-in the tokio runtime.
// #todo introduce StatusCode, canonical reason.

// #todo implement general http/fetch.

use std::{collections::HashMap, str::FromStr, time::Duration};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tan::{context::Context, error::Error, expr::Expr, util::module_util::require_module};

/// Tries to extract a header from a function argument.
fn extract_headers(arg: Option<&Expr>) -> Result<Option<HeaderMap>, Error> {
    if let Some(headers) = arg {
        let Some(headers) = headers.as_map() else {
            return Err(Error::invalid_arguments(
                "`headers` argument should be a Map",
                headers.range(),
            ));
        };

        let mut req_headers = HeaderMap::new();

        for (key, value) in headers.iter() {
            let Some(value) = value.as_string() else {
                return Err(Error::invalid_arguments(
                    "`headers` values should be Stringable",
                    value.range(),
                ));
            };
            // #todo argh, remove the unwraps!
            req_headers.insert(
                HeaderName::from_str(key.as_str()).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }

        Ok(Some(req_headers))
    } else {
        Ok(None)
    }
}

fn build_tan_response(resp: reqwest::Result<reqwest::blocking::Response>) -> Result<Expr, Error> {
    let Ok(resp) = resp else {
        // #todo should return Error::Io, ideally wrap the lower-level error.
        // #todo return a better error.
        // #todo more descriptive error needed here.
        println!(">>> {resp:?}");
        return Err(Error::general("failed http request"));
    };

    let status = resp.status().as_u16() as i64;

    let Ok(body) = resp.text() else {
        // #todo return a better error.
        // #todo more descriptive error needed here.
        return Err(Error::general("cannot read http response body"));
    };

    let mut tan_response = HashMap::new();
    tan_response.insert("status".to_string(), Expr::Int(status));
    tan_response.insert("body".to_string(), Expr::string(body));
    // #todo also include response headers.

    Ok(Expr::map(tan_response))
}

pub fn http_get(args: &[Expr]) -> Result<Expr, Error> {
    let [url, ..] = args else {
        return Err(Error::invalid_arguments(
            "`get` requires `url` argument",
            None,
        ));
    };

    let Some(url) = url.as_stringable() else {
        return Err(Error::invalid_arguments(
            "`url` argument should be a Stringable",
            url.range(),
        ));
    };

    let client = reqwest::blocking::Client::new();

    let mut req = client.get(url);

    if let Some(headers) = extract_headers(args.get(1))? {
        req = req.headers(headers);
    }

    let resp = req.send();

    build_tan_response(resp)
}

// #example (http/post "https://httpbin.org/post" "payload" {"user-agent" "tan" "x-tan-header" "it works"})
// #insight Also supports headers as a third parameter.
// #todo support non-string bodies.
pub fn http_post(args: &[Expr]) -> Result<Expr, Error> {
    // #insight `_` does not work in the pattern.
    // #insight header are extacted later in the function.
    let [url, body, ..] = args else {
        return Err(Error::invalid_arguments(
            "`post` requires `url` and `body` argument",
            None,
        ));
    };

    let Some(url) = url.as_stringable() else {
        return Err(Error::invalid_arguments(
            "`url` argument should be a Stringable",
            url.range(),
        ));
    };

    // #insight
    // the following doesn't work:
    // let Some(body) = body.as_stringable() else {

    // #todo support stringables and streaming.
    let Expr::String(body) = body.unpack() else {
        return Err(Error::invalid_arguments(
            "`body` argument should be a Stringable",
            body.range(),
        ));
    };

    let body = body.clone();

    // #todo support streaming.
    // #todo use async

    // let client = reqwest::blocking::Client::new();
    // #todo #temp hack to workaround a timeout issue, we need a more robust solution.
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3 * 60))
        .build()
        .unwrap();

    let mut req = client.post(url);

    if let Some(headers) = extract_headers(args.get(2))? {
        req = req.headers(headers);
    }

    let resp = req.body(body).send();

    build_tan_response(resp)
}

// (http/send :POST "https://api.site.com/create" )
// (let resp (http/post "https://api.site.com/create" "body" { :content-encoding "application/json" }))
// (resp :status)

pub fn import_lib_http_client(context: &mut Context) {
    let module = require_module("network/http/client", context);
    // (get url headers)
    module.insert_invocable("get", Expr::foreign_func(&http_get));
    // (post url body headers)
    module.insert_invocable("post", Expr::foreign_func(&http_post));
}

// #todo add a unit test that at least exercises these functions.
// #todo use https://httpbin.org/ for testing.
