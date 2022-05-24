// Like import from JS
use anchor_lang::prelude::*;

declare_id!("EpThCWvyS4JX28Sy1YuHcxMvbZ4vYWRw6EdvBdzVmRrC");

/**
 * pub mod tells us that this is a Rust "module" which is an easy way to define a
 * collection of functions and variables — kinda like a class if you know what that is.
 * And we call this module myepicproject. Within here we write a function start_stuff_off which takes something called a Context and outputs a ProgramResult.
 * You can see this function doesn't do anything except call Ok(()) which is just a Result type
 */
/**
 * This is how we tell our program, "Hey — everything in this little module below is our program that we want to create handlers for that other people can call". You'll see how this comes into play. But, essentially this lets us actually call our Solana program from our frontend via a fetch request. We'll be seeing this #[blah] syntax a few places.
 */
#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        // Note: We do &mut to get a "mutable reference" to base_account. When we do this it actually gives us the power to make changes to base_account. Otherwise, we'd simply be working w/ a "local copy" of base_account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            vote: 0,
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    // The function accepts an index param from the user. We also reference the user from the Context
    pub fn update_item(ctx: Context<UpdateItem>, gif_index: u32, vote: i32) -> Result<()> {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;

        // Add it to the gif_list vector.
        base_account.gif_list[gif_index as usize].vote += vote;
        Ok(())
    }
}

// Ref: https://buildspace.so/p/CObd6d35ce-3394-4bd8-977e-cbee82ae07a3/lessons/LE97b3e2e8-b4dd-4b2e-a52c-954c63bbc5da
// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Add the signer who calls the UpdateItem method to the struct so that we can save it
#[derive(Accounts)]
pub struct UpdateItem<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
// Like an object in JS
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub vote: i32
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account. (Something like an array)
    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub gif_list: Vec<ItemStruct>,
}
