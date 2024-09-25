use anchor_lang::prelude::*;

declare_id!("95Rg7EaTLp4tg9qqQNram7Ze9vFJdSRfRfN4GGN8Die8");

#[program]
pub mod noter {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>,content : String) -> Result<()> {

        let note = &mut ctx.accounts.note;
        let user = &mut ctx.accounts.user;

        note.content = content;
        note.user = *user.key;

        // msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn delete_note(_ctx: Context<DeleteNote>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(
        init,
        payer = user, //payer 
        space = 2000 // ginving thr sapace 
    )]
    pub note: Account<'info,Note>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        has_one = user,
        close = user,
    )]
    pub note: Account<'info,Note>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Note{
        pub content: String,    
        pub user: Pubkey,
}