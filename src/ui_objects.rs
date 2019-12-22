pub enum MovementInput {
    move_up,
    move_down,
    move_left,
    move_right,
    primary_fire,
    select,
    esc,
    none
}


pub enum Screen {
    start_screen,
    continue_screen,
    options_screen,
    confirm_resolution_screen,
    highscore_screen,
    level_screen,
    pause_screen
}
pub struct Button {
    pub boundaries: crate::game_objects::Rectangle,
    pub label: String
}

pub struct Menu {
    pub selectedButton: Option<Button>,
    pub buttons : Vec<Button>
}


pub struct Scene{

}