use serde::Deserialize;
use std::{collections::HashSet, env, path::Path, process::Command};


#[derive(Deserialize, Debug)]
pub struct Repos {
    //total_count: i32,
    //incomplete_results: bool,
    items: HashSet<Repo>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Repo {
    id: usize,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
}

//const REPO: &str = "https://api.github.com/users/ilyes-guy/repos?visibility=all&per_page=1000";
const API: &str = "https://api.github.com/search/repositories?q=user:";

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 4 {
        println!("requires 3 args github_username github_oken ");
        std::process::exit(1)
    }

    let repos_path = &arguments[1];
    let github_username = &arguments[2];
    let github_token = &arguments[3];
    //let gitlab_token = &arguments[4];

    if !Path::new(repos_path).exists() {
        println!("path is wrong");
        std::process::exit(1)
    } else {
        let root = Path::new(repos_path);
        assert!(env::set_current_dir(&root).is_ok());
    }

    let mut repos = String::new();
    repos.push_str(API);
    repos.push_str(github_username);
    repos.push_str("&per_page=1000");

    let mut token_bearer = String::from("Bearer ");
    token_bearer.push_str(github_token);
    let client = reqwest::Client::new();
    let res = client
        .get(repos)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "random dudde")
        .header(
            "Authorization",
            token_bearer,
        )
        .send()
        .await;

    let mut all_repos: Vec<String> = Vec::new();
    match res {
        Ok(result) => {
            match result.json::<Repos>().await {
                Ok(result) => {
                    println!("you have: {:?} repos", result.items.len());
                    for repo in result.items {
                        println!("{}", repo.name);
                        all_repos.push(repo.name);
                    }
                }
                Err(..) => {
                    println!(
                        "*********************** maybe a wrong username *********************"
                    );
                }
            };
        }
        Err(..) => {
            println!("idk what the error");
        }
    };


    println!("starting to clone:");
    let mut cloned_repos: u32 = 0;
    for repo in all_repos {
        let mut repo_url: String = String::new();

        repo_url.push_str("https://");
        repo_url.push_str(&github_token);
        repo_url.push_str("@github.com/");
        repo_url.push_str(github_username);
        repo_url.push_str("/");
        repo_url.push_str(&repo);

        println!("{:?}", repo_url);

        let output = Command::new("git").arg("clone").arg(repo_url).output();

        println!("{:?}", output);

        match output {
            Ok(result) => {
                cloned_repos += 1;
                println!("{:?}", result);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    println!("{} repos were cloned", cloned_repos);
}
