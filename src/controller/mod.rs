use log::{debug, info};

use crate::api::v1::{
    characters::{Character, InventorySlot},
    items::{handler::call_get_item, GetItemRequest, GetItemResponse},
    my_characters::{
        handler::{
            call_action_crafting, call_action_gathering, call_action_ge_sell_item,
            call_action_move, call_get_my_characters,
        },
        ActionCraftingRequest, ActionCraftingResponse, ActionGatheringRequest,
        ActionGatheringResponse, ActionGeSellItemRequest, ActionGeSellItemResponse,
        ActionMoveRequest, ActionMoveResponse, MyCharacters,
    },
    status::handler::call_get_status,
};

pub const COPPER_ROCKS_LOCATION: (i32, i32) = (2, 0);
pub const MINING_WORKSHOP_LOCATION: (i32, i32) = (1, 5);
pub const WEAPON_CRAFTING_WORKSHOP_LOCATION: (i32, i32) = (2, 1);
pub const GRAND_EXCHANGE_LOCATION: (i32, i32) = (5, 1);

pub fn start_playing(http_client: &mut ureq::Agent, api_token: String) {
    call_get_status(http_client);
    let my_characters: Option<Vec<MyCharacters>> = call_get_my_characters(http_client, &api_token);

    if let Some(characters) = my_characters {
        let mut mining_threads: Vec<std::thread::JoinHandle<()>> = vec![];
        for character in characters.into_iter() {
            info!("Found character: {:?}", character);
            let mut mining_active: bool = true;
            let character: Character = character.to_character();
            let mut new_client: ureq::Agent = http_client.clone();
            let duplicate_token: String = api_token.clone();
            mining_threads.push(std::thread::spawn(move || {
                start_mining(
                    character,
                    &mut new_client,
                    &duplicate_token,
                    &mut mining_active,
                )
            }));
        }
        for thread in mining_threads {
            thread.join().unwrap();
        }
    }
}

pub fn start_mining(
    mut character: Character,
    http_client: &mut ureq::Agent,
    api_token: &String,
    mining_active: &mut bool,
) {
    try_craft_copper(http_client, api_token, &mut character);
    if character.x != COPPER_ROCKS_LOCATION.0 || character.y != COPPER_ROCKS_LOCATION.1 {
        debug!(
            "Character {} is not at copper rocks. Moving...",
            character.name.clone()
        );
        let movement: ActionMoveRequest = ActionMoveRequest {
            x: COPPER_ROCKS_LOCATION.0,
            y: COPPER_ROCKS_LOCATION.1,
        };
        let action_move: Option<ActionMoveResponse> =
            call_action_move(http_client, api_token, &character.name, &movement);
        if let Some(action_move_response) = action_move {
            character = action_move_response.data.character;
        }
    } else {
        debug!(
            "Character {} is at copper rocks ({}{}). Mining...",
            character.name.clone(),
            character.x,
            character.y
        );
    }
    while *mining_active {
        character.wait_for_cooldown();
        let gathering_response: Option<ActionGatheringResponse> = call_action_gathering(
            http_client,
            api_token,
            character.name.clone(),
            &ActionGatheringRequest {
                name: character.name.clone(),
            },
        );

        match gathering_response {
            Some(gathering) => {
                character = gathering.data.character;
                character.wait_for_cooldown();
            }
            None => {
                *mining_active = false;
                return;
            }
        }

        if character.is_inventory_full() {
            *mining_active = false;
            let crafted_copper: bool = try_craft_copper(http_client, api_token, &mut character);
            if !crafted_copper {
                return;
            }

            let move_result: Option<ActionMoveResponse> = call_action_move(
                http_client,
                api_token,
                &character.name,
                &ActionMoveRequest {
                    x: WEAPON_CRAFTING_WORKSHOP_LOCATION.0,
                    y: WEAPON_CRAFTING_WORKSHOP_LOCATION.1,
                },
            );
            if let Some(movement) = move_result {
                character = movement.data.character;
                character.wait_for_cooldown();
                let copper_dagger_count: i32 = character
                    .inventory
                    .iter()
                    .find(|slot| slot.code == "copper_dagger")
                    .unwrap()
                    .quantity;
                let crafting_result: Option<ActionCraftingResponse> = call_action_crafting(
                    http_client,
                    api_token,
                    character.name.to_string(),
                    ActionCraftingRequest {
                        code: "copper_dagger".to_string(),
                        quantity: copper_dagger_count / 3,
                    },
                );
                if let Some(crafting) = crafting_result {
                    character = crafting.data.character;
                    character.wait_for_cooldown();
                    character = sell_all_items(http_client, api_token, character);
                    *mining_active = true;
                }
            }
        }
    }
}

