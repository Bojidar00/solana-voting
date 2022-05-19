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
        topic.use_organisation=false;

        Ok(())
    }
    pub fn create_with_organisation(ctx: Context<Create>, topic_: String, applications_deadline:i32, voting_deadline:i32, organisation:Pubkey) -> Result<()> {
        let topic = &mut ctx.accounts.vote_account;
        topic.topic=topic_;
        topic.options_count=0;
        topic.applications_deadline=Clock::get()?.unix_timestamp + (applications_deadline * 86400) as i64;
        topic.voting_deadline=Clock::get()?.unix_timestamp + (applications_deadline * 86400) as i64 + (voting_deadline * 86400) as i64;
        topic.use_organisation=true;
        topic.organisation=organisation;
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

    pub fn create_organisation(ctx: Context<CreateOrganisation>,name:String)-> Result<()>{
        let organisation = &mut ctx.accounts.organisation_account;
        organisation.name=name;
        organisation.participants=0;
        organisation.authority= ctx.accounts.user.key();
        Ok(())
    }

    pub fn join_organisation(ctx: Context<JoinOrganisation>)-> Result<()>{
        let participant = &mut ctx.accounts.organisation_participant;
        participant.allowed_to_vote=false;
        let organisation = &mut ctx.accounts.organisation_account;
        organisation.participants+=1;
        Ok(())
    }

    pub fn allow_voting(ctx: Context<VotingRight>)-> Result<()>{
        let participant = &mut ctx.accounts.participant;
        participant.allowed_to_vote=true;
        Ok(())
    } 

    pub fn disallow_voting(ctx: Context<VotingRight>)-> Result<()>{
        let participant = &mut ctx.accounts.participant;
        participant.allowed_to_vote=false;
        Ok(())
    } 

    pub fn vote(ctx: Context<Vote>)-> Result<()>{
        let topic = &mut ctx.accounts.vote_account;
        if topic.applications_deadline > Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingNotStarted)?;
        }
        if topic.voting_deadline < Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingIsOver)?;
        } 
        if topic.use_organisation==false{
        let option =&mut ctx.accounts.option_account;
        option.votes+=1;
        let voter =&mut ctx.accounts.voter_account;
        voter.voted=true;}
        else{
            Err(VotingErr::OrganisationNeeded)?;
        }
        Ok(())
    }
    pub fn vote_with_organisation(ctx: Context<VoteWithOrganisation>)-> Result<()>{
        let topic = &mut ctx.accounts.vote_account;
        if topic.applications_deadline > Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingNotStarted)?;
        }
        if topic.voting_deadline < Clock::get()?.unix_timestamp{
            Err(VotingErr::VotingIsOver)?;
        } 
        if topic.use_organisation==true{
            let participant = &mut ctx.accounts.organisation_participant;
            if participant.allowed_to_vote==true{
                let option =&mut ctx.accounts.option_account;
                option.votes+=1;
                let voter =&mut ctx.accounts.voter_account;
                voter.voted=true;
            }else{
                Err(VotingErr::NoPermisson)?;
            }
        }else{
            Err(VotingErr::NoOrganisation)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 32 + 32 + 16)]
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
pub struct CreateOrganisation<'info>{
    #[account(init, payer = user, space = 32 + 32 + 32)]
    pub organisation_account: Account<'info, Organisation>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct JoinOrganisation<'info>{
    #[account(mut)]
    pub organisation_account: Account<'info, Organisation>,
    #[account(init, payer = user, space = 32 + 32 ,seeds=[organisation_account.key().as_ref(),user.key().as_ref()],bump)]
    pub organisation_participant: Account<'info, OrganisationParticipant>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct VotingRight<'info>{
    #[account(mut, has_one = authority)]
    pub organisation_account: Account<'info, Organisation>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub participant: Account<'info, OrganisationParticipant>,
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

#[derive(Accounts)]
pub struct VoteWithOrganisation<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteTopic>,
    #[account(mut ,seeds=[vote_account.key().as_ref(),&[(option_account.id)]],bump=option_account.bump)]
    pub option_account: Account<'info, VoteOption>,
    #[account(init, payer=user,space=16 ,seeds=[vote_account.key().as_ref(),user.key().as_ref()],bump)]
    pub voter_account: Account<'info, Voter>,
    #[account(mut, seeds=[vote_account.organisation.as_ref(),user.key().as_ref()],bump)]
    pub organisation_participant: Account<'info, OrganisationParticipant>,
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
pub use_organisation: bool,
pub organisation: Pubkey,
}

#[account]
pub struct Organisation{
    pub name: String,
    pub participants: u128,
    pub authority: Pubkey,
}

#[account]
pub struct OrganisationParticipant{
    pub allowed_to_vote:bool
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
pub voted:bool
}


#[error_code]
pub enum VotingErr {
    #[msg("Application period is over!")]
    ApplicationIsOver,

    #[msg("Voting period has not started yet!")]
    VotingNotStarted,

    #[msg("Voting period is over!")]
    VotingIsOver,

    #[msg("Please use 'vote' function instead of 'vote_with_organisation'!")]
    NoOrganisation,

    #[msg("You are not allowed to vote!")]
    NoPermisson,

    #[msg("Please use 'vote_with_organisation' function instead of 'vote'!")]
    OrganisationNeeded,
}

