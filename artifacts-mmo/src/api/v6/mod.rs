
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetServerDetailsGet {}

impl GetServerDetailsGet {
	pub fn path() -> &'static str {
		"/"
	}

	pub fn description() -> &'static str {
		"Return the status of the game server."
	}

	pub fn summary() -> &'static str {
		"Get Server Details"
	}

	pub fn operation_id() -> &'static str {
		"get_server_details__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Server details",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetServerDetailsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetServerDetailsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateAccountAccountsCreatePost {}

impl CreateAccountAccountsCreatePost {
	pub fn path() -> &'static str {
		"/accounts/create"
	}

	pub fn description() -> &'static str {
		""
	}

	pub fn summary() -> &'static str {
		"Create Account"
	}

	pub fn operation_id() -> &'static str {
		"create_account_accounts_create_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for CreateAccountAccountsCreatePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			CreateAccountAccountsCreatePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ForgotPasswordAccountsForgotPasswordPost {}

impl ForgotPasswordAccountsForgotPasswordPost {
	pub fn path() -> &'static str {
		"/accounts/forgot_password"
	}

	pub fn description() -> &'static str {
		"Request a password reset."
	}

	pub fn summary() -> &'static str {
		"Forgot Password"
	}

	pub fn operation_id() -> &'static str {
		"forgot_password_accounts_forgot_password_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ForgotPasswordAccountsForgotPasswordPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ForgotPasswordAccountsForgotPasswordPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ResetPasswordAccountsResetPasswordPost {}

impl ResetPasswordAccountsResetPasswordPost {
	pub fn path() -> &'static str {
		"/accounts/reset_password"
	}

	pub fn description() -> &'static str {
		"Reset password with a token. Use /forgot_password to get a token by email."
	}

	pub fn summary() -> &'static str {
		"Reset Password"
	}

	pub fn operation_id() -> &'static str {
		"reset_password_accounts_reset_password_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ResetPasswordAccountsResetPasswordPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ResetPasswordAccountsResetPasswordPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountAccountsAccountGet {}

impl GetAccountAccountsAccountGet {
	pub fn path() -> &'static str {
		"/accounts/{account}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a character."
	}

	pub fn summary() -> &'static str {
		"Get Account"
	}

	pub fn operation_id() -> &'static str {
		"get_account_accounts__account__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAccountAccountsAccountGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAccountAccountsAccountGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountAchievementsAccountsAccountAchievementsGet {}

impl GetAccountAchievementsAccountsAccountAchievementsGet {
	pub fn path() -> &'static str {
		"/accounts/{account}/achievements"
	}

	pub fn description() -> &'static str {
		"Retrieve the achievements of a account."
	}

	pub fn summary() -> &'static str {
		"Get Account Achievements"
	}

	pub fn operation_id() -> &'static str {
		"get_account_achievements_accounts__account__achievements_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAccountAchievementsAccountsAccountAchievementsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAccountAchievementsAccountsAccountAchievementsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountCharactersAccountsAccountCharactersGet {}

impl GetAccountCharactersAccountsAccountCharactersGet {
	pub fn path() -> &'static str {
		"/accounts/{account}/characters"
	}

	pub fn description() -> &'static str {
		"Account character lists."
	}

	pub fn summary() -> &'static str {
		"Get Account Characters"
	}

	pub fn operation_id() -> &'static str {
		"get_account_characters_accounts__account__characters_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Accounts",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAccountCharactersAccountsAccountCharactersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAccountCharactersAccountsAccountCharactersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllAchievementsAchievementsGet {}

impl GetAllAchievementsAchievementsGet {
	pub fn path() -> &'static str {
		"/achievements"
	}

	pub fn description() -> &'static str {
		"List of all achievements."
	}

	pub fn summary() -> &'static str {
		"Get All Achievements"
	}

	pub fn operation_id() -> &'static str {
		"get_all_achievements_achievements_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Achievements",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllAchievementsAchievementsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllAchievementsAchievementsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAchievementAchievementsCodeGet {}

impl GetAchievementAchievementsCodeGet {
	pub fn path() -> &'static str {
		"/achievements/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a achievement."
	}

	pub fn summary() -> &'static str {
		"Get Achievement"
	}

	pub fn operation_id() -> &'static str {
		"get_achievement_achievements__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Achievements",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAchievementAchievementsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAchievementAchievementsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllBadgesBadgesGet {}

impl GetAllBadgesBadgesGet {
	pub fn path() -> &'static str {
		"/badges"
	}

	pub fn description() -> &'static str {
		"List of all badges."
	}

	pub fn summary() -> &'static str {
		"Get All Badges"
	}

	pub fn operation_id() -> &'static str {
		"get_all_badges_badges_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Badges",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllBadgesBadgesGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllBadgesBadgesGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetBadgeBadgesCodeGet {}

