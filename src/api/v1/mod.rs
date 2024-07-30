pub mod accounts;
pub mod characters;
pub mod events;
pub mod grand_exchange;
pub mod items;
pub mod maps;
pub mod monsters;
pub mod my_account;
pub mod my_characters;
pub mod resources;
pub mod status;
pub mod token;

pub trait QueryParameters {
    fn get_path(parameter: String) -> String;
}
