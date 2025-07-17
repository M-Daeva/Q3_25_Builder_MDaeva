use anchor_lang::{
    error::{AnchorError, Error, ProgramErrorWithOrigin},
    prelude::ProgramError,
    AnchorDeserialize, Id, Result,
};
use solana_pubkey::Pubkey;

use amm::state::PoolConfig;

use crate::helpers::suite::{core::App, types::AppUser};

const DISCRIMINATOR_SPACE: usize = 8;

pub trait AppExtension {
    // fn chat_try_send_message(
    //     &mut self,
    //     sender: AppUser,
    //     data: &str,
    //     timestamp: &Timestamp,
    // ) -> StdResult<AppResponse>;

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<PoolConfig>;
}

impl AppExtension for App {
    // fn chat_try_send_message(
    //     &mut self,
    //     sender: AppUser,
    //     data: &str,
    //     timestamp: &Timestamp,
    // ) -> StdResult<AppResponse> {
    //     self.app
    //         .execute_contract(
    //             sender.into(),
    //             self.get_chat_address(),
    //             &ExecuteMsg::SendMessage {
    //                 data: data.to_string(),
    //                 timestamp: timestamp.to_owned(),
    //             },
    //             &[],
    //         )
    //         .map_err(parse_err)
    // }

    fn amm_query_pool_config(&self, pool_id: u64) -> Result<PoolConfig> {
        get_data(self, &self.pda.amm_pool_config(pool_id))
    }
}

fn get_data<T>(app: &App, pda: &Pubkey) -> Result<T>
where
    T: AnchorDeserialize,
{
    let data = &mut &app.litesvm.get_account(pda).unwrap().data[DISCRIMINATOR_SPACE..];

    Ok(T::deserialize(data)?)
}
