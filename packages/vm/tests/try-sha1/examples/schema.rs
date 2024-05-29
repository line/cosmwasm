use cosmwasm_schema::write_api;

use try_sha1::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
    }
}
