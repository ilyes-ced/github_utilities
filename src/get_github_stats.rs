use serde::Deserialize;
use std::{collections::{HashSet, HashMap, hash_map::Entry}, env};

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


#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct AuthorData{
    name: String,
    email: String,
    date: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct Author{
    author: AuthorData,
}



#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct Commit{
    sha: String,
    node_id: String,
    commit: Author,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct Sha{
    sha: String,
}


#[derive(Deserialize, Debug, PartialEq, Eq)]
struct Commits(HashSet<Commit>);


//const REPO: &str = "https://api.github.com/users/ilyes-guy/repos?visibility=all&per_page=1000";
const API: &str = "https://api.github.com/search/repositories?q=user:";
//const API: &str = "https://api.github.com/repos/ilyes-guy";




#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("requires 3 args github_username github_oken ");
        std::process::exit(1)
    }




    let github_username = &arguments[1];
    let github_token = &arguments[2];

    println!("{:?}", github_username);
    println!("{:?}", github_token);





    let mut repos = String::new();
    repos.push_str(API);
    repos.push_str(github_username);
    repos.push_str("&per_page=1000");



    let client = reqwest::Client::new();
    let mut token_bearer = String::from("Bearer ");
    token_bearer.push_str(github_token);

    
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
                    println!("{:?}", result.items.len());
                    for repo in result.items {
                        println!("{:?}", repo.name);
                        all_repos.push(repo.name);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);   
                    println!(
                        "*********************** maybe a wrong username or token or both *********************"
                    );
                }
            };
        }
        Err(..) => {
            println!("maybe internet connection bad");
        }
    };




    //let mut all_repos: Vec<String> = Vec::new();
    //all_repos.push(String::from("slack_clone"));


    let mut repo_commits: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();

    let mut token_bearer = String::from("Bearer ");
    token_bearer.push_str(github_token);

    for repo in &all_repos{
        let mut repo_url = String::new();
        repo_url.push_str("https://api.github.com/repos/");
        repo_url.push_str(github_username);
        repo_url.push_str("/");
        repo_url.push_str(&repo);
        repo_url.push_str("/commits");
        repo_url.push_str("?per_page=100");
        repo_url.push_str("&author=");
        repo_url.push_str(&github_username);
        println!("{:?}", repo_url);   

        let res = client
        .get(repo_url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "random dudde")
        .header(
            "Authorization",
            &token_bearer,
        )
        .send()
        .await;




        match res {
            Ok(result) => {
                match result.json::<Commits>().await {
                    Ok(result) => {
                        println!("{:?}", result.0.len());   
                        for commit in result.0{
                            let author_data = commit.commit.author;
                            if &author_data.name == github_username {
                                println!("{:?}", author_data);
                                //let values = match repo_commits.entry(repo.to_string()) {
                                //    Entry::Occupied(o) => o.into_mut(),
                                //    Entry::Vacant(v) => v.insert(HashMap::from([(author_data.date.clone(), HashSet::new())])),
                                //};
                                //commit.sha in the hashset

                                repo_commits
                                    .entry(repo.to_string()).or_default()
                                    .entry(author_data.date).or_default().insert(commit.sha);
                                
                               
                                


                            }
                        };
                        println!("{:?}", repo_commits);
                    },
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
            },
        }
    }








}