impl GetBadgeBadgesCodeGet {
	pub fn path() -> &'static str {
		"/badges/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a badge."
	}

	pub fn summary() -> &'static str {
		"Get Badge"
	}

	pub fn operation_id() -> &'static str {
		"get_badge_badges__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Badges",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetBadgeBadgesCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetBadgeBadgesCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct CreateCharacterCharactersCreatePost {}

impl CreateCharacterCharactersCreatePost {
	pub fn path() -> &'static str {
		"/characters/create"
	}

	pub fn description() -> &'static str {
		"Create new character on your account. You can create up to 5 characters."
	}

	pub fn summary() -> &'static str {
		"Create Character"
	}

	pub fn operation_id() -> &'static str {
		"create_character_characters_create_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for CreateCharacterCharactersCreatePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			CreateCharacterCharactersCreatePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DeleteCharacterCharactersDeletePost {}

impl DeleteCharacterCharactersDeletePost {
	pub fn path() -> &'static str {
		"/characters/delete"
	}

	pub fn description() -> &'static str {
		"Delete character on your account."
	}

	pub fn summary() -> &'static str {
		"Delete Character"
	}

	pub fn operation_id() -> &'static str {
		"delete_character_characters_delete_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for DeleteCharacterCharactersDeletePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			DeleteCharacterCharactersDeletePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharacterCharactersNameGet {}

impl GetCharacterCharactersNameGet {
	pub fn path() -> &'static str {
		"/characters/{name}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a character."
	}

	pub fn summary() -> &'static str {
		"Get Character"
	}

	pub fn operation_id() -> &'static str {
		"get_character_characters__name__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetCharacterCharactersNameGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetCharacterCharactersNameGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllEffectsEffectsGet {}

impl GetAllEffectsEffectsGet {
	pub fn path() -> &'static str {
		"/effects"
	}

	pub fn description() -> &'static str {
		"List of all effects. Effects are used by equipment, tools, runes, consumables and monsters. An effect is an action that produces an effect on the game."
	}

	pub fn summary() -> &'static str {
		"Get All Effects"
	}

	pub fn operation_id() -> &'static str {
		"get_all_effects_effects_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Effects",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllEffectsEffectsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllEffectsEffectsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetEffectEffectsCodeGet {}

impl GetEffectEffectsCodeGet {
	pub fn path() -> &'static str {
		"/effects/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a badge."
	}

	pub fn summary() -> &'static str {
		"Get Effect"
	}

	pub fn operation_id() -> &'static str {
		"get_effect_effects__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Effects",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetEffectEffectsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetEffectEffectsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllEventsEventsGet {}

impl GetAllEventsEventsGet {
	pub fn path() -> &'static str {
		"/events"
	}

	pub fn description() -> &'static str {
		"Fetch events details."
	}

	pub fn summary() -> &'static str {
		"Get All Events"
	}

	pub fn operation_id() -> &'static str {
		"get_all_events_events_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Events",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllEventsEventsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllEventsEventsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllActiveEventsEventsActiveGet {}

impl GetAllActiveEventsEventsActiveGet {
	pub fn path() -> &'static str {
		"/events/active"
	}

	pub fn description() -> &'static str {
		"Fetch active events details."
	}

	pub fn summary() -> &'static str {
		"Get All Active Events"
	}

	pub fn operation_id() -> &'static str {
		"get_all_active_events_events_active_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Events",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllActiveEventsEventsActiveGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllActiveEventsEventsActiveGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetGeSellHistoryGrandexchangeHistoryCodeGet {}

impl GetGeSellHistoryGrandexchangeHistoryCodeGet {
	pub fn path() -> &'static str {
		"/grandexchange/history/{code}"
	}

	pub fn description() -> &'static str {
		"Fetch the sales history of the item for the last 7 days."
	}

	pub fn summary() -> &'static str {
		"Get Ge Sell History"
	}

	pub fn operation_id() -> &'static str {
		"get_ge_sell_history_grandexchange_history__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Grand Exchange",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetGeSellHistoryGrandexchangeHistoryCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetGeSellHistoryGrandexchangeHistoryCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetGeSellOrdersGrandexchangeOrdersGet {}

impl GetGeSellOrdersGrandexchangeOrdersGet {
	pub fn path() -> &'static str {
		"/grandexchange/orders"
	}

	pub fn description() -> &'static str {
		"Fetch all sell orders."
	}

	pub fn summary() -> &'static str {
		"Get Ge Sell Orders"
	}

	pub fn operation_id() -> &'static str {
		"get_ge_sell_orders_grandexchange_orders_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Grand Exchange",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetGeSellOrdersGrandexchangeOrdersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetGeSellOrdersGrandexchangeOrdersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetGeSellOrderGrandexchangeOrdersIdGet {}

