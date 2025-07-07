use std::collections::HashMap;
use std::io::{self, Write};
use sha2::{Sha256, Digest};
use rand::Rng;

#[derive(Debug)]
struct Proposal {
    title: String,
    votes: Vec<String>, // Stores hashed commitments
}

fn hash_vote(vote: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(vote.as_bytes());
    hasher.update(salt.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() {
    let mut proposals: HashMap<String, Proposal> = HashMap::new();

    println!("🛡️ ShieldVote CLI - Simulate Confidential DAO Voting\n");

    loop {
        println!("Select an option:\n1. Create Proposal\n2. Commit Vote\n3. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                print!("Enter proposal title: ");
                io::stdout().flush().unwrap();

                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                proposals.insert(
                    title.clone(),
                    Proposal {
                        title,
                        votes: Vec::new(),
                    },
                );

                println!("✅ Proposal created.\n");
            }

            "2" => {
                if proposals.is_empty() {
                    println!("⚠️ No proposals available. Create one first.\n");
                    continue;
                }

                println!("Available proposals:");
                for (i, title) in proposals.keys().enumerate() {
                    println!("{}: {}", i + 1, title);
                }

                print!("Enter proposal number: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: usize = match num.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("❌ Invalid number.\n");
                        continue;
                    }
                };

                let selected_title = match proposals.keys().nth(num - 1) {
                    Some(t) => t.clone(),
                    None => {
                        println!("❌ Invalid selection.\n");
                        continue;
                    }
                };

                print!("Enter your vote (e.g., yes/no): ");
                io::stdout().flush().unwrap();
                let mut vote = String::new();
                io::stdin().read_line(&mut vote).unwrap();
                let vote = vote.trim();

                let salt: u64 = rand::thread_rng().gen();
                let salt_str = salt.to_string();
                let commitment = hash_vote(vote, &salt_str);

                println!("🔐 Your vote has been hashed and committed.");
                println!("🧂 Salt: {}", salt_str);
                println!("📦 Commitment (hash): {}\n", commitment);

                if let Some(proposal) = proposals.get_mut(&selected_title) {
                    proposal.votes.push(commitment);
                }
            }

            "3" => {
                println!("👋 Exiting ShieldVote CLI. Goodbye!");
                break;
            }

            _ => println!("❌ Invalid option.\n"),
        }
    }
}
