use anchor_lang::prelude::*;

declare_id!("FNbsp7QAZe7gRu3osyVSn2ZmrGWCXJoe9X83STVngzMc");

const MAX_VOTES: u32 = 5; // Reduced to 5 for smaller size

#[program]
pub mod shieldvote_anchor {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        deadline_commit: i64,
        deadline_reveal: i64,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.creator = *ctx.accounts.creator.key;
        proposal.title = title;
        proposal.deadline_commit = deadline_commit;
        proposal.deadline_reveal = deadline_reveal;
        proposal.commitment_count = 0;
        proposal.reveal_count = 0;
        
        // Initialize arrays with default values
        for i in 0..MAX_VOTES as usize {
            proposal.commitments[i] = Commitment::default();
            proposal.reveals[i] = Reveal::default();
        }
        
        Ok(())
    }

    pub fn commit_vote(ctx: Context<CommitVote>, commitment: String) -> Result<()> {
        let clock = Clock::get()?;
        let proposal = &mut ctx.accounts.proposal;

        require!(
            clock.unix_timestamp <= proposal.deadline_commit,
            CustomError::CommitPhaseEnded
        );

        require!(
            proposal.commitment_count < MAX_VOTES,
            CustomError::TooManyVotes
        );

        let index = proposal.commitment_count as usize;
        proposal.commitments[index] = Commitment {
            voter: ctx.accounts.voter.key(),
            commitment,
        };
        proposal.commitment_count += 1;

        Ok(())
    }

    pub fn reveal_vote(ctx: Context<RevealVote>, vote: String, salt: String) -> Result<()> {
        let clock = Clock::get()?;
        let proposal = &mut ctx.accounts.proposal;

        require!(
            clock.unix_timestamp > proposal.deadline_commit
                && clock.unix_timestamp <= proposal.deadline_reveal,
            CustomError::NotRevealPhase
        );

        require!(
            proposal.reveal_count < MAX_VOTES,
            CustomError::TooManyVotes
        );

        let index = proposal.reveal_count as usize;
        proposal.reveals[index] = Reveal {
            voter: ctx.accounts.voter.key(),
            vote,
            salt,
        };
        proposal.reveal_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = 8 + Proposal::SIZE)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CommitVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevealVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub title: String,
    pub deadline_commit: i64,
    pub deadline_reveal: i64,
    pub commitment_count: u32,
    pub reveal_count: u32,
    pub commitments: [Commitment; MAX_VOTES as usize],
    pub reveals: [Reveal; MAX_VOTES as usize],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Commitment {
    pub voter: Pubkey,
    pub commitment: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Reveal {
    pub voter: Pubkey,
    pub vote: String,
    pub salt: String,
}

impl Proposal {
    // Conservative calculation with extra buffer for serialization
    pub const SIZE: usize = 
        32 +                        // creator
        4 + 64 +                    // title (reduced size)
        8 +                         // deadline_commit
        8 +                         // deadline_reveal
        4 +                         // commitment_count
        4 +                         // reveal_count
        (32 + 4 + 64) * 5 +         // commitments array (5 entries, 64 bytes per string)
        (32 + 4 + 32 + 4 + 32) * 5 + // reveals array (5 entries, 32 bytes per string)
        200;                        // Extra buffer for Borsh serialization
}

#[error_code]
pub enum CustomError {
    #[msg("Commit phase has ended.")]
    CommitPhaseEnded,
    #[msg("Not in reveal phase.")]
    NotRevealPhase,
    #[msg("Too many votes for this proposal.")]
    TooManyVotes,
}
