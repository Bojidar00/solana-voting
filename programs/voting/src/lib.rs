use anchor_lang::prelude::*;

declare_id!("87WLQAHeR49beh5N1M7SCMyv9fvp2hkMSiwG88zqUXd9");



#[program]
pub mod voting {
    use super::*;
   

  
    
    pub fn create(ctx: Context<Create>, topic_: String, applications_deadline:i32, voting_deadline:i32) -> Result<()> {
        let topic = &mut ctx.accounts.vote_account;
        topic.topic=topic_;
        topic.options_count=0;
        topic.applications_deadline=Clock::get()?.unix_timestamp + (applications_deadline * 86400) as i64;
        topic.voting_deadline=Clock::get()?.unix_timestamp + (applications_deadline * 86400) as i64 + (voting_deadline * 86400) as i64;

        Ok(())
    }
    pub fn add_option(ctx: Context<AddOption>, option_name: String) -> Result<()> {
        let topic = &mut ctx.accounts.vote_account;
        if topic.applications_deadline < Clock::get()?.unix_timestamp{
            Err(VotingErr::ApplicationIsOver)?;
        }
        topic.options_count+=1;
        let option =&mut ctx.accounts.option_account;
        option.name=option_name;
        option.votes=0;
        option.id=topic.options_count;
        option.bump=*ctx.bumps.get("option_account").unwrap(); 
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>)-> Result<()>{
        let topic = &mut ctx.accounts.vote_account;
        if topic.applications_deadline > Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingNotStarted)?;
        }
        if topic.voting_deadline < Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingNotStarted)?;
        }

        let option =&mut ctx.accounts.option_account;
        option.votes+=1;
        let voter =&mut ctx.accounts.voter_account;
        voter.voted=true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 32 + 32)]
    pub vote_account: Account<'info, VoteTopic>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
    
}


#[derive(Accounts)]
pub struct AddOption<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteTopic>,
    #[account(init, payer = user, space = 32 + 32,seeds=[vote_account.key().as_ref(),&[vote_account.options_count +1]],bump)]
    pub option_account: Account<'info, VoteOption>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}


#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteTopic>,
    #[account(mut ,seeds=[vote_account.key().as_ref(),&[(option_account.id)]],bump=option_account.bump)]
    pub option_account: Account<'info, VoteOption>,
    #[account(init, payer=user,space=16 ,seeds=[vote_account.key().as_ref(),user.key().as_ref()],bump)]
    pub voter_account: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}


#[account]
pub struct VoteTopic {
pub topic: String,
pub options_count:u8,
pub applications_deadline:i64,
pub voting_deadline:i64,
}


#[account]
pub struct VoteOption {
pub id:u8,
pub name: String,
pub votes: u64,
pub bump: u8
}

#[account]
pub struct Voter {
pub voted:bool,
}


#[error_code]
pub enum VotingErr {
    #[msg("Application period is over!")]
    ApplicationIsOver,

    #[msg("Voting period has not started yet!")]
    VotingNotStarted,

    #[msg("Voting period is over!")]
    VotingIsOver,
}
