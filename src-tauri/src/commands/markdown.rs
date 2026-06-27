use pulldown_cmark::{Parser, Options, html};

#[tauri::command]
pub fn render_markdown(md: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&md, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
