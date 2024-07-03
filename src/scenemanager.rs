//use minifb::{Key, Window, WindowOptions};
use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Screen {
    Title,
    Game,
    RocketSelect,
}