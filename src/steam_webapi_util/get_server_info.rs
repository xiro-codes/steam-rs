//! # Implements the `GetServerInfo` endpoint

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{ErrorHandle, SteamWebAPIUtilError},
    macros::do_http,
    Steam, BASE,
};

use super::INTERFACE;

const ENDPOINT: &str = "GetServerInfo";
const VERSION: &str = "1";

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    /// Returns Unix timestamp of WebAPI server time.
    #[serde(rename = "servertime")]
    pub server_time: u32,

    /// Returns time string of WebAPI server time.
    #[serde(rename = "servertimestring")]
    pub server_time_string: String,
}

impl Steam {
    /// Returns WebAPI server time & checks server status.
    ///
    /// # Example
    ///
    /// ```
    ///     // Retrieves server info.
    ///     let server_info = Steam::get_server_info().await.unwrap();
    ///
    ///     // Prints the current server time as a string.
    ///     println!("{}", server_info.server_time_string);
    /// ```
    pub async fn get_server_info() -> Result<ServerInfo, SteamWebAPIUtilError> {
        let url = format!("{}/{}/{}/v{}/", BASE, INTERFACE, ENDPOINT, VERSION);
        let json = do_http!(url, Value, ErrorHandle, SteamWebAPIUtilError::GetServerInfo);
        let server_info: ServerInfo = ErrorHandle!(
            serde_json::from_value(json.to_owned()),
            SteamWebAPIUtilError::GetServerInfo
        );
        Ok(server_info)
    }
}
