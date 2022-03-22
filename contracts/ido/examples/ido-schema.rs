use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use starterra_token::ido::{
    AllocationInfo, ConfigResponse, ExecuteMsg, InstantiateMsg, ParticipantInfoResponse,
    ParticipantResponse, ParticipantsResponse, QueryMsg, StateResponse, StatusResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(StateResponse), &out_dir);
    export_schema(&schema_for!(StatusResponse), &out_dir);
    export_schema(&schema_for!(ParticipantInfoResponse), &out_dir);
    export_schema(&schema_for!(AllocationInfo), &out_dir);
    export_schema(&schema_for!(ParticipantResponse), &out_dir);
    export_schema(&schema_for!(ParticipantsResponse), &out_dir);
}
