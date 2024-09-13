// PATH: src/mods/projects.rs
use rocket::response::Redirect;
use serde::Serialize;

// GIT Projects
#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub tech: Vec<String>,
    pub repo: String,
}

// modify this to pull from git
pub fn get_projects() -> Vec<Project> {
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
pub fn redirect(repo: &str) -> Redirect {
    let base_url = "https://git.zoa.sh/";
    Redirect::to(format!("{}{}.git", base_url, repo))
}