impl GetGeSellOrderGrandexchangeOrdersIdGet {
	pub fn path() -> &'static str {
		"/grandexchange/orders/{id}"
	}

	pub fn description() -> &'static str {
		"Retrieve the sell order of a item."
	}

	pub fn summary() -> &'static str {
		"Get Ge Sell Order"
	}

	pub fn operation_id() -> &'static str {
		"get_ge_sell_order_grandexchange_orders__id__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Grand Exchange",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetGeSellOrderGrandexchangeOrdersIdGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetGeSellOrderGrandexchangeOrdersIdGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllItemsItemsGet {}

impl GetAllItemsItemsGet {
	pub fn path() -> &'static str {
		"/items"
	}

	pub fn description() -> &'static str {
		"Fetch items details."
	}

	pub fn summary() -> &'static str {
		"Get All Items"
	}

	pub fn operation_id() -> &'static str {
		"get_all_items_items_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Items",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllItemsItemsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllItemsItemsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetItemItemsCodeGet {}

impl GetItemItemsCodeGet {
	pub fn path() -> &'static str {
		"/items/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a item."
	}

	pub fn summary() -> &'static str {
		"Get Item"
	}

	pub fn operation_id() -> &'static str {
		"get_item_items__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Items",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetItemItemsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetItemItemsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountsLeaderboardLeaderboardAccountsGet {}

impl GetAccountsLeaderboardLeaderboardAccountsGet {
	pub fn path() -> &'static str {
		"/leaderboard/accounts"
	}

	pub fn description() -> &'static str {
		"Fetch leaderboard details."
	}

	pub fn summary() -> &'static str {
		"Get Accounts Leaderboard"
	}

	pub fn operation_id() -> &'static str {
		"get_accounts_leaderboard_leaderboard_accounts_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Leaderboard",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAccountsLeaderboardLeaderboardAccountsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAccountsLeaderboardLeaderboardAccountsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharactersLeaderboardLeaderboardCharactersGet {}

impl GetCharactersLeaderboardLeaderboardCharactersGet {
	pub fn path() -> &'static str {
		"/leaderboard/characters"
	}

	pub fn description() -> &'static str {
		"Fetch leaderboard details."
	}

	pub fn summary() -> &'static str {
		"Get Characters Leaderboard"
	}

	pub fn operation_id() -> &'static str {
		"get_characters_leaderboard_leaderboard_characters_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Leaderboard",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetCharactersLeaderboardLeaderboardCharactersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetCharactersLeaderboardLeaderboardCharactersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllMapsMapsGet {}

impl GetAllMapsMapsGet {
	pub fn path() -> &'static str {
		"/maps"
	}

	pub fn description() -> &'static str {
		"Fetch maps details."
	}

	pub fn summary() -> &'static str {
		"Get All Maps"
	}

	pub fn operation_id() -> &'static str {
		"get_all_maps_maps_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Maps",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllMapsMapsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllMapsMapsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetMapMapsXYGet {}

impl GetMapMapsXYGet {
	pub fn path() -> &'static str {
		"/maps/{x}/{y}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a map."
	}

	pub fn summary() -> &'static str {
		"Get Map"
	}

	pub fn operation_id() -> &'static str {
		"get_map_maps__x___y__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Maps",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetMapMapsXYGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetMapMapsXYGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllMonstersMonstersGet {}

impl GetAllMonstersMonstersGet {
	pub fn path() -> &'static str {
		"/monsters"
	}

	pub fn description() -> &'static str {
		"Fetch monsters details."
	}

	pub fn summary() -> &'static str {
		"Get All Monsters"
	}

	pub fn operation_id() -> &'static str {
		"get_all_monsters_monsters_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Monsters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllMonstersMonstersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllMonstersMonstersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetMonsterMonstersCodeGet {}

impl GetMonsterMonstersCodeGet {
	pub fn path() -> &'static str {
		"/monsters/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a monster."
	}

	pub fn summary() -> &'static str {
		"Get Monster"
	}

	pub fn operation_id() -> &'static str {
		"get_monster_monsters__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Monsters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetMonsterMonstersCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetMonsterMonstersCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetBankDetailsMyBankGet {}

impl GetBankDetailsMyBankGet {
	pub fn path() -> &'static str {
		"/my/bank"
	}

	pub fn description() -> &'static str {
		"Fetch bank details."
	}

	pub fn summary() -> &'static str {
		"Get Bank Details"
	}

	pub fn operation_id() -> &'static str {
		"get_bank_details_my_bank_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetBankDetailsMyBankGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetBankDetailsMyBankGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetBankItemsMyBankItemsGet {}

impl GetBankItemsMyBankItemsGet {
	pub fn path() -> &'static str {
		"/my/bank/items"
	}

	pub fn description() -> &'static str {
		"Fetch all items in your bank."
	}

	pub fn summary() -> &'static str {
		"Get Bank Items"
	}

	pub fn operation_id() -> &'static str {
		"get_bank_items_my_bank_items_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetBankItemsMyBankItemsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetBankItemsMyBankItemsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ChangePasswordMyChangePasswordPost {}

