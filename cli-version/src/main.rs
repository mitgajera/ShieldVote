use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_program,
};
use anchor_client::{Client, Cluster, Program};
use solana_sdk::commitment_config::CommitmentConfig;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rand::Rng;
use sha2::{Sha256, Digest};
use std::rc::Rc;
use std::str::FromStr;

use shieldvote_anchor::instruction;
use shieldvote_anchor::accounts;

#[derive(Parser)]
#[command(name = "ShieldVote CLI")]
#[command(about = "Confidential DAO voting system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        commit_deadline: i64,
        #[arg(long)]
        reveal_deadline: i64,
    },
    Commit {
        #[arg(long)]
        proposal: String,
        #[arg(long)]
        vote: String,
    },
    Reveal {
        #[arg(long)]
        proposal: String,
        #[arg(long)]
        vote: String,
        #[arg(long)]
        salt: String,
    },
}

fn get_program(wallet: &Keypair, program_id: &Pubkey) -> Program<Rc<Keypair>> {
    let wallet_clone = Keypair::from_bytes(&wallet.to_bytes()).unwrap();
    let client = Client::new_with_options(Cluster::Devnet, Rc::new(wallet_clone), CommitmentConfig::processed());
    client.program(*program_id).unwrap()
}

fn hash_vote(vote: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(vote.as_bytes());
    hasher.update(salt.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // ✅ Use the official keypair loader from Solana SDK
    let payer = read_keypair_file("D:\\Wd\\w3\\my-wallet\\my-keypair.json")
        .expect("Failed to read keypair file");

    let program_id = Pubkey::from_str("FNbsp7QAZe7gRu3osyVSn2ZmrGWCXJoe9X83STVngzMc")?;
    let program = get_program(&payer, &program_id);

    match cli.command {
        Commands::Create { title, commit_deadline, reveal_deadline } => {
            let proposal = Keypair::new();

            program
                .request()
                .accounts(accounts::CreateProposal {
                    proposal: proposal.pubkey(),
                    creator: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(instruction::CreateProposal {
                    title,
                    deadline_commit: commit_deadline,
                    deadline_reveal: reveal_deadline,
                })
                .signer(&proposal)
                .send()?;

            println!("✅ Proposal created: {}", proposal.pubkey());
        }

        Commands::Commit { proposal, vote } => {
            let proposal = Pubkey::from_str(&proposal)?;
            let salt: u64 = rand::thread_rng().gen();
            let salt_str = salt.to_string();
            let commitment = hash_vote(&vote, &salt_str);

            println!("🔐 Vote committed. Save this:");
            println!("   Salt: {}", salt_str);
            println!("   Commitment: {}", commitment);

            program
                .request()
                .accounts(accounts::CommitVote {
                    proposal,
                    voter: payer.pubkey(),
                })
                .args(instruction::CommitVote { commitment })
                .send()?;
        }

        Commands::Reveal { proposal, vote, salt } => {
            let proposal = Pubkey::from_str(&proposal)?;

            program
                .request()
                .accounts(accounts::RevealVote {
                    proposal,
                    voter: payer.pubkey(),
                })
                .args(instruction::RevealVote { vote, salt })
                .send()?;
        }
    }

    Ok(())
}
