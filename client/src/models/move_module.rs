/*
 * Aptos Node API
 *
 * The Aptos Node API is a RESTful API for client applications to interact with the Aptos blockchain.
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MoveModule : A Move module
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveModule {
    /// A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1. 
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Friends of the module
    #[serde(rename = "friends")]
    pub friends: Vec<String>,
    /// Public functions of the module
    #[serde(rename = "exposed_functions")]
    pub exposed_functions: Vec<models::MoveFunction>,
    /// Structs of the module
    #[serde(rename = "structs")]
    pub structs: Vec<models::MoveStruct>,
}

impl MoveModule {
    /// A Move module
    pub fn new(address: String, name: String, friends: Vec<String>, exposed_functions: Vec<models::MoveFunction>, structs: Vec<models::MoveStruct>) -> MoveModule {
        MoveModule {
            address,
            name,
            friends,
            exposed_functions,
            structs,
        }
    }
}

