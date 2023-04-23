use std::{
    env,
    path::Path,
    process::Command,
    collections::HashSet
};
use serde::{Deserialize};



#[derive(Deserialize, Debug)]
pub struct GithubUser {
    login: String,
    id: usize,
    url: String,
    #[serde(rename = "type")]
    ty: String,
    name: String,
    followers: usize
}
#[derive(Deserialize, Debug)]
pub struct Repos {
    total_count: i32,
    incomplete_results: bool,
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
const REPO: &str = "https://api.github.com/search/repositories?q=user:ilyes-guy&per_page=1000";



#[tokio::main]
async fn main() {


    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 4 {
        println!("requires 3 args path gitoken lab token");
        std::process::exit(1)
    }

    let repos_patth = &arguments[1];
    let github_token = &arguments[2];
    let gitlab_token = &arguments[3];


    let client = reqwest::Client::new();
    let res = client
        .get(REPO)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "random dudde")
        .header("Authorization", "Bearer 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")
        .send()
        .await;



    match res {
        Ok(result) => {
            match result.json::<Repos>().await {
                Ok(result) => {
                    println!("{:?}", result.items.len());                
                    for repo in result.items{
                        println!("{:?}", repo.name);                
                    }
                },
                Err(err) => {
                    println!("idk the result");
                    println!("{:?}", err);
                }
            };
        },
        Err(err) => {
            println!("idk what the error");
            println!("{:?}", err);
        }
    };







    println!("path doesnt exist {} ", &arguments[1] );
    println!("path doesnt exist {} ", &arguments[2] );
    println!("path doesnt exist {} ", &arguments[3] );


    let output = Command::new("git clone")
        .arg("Hello world")
        .output();




    match output{
        Ok(result) => {

        },
        Err(err) => {

        }
    }


}







