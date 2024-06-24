use pry::*;
use pry::imports::*;

fn main() {
    // Path to your PDF file
    let path = "/Users/kleb/bethesda/work/repo/dev_util/xilinix-specs/pg383-aes.pdf";
    let doc = Document::load(path).expect("Failed to load PDF");

    // Iterate over pages using get_pages to get page object IDs
    let pages = doc.get_pages(); // Returns a BTreeMap<u32, ObjectId>
    for (page_number, object_id) in pages {
        println!("Processing Page: {}", page_number);

        // Retrieve the page dictionary using the object ID
        if let Some(Object::Dictionary(page_dict)) = doc.objects.get(&object_id) {
            let content = extract_text(&doc, page_dict);
            println!("Extracted Text:\n{}", content);
        }
    }
}

fn extract_text(doc: &Document, page: &Dictionary) -> String {
    let mut content = String::new();

    // Check if the page has 'Contents' and extract it
    if let Ok(contents) = page.get(b"Contents") {
        match contents {
            Object::Reference(reference) => {
                // Follow the reference to find the content stream
                if let Some(Object::Stream(stream)) = doc.objects.get(&reference) {
                    // Attempt to decode the stream into text
                    if let Ok(text) = String::from_utf8(stream.content.clone()) {
                        content.push_str(&text);
                    }
                }
            },
            _ => {} // Handle other types if necessary
        }
    }

    content
}
