use near_units::parse_near;
use std::{env, fs};
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_example(&alice, &contract, &worker).await?;
    Ok(())
}

async fn test_example(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let user_account = worker.view_account(user.id()).await?;
    let contract_account = worker.view_account(contract.id()).await?;

    println!("user_account={user_account:?}, contract_account={contract_account:?}");

    assert_ne!(user_account, contract_account);
    println!("      Passed ✅ test example");
    Ok(())
}