impl ChangePasswordMyChangePasswordPost {
	pub fn path() -> &'static str {
		"/my/change_password"
	}

	pub fn description() -> &'static str {
		"Change your account password. Changing the password reset the account token."
	}

	pub fn summary() -> &'static str {
		"Change Password"
	}

	pub fn operation_id() -> &'static str {
		"change_password_my_change_password_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ChangePasswordMyChangePasswordPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ChangePasswordMyChangePasswordPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetMyCharactersMyCharactersGet {}

impl GetMyCharactersMyCharactersGet {
	pub fn path() -> &'static str {
		"/my/characters"
	}

	pub fn description() -> &'static str {
		"List of your characters."
	}

	pub fn summary() -> &'static str {
		"Get My Characters"
	}

	pub fn operation_id() -> &'static str {
		"get_my_characters_my_characters_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetMyCharactersMyCharactersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetMyCharactersMyCharactersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountDetailsMyDetailsGet {}

impl GetAccountDetailsMyDetailsGet {
	pub fn path() -> &'static str {
		"/my/details"
	}

	pub fn description() -> &'static str {
		"Fetch account details."
	}

	pub fn summary() -> &'static str {
		"Get Account Details"
	}

	pub fn operation_id() -> &'static str {
		"get_account_details_my_details_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAccountDetailsMyDetailsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAccountDetailsMyDetailsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetGeSellHistoryMyGrandexchangeHistoryGet {}

impl GetGeSellHistoryMyGrandexchangeHistoryGet {
	pub fn path() -> &'static str {
		"/my/grandexchange/history"
	}

	pub fn description() -> &'static str {
		"Fetch your sales history of the last 7 days."
	}

	pub fn summary() -> &'static str {
		"Get Ge Sell History"
	}

	pub fn operation_id() -> &'static str {
		"get_ge_sell_history_my_grandexchange_history_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetGeSellHistoryMyGrandexchangeHistoryGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetGeSellHistoryMyGrandexchangeHistoryGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetGeSellOrdersMyGrandexchangeOrdersGet {}

impl GetGeSellOrdersMyGrandexchangeOrdersGet {
	pub fn path() -> &'static str {
		"/my/grandexchange/orders"
	}

	pub fn description() -> &'static str {
		"Fetch your sell orders details."
	}

	pub fn summary() -> &'static str {
		"Get Ge Sell Orders"
	}

	pub fn operation_id() -> &'static str {
		"get_ge_sell_orders_my_grandexchange_orders_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My account",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetGeSellOrdersMyGrandexchangeOrdersGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetGeSellOrdersMyGrandexchangeOrdersGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllCharactersLogsMyLogsGet {}

impl GetAllCharactersLogsMyLogsGet {
	pub fn path() -> &'static str {
		"/my/logs"
	}

	pub fn description() -> &'static str {
		"History of the last 250 actions of all your characters."
	}

	pub fn summary() -> &'static str {
		"Get All Characters Logs"
	}

	pub fn operation_id() -> &'static str {
		"get_all_characters_logs_my_logs_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllCharactersLogsMyLogsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllCharactersLogsMyLogsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetCharacterLogsMyLogsNameGet {}

impl GetCharacterLogsMyLogsNameGet {
	pub fn path() -> &'static str {
		"/my/logs/{name}"
	}

	pub fn description() -> &'static str {
		"History of the last actions of your character."
	}

	pub fn summary() -> &'static str {
		"Get Character Logs"
	}

	pub fn operation_id() -> &'static str {
		"get_character_logs_my_logs__name__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetCharacterLogsMyLogsNameGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetCharacterLogsMyLogsNameGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionBuyBankExpansionMyNameActionBankBuyExpansionPost {}

impl ActionBuyBankExpansionMyNameActionBankBuyExpansionPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/bank/buy_expansion"
	}

	pub fn description() -> &'static str {
		"Buy a 25 slots bank expansion."
	}

	pub fn summary() -> &'static str {
		"Action Buy Bank Expansion"
	}

	pub fn operation_id() -> &'static str {
		"action_buy_bank_expansion_my__name__action_bank_buy_expansion_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionBuyBankExpansionMyNameActionBankBuyExpansionPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionBuyBankExpansionMyNameActionBankBuyExpansionPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionDepositBankGoldMyNameActionBankDepositGoldPost {}

impl ActionDepositBankGoldMyNameActionBankDepositGoldPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/bank/deposit/gold"
	}

	pub fn description() -> &'static str {
		"Deposit gold in a bank on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Deposit Bank Gold"
	}

	pub fn operation_id() -> &'static str {
		"action_deposit_bank_gold_my__name__action_bank_deposit_gold_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionDepositBankGoldMyNameActionBankDepositGoldPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionDepositBankGoldMyNameActionBankDepositGoldPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionDepositBankItemMyNameActionBankDepositItemPost {}

