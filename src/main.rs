// PATH: src/main.rs
#[macro_use] extern crate rocket;

mod mods;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use rocket::State;

use mods::cache::Cacher;
use mods::state::{AppState, init_app_state};
use mods::projects::{redirect, get_projects};
use mods::net::ip;
use mods::text::text_version;
use mods::agent::UserAgentFairing;
use mods::dynamic::generate_stars;

// Main
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

// Lift Off
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(init_app_state())
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
        }
    ))
}
