crate::ix!();

pub fn setup_webview(url: &str) -> Option<Pin<Box<WebView>>> {

    let title = "YouTube Video";
    let content = Content::Url(url.to_string());
    let (width, height) = (1920, 1080);

    println!(
        "Attempting to create WebView ({} x {}) with URL: {}, title: {}", 
        width, 
        height, 
        url, 
        title
    );

    match WebView::new(title, content, width, height, true, true) {

        Ok(webview) => {
            println!("WebView created successfully.");
            Some(Box::pin(webview))
        },
        Err(e) => {
            eprintln!("Failed to create WebView: {:?}", e);
            None
        }
    }
}

