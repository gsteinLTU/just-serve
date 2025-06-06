use axum::body::Body;
use axum::http::{HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get_service;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use axum::Router;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = r#"
just-serve
A simple HTTP server for serving static files without having to think about CORS.

USAGE: 
    just-serve [OPTIONS] [PATH]
    
OPTIONS:
    -p, --port <PORT>  Port to listen on (default: 8080)

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

ARGS:
    PATH             Path to serve files from (default: current directory)

"#;


#[derive(Debug)]
struct AppArgs {
    port: u16,
    dir: String,
}

#[tokio::main]
async fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        println!("{}", HELP);
        return;
    }

    if pargs.contains(["-V", "--version"]) {
        println!("just-serve {}", VERSION);
        return;
    }

    let args = AppArgs {
        port: match pargs.opt_value_from_str(["-p", "--port"]) {
            Ok(Some(port)) => port,
            Ok(None) => 8080,
            Err(e) => {
                eprintln!("Error: Invalid port number: {}", e);
                eprintln!("Port must be a number between 1 and 65535");
                std::process::exit(1);
            }
        },
        dir: pargs.free_from_str().unwrap_or_else(|_| ".".to_string()),
    };

    // Validate that the directory exists and is accessible
    let dir_path = std::path::Path::new(&args.dir);
    if !dir_path.exists() {
        eprintln!("Error: Directory '{}' does not exist", args.dir);
        std::process::exit(1);
    }

    if !dir_path.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.dir);
        std::process::exit(1);
    }

    // Canonicalize the directory path to get the absolute path
    let canonical_dir = match dir_path.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: Unable to access directory '{}': {}", args.dir, e);
            std::process::exit(1);
        }
    };

    println!("Serving directory: {}", canonical_dir.display());

    let serve_dir = ServeDir::new(&canonical_dir);

    let cors = CorsLayer::very_permissive();

    let app = Router::new()
        .fallback_service(get_service(serve_dir).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") }))
        .layer(cors)
        .layer(axum::middleware::from_fn(additional_cors));

    let addr = format!("0.0.0.0:{}", args.port);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AddrInUse => {
                    eprintln!("Error: Port {} is already in use", args.port);
                    std::process::exit(1);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied to bind to port {}", args.port);
                    eprintln!("Try using a port number above 1024, or run with elevated privileges");
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Error: Failed to bind to address {}: {}", addr, e);
                    std::process::exit(1);
                }
            }
        }
    };

    println!("Listening on {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Error: Failed to start server: {}", e);
        std::process::exit(1);
    }
}

// Additional CORS features are handled by this middleware
async fn additional_cors(req: Request<Body>, next: Next) -> Result<Response, axum::http::StatusCode> {
    let mut response = next.run(req).await;
    
    // Overwrite or insert the PNA header unconditionally
    response.headers_mut().insert(
        "Access-Control-Allow-Private-Network",
        HeaderValue::from_static("true"),
    );
    Ok(response)
}