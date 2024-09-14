// PATH: src/mods/text.rs
use rocket::response::content::RawText;
use scraper::{Html, Selector, ElementRef};
use csscolorparser::Color;
use std::collections::HashMap;
use regex::Regex;
use std::fs;

use rocket::State;
use crate::AppState;
use crate::context;
use crate::get_projects;

// curl zoa.sh
//convert CSS color to ANSI color code
fn css_color_to_ansi(color: &str) -> String {
    if let Ok(parsed_color) = Color::from_html(color) {
        let rgba = parsed_color.to_rgba8();
        format!("\x1b[38;2;{};{};{}m", rgba[0], rgba[1], rgba[2])
    } else {
        // Default to light gray if color parsing fails
        "\x1b[38;2;200;200;200m".to_string()
    }
}

// Improved CSS parser
fn parse_css(css_content: &str) -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let re = Regex::new(r"((?:\.|#|\w+)(?:[.#]\w+)*)\s*\{([^}]+)\}").unwrap();
    for cap in re.captures_iter(css_content) {
        let selector = cap[1].to_string();
        let properties = cap[2].to_string();
        if let Some(color) = properties.split(';')
            .find(|prop| prop.contains("color:"))
            .and_then(|prop| prop.split(':').nth(1))
            .map(|c| c.trim().to_string())
        {
            styles.insert(selector, css_color_to_ansi(&color));
        }
    }
    // Add default text color
    styles.insert("default".to_string(), css_color_to_ansi("#00ff00"));
    styles
}

fn html_to_formatted_text(html: &str, css_styles: &HashMap<String, String>) -> String {
    let document = Html::parse_document(html);
    let mut formatted_text = String::new();
    let body_selector = Selector::parse("body").unwrap();
    
    if let Some(body) = document.select(&body_selector).next() {
        format_element(&body, css_styles, &mut formatted_text, 0);
    }
    
    formatted_text
} 

fn format_element(element: &ElementRef, css_styles: &HashMap<String, String>, output: &mut String, depth: usize) {
    let tag_name = element.value().name();
    
    let style = element.value().attr("class")
        .and_then(|class| css_styles.get(&format!(".{}", class)))
        .or_else(|| css_styles.get(tag_name))
        .unwrap_or_else(|| css_styles.get("default").unwrap());

    let indent = "  ".repeat(depth);

    match tag_name {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
            let text = element.text().collect::<String>().trim().to_string();
            let header_color = "\x1b[38;2;139;92;246m"; // Purple color
            output.push_str(&format!("\n{}{}{}\x1b[1m{}\x1b[0m\n", indent, header_color, style, text));
        },
        "p" | "blockquote" | "dt" => {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                let formatting = if tag_name == "blockquote" { "\x1b[3m" } else { "" };
                output.push_str(&format!("{}{}{}{}\x1b[0m\n", indent, style, formatting, text));
            }
        },
        "ul" | "ol" => {
            for child in element.children() {
                if let Some(child) = ElementRef::wrap(child) {
                    if child.value().name() == "li" {
                        let content = child.text().collect::<String>().trim().to_string();
                        output.push_str(&format!("{}{}• {}\x1b[0m\n", indent, style, content));
                    } else {
                        format_element(&child, css_styles, output, depth + 1);
                    }
                }
            }
        },
        "dd" => {
            if let Some(ul) = element.select(&Selector::parse("ul").unwrap()).next() {
                for li in ul.select(&Selector::parse("li").unwrap()) {
                    let content = li.text().collect::<String>().trim().to_string();
                    output.push_str(&format!("{}{}• {}\x1b[0m\n", indent, style, content));
                }
            } else {
                let content = element.text().collect::<String>().trim().to_string();
                output.push_str(&format!("{}{}  {}\x1b[0m\n", indent, style, content));
            }
        },
        "a" => {
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                output.push_str(&format!("{}{}\x1b[4m{}\x1b[0m", indent, style, text));
            }
        },
        _ => {
            for child in element.children() {
                if let Some(child) = ElementRef::wrap(child) {
                    format_element(&child, css_styles, output, depth + 1);
                }
            }
        }
    }
}

#[get("/text")]
pub fn text_version(state: &State<AppState>) -> RawText<String> {
    let projects = get_projects();
    let context = context! {
        projects: projects,
        // text based parsing includes
    };

    let template_content = fs::read_to_string("templates/main_content.html.hbs")
        .expect("Failed to read template file");

    let mut reg = handlebars::Handlebars::new();
    reg.register_template_string("main_content", template_content)
        .expect("Failed to register template");
    let html = reg.render("main_content", &context)
        .expect("Failed to render template");

    let css_styles = parse_css(&state.minified_css);
    let formatted_text = html_to_formatted_text(&html, &css_styles);
    
    RawText(formatted_text)
}
