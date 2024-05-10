use erc20_example::{helpers::TxKind, scenario};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    scenario::basic_eth().await;
    // scenario::run(20, 200, TxKind::Deploy).await;
    // scenario::deploy_erc20(20, 200).await;
    // scenario::mint_erc20(20, 200).await;
    // scenario::transfer_erc20(20, 200).await;
}
