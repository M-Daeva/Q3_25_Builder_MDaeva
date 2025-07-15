use solana_keypair::Keypair;
use solana_signer::Signer;

use crate::helpers::suite::types::ProjectAccount;

#[test]
fn default() {
    // let mut test_environment = setup_escrow_test();

    println!("{:#?}", ProjectAccount::Admin);
    println!("{:#?}", ProjectAccount::Admin.pubkey());
    println!("{:#?}\n", ProjectAccount::Admin.keypair().pubkey());
}
