use anchor_lang::prelude::*;

declare_id!("87WLQAHeR49beh5N1M7SCMyv9fvp2hkMSiwG88zqUXd9");



#[program]
pub mod voting {
    use super::*;
   

  

    pub fn create(ctx: Context<Create>, topic_: String, option1: String, option2: String) -> Result<()> {
        let topic = &mut ctx.accounts.vote_account;
        topic.topic=topic_;
        topic.option1=option1;
        topic.option2=option2;
        topic.option1_count=0;
        topic.option2_count=0;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option: u8)-> Result<()>{
        let topic = &mut ctx.accounts.vote_account;
        if option == 1 {
            topic.option1_count+=1;
        }else if option == 2{
            topic.option2_count+=1;
        }else {
            return Err(error!(OptionErr::WrongOption));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 32 + 32)]
    pub vote_account: Account<'info, Topic>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
    
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, Topic>
}


#[account]
pub struct Topic {
pub topic: String,
pub option1: String,
pub option2: String,
pub option1_count: u64,
pub option2_count: u64,
}

#[error_code]
pub enum OptionErr {
    #[msg("Wrong option!")]
    WrongOption,
}