impl ActionDepositBankItemMyNameActionBankDepositItemPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/bank/deposit/item"
	}

	pub fn description() -> &'static str {
		"Deposit multiple items in a bank on the character's map.
The cooldown will be 3 seconds multiplied by the number of different items withdrawn."
	}

	pub fn summary() -> &'static str {
		"Action Deposit Bank Item"
	}

	pub fn operation_id() -> &'static str {
		"action_deposit_bank_item_my__name__action_bank_deposit_item_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionDepositBankItemMyNameActionBankDepositItemPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionDepositBankItemMyNameActionBankDepositItemPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost {}

impl ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/bank/withdraw/gold"
	}

	pub fn description() -> &'static str {
		"Withdraw gold from your bank."
	}

	pub fn summary() -> &'static str {
		"Action Withdraw Bank Gold"
	}

	pub fn operation_id() -> &'static str {
		"action_withdraw_bank_gold_my__name__action_bank_withdraw_gold_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionWithdrawBankGoldMyNameActionBankWithdrawGoldPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionWithdrawBankItemMyNameActionBankWithdrawItemPost {}

impl ActionWithdrawBankItemMyNameActionBankWithdrawItemPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/bank/withdraw/item"
	}

	pub fn description() -> &'static str {
		"Take items from your bank and put them in the character's inventory.
The cooldown will be 3 seconds multiplied by the number of different items withdrawn."
	}

	pub fn summary() -> &'static str {
		"Action Withdraw Bank Item"
	}

	pub fn operation_id() -> &'static str {
		"action_withdraw_bank_item_my__name__action_bank_withdraw_item_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionWithdrawBankItemMyNameActionBankWithdrawItemPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionWithdrawBankItemMyNameActionBankWithdrawItemPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionChangeSkinMyNameActionChangeSkinPost {}

impl ActionChangeSkinMyNameActionChangeSkinPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/change_skin"
	}

	pub fn description() -> &'static str {
		"Change the skin of your character."
	}

	pub fn summary() -> &'static str {
		"Action Change Skin"
	}

	pub fn operation_id() -> &'static str {
		"action_change_skin_my__name__action_change_skin_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionChangeSkinMyNameActionChangeSkinPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionChangeSkinMyNameActionChangeSkinPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionCraftingMyNameActionCraftingPost {}

impl ActionCraftingMyNameActionCraftingPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/crafting"
	}

	pub fn description() -> &'static str {
		"Crafting an item. The character must be on a map with a workshop."
	}

	pub fn summary() -> &'static str {
		"Action Crafting"
	}

	pub fn operation_id() -> &'static str {
		"action_crafting_my__name__action_crafting_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionCraftingMyNameActionCraftingPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionCraftingMyNameActionCraftingPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionDeleteItemMyNameActionDeletePost {}

impl ActionDeleteItemMyNameActionDeletePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/delete"
	}

	pub fn description() -> &'static str {
		"Delete an item from your character's inventory."
	}

	pub fn summary() -> &'static str {
		"Action Delete Item"
	}

	pub fn operation_id() -> &'static str {
		"action_delete_item_my__name__action_delete_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionDeleteItemMyNameActionDeletePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionDeleteItemMyNameActionDeletePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionEquipItemMyNameActionEquipPost {}

impl ActionEquipItemMyNameActionEquipPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/equip"
	}

	pub fn description() -> &'static str {
		"Equip an item on your character."
	}

	pub fn summary() -> &'static str {
		"Action Equip Item"
	}

	pub fn operation_id() -> &'static str {
		"action_equip_item_my__name__action_equip_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionEquipItemMyNameActionEquipPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionEquipItemMyNameActionEquipPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionFightMyNameActionFightPost {}

impl ActionFightMyNameActionFightPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/fight"
	}

	pub fn description() -> &'static str {
		"Start a fight against a monster on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Fight"
	}

	pub fn operation_id() -> &'static str {
		"action_fight_my__name__action_fight_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionFightMyNameActionFightPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionFightMyNameActionFightPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGatheringMyNameActionGatheringPost {}

impl ActionGatheringMyNameActionGatheringPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/gathering"
	}

	pub fn description() -> &'static str {
		"Harvest a resource on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Gathering"
	}

	pub fn operation_id() -> &'static str {
		"action_gathering_my__name__action_gathering_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGatheringMyNameActionGatheringPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGatheringMyNameActionGatheringPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGiveGoldMyNameActionGiveGoldPost {}

impl ActionGiveGoldMyNameActionGiveGoldPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/give/gold"
	}

	pub fn description() -> &'static str {
		"Give gold to another character in your account on the same map."
	}

	pub fn summary() -> &'static str {
		"Action Give Gold"
	}

	pub fn operation_id() -> &'static str {
		"action_give_gold_my__name__action_give_gold_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGiveGoldMyNameActionGiveGoldPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGiveGoldMyNameActionGiveGoldPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGiveItemsMyNameActionGiveItemPost {}

