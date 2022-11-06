use std::{env, error::Error, io::{Write, stderr}};
use serde_json::json;
use serde::{Deserialize};

static GITHUB_GRAPHQL_ENDPOINT: &str = "https://api.github.com/graphql";
static GITHUB_REST_ENDPOINT: &str = "https://api.github.com";
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(Debug)]
struct Arguments {
    program_name: String,
    user_name: String,
    sub_command: String,
    repo_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Node {
    name: String,
    visibility: String,
    description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Repos {
    total_count: i32,
    nodes: Vec<Node>,
}

#[derive(Deserialize)]
struct User {
    repositories: Repos,
}

#[derive(Deserialize)]
struct Data {
    user: User,
}

#[derive(Deserialize)]
struct GetResponse {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct PatchResponse {
    name: String,
    private: bool,
}

// #[derive(Debug, Clone)]
// struct ArgErr {
//     message: String,
//     line: useze,
//     column: usize,
// }

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn print_repos(nodes: &Vec<Node>) {
    let name_length = nodes.iter().map(|node|node.name.len()).max().unwrap_or(10);
    let visibility_length = 10;
    let description_length = 50;

    println!(" {:<name_length$} {:<visibility_length$} {:<description_length$}", "repo_name", "visibility", "description");
    println!("-{}-{}-{}-", "=".repeat(name_length), "=".repeat(visibility_length), "=".repeat(description_length));
    for repo in nodes {
        println!("|{:<name_length$}|{:<visibility_length$}|{:<description_length$}", repo.name, repo.visibility, repo.description.as_ref().unwrap_or(&"".to_string()));
    }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} COMMAND USER_NAME\n", program_name);
    println!("Command line interface to change visibility of GitHub repositories.\n");
    println!("Commands:");
    println!(" repos      list repositories with visibility status");
    println!(" change     change visibility of the repository");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("invalid arguments.");
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let mut repo_list: Option<Vec<String>> = None;
    if args.len() > 3 {
        repo_list = Some(args[3..].iter().map(|s| s.to_string()).collect());
    }

    Arguments {
        program_name: args[0].clone(),
        sub_command: args[1].clone(),
        user_name: args[2].clone(),
        repo_list: repo_list,
    }
}

async fn change_visibility(args: &Arguments) -> reqwest::Result<()> {
    if args.repo_list.is_none() {
        println!("invalid arguments.");
        println!("Usage: {} change USER_NAME REPO_NAME:(private|public) ...", args.program_name);
        std::process::exit(1);
    }

    for repo in args.repo_list.as_ref().unwrap() {
        let status: Vec<&str> = repo.split(':').collect();
        let body = json!({"visibility": status[1]});

        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let res = client.post(format!("{}/repos/{}/{}", GITHUB_REST_ENDPOINT, args.user_name, status[0]))
            .basic_auth(&args.user_name, Some(env!("GITHUB_AUTH_TOKEN")))
            .json(&body)
            .send()
            .await?;

        match res.json::<PatchResponse>().await {
            Err(_) => println!("failed to change visibility of {}", status[0]),
            Ok(result) => println!("{:?} is now private: {}", result.name, result.private),
        }
    }

    Ok(())
}

async fn list_repos(args: &Arguments) -> reqwest::Result<()> {
    let val = "{
        user(login: \"{USERNAME}\") {
            login
            name
            repositories(first: 100){
                totalCount
                nodes {
                    name
                    visibility
                    description
                }
            }
        }
    }".replace("{USERNAME}", &args.user_name);

    let query = json!({"query": val});

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let res = client.post(GITHUB_GRAPHQL_ENDPOINT)
        .basic_auth(&args.user_name, Some(env!("GITHUB_AUTH_TOKEN")))
        .json(&query)
        .send()
        .await?;

    let user = res.json::<GetResponse>().await?.data.user;
    print_repos(&user.repositories.nodes);
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    if args.sub_command  == "repos" {
        if let Err(err) = list_repos(&args).await {
            print_error(&err);
            std::process::exit(1);
        }
    } else if args.sub_command == "change" {
        if let Err(err) = change_visibility(&args).await {
            print_error(&err);
            std::process::exit(1);
        }
    } else {
        print_usage(&args.program_name);
    }
}
