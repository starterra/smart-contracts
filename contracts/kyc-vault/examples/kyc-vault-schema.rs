use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use starterra_token::kyc_vault::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, IsAcceptedResponse, IsAcceptedVerifiedResponse,
    IsVerifiedResponse, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(IsVerifiedResponse), &out_dir);
    export_schema(&schema_for!(IsAcceptedResponse), &out_dir);
    export_schema(&schema_for!(IsAcceptedVerifiedResponse), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
