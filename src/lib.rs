pub mod star_shooter_logic {
    pub mod game_objects {
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
            pub id: Vec<u32>,
            pub visible_boxes: Vec<rectangle>,
            pub collision_boxes: Vec<rectangle>,
            pub texture_ids: Vec<u32>,
            pub hitpoints: Vec<u32>,
            pub collisiondamages: Vec<u32>
        }

        enum collidable_object_state{
            inactive,
            spawning,
            idle,
            standard_attack,
            special_attack,
            dying
        }

        pub struct rectangle{
            pub x: f32,
            pub y: f32,
            pub width: f32,
            pub height: f32
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
            alied_objects: crate::star_shooter_logic::game_objects::collidable_object_table,
            neutral_objects: crate::star_shooter_logic::game_objects::collidable_object_table
            
        }
    }

    pub fn compute_all_collision_damage(rect: &crate::star_shooter_logic::game_objects::rectangle, enemyRectangles: &Vec<crate::star_shooter_logic::game_objects::rectangle>, enemyDamageValues: &Vec<u32>) -> u32 {
        return 0;
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_all_collision_damage_test() {
        let enemies = crate::star_shooter_logic::game_objects::collidable_object_table{
            id : vec!(1,2),
            visible_boxes: vec!(
                crate::star_shooter_logic::game_objects::rectangle{
                    x: 0.,
                    y: 0.,
                    width: 20.,
                    height: 20.
                },
                crate::star_shooter_logic::game_objects::rectangle{
                    x: 100.,
                    y: 100.,
                    width: 20.,
                    height: 20.
                }
            ),
            collision_boxes: vec!(
                crate::star_shooter_logic::game_objects::rectangle{
                    x: 0.,
                    y: 0.,
                    width: 20.,
                    height: 20.
                },
                crate::star_shooter_logic::game_objects::rectangle{
                    x: 100.,
                    y: 100.,
                    width: 20.,
                    height: 20.
                }
            ),
            texture_ids: vec!(0,0),
            hitpoints: vec!(20,20),
            collisiondamages: vec!(5,0)
        };

        let mut hero = crate::star_shooter_logic::game_objects::rectangle{
            x: 0.,
            y: 0.,
            width: 5.,
            height: 5.
        };

        {
            let actualResult = crate::star_shooter_logic::compute_all_collision_damage(&hero,&enemies.collision_boxes,&enemies.collisiondamages);

            assert_eq!(actualResult, 5);
        }

        hero.x = 100.;
        hero.y = 100.;

        {
            let actualResult = crate::star_shooter_logic::compute_all_collision_damage(&hero,&enemies.collision_boxes,&enemies.collisiondamages);

            assert_eq!(actualResult, 0);

        }
    }

}
