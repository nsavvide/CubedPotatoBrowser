pub fn infer_request_type(url: &str) -> &str {
    let url = url.to_ascii_lowercase();
    if url.ends_with(".js") {
        "script"
    } else if url.ends_with(".css") {
        "stylesheet"
    } else if url.ends_with(".png") || url.ends_with(".jpg") || url.ends_with(".jpeg") || url.ends_with(".gif") || url.ends_with(".webp") {
        "image"
    } else if url.ends_with(".woff") || url.ends_with(".woff2") || url.ends_with(".ttf") {
        "font"
    } else if url.ends_with(".html") || url.ends_with(".htm") {
        "document"
    } else if url.contains("xhr") || url.contains("ajax") {
        "xhr"
    } else {
        "other"
    }
}

