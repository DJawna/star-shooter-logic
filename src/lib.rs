pub mod star_shooter_logic {
    mod game_objects {
        pub struct imagefiletable {
            id: Vec<u32>,
            paths: Vec<String>,
        }

        pub struct texturetable {
            id: Vec<u32>,
            imageids: Vec<u32>,
            xcoordinates: Vec<u32>,
            ycoordinates: Vec<u32>,
            widths: Vec<u32>,
            heights: Vec<u32>,
        }

        /***  game objects contain the positions of the gameobjects ***/
        pub struct collidable_object_table {
            id: Vec<u32>,
            visible_boxes: Vec<rectangle>,
            collision_boxes: Vec<rectangle>,
            texture_ids: Vec<u32>,
            hitpoints: Vec<u32>,
            widthoffsets: Vec<f32>,
            heightoffsets: Vec<f32>,
        }

        enum collidable_object_state{
            inactive,
            spawning,
            idle,
            standard_attack,
            special_attack,
            dying
        }

        struct rectangle{
            x: f32,
            y: f32,
            width: f32,
            height: f32
        }
    }

    mod ui_objects {
        pub enum movement_input {
            move_up,
            move_down,
            move_left,
            move_right,
            primary_fire,
            select,
            esc
        }

        pub enum screen {
            start_screen,
            continue_screen,
            options_screen,
            confirm_resolution_screen,
            highscore_screen,
            level_screen,
            pause_screen
        }
    }


    mod state_objects{
        struct LevelState {
            hostile_objects: crate::star_shooter_logic::game_objects::collidable_object_table,
            alied_objects: crate::star_shooter_logic::game_objects::collidable_object_table
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
