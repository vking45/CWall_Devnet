use crate::errors::WallErrors;
use anchor_lang::prelude::*;
use crate::state::wall::*;
use solana_program::pubkey::Pubkey;

pub const DAO_ADDRESS : Pubkey = solana_program::pubkey!("HJHp2ax4Y6UQ2bn2a4LJqEjmyzqrbLpZvq5GuuZHZwJn");

pub fn change_title(ctx : Context<ChangeWallContent>, new_title : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_title(new_title)?;
    Ok(())
}

pub fn change_desc(ctx : Context<ChangeWallContent>, new_desc : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_description(new_desc)?;
    Ok(())
}

pub fn change_art1(ctx : Context<ChangeWallContent>, new_art : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_art1(new_art)?;
    Ok(())
}

pub fn change_art2(ctx : Context<ChangeWallContent>, new_art : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_art2(new_art)?;
    Ok(())
}

pub fn change_art3(ctx : Context<ChangeWallContent>, new_art : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_art3(new_art)?;
    Ok(())
}

pub fn change_art4(ctx : Context<ChangeWallContent>, new_art : String) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_art4(new_art)?;
    Ok(())
}

pub fn transfer_auth(ctx : Context<ChangeWallContent>, new_auth : Pubkey) -> Result<()> {
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.transfer_auth(new_auth)?;
    Ok(())
}

pub fn change_state_landscape(ctx: Context<ChangeWallContent>) -> Result<()>{
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_state_landscape()?;
    Ok(())
}

pub fn change_state_portrait(ctx: Context<ChangeWallContent>) -> Result<()>{
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_state_portrait()?;
    Ok(())
}

pub fn change_state_square(ctx: Context<ChangeWallContent>) -> Result<()>{
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), ctx.accounts.authority.key(), WallErrors::WallAuthorityError);
    wall.change_state_square()?;
    Ok(())
}

pub fn content_mod_true(ctx: Context<ChangeWallContent>) -> Result<()>{
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), DAO_ADDRESS, WallErrors::DAOApprovalError);
    wall.content_mod_true()?;
    Ok(())
}

pub fn content_mod_false(ctx: Context<ChangeWallContent>) -> Result<()>{
    let wall : &mut Account<Wall> = &mut ctx.accounts.wall;
    require_keys_eq!(wall.get_authority(), DAO_ADDRESS, WallErrors::DAOApprovalError);
    wall.content_mod_false()?;
    Ok(())
}

#[derive(Accounts)]
pub struct ChangeWallContent<'info>{
    #[account(mut)]
    pub wall : Account<'info, Wall>,
    pub authority : Signer<'info>,
}
