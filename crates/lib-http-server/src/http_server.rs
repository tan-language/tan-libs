use std::{collections::HashMap, sync::Arc};

use axum::{
    body::to_bytes,
    extract::Request,
    handler::HandlerWithoutStateExt,
    http::{header, HeaderMap, HeaderName, StatusCode},
    routing::any,
    Router,
};
use mime_guess::from_path;
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
// #todo should be able to use `impl IntoResponse` instead.
pub type HandlerResponse = (StatusCode, HeaderMap, Vec<u8>);

fn internal_server_error_response(reason: &str) -> HandlerResponse {
    let mut header_map = HeaderMap::new();
    header_map.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        header_map,
        format!("internal server error: {reason}").into_bytes(),
    )
}

// #todo find a better name.
async fn tan_request_from_axum_request(axum_req: Request) -> Result<Expr, String> {
    // #todo consider custom object, not map?
    // #todo what else to pass to tan_req? (headers, method, ...)

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
            return Err(String::from("invalid request body"));
        };
        let Ok(body) = String::from_utf8(bytes.to_vec()) else {
            return Err(String::from("invalid request body"));
        };

        map.insert("body".to_string(), Expr::string(body));
    }

    // #todo send headers
    // #todo parse form-encoded and JSON bodies.

    map.insert("method".to_string(), Expr::string(method));

    // #todo consider "/http/Request".
    Ok(annotate_type(Expr::map(map), "http/Request"))
}

// #todo find a better name.
fn axum_response_from_tan_response(tan_resp: Expr) -> HandlerResponse {
    // #todo the handler should return a tuple (status, headers, body)
    // #todo add tan-side helpers to generate this tuple!
    // #todo set content type depending on output.
    // #todo currently we use an array for the tuple.

    // #insight elaborate handling of the value to avoid excessive cloning.

    let Some(rwlock) = tan_resp.as_array_consuming() else {
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

    (status_code, header_map, body.into_bytes())
}

async fn run_server(options: HashMap<String, Expr>, handler: Expr, context: &mut Context) {
    // #todo #think should have separate context per thread? per task/fiber?
    let mut context = context.clone();

    // #todo #hack temp solution, proper parsing needed!
    // #todo provide option to config static-files directory (e.g. public)
    let static_files_dir = "./public";
    let serve_static_files = options.contains_key("serve-static-files");

    let axum_handler = move |axum_req: Request| async move {
        if serve_static_files {
            // #todo #temp if the path contains a dot (i.e. has extension) serve as static file.
            // #todo for more robust solution, make the exact pattern configurable.
            // #todo alternatively (and additionally) try to serve a static file if the tan handler returns a 404.
            let path = axum_req.uri().path();
            if path.contains('.') {
                // let serve_file = ServeFile::new(axum_req.uri().path());
                // return serve_file;

                // #todo _really_ nasty code, use tower's ServeFile instead.
                let path = format!("{static_files_dir}{path}");
                if let Ok(file_contents) = tokio::fs::read(&path).await {
                    let mime_type = from_path(&path).first_or_octet_stream(); // Guess MIME type
                    let mut header_map = HeaderMap::new();
                    header_map.insert(header::CONTENT_TYPE, mime_type.to_string().parse().unwrap());
                    return (StatusCode::FOUND, header_map, file_contents);
                } else {
                    return (
                        StatusCode::NOT_FOUND,
                        HeaderMap::new(),
                        "File not found".to_string().into_bytes(),
                    );
                }
            }
        }

        let tan_req = tan_request_from_axum_request(axum_req).await;

        if let Err(reason) = tan_req {
            return internal_server_error_response(&reason);
        }

        let tan_req = tan_req.unwrap();

        // #todo handle conversion of more return types.
        let result = invoke_func(&handler, vec![tan_req], &mut context);

        match result {
            Ok(tan_resp) => axum_response_from_tan_response(tan_resp),
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

    if serve_static_files {
        // #todo this is very hackish, implement properly!
        // #todo make the catch-all pattern configurable!

        // #insight
        // Originally I tried a `static/*` prefix for asset files. This would
        // force all static files to be inside a static sub-directory and would
        // not play well with 'well-known' files like robots.txt, favicon.ico,
        // etc. It was _not_ a good idea.

        let router = Router::new()
            .route("/", any(axum_handler.clone()))
            .route("/*path", any(axum_handler));

        // #todo add handle error?
        // .handle_error(error_handler));

        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    } else {
        axum::serve(listener, axum_handler.into_make_service())
            .await
            .unwrap();
    };
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
