crate::ix!();

/// Fetches a YouTube URL from the clipboard if present and valid.
pub fn get_url_from_clipboard() -> Option<String> {
    let mut clipboard_context: ClipboardContext = ClipboardProvider::new().unwrap();
    let clipboard_contents = clipboard_context.get_contents().unwrap_or_else(|_| "".to_string());
    if is_youtube_url(&clipboard_contents) {
        println!("Using YouTube URL from clipboard.");
        Some(convert_to_embed_url(&clipboard_contents))
    } else {
        None
    }
}

/// Fetches a YouTube URL from the YOUTUBE_URL environment variable if set and valid.
pub fn get_url_from_env() -> Option<String> {
    env::var("YOUTUBE_URL").ok()
        .filter(|url| is_youtube_url(url))
        .map(|url| {
            println!("Using YouTube URL from environment variable.");
            convert_to_embed_url(&url)
        })
}

/// Checks if the given string is a valid YouTube URL.
pub fn is_youtube_url(url: &str) -> bool {
    url.contains("youtube.com/watch?v=") || url.contains("youtu.be/")
}

/// Converts a standard YouTube URL to an embeddable format.
pub fn convert_to_embed_url(url: &str) -> String {
    if url.contains("watch?v=") {
        url.replacen("watch?v=", "embed/", 1)
    } else if let Some(id) = url.split("youtu.be/").nth(1) {
        format!("https://www.youtube.com/embed/{}", id)
    } else {
        url.to_string()
    }
}

/// Determines which source to use for the YouTube URL, prioritizing the clipboard.
pub fn determine_youtube_url() -> Option<String> {
    get_url_from_clipboard()
        .or_else(get_url_from_env)
}

