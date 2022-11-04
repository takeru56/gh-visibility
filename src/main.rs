use std::env;
use serde_json::json;
use serde::{Deserialize};

static GITHUB_GRAPHQL_ENDPOINT: &str = "https://api.github.com/graphql";
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

struct Arguments {
    program_name: String,
    user_name: String,
    sub_command: String,
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
    login: String,
    name: String,
    repositories: Repos,
}

#[derive(Deserialize)]
struct Data {
    user: User,
}

#[derive(Deserialize)]
struct Response {
    data: Data,
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
    println!("Usage: {} USER_NAME COMMAND\n", program_name);
    println!("Command line interface to change visibility of GitHub repositories.\n");
    println!("Commands:");
    println!(" list       list repositories with visibility status");
    println!(" change     change visibility of the repository");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("invalid arguments.");
        print_usage(&args[0]);
        std::process::exit(1);
    }
    Arguments {
        program_name: args[0].clone(),
        user_name: args[1].clone(),
        sub_command: args[2].clone(),
    }
}

async fn list_repos(args: &Arguments) -> reqwest::Result<()>{
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
        .await
        .expect("failed to get response");

    let user = res.json::<Response>().await?.data.user;
    print_repos(&user.repositories.nodes);
    Ok(())
}

#[tokio::main]
async fn main() -> reqwest::Result<()>{
    let args = parse_args();
    if args.sub_command  == "list" {
        list_repos(&args).await?;
    }
    Ok(())
}
