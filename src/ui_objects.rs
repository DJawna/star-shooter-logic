pub enum MovementInput {
    move_up,
    move_down,
    move_left,
    move_right,
    primary_fire,
    select,
    esc
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