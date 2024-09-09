use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use tera::{Tera, Context};
use pulldown_cmark::{Parser, html};
use crate::generator::front_matter::extract_front_matter;

pub fn render_site() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    for entry in WalkDir::new("content").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let content = fs::read_to_string(entry.path()).expect("Unable to read file");

            // Extract front matter and content
            let (front_matter, content) = extract_front_matter(&content);

            // Parse markdown content to HTML
            let parser = Parser::new(content);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);

            let mut context = Context::new();
            context.insert("content", &html_content);

            // Insert front matter into context
            if let Some(ref fm) = front_matter {
                if let Some(title) = &fm.title {
                    context.insert("title", &title);
                } else {
                    context.insert("title", "Default Title");
                }
                if let Some(date) = &fm.date {
                    context.insert("date", &date);
                } else {
                    context.insert("date", "No Date Provided");
                }
            } else {
                context.insert("title", "Default Title");
                context.insert("date", "No Date Provided");
            }

            // Determine the template to use
            let template_name = front_matter
                .and_then(|fm| fm.template)
                .unwrap_or_else(|| "template.html".to_string());

            let output = tera.render(&template_name, &context).expect("Unable to render template");
            let output_path = format!("output/{}", entry.path().file_name().unwrap().to_str().unwrap().replace(".md", ".html"));
            fs::create_dir_all(Path::new(&output_path).parent().unwrap()).expect("Unable to create directories");
            fs::write(output_path, output).expect("Unable to write file");
        }
    }
}