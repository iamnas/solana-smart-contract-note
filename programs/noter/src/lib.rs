use anchor_lang::prelude::*;

declare_id!("95Rg7EaTLp4tg9qqQNram7Ze9vFJdSRfRfN4GGN8Die8");

#[program]
pub mod noter {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let user = &mut ctx.accounts.user;

        note.content = content;
        note.user = *user.key;

        // msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // pub fn update_note(ctx: Context<UpdateNote>, content: String) -> Result<()> {
    //     let note = &mut ctx.accounts.note;
    //     // let user = &mut ctx.accounts.user;

    //     note.content = content;
    //     // note.user = *user.key;

    //     // msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }


    pub fn update_note(ctx: Context<UpdateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        // let user = &ctx.accounts.user;

        // Ensure the user is the one who created the note
        // if note.user != *user.key {
        //     // return err!(MyError::DataTooLarge);
        //     return err!(MyError::DataTooLarge);
        // }

        note.content = content;

        // msg!("Note updated by: {:?}", ctx.program_id);
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
        space = 8+ Note::INIT_SPACE // ginving thr sapace 
    )]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(
        mut,
        has_one = user,
        realloc = 8+ Note::INIT_SPACE,
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        has_one = user,
        close = user,
    )]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Note {
    #[max_len(100)]
    pub content: String,
    pub user: Pubkey,
}



// // Custom error codes
// #[error]
// pub enum ErrorCode {
//     #[msg("Unauthorized action.")]
//     Unauthorized,
// }
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized action.")]
    Unauthorized
}