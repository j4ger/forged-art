use leptos::*;

use super::websocket::GAME_INFO_STORE;

#[server(GetNewUuid, "/api", "GetCbor", "new_uuid")]
pub async fn get_new_uuid() -> Result<String, ServerFnError> {
    use nanoid::nanoid;
    Ok(nanoid!(12))
}

#[server(GetPlayerName, "/api", "GetCbor", "get_name")]
pub async fn get_player_name(uuid: String, game_id: String) -> Result<String, ServerFnError> {
    Ok(GAME_INFO_STORE
        .get(&game_id)
        .map_or("".to_string(), |game| {
            game.players
                .iter()
                .find(|player| player.0 == uuid)
                .map_or("".to_string(), |player| player.1.clone())
        }))
}

