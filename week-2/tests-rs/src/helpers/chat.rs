use cosmwasm_std::{StdResult, Timestamp};
use cw_multi_test::{AppResponse, Executor};

use coral_base::{
    chat::msg::{ExecuteMsg, QueryMsg},
    error::parse_err,
};

use crate::helpers::suite::{core::Project, types::ProjectAccount};

pub trait ChatExtension {
    fn chat_try_send_message(
        &mut self,
        sender: ProjectAccount,
        data: &str,
        timestamp: &Timestamp,
    ) -> StdResult<AppResponse>;

    fn chat_query_last_image_id(&self, owner: ProjectAccount) -> StdResult<EncryptedResponse>;
}

impl ChatExtension for Project {
    #[track_caller]
    fn chat_try_send_message(
        &mut self,
        sender: ProjectAccount,
        data: &str,
        timestamp: &Timestamp,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_chat_address(),
                &ExecuteMsg::SendMessage {
                    data: data.to_string(),
                    timestamp: timestamp.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn chat_query_last_image_id(&self, owner: ProjectAccount) -> StdResult<EncryptedResponse> {
        self.app.wrap().query_wasm_smart(
            self.get_chat_address(),
            &QueryMsg::LastImageId {
                owner: owner.to_string(),
            },
        )
    }
}
