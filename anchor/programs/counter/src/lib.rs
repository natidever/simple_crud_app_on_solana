#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");


#[program]
pub mod crud {
    use super::*;

    pub fn initalize_journal(ctx:Context<CreateEntry>,title:String,content:String)->Result<()>{
        


        let journal_entry = & mut ctx.accounts.journal_entry;
         journal_entry.title=title;
         journal_entry.content = content;

         return Ok(());

    }


}








pub fn update_journal_entry(ctx:Context<UpdateJournalEntry>,_title:String,content:String)->Result<()>{
 
 let journal_entry = & mut ctx.accounts.journal_entry; 

 journal_entry.content=content;
 Ok(())

 
  

}

pub fn delete_journal_entry(_ctx:Context<DeleteJournalEntry>,_title:String)->Result<()>{

    Ok(())

}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteJournalEntry <'info>{
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        mut,
        seeds=[title.as_bytes(),owner.key().as_ref()],
        bump,
        close = owner
    
    )]
    pub journal_entry:Account<'info,JournalEntryState>,
    pub system_program:Program<'info,System>

}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateJournalEntry<'info>{
    // first we need to get the account to be update then we 
    // use this account through the context to update the account
    #[account(mut)]
     pub owner:Signer<'info>,

    #[account(
        mut,
        seeds=[title.as_bytes(),owner.key().as_ref()],
        bump,
        realloc=8+JournalEntryState::INIT_SPACE,
        realloc::zero=true,
        realloc::payer=owner,
    )]
    pub journal_entry:Account<'info,JournalEntryState>,
    pub system_program:Program<'info,System>
}















#[account]
#[derive(InitSpace)]
pub struct JournalEntryState{
    pub owner:Pubkey,
    #[max_len(20)]
    pub title:String,
    #[max_len(260)]
    pub content:String,
}


#[derive(Accounts)]
#[instruction(title:String)]

pub struct CreateEntry<'info> {

    #[account(mut)]
    pub owner:Signer< 'info>,
    #[account(
        init,
        payer = owner,
        space=8+JournalEntryState::INIT_SPACE,
        seeds=[title.as_bytes(),owner.key().as_ref()],
        bump
    )]
    pub journal_entry:Account<'info,JournalEntryState>,
    pub system_program :Program<'info,System>
    

}
