use axum::{extract::Multipart, http::header, response::IntoResponse, routing::post, Router};
use server::{error, Result};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    // Create server
    let app = Router::new().route("/resize", post(resize));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}

async fn resize(mut multipart: Multipart) -> Result<impl IntoResponse> {
    if let Some(field) = multipart.next_field().await? {
        // Load image
        let data = field.bytes().await?;
        let image = image::load_from_memory(&data)?;

        // Calculate new height maintaining aspect ratio
        let new_width = 256;
        let ratio = image.width() as f32 / new_width as f32;
        let new_height = (image.height() as f32 / ratio) as u32;
        let resized = image.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

        // Convert to bytes
        let mut bytes = Vec::new();
        let mut cursor = Cursor::new(&mut bytes);
        resized.write_to(&mut cursor, image::ImageFormat::Jpeg)?;
        Ok(([(header::CONTENT_TYPE, "image/jpeg")], bytes))
    } else {
        Err(error!("No file uploaded"))?
    }
}
