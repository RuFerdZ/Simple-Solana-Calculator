use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;
   
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        // & means that the calculator in context is passed by reference
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {   // this includes all accounts that is associated with the Create context
    #[account(init, payer=user, space = 264)]
    pub calculator: Account<'info, Calculator>,    // since we create a new calculator account, we need to pass the space and initialise it, and the payer for the creation of the account is the user

    #[account(mut)]
    pub user: Signer<'info>,    // mutable means that the user account can be modified

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]                              // we dont need to initialise because we are not creating a new account. we are just modifying the existing one.
    pub calculator: Account<'info, Calculator>,  // we declare this as mutable since we are modifying the calculator account
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]                              
    pub calculator: Account<'info, Calculator>,  
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]                              
    pub calculator: Account<'info, Calculator>,  
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]                              
    pub calculator: Account<'info, Calculator>,  
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}