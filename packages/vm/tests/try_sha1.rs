use cosmwasm_std::{coins, ContractResult, Response};
use cosmwasm_vm::testing::{instantiate, mock_env, mock_info, mock_instance};
use schemars::JsonSchema;
use serde::Serialize;

static WASM: &[u8] = include_bytes!("./try-sha1.wasm");

#[derive(Serialize, JsonSchema)]
struct InstantiateMsg {}

#[test]
fn sha1_fails_with_error_code_10() {
    let mut deps = mock_instance(WASM, &[]);

    let info = mock_info("creator", &coins(1000, "earth"));
    // we can just call .unwrap() to assert this was a success
    let res: ContractResult<Response> = instantiate(&mut deps, mock_env(), info, InstantiateMsg {});
    let msg = res.unwrap_err();
    assert_eq!(msg, "Hash Calculation error: Unknown error: 10")
}
