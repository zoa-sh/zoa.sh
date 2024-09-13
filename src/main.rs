#[macro_use] extern crate rocket;

use css_minify::optimizations::{Level, Minifier};
use rocket::fs::{FileServer, relative};
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket_dyn_templates::{Template, context};
use rocket::response::Redirect;
use rocket::request::{Outcome, FromRequest};
use serde::Serialize;
use std::net::IpAddr;
use std::fs;
use std::sync::Arc;
use rocket::State;
use rand::Rng;
use rocket::Data;

use rocket::response::content::RawText;
use scraper::{Html, Selector, ElementRef};
use csscolorparser::Color;
use std::collections::HashMap;
use regex::Regex;

// Caching
struct Cacher;
#[rocket::async_trait]
impl Fairing for Cacher {
    fn info(&self) -> Info {
        Info {
            name: "Cacher",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.adjoin_header(Header::new("Cache-Control", "max-age=31536000"));
    }
}

// Minify CSS
struct AppState {
    minified_css: Arc<String>,
}

fn init_app_state() -> AppState {
    let css_content = fs::read_to_string("static/css/styles.css")
        .expect("Failed to read CSS file");
    let minified_css = minify_css(&css_content);
    
    AppState {
        minified_css: Arc::new(minified_css),
    }
}

fn minify_css(input: &str) -> String {
    let mut minifier = Minifier::default();
    minifier.minify(input, Level::One).expect("Failed to minify CSS")
}

// GIT Projects
#[derive(Serialize)]
struct Project {
    name: String,
    description: String,
    tech: Vec<String>,
    repo: String,
}

// modify this to pull from git
fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "This site!".to_string(),
            description: "The site your looking at right now!".to_string(),
            tech: vec!["Rust".to_string(), "Rocket".to_string(), "Javascript".to_string(), "CSS".to_string(), "HTML".to_string()],
            repo: "zoa-sh".to_string(),
        },
        Project {
            name: "Project 2".to_string(),
            description: "Description of Project 2".to_string(),
            tech: vec!["Python".to_string(), "Django".to_string(), "PostgreSQL".to_string()],
            repo: "project-2".to_string(),
        },
        Project {
            name: "Project 3".to_string(),
            description: "Description of Project 3".to_string(),
            tech: vec!["Python".to_string(), "CSS".to_string()],
            repo: "project-3".to_string(),
        },
    ]
}

#[get("/redirect/<repo>")]
fn redirect(repo: &str) -> Redirect {
    let base_url = "https://git.zoa.sh/";
    Redirect::to(format!("{}{}.git", base_url, repo))
}



// MAIN
#[get("/")]
fn index(state: &State<AppState>) -> Template {
    let stars = generate_stars(100);
    let scripts = format!(
        "{}{}{}",
        include_str!("../static/js/title_animation.js"),
        include_str!("../static/js/open_image.js"),
        include_str!("../static/js/show_more_projects.js")
    );

    let css = state.minified_css.as_str();

    let projects = get_projects();

    Template::render("index", context! {
        stars: stars,
        scripts: scripts,
        css: css,
        projects: projects,
    })
}

// Randomly Generated Stars
fn generate_stars(count: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        format!(r#"<div class="star" style="left:{}%; top:{}%;"></div>"#, x, y)
    }).collect()
}

// zoa.sh/ip
struct ClientIp(IpAddr);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.client_ip() {
            Some(ip) => Outcome::Success(ClientIp(ip)),
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}
// move this to /net/ip later when u finish the nettools
#[get("/ip")]
fn ip(client_ip: ClientIp) -> String {
    format!("IP Address: {}", client_ip.0)
}


// curl zoa.sh
// MODIFY THIS FOR AUTO PARSING AND ADD COLORS
fn parse_css(css_content: &str) -> HashMap<String, String> {
    let mut styles = HashMap::new();
    let re = Regex::new(r"(\.\w+|\w+)\s*\{([^}]+)\}").unwrap();
    for cap in re.captures_iter(css_content) {
        let selector = cap[1].to_string();
        let properties = cap[2].to_string();
        if let Some(color) = properties.split(';')
            .find(|prop| prop.contains("color:"))
            .and_then(|prop| prop.split(':').nth(1))
            .map(|c| c.trim().to_string())
        {
            styles.insert(selector, color);
        }
    }
    // Add default text color for elements that don't have a specific color
    styles.insert("default".to_string(), "#32CD32".to_string());
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
        .map(|color| get_ansi_color(color))
        .unwrap_or_default();

    let indent = "  ".repeat(depth);

    match tag_name { // possibly need stacking formats 
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

fn get_ansi_color(color: &str) -> String {
    if let Ok(parsed_color) = Color::from_html(color) {
        let rgba = parsed_color.to_rgba8();
        if rgba[0] == 0 && rgba[1] == 0 && rgba[2] == 0 {
            // If the color is black, use light gray instead
            "\x1b[38;2;200;200;200m".to_string()
        } else {
            format!("\x1b[38;2;{};{};{}m", rgba[0], rgba[1], rgba[2])
        }
    } else {
        // Default to light gray if color parsing fails
        "\x1b[38;2;200;200;200m".to_string()
    }
}

#[get("/text")]
fn text_version(state: &State<AppState>) -> RawText<String> {
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

struct UserAgentFairing;

#[rocket::async_trait]
impl Fairing for UserAgentFairing {
    fn info(&self) -> Info {
        Info {
            name: "User-Agent Checker",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(user_agent) = request.headers().get_one("User-Agent") {
            if user_agent.contains("curl") {
                let path = request.uri().path(); // we need to modify this to parse /text for paths
                if path == "/" {
                    request.set_uri(request.uri().map_path(|_| "/text").unwrap());
                } else if path == "/ip" {
                    request.set_uri(request.uri().map_path(|_| "/ip").unwrap());
                }
            }
        }
    }
}


// Lift Off
#[launch]
fn rocket() -> _ {
    let app_state = init_app_state();

    rocket::build()
        .manage(app_state)
        .attach(Cacher)
        .attach(UserAgentFairing)
        .mount("/", routes![index, redirect, text_version, ip])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper(
                "length",
                Box::new(|h: &rocket_dyn_templates::handlebars::Helper,
                          _: &rocket_dyn_templates::handlebars::Handlebars,
                          _: &rocket_dyn_templates::handlebars::Context,
                          _: &mut rocket_dyn_templates::handlebars::RenderContext,
                          out: &mut dyn rocket_dyn_templates::handlebars::Output| -> rocket_dyn_templates::handlebars::HelperResult {
                    let param = h.param(0).and_then(|v| v.value().as_array());
                    if let Some(array) = param {
                        out.write(&array.len().to_string())?;
                    }
                    Ok(())
                })
            );
        }))
}
