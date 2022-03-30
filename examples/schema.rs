use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_chess::cwchess::{CwChessAction, CwChessColor, CwChessGame, CwChessGameOver};
use cosmwasm_chess::msg::{ExecuteMsg, GameSummary, InstantiateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(CwChessAction), &out_dir);
    export_schema(&schema_for!(CwChessColor), &out_dir);
    export_schema(&schema_for!(CwChessGame), &out_dir);
    export_schema(&schema_for!(CwChessGameOver), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(GameSummary), &out_dir);
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}
