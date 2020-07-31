// here goes everything for the organiation of all the components of the game

use crate::typedefs::KeyCode;
use std::collections::HashMap;


pub enum UserInputs {
    move_up,
    move_down,
    move_left,
    move_right,
    primary_fire,
    select, // for example when user selects a menu
    esc,
    use_special_item,
    pause
}



struct GameSettings{
    width: u32,
    height: u32,
    key_bindings: HashMap<KeyCode,UserInputs>
}

impl GameSettings {
    /**
    Creates defaultssettings for the game
    */
    fn CrreateDefaults(){
        GameSettings{
            width: 800,
            height: 600,
            key_bindings: hashmap!{
                // enter the default keybindings here
                1 => UserInputs::move_up,
                2 => UserInputs::move_down,
                3 => UserInputs::move_left,
                4 => UserInputs::move_right,
                5 => UserInputs::primary_fire,
                6 => UserInputs::select,
                7 => UserInputs::esc,
                8 => UserInputs::use_special_item,
                9 => UserInputs::pause
            }
        };
    }
}