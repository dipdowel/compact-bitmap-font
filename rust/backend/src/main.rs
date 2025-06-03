use axum::{
    Router,
    routing::post,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use axum_extra::extract::multipart::Multipart;
use std::{
    io::{Cursor, Write},
    net::SocketAddr,
};
use tokio::net::TcpListener;
use zip::{ZipWriter, write::FileOptions};
use compiler::compile_font;

/// POST /upload — Accepts a PNG and a JSON file, returns a zip archive.
/// Responds with 400 Bad Request if either file is missing or invalid.
async fn upload(mut multipart: Multipart) -> Response {
    let mut png_data = None;
    let mut json_data = None;

    // Iterate over form fields
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        let content_type = field.content_type().unwrap_or("").to_string();
        let data = field.bytes().await.unwrap_or_default();

        match (name.as_str(), content_type.as_str()) {
            ("png", "image/png") => png_data = Some(data),
            ("json", "application/json") => json_data = Some(data),
            _ => {}
        }
    }

    // Validate both files are present
    let (png, json) = match (png_data, json_data) {
        (Some(p), Some(j)) => (p, j),
        _ => return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "text/plain; charset=utf-8")
            .body("Missing PNG or JSON file".into())
            .unwrap()
    };

    // Build zip archive in memory
    let mut buffer = Vec::new();
    {

        let json_str = match std::str::from_utf8(&json) {
            Ok(s) => s.to_string(),
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "text/plain")
                    .body("Invalid UTF-8 in JSON file".into())
                    .unwrap();
            }
        };


        let compiled = compile_font(&png, json_str, false);

        if compiled.is_err() {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "text/plain")
                .body("Failed to compile font from provided assets".into())
                .unwrap();
        }
        let compiled = compiled.unwrap();


        let png_data:Vec<u8> = compiled.font_sample_png_data
            .iter()
            .flat_map(|&num| num.to_ne_bytes()) // Convert each u32 to 4 bytes
            .collect();

 

        let cursor = Cursor::new(&mut buffer);
        let mut zip = ZipWriter::new(cursor);
        let opts: FileOptions<'_, ()> = FileOptions::default();




        zip.start_file( format!("{}.sample.png", compiled.file_name.clone()), opts).unwrap();
        zip.write_all(&png_data).unwrap();

        zip.start_file(compiled.file_name, opts).unwrap();
        zip.write_all(&compiled.cbf_binary_file_data).unwrap();

        zip.finish().unwrap();
    }

    // Return zipped file
    (
        StatusCode::OK,
        [("Content-Type", "application/zip")],
        buffer,
    ).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(upload));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