impl ActionGiveItemsMyNameActionGiveItemPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/give/item"
	}

	pub fn description() -> &'static str {
		"Give items to another character in your account on the same map.
The cooldown will be 3 seconds multiplied by the number of different items given."
	}

	pub fn summary() -> &'static str {
		"Action Give Items"
	}

	pub fn operation_id() -> &'static str {
		"action_give_items_my__name__action_give_item_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGiveItemsMyNameActionGiveItemPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGiveItemsMyNameActionGiveItemPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGeBuyItemMyNameActionGrandexchangeBuyPost {}

impl ActionGeBuyItemMyNameActionGrandexchangeBuyPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/grandexchange/buy"
	}

	pub fn description() -> &'static str {
		"Buy an item at the Grand Exchange on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Ge Buy Item"
	}

	pub fn operation_id() -> &'static str {
		"action_ge_buy_item_my__name__action_grandexchange_buy_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGeBuyItemMyNameActionGrandexchangeBuyPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGeBuyItemMyNameActionGrandexchangeBuyPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGeCancelSellOrderMyNameActionGrandexchangeCancelPost {}

impl ActionGeCancelSellOrderMyNameActionGrandexchangeCancelPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/grandexchange/cancel"
	}

	pub fn description() -> &'static str {
		"Cancel a sell order at the Grand Exchange on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Ge Cancel Sell Order"
	}

	pub fn operation_id() -> &'static str {
		"action_ge_cancel_sell_order_my__name__action_grandexchange_cancel_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGeCancelSellOrderMyNameActionGrandexchangeCancelPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGeCancelSellOrderMyNameActionGrandexchangeCancelPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionGeCreateSellOrderMyNameActionGrandexchangeSellPost {}

impl ActionGeCreateSellOrderMyNameActionGrandexchangeSellPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/grandexchange/sell"
	}

	pub fn description() -> &'static str {
		"Create a sell order at the Grand Exchange on the character's map. Please note there is a 3% listing tax, charged at the time of posting, on the total price."
	}

	pub fn summary() -> &'static str {
		"Action Ge Create Sell Order"
	}

	pub fn operation_id() -> &'static str {
		"action_ge_create_sell_order_my__name__action_grandexchange_sell_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionGeCreateSellOrderMyNameActionGrandexchangeSellPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionGeCreateSellOrderMyNameActionGrandexchangeSellPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionMoveMyNameActionMovePost {}

impl ActionMoveMyNameActionMovePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/move"
	}

	pub fn description() -> &'static str {
		"Moves a character on the map using the map's X and Y position."
	}

	pub fn summary() -> &'static str {
		"Action Move"
	}

	pub fn operation_id() -> &'static str {
		"action_move_my__name__action_move_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionMoveMyNameActionMovePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionMoveMyNameActionMovePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionNpcBuyItemMyNameActionNpcBuyPost {}

impl ActionNpcBuyItemMyNameActionNpcBuyPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/npc/buy"
	}

	pub fn description() -> &'static str {
		"Buy an item from an NPC on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Npc Buy Item"
	}

	pub fn operation_id() -> &'static str {
		"action_npc_buy_item_my__name__action_npc_buy_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionNpcBuyItemMyNameActionNpcBuyPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionNpcBuyItemMyNameActionNpcBuyPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionNpcSellItemMyNameActionNpcSellPost {}

impl ActionNpcSellItemMyNameActionNpcSellPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/npc/sell"
	}

	pub fn description() -> &'static str {
		"Sell an item to an NPC on the character's map."
	}

	pub fn summary() -> &'static str {
		"Action Npc Sell Item"
	}

	pub fn operation_id() -> &'static str {
		"action_npc_sell_item_my__name__action_npc_sell_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionNpcSellItemMyNameActionNpcSellPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionNpcSellItemMyNameActionNpcSellPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionRecyclingMyNameActionRecyclingPost {}

impl ActionRecyclingMyNameActionRecyclingPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/recycling"
	}

	pub fn description() -> &'static str {
		"Recycling an item. The character must be on a map with a workshop (only for equipments and weapons)."
	}

	pub fn summary() -> &'static str {
		"Action Recycling"
	}

	pub fn operation_id() -> &'static str {
		"action_recycling_my__name__action_recycling_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionRecyclingMyNameActionRecyclingPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionRecyclingMyNameActionRecyclingPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionRestMyNameActionRestPost {}

impl ActionRestMyNameActionRestPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/rest"
	}

	pub fn description() -> &'static str {
		"Recovers hit points by resting. (1 second per 5 HP, minimum 3 seconds)"
	}

	pub fn summary() -> &'static str {
		"Action Rest"
	}

	pub fn operation_id() -> &'static str {
		"action_rest_my__name__action_rest_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionRestMyNameActionRestPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionRestMyNameActionRestPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionTaskCancelMyNameActionTaskCancelPost {}

