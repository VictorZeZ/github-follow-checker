use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct GithubUser {
    login: String,
}

async fn get_users(
    client: &Client,
    username: &str,
    endpoint: &str,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut users_set = HashSet::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/{}?per_page=100&page={}",
            username, endpoint, page
        );

        let response = client
            .get(url)
            .header("User-Agent", "github-follow-checker")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(
                format!("GitHub API error: {}", response.status()).into()
            );
        }

        let users: Vec<GithubUser> = response.json().await?;

        if users.is_empty() {
            break;
        }

        for user in users {
            users_set.insert(user.login);
        }

        page += 1;
    }

    Ok(users_set)
}

async fn check_followers() -> Result<(), Box<dyn std::error::Error>> {
    print!("GitHub username: ");
    io::stdout().flush()?;

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    let username = username.trim();

    if username.is_empty() {
        println!("Username cannot be empty.\n");
        return Ok(());
    }

    println!("\nGetting GitHub data...\n");

    let client = Client::new();

    let followers = get_users(
        &client,
        username,
        "followers",
    )
    .await?;

    let following = get_users(
        &client,
        username,
        "following",
    )
    .await?;

    println!("================================");
    println!("You follow, but they don't follow you back:");
    println!("================================");

    let not_following_back: Vec<_> =
        following.difference(&followers).collect();

    if not_following_back.is_empty() {
        println!("None");
    } else {
        for user in not_following_back {
            println!("- {}", user);
        }
    }

    println!();

    println!("================================");
    println!("They follow you, but you don't follow them:");
    println!("================================");

    let not_followed_by_you: Vec<_> =
        followers.difference(&following).collect();

    if not_followed_by_you.is_empty() {
        println!("None");
    } else {
        for user in not_followed_by_you {
            println!("- {}", user);
        }
    }

    println!("\nDone.");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match check_followers().await {
            Ok(_) => {}
            Err(error) => {
                println!("\nError: {}\n", error);
            }
        }

        println!("--------------------------------");
        println!("Press Enter to check another account.");
        println!("Type 'q' and press Enter to exit.");
        print!("> ");

        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;

        if command.trim().eq_ignore_ascii_case("q")
            || command.trim().eq_ignore_ascii_case("exit")
        {
            println!("Goodbye!");
            break;
        }

        println!();
    }

    Ok(())
}