pub fn sell_all_items(
    http_client: &mut ureq::Agent,
    api_token: &String,
    mut character: Character,
) -> Character {
    if character.x != GRAND_EXCHANGE_LOCATION.0 || character.y != GRAND_EXCHANGE_LOCATION.1 {
        let move_result: Option<ActionMoveResponse> = call_action_move(
            http_client,
            api_token,
            &character.name,
            &ActionMoveRequest {
                x: GRAND_EXCHANGE_LOCATION.0,
                y: GRAND_EXCHANGE_LOCATION.1,
            },
        );
        if let Some(movement_to_ge) = move_result {
            character = movement_to_ge.data.character;
            character.wait_for_cooldown();
        }
    }
    let inventory: Vec<InventorySlot> = character.inventory.clone();
    for slot in inventory.iter() {
        if slot.quantity <= 0 {
            continue;
        }
        let item_result: Option<GetItemResponse> = call_get_item(
            http_client,
            api_token,
            GetItemRequest {
                code: slot.code.clone(),
            },
        );
        if let Some(item) = item_result {
            character = sell_inventory_item(http_client, api_token, character, item, slot.quantity);
        }
    }

    character
}

pub fn sell_inventory_item(
    http_client: &mut ureq::Agent,
    api_token: &String,
    mut character: Character,
    item: GetItemResponse,
    quantity: i32,
) -> Character {
    let sell_result: Option<ActionGeSellItemResponse> = call_action_ge_sell_item(
        http_client,
        api_token,
        character.name.to_string(),
        ActionGeSellItemRequest {
            code: item.data.item.code,
            quantity,
            price: item.data.ge.unwrap().sell_price.unwrap() as f32,
        },
    );
    if let Some(sell) = sell_result {
        character = sell.data.character;
        character.wait_for_cooldown();
    }
    character
}

pub fn try_craft_copper(
    http_client: &mut ureq::Agent,
    api_token: &String,
    character: &mut Character,
) -> bool {
    if character.inventory.is_empty() {
        debug!(
            "{}'s inventory is empty -- cannot craft copper.",
            character.name
        );
        return false;
    }
    if !character
        .inventory
        .iter()
        .any(|item| item.code == "copper_ore" && item.quantity >= 6)
    {
        debug!(
            "{} does not have enough copper ore to craft.",
            character.name
        );
        return false;
    }
    if character.x != MINING_WORKSHOP_LOCATION.0 && character.y != MINING_WORKSHOP_LOCATION.1 {
        let move_result: Option<ActionMoveResponse> = call_action_move(
            http_client,
            api_token,
            &character.name,
            &ActionMoveRequest {
                x: MINING_WORKSHOP_LOCATION.0,
                y: MINING_WORKSHOP_LOCATION.1,
            },
        );
        if let Some(movement) = move_result {
            character.x = movement.data.character.x;
            character.y = movement.data.character.y;
            movement.data.character.wait_for_cooldown();
        } else {
            return false;
        }
    }
    let copper_ore_count: i32 = character
        .inventory
        .iter()
        .find(|slot| slot.code == "copper_ore")
        .unwrap()
        .quantity;

    let crafting_result: Option<ActionCraftingResponse> = call_action_crafting(
        http_client,
        api_token,
        character.name.to_string(),
        ActionCraftingRequest {
            code: "copper".to_string(),
            quantity: copper_ore_count / 6,
        },
    );
    if let Some(crafting) = crafting_result {
        crafting.data.character.wait_for_cooldown();
    } else {
        return false;
    }
    true
}
