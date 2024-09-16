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
    pub org: String,
}

// modify this to pull from git
pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "This site!".to_string(),
            description: "The site you're looking at right now!".to_string(),
            tech: vec![
                "Rust".to_string(),
                "Rocket".to_string(),
                "Javascript".to_string(),
                "CSS".to_string(),
                "HTML".to_string(),
            ],
            repo: "zoa.sh".to_string(),
            org: "zoa-sh".to_string(),
        },
        Project {
            name: "NyxPSI".to_string(),
            description: "Next-gen network protocol for reliable data transfer in lossy environments. Outperforms TCP/UDP in high packet loss scenarios.".to_string(),
            tech: vec!["Rust".to_string(),
                "UDP".to_string(),
                "FEC".to_string(),
            ],
            repo: "nyxpsi".to_string(),
            org: "nyxpsi".to_string(),
        },
    ]
}

#[get("/redirect/<org>/<repo>")]
pub fn redirect(org: &str, repo: &str) -> Redirect {
    let base_url = "https://github.com/";
    Redirect::to(format!("{}{}/{}", base_url, org, repo))
}
