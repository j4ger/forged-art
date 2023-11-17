use leptos::*;

#[server(GetIdent, "/api", "GetCbor", "ident")]
pub async fn get_identicon(name: String) -> Result<String, ServerFnError> {
    use base64::{engine::general_purpose, Engine};
    use identicon::Identicon;
    use image::ImageOutputFormat;
    use md5::{Digest, Md5};
    use std::io::Cursor;

    let mut hasher = Md5::new();
    hasher.update(name.as_bytes());
    let name = hasher.finalize();

    let icon = Identicon::new(name.as_slice()).image();

    let buffer = Vec::new();
    let mut cursored_buffer = Cursor::new(buffer);

    // TODO: use WebP for better compression
    icon.write_to(&mut cursored_buffer, ImageOutputFormat::Png)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let result = general_purpose::STANDARD.encode(cursored_buffer.into_inner());

    Ok(result)
}
