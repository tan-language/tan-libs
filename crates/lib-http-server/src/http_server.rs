use std::{collections::HashMap, sync::Arc};

use axum::{
    body::to_bytes,
    extract::Request,
    handler::HandlerWithoutStateExt,
    http::{header, HeaderMap, HeaderName, StatusCode},
};

use tan::{
    context::Context,
    error::Error,
    eval::invoke_func,
    expr::{annotate_type, Expr},
    util::{args::unpack_stringable_arg, module_util::require_module},
};

static DEFAULT_ADDRESS: &str = "127.0.0.1";
static DEFAULT_PORT: i64 = 8000; // #todo what should be the default port?

// #see https://docs.rs/axum/latest/axum/response/index.html

// #todo support post method and body!
// #todo support redirects.

// #todo have option to server static files
// #todo have option to act as reverse proxy to the tan service.

// #todo find a better name.
// #todo use something from Axum.
pub type HandlerResponse = (StatusCode, HeaderMap, String);

fn internal_server_error_response(reason: &str) -> HandlerResponse {
    let mut header_map = HeaderMap::new();
    header_map.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        header_map,
        format!("internal server error: {reason}"),
    )
}

async fn run_server(options: HashMap<String, Expr>, handler: Expr, context: &mut Context) {
    // #todo #think should have separate context per thread? per task/fiber?
    let mut context = context.clone();

    let axum_handler = |axum_req: Request| async move {
        // #todo consider custom object, not map?
        // #todo handle POST body parsing.
        // #todo what else to pass to tan_req? (headers, method, ...)

        // Encode Tan Request.

        let mut map = HashMap::new();

        map.insert("uri".to_string(), Expr::string(axum_req.uri().to_string()));

        // parse headers.

        let mut tan_headers = HashMap::new();
        for (name, value) in axum_req.headers() {
            let value = String::from_utf8_lossy(value.as_bytes()).to_string();
            tan_headers.insert(name.to_string(), Expr::string(value));
        }
        map.insert("headers".to_string(), Expr::map(tan_headers));

        let method = axum_req.method().to_string();

        if method == "POST" {
            // #todo consider automatically decoding on :method POST, :content-type "application/x-www-form-urlencoded" -> better NO!

            // #todo think about the body limit here!
            let Ok(bytes) = to_bytes(axum_req.into_body(), usize::MAX).await else {
                return internal_server_error_response("invalid request body");
            };
            let Ok(body) = String::from_utf8(bytes.to_vec()) else {
                return internal_server_error_response("invalid request body");
            };

            map.insert("body".to_string(), Expr::string(body));
        }

        // #todo send headers
        // #todo parse form-encoded and JSON bodies.

        map.insert("method".to_string(), Expr::string(method));

        // #todo consider "/http/Request".
        let req = annotate_type(Expr::map(map), "http/Request");

        // #todo handle conversion of more return types.
        let result = invoke_func(&handler, vec![req], &mut context);

        match result {
            Ok(value) => {
                // Decode Tan Response.

                // #todo the handler should return a tuple (status, headers, body)
                // #todo add tan-side helpers to generate this tuple!
                // #todo set content type depending on output.
                // #todo currently we use an array for the tuple.

                // #insight elaborate handling of the value to avoid excessive cloning.

                let Some(rwlock) = value.as_array_consuming() else {
                    return internal_server_error_response("invalid response");
                };

                // #ai
                // #insight hack to take ownership of the Arc<RwLock> inner value.
                let dummy = Vec::new();
                let tuple = {
                    let mut write_guard = rwlock.write().unwrap();
                    std::mem::replace(&mut *write_guard, dummy)
                };

                let mut tuple = tuple.into_iter();

                // (status-code, headers, body)

                let Some(status_code) = tuple.next() else {
                    return internal_server_error_response("missing status-code");
                };

                let Some(status_code) = status_code.as_int() else {
                    return internal_server_error_response("invalid status-code");
                };

                let Ok(status_code) = StatusCode::from_u16(status_code as u16) else {
                    return internal_server_error_response("invalid status-code");
                };

                let Some(headers) = tuple.next() else {
                    return internal_server_error_response("missing headers");
                };

                let Some(headers) = headers.as_map() else {
                    return internal_server_error_response("invalid headers");
                };

                let mut header_map = HeaderMap::new();
                for (name, value) in headers.iter() {
                    let Some(value) = value.as_stringable() else {
                        return internal_server_error_response(&format!("invalid header `{name}`"));
                    };
                    let Ok(name) = HeaderName::try_from(name) else {
                        return internal_server_error_response(&format!("invalid header `{name}`"));
                    };
                    header_map.insert(name, value.parse().unwrap());
                }

                // #todo body can be optional, e.g. redirect response.
                // #todo support a Stream.
                let Some(body) = tuple.next() else {
                    return internal_server_error_response("missing body");
                };

                let Some(body) = body.as_stringable_consuming() else {
                    return internal_server_error_response("invalid body");
                };

                (status_code, header_map, body)
            }
            Err(error) => {
                // #todo report that the handler returned non-stringable response.
                // #todo should also log/trace or println?
                internal_server_error_response(&error.to_string())
            }
        }
    };

    let address = if options.contains_key("address") {
        if let Some(address) = options["address"].as_stringable() {
            address
        } else {
            DEFAULT_ADDRESS
        }
    } else {
        DEFAULT_ADDRESS
    };

    let port = if options.contains_key("port") {
        if let Some(port) = options["port"].as_int() {
            port
        } else {
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    };

    let addr = format!("{address}:{port}");

    // run it
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // #todo add some kind of tracing?
    // println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, axum_handler.into_make_service())
        .await
        .unwrap();
}

// #todo investigate the Go http-serve API.

// (http/serve {:port 8000} (Func [] "hello world!"))
pub fn http_serve(args: &[Expr], context: &mut Context) -> Result<Expr, Error> {
    // #todo consider other name instead of handler, e.g. `callback`.
    let [options, handler] = args else {
        return Err(Error::invalid_arguments(
            "`serve` requires `options` and `handler` arguments",
            None,
        ));
    };

    let Some(options) = options.as_map() else {
        return Err(Error::invalid_arguments(
            "`options` argument should be a Map",
            options.range(),
        ));
    };

    let Expr::Func(..) = handler.unpack() else {
        return Err(Error::invalid_arguments(
            "`handler` argument should be a Func",
            handler.range(),
        ));
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    // #todo can we remove the clones?
    rt.block_on(run_server(options.clone(), handler.clone(), context));

    // #insight never returns!
    Ok(Expr::Never)
}

// #todo what is a good name?
// #todo consider automatically decoding on :method POST, :content-type "application/x-www-form-urlencoded"
// #insight better don't decode automatically, avoid unnecessary magic.
pub fn read_form_urlencoded(args: &[Expr], _context: &mut Context) -> Result<Expr, Error> {
    let body = unpack_stringable_arg(args, 0, "body")?;
    let data: HashMap<_, _> = url::form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .map(|(k, v)| (k, Expr::string(v)))
        .collect();
    Ok(Expr::map(data))
}

pub fn import_lib_http_server(context: &mut Context) {
    let module = require_module("network/http/server", context);
    module.insert("serve", Expr::ForeignFunc(Arc::new(http_serve)));
    // #todo move to another namespace.
    // #todo what would be a good name?
    // #insight form-urlencoded is more accurate and actually different than urlencoded.
    module.insert(
        "read-form-urlencoded",
        Expr::ForeignFunc(Arc::new(read_form_urlencoded)),
    );
}
