# 🛡️ ShieldVote - Confidential DAO Voting with Commitment Schemes

ShieldVote is a Solana-based confidential voting system that enables anonymous and tamper-resistant DAO voting using commitment schemes. Built with Anchor, this project brings secure, verifiable, and privacy-preserving governance to decentralized communities.

---

## ✨ Why ShieldVote?

Most DAO voting systems lack privacy — votes are publicly visible and susceptible to social pressure. ShieldVote addresses this with a two-phase **commit-reveal** protocol:

1. **Commit Phase**: Users submit a cryptographic commitment (e.g., hash of vote + salt).
2. **Reveal Phase** *(optional)*: Users reveal vote + salt to verify the commitment.

This ensures:
- 🕵️ **Confidential Voting** — votes are hidden during the commit phase.
- ✅ **Verifiable Tallying** — commitments can be verified during reveal.
- 🛡️ **Censorship Resistance** — votes cannot be influenced by public knowledge.

---

## 🛠️ Tech Stack

- **Solana** + **Anchor** `v0.29.0` — smart contract framework
- **Rust CLI** — for simulation and local testing
- *(Optional)*: Future support for **ZK-proofs** or **Arcium-style encrypted ballots**

---

## 🧩 Architecture

```text
+-------------------+       +-------------------+
|    DAO Voter      |       |   Other Voters    |
+-------------------+       +-------------------+
|                   |       |                   |
|  Commit Hash      |       |  Commit Hash      |
|  (vote || salt)   |       |  (vote || salt)   |
|------------------->       |-------------------> 
|                   |       |                   
+--------v----------+       +--------v----------+
|   ShieldVote      |       |   ShieldVote      |
|   Anchor Program  |       |   Anchor Program  |
|   (Solana Devnet) |       |   (Solana Devnet) |
+-------------------+       +-------------------+
```

## 🚀 Getting Started

### Prerequisites
Make sure the following are installed:

✅ Rust  
✅ Solana CLI  
✅ Anchor CLI v0.29.0  
✅ Wallet keypair on Devnet (e.g., my-keypair.json)  

### Build and Deploy
```bash
anchor build
anchor deploy --provider.cluster devnet
```
⚠️ Ensure Anchor.toml points to the correct wallet and Devnet.

### Local Test (Optional)
```bash
anchor test
```

## 📁 Project Structure
```
ShieldVote/
├── cli-version/
│   └── src/main.rs              # CLI vote simulator
│
├── anchor-version/
│   └── programs/shieldvote_anchor/
│       └── src/lib.rs           # Anchor smart contract
│
├── Anchor.toml
├── Cargo.toml
└── README.md
```

## ✅ Features
- 🗳️ Initialize DAO proposals
- 🔐 Submit hashed vote commitments
