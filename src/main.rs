use anyhow::{Context, Result};
use clap::Parser;
use git2::{Repository, Commit};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "wtf")]
#[command(author = "Your Name")]
#[command(version)]
#[command(about = "Explains Git repositories in plain language", long_about = None)]
struct Args {
    /// Path to the git repository
    #[arg(default_value = ".")]
    repo_path: PathBuf,

    /// Number of commits to analyze
    #[arg(short, long, default_value_t = 5)]
    num_commits: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

fn get_commit_details(commit: &Commit) -> Result<String> {
    let author = commit.author();
    let message = commit.message().unwrap_or("No commit message");
    let time = commit.time();
    let datetime = time.seconds();
    
    let details = format!(
        "Commit: {}\nAuthor: {}\nDate: {}\nMessage: {}",
        commit.id(),
        author.name().unwrap_or("Unknown"),
        datetime,
        message
    );
    
    Ok(details)
}

async fn get_plain_language_description(api_key: &str, content: &str, prompt: &str) -> Result<String> {
    let client = Client::new();
    
    println!("Sending request to OpenAI API...");
    
    let request = OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: content.to_string(),
            },
        ],
        temperature: 0.7,
    };
    
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    // Check if the response is successful
    if !response.status().is_success() {
        let error_text = response.text().await?;
        println!("OpenAI API error response: {}", error_text);
        return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
    }
    
    println!("Received successful response from OpenAI API");
    
    // Parse the response
    match response.json::<OpenAIResponse>().await {
        Ok(response_data) => {
            if let Some(choice) = response_data.choices.first() {
                Ok(choice.message.content.clone())
            } else {
                anyhow::bail!("No choices in OpenAI API response")
            }
        },
        Err(e) => {
            println!("Error parsing OpenAI API response: {}", e);
            Err(anyhow::anyhow!("Failed to parse OpenAI API response: {}", e))
        }
    }
}

async fn analyze_repository(args: Args) -> Result<()> {
    // Check if .env file is being loaded
    println!("Attempting to load .env file...");
    let env_result = dotenv();
    match env_result {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(e) => println!("Warning: Could not load .env file: {:?}", e),
    }
    
    // Check all possible environment variables
    println!("\nChecking environment variables:");
    for (key, value) in env::vars() {
        if key.contains("API") || key.contains("KEY") {
            let masked_value = if value.len() > 8 {
                format!("{}...{}", &value[..4], &value[value.len()-4..])
            } else {
                "[value too short]".to_string()
            };
            println!("Found environment variable: {} = {}", key, masked_value);
        }
    }
    
    // Read API key directly from .env file instead of using environment variables
    println!("\nReading API key directly from .env file...");
    let env_contents = std::fs::read_to_string(".env")
        .context("Failed to read .env file")?;
    
    let mut api_key = String::new();
    for line in env_contents.lines() {
        if line.starts_with("OPENAI_API_KEY=") {
            api_key = line.trim_start_matches("OPENAI_API_KEY=").to_string();
            let masked_key = if api_key.len() > 8 {
                format!("{}...{}", &api_key[..4], &api_key[api_key.len()-4..])
            } else {
                "[key too short]".to_string()
            };
            println!("Using API key from .env file: {}", masked_key);
            break;
        }
    }
    
    if api_key.is_empty() {
        return Err(anyhow::anyhow!("OPENAI_API_KEY not found in .env file"));
    }
    
    // Open the repository with improved error handling
    let repo = match Repository::open(&args.repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error: Failed to open Git repository at {:?}", args.repo_path);
            eprintln!("Make sure you're running this from a valid Git repository or specify a valid path with --repo-path");
            eprintln!("Detailed error: {}", e);
            return Err(anyhow::anyhow!("Repository not found"));
        }
    };
    
    // Get the latest commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    let mut commits = Vec::new();
    let mut commit_details = Vec::new();
    
    // Count available commits
    let commit_count = revwalk.count();
    
    // Reset revwalk to start from the beginning again
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    // Determine how many commits to analyze
    let num_to_analyze = std::cmp::min(args.num_commits, commit_count);
    
    if num_to_analyze == 0 {
        println!("No commits found in the repository.");
        return Ok(());
    }
    
    println!("Found {} commits, will analyze {}.", commit_count, num_to_analyze);
    
    for (i, oid) in revwalk.take(num_to_analyze).enumerate() {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let details = get_commit_details(&commit)?;
        
        println!("Analyzing commit {} of {}...", i + 1, num_to_analyze);
        commit_details.push(details);
        commits.push(commit);
    }
    
    // Get project description
    let readme_content = match repo.find_file("README.md") {
        Ok(content) => content,
        Err(_) => "No README.md found".to_string(),
    };
    
    let project_description_prompt = "You are an AI assistant that provides concise project descriptions. Based on the README content and other information provided, give a brief, clear description of what this project is about in plain English. Keep it under 100 words.";
    
    let project_description = get_plain_language_description(
        &api_key, 
        &readme_content, 
        project_description_prompt
    ).await?;
    
    // Get plain language commit descriptions
    let commit_prompt = "You are an AI assistant that explains git commits in plain language. For each commit, explain what changes were made in simple terms that anyone can understand. Focus on the practical impact of the changes rather than technical details.";
    
    let commit_descriptions = get_plain_language_description(
        &api_key,
        &commit_details.join("\n\n---\n\n"),
        commit_prompt
    ).await?;
    
    // Get detailed analysis of the last 5 edits
    // Only analyze file changes if there are multiple commits
    let edits_description = if commits.len() > 1 {
        let mut file_changes = Vec::new();
        for commit in &commits {
            if let Some(parent) = commit.parent(0).ok() {
                let diff = repo.diff_tree_to_tree(
                    Some(&parent.tree()?),
                    Some(&commit.tree()?),
                    None,
                )?;
                
                let mut diff_stats = String::new();
                diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                    diff_stats.push_str(&format!("{}", String::from_utf8_lossy(line.content())));
                    true
                })?;
                
                file_changes.push(diff_stats);
            }
        }
        
        let edits_prompt = "You are an AI assistant that explains code changes in plain language. For each edit, explain what was changed and why it might have been changed. Focus on the functional impact rather than listing every line change. Make it understandable to non-technical people.";
        
        get_plain_language_description(
            &api_key,
            &file_changes.join("\n\n---\n\n"),
            edits_prompt
        ).await?
    } else {
        "Repository has only one commit, so there are no previous versions to compare changes against.".to_string()
    };
    
    // Print results
    println!("\n=== PROJECT DESCRIPTION ===\n");
    println!("{}", project_description);
    
    println!("\n=== LAST {} COMMITS IN PLAIN LANGUAGE ===\n", args.num_commits);
    println!("{}", commit_descriptions);
    
    println!("\n=== DETAILED ANALYSIS OF RECENT EDITS ===\n");
    println!("{}", edits_description);
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    analyze_repository(args).await
}

// Helper trait to find files in a repository
trait RepositoryExt {
    fn find_file(&self, path: &str) -> Result<String>;
}

impl RepositoryExt for Repository {
    fn find_file(&self, path: &str) -> Result<String> {
        let head = self.head()?;
        let tree = head.peel_to_tree()?;
        
        let entry = tree.get_path(std::path::Path::new(path))?;
        let object = entry.to_object(self)?;
        let blob = object.as_blob().ok_or_else(|| anyhow::anyhow!("Not a blob"))?;
        
        let content = String::from_utf8_lossy(blob.content()).to_string();
        Ok(content)
    }
}