/* Generate schema
 * To generate: cargo run
 */
use cosmwasm_schema::write_api;
use hello_wasm::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        // execute: ExecuteMsg, <-- add execute message here
    }
}