impl ActionTaskCancelMyNameActionTaskCancelPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/task/cancel"
	}

	pub fn description() -> &'static str {
		"Cancel a task for 1 tasks coin."
	}

	pub fn summary() -> &'static str {
		"Action Task Cancel"
	}

	pub fn operation_id() -> &'static str {
		"action_task_cancel_my__name__action_task_cancel_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionTaskCancelMyNameActionTaskCancelPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionTaskCancelMyNameActionTaskCancelPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionCompleteTaskMyNameActionTaskCompletePost {}

impl ActionCompleteTaskMyNameActionTaskCompletePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/task/complete"
	}

	pub fn description() -> &'static str {
		"Complete a task."
	}

	pub fn summary() -> &'static str {
		"Action Complete Task"
	}

	pub fn operation_id() -> &'static str {
		"action_complete_task_my__name__action_task_complete_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionCompleteTaskMyNameActionTaskCompletePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionCompleteTaskMyNameActionTaskCompletePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionTaskExchangeMyNameActionTaskExchangePost {}

impl ActionTaskExchangeMyNameActionTaskExchangePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/task/exchange"
	}

	pub fn description() -> &'static str {
		"Exchange 6 tasks coins for a random reward. Rewards are exclusive items or resources."
	}

	pub fn summary() -> &'static str {
		"Action Task Exchange"
	}

	pub fn operation_id() -> &'static str {
		"action_task_exchange_my__name__action_task_exchange_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionTaskExchangeMyNameActionTaskExchangePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionTaskExchangeMyNameActionTaskExchangePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionAcceptNewTaskMyNameActionTaskNewPost {}

impl ActionAcceptNewTaskMyNameActionTaskNewPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/task/new"
	}

	pub fn description() -> &'static str {
		"Accepting a new task."
	}

	pub fn summary() -> &'static str {
		"Action Accept New Task"
	}

	pub fn operation_id() -> &'static str {
		"action_accept_new_task_my__name__action_task_new_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionAcceptNewTaskMyNameActionTaskNewPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionAcceptNewTaskMyNameActionTaskNewPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionTaskTradeMyNameActionTaskTradePost {}

impl ActionTaskTradeMyNameActionTaskTradePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/task/trade"
	}

	pub fn description() -> &'static str {
		"Trading items with a Tasks Master."
	}

	pub fn summary() -> &'static str {
		"Action Task Trade"
	}

	pub fn operation_id() -> &'static str {
		"action_task_trade_my__name__action_task_trade_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionTaskTradeMyNameActionTaskTradePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionTaskTradeMyNameActionTaskTradePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionUnequipItemMyNameActionUnequipPost {}

impl ActionUnequipItemMyNameActionUnequipPost {
	pub fn path() -> &'static str {
		"/my/{name}/action/unequip"
	}

	pub fn description() -> &'static str {
		"Unequip an item on your character."
	}

	pub fn summary() -> &'static str {
		"Action Unequip Item"
	}

	pub fn operation_id() -> &'static str {
		"action_unequip_item_my__name__action_unequip_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionUnequipItemMyNameActionUnequipPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionUnequipItemMyNameActionUnequipPost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ActionUseItemMyNameActionUsePost {}

impl ActionUseItemMyNameActionUsePost {
	pub fn path() -> &'static str {
		"/my/{name}/action/use"
	}

	pub fn description() -> &'static str {
		"Use an item as a consumable."
	}

	pub fn summary() -> &'static str {
		"Action Use Item"
	}

	pub fn operation_id() -> &'static str {
		"action_use_item_my__name__action_use_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"My characters",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for ActionUseItemMyNameActionUsePost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			ActionUseItemMyNameActionUsePost::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllNpcsNpcsDetailsGet {}

impl GetAllNpcsNpcsDetailsGet {
	pub fn path() -> &'static str {
		"/npcs/details"
	}

	pub fn description() -> &'static str {
		"Fetch NPCs details."
	}

	pub fn summary() -> &'static str {
		"Get All Npcs"
	}

	pub fn operation_id() -> &'static str {
		"get_all_npcs_npcs_details_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"NPCs",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllNpcsNpcsDetailsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllNpcsNpcsDetailsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetNpcNpcsDetailsCodeGet {}

impl GetNpcNpcsDetailsCodeGet {
	pub fn path() -> &'static str {
		"/npcs/details/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a NPC."
	}

	pub fn summary() -> &'static str {
		"Get Npc"
	}

	pub fn operation_id() -> &'static str {
		"get_npc_npcs_details__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"NPCs",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetNpcNpcsDetailsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetNpcNpcsDetailsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllNpcsItemsNpcsItemsGet {}

impl GetAllNpcsItemsNpcsItemsGet {
	pub fn path() -> &'static str {
		"/npcs/items"
	}

	pub fn description() -> &'static str {
		"Retrieve the list of all NPC items."
	}

	pub fn summary() -> &'static str {
		"Get All Npcs Items"
	}

	pub fn operation_id() -> &'static str {
		"get_all_npcs_items_npcs_items_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"NPCs",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllNpcsItemsNpcsItemsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllNpcsItemsNpcsItemsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetNpcItemsNpcsItemsCodeGet {}

impl GetNpcItemsNpcsItemsCodeGet {
	pub fn path() -> &'static str {
		"/npcs/items/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the items list of a NPC. If the NPC has items to buy, sell or trade, they will be displayed."
	}

	pub fn summary() -> &'static str {
		"Get Npc Items"
	}

	pub fn operation_id() -> &'static str {
		"get_npc_items_npcs_items__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"NPCs",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetNpcItemsNpcsItemsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetNpcItemsNpcsItemsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllResourcesResourcesGet {}

impl GetAllResourcesResourcesGet {
	pub fn path() -> &'static str {
		"/resources"
	}

	pub fn description() -> &'static str {
		"Fetch resources details."
	}

	pub fn summary() -> &'static str {
		"Get All Resources"
	}

	pub fn operation_id() -> &'static str {
		"get_all_resources_resources_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Resources",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllResourcesResourcesGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllResourcesResourcesGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetResourceResourcesCodeGet {}

impl GetResourceResourcesCodeGet {
	pub fn path() -> &'static str {
		"/resources/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a resource."
	}

	pub fn summary() -> &'static str {
		"Get Resource"
	}

	pub fn operation_id() -> &'static str {
		"get_resource_resources__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Resources",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetResourceResourcesCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetResourceResourcesCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllTasksTasksListGet {}

impl GetAllTasksTasksListGet {
	pub fn path() -> &'static str {
		"/tasks/list"
	}

	pub fn description() -> &'static str {
		"Fetch the list of all tasks."
	}

	pub fn summary() -> &'static str {
		"Get All Tasks"
	}

	pub fn operation_id() -> &'static str {
		"get_all_tasks_tasks_list_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Tasks",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllTasksTasksListGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllTasksTasksListGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetTaskTasksListCodeGet {}

impl GetTaskTasksListCodeGet {
	pub fn path() -> &'static str {
		"/tasks/list/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a task."
	}

	pub fn summary() -> &'static str {
		"Get Task"
	}

	pub fn operation_id() -> &'static str {
		"get_task_tasks_list__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Tasks",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetTaskTasksListCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetTaskTasksListCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllTasksRewardsTasksRewardsGet {}

impl GetAllTasksRewardsTasksRewardsGet {
	pub fn path() -> &'static str {
		"/tasks/rewards"
	}

	pub fn description() -> &'static str {
		"Fetch the list of all tasks rewards. To obtain these rewards, you must exchange 6 task coins with a tasks master."
	}

	pub fn summary() -> &'static str {
		"Get All Tasks Rewards"
	}

	pub fn operation_id() -> &'static str {
		"get_all_tasks_rewards_tasks_rewards_get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Tasks",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetAllTasksRewardsTasksRewardsGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetAllTasksRewardsTasksRewardsGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetTasksRewardTasksRewardsCodeGet {}

impl GetTasksRewardTasksRewardsCodeGet {
	pub fn path() -> &'static str {
		"/tasks/rewards/{code}"
	}

	pub fn description() -> &'static str {
		"Retrieve the details of a tasks reward."
	}

	pub fn summary() -> &'static str {
		"Get Tasks Reward"
	}

	pub fn operation_id() -> &'static str {
		"get_tasks_reward_tasks_rewards__code__get"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Tasks",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::GET
	}


}

impl Endpoint for GetTasksRewardTasksRewardsCodeGet {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GetTasksRewardTasksRewardsCodeGet::http_method()
		}


}


#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GenerateTokenTokenPost {}

impl GenerateTokenTokenPost {
	pub fn path() -> &'static str {
		"/token"
	}

	pub fn description() -> &'static str {
		"Use your account as HTTPBasic Auth to generate your token to use the API. You can also generate your token directly on the website."
	}

	pub fn summary() -> &'static str {
		"Generate Token"
	}

	pub fn operation_id() -> &'static str {
		"generate_token_token_post"
	}

	pub fn tags() -> [&'static str; 1] {
		[
			"Token",
		]
	}

	pub fn http_method() -> http::HttpMethod {
		http::Method::POST
	}


}

impl Endpoint for GenerateTokenTokenPost {
		type Response = TODO;
		type RequestBody = TODO;

		fn http_request_method() -> http::Method {
			GenerateTokenTokenPost::http_method()
		}


}

