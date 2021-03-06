use std::env::current_dir;
use std::fs::create_dir_all;

use starterra_token::staking_gateway::{
    ConfigResponse, CanStakeResponse, ExecuteMsg, InstantiateMsg, QueryMsg, AddressesResponse
};
use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
    export_schema(&schema_for!(CanStakeResponse), &out_dir);
    export_schema(&schema_for!(AddressesResponse), &out_dir);
}
