use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger, HttpRequest};
use actix_files as fs;

async fn index(req: HttpRequest) -> Result<fs::NamedFile> {
    // Check if the request path is "/"
    if req.path() == "/" {
        // Serve the index.html file
        return Ok(fs::NamedFile::open("frontend/index.html")?);
    }
    // Serve the directory listing
    Ok(fs::NamedFile::open(".")?)
}

async fn search(_req: HttpRequest) -> HttpResponse {
    // Redirect to the search page
    HttpResponse::Found().set_header("location", "/frontend/search.html").finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start Actix Web server
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // Serve static files from the specified directories
            .service(fs::Files::new("/", "frontend").index_file("index.html"))
            .service(fs::Files::new("/backend", "backend"))
            .service(fs::Files::new("/wasm", "wasm"))
            // Define custom routes
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search))
    })
    .bind("127.0.0.1:8080")?
    .run();

    println!("Server running at http://127.0.0.1:8080");

    server.await
}
