crate::ix!();

pub struct MyApp {
    youtube_url: Option<String>,
    webview:     Option<Pin<Box<WebView>>>,
}

impl Debug for MyApp {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyApp")
            .field("youtube_url", &self.youtube_url)
            .field("webview", &self.webview.as_ref().map(|_| "WebView [native resource]"))
            .finish()
    }
}

impl MyApp {

    pub fn new(youtube_url: Option<String>) -> Self {
        Self {
            youtube_url,
            webview: None,
        }
    }
}

impl epi::App for MyApp {

    fn setup(&mut self, _ctx: &CtxRef, frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {

        if let Some(url) = &self.youtube_url {

            self.webview = setup_webview(url);

            if self.webview.is_none() {
                eprintln!("Failed to initialize WebView with given URL.");
                frame.quit();  // Exit if the WebView can't be created
            }

        } else {
            eprintln!("No valid YouTube URL found in clipboard.");
            frame.quit();
        }
    }

    fn update(&mut self, _ctx: &CtxRef, frame: &epi::Frame) {
        /*
        if self.webview.is_some() {
            frame.quit();  // Exit after preparing the webview
        }
        */
    }

    fn name(&self) -> &str {
        "YouTube Fullscreen Launcher"
    }
}
