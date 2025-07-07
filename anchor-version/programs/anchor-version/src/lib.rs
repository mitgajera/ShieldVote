use anchor_lang::prelude::*;

declare_id!("FNbsp7QAZe7gRu3osyVSn2ZmrGWCXJoe9X83STVngzMc");

#[program]
pub mod shieldvote_anchor {
    use super::*;

    pub fn initialize_proposal(ctx: Context<InitializeProposal>, title: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.creator = *ctx.accounts.creator.key;
        proposal.title = title;
        proposal.votes = Vec::new();
        Ok(())
    }

    pub fn submit_vote(ctx: Context<SubmitVote>, commitment: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.votes.push(commitment);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 64 + (32 * 100))]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[account]
pub struct Proposal {
    pub creator: Pubkey,
    pub title: String,
    pub votes: Vec<String>,
}
