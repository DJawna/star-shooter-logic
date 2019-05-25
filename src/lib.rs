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

        impl rectangle{
            pub fn does_overlap(&self, other: &rectangle) -> bool {
                fn number_within_segment(number_to_test: f32, beginn_of_segment: f32, segment_lenght: f32) -> bool{
                    (number_to_test >= beginn_of_segment) && (number_to_test <= (beginn_of_segment+ segment_lenght))
                }
                (number_within_segment(self.x, other.x,other.width) || number_within_segment(other.x,self.x,self.width))
                &&
                (number_within_segment(self.y, other.y,other.height) || number_within_segment(other.y,self.y,self.height))
            }

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
        let mut damage_aggregate =0;
        for (index, enemy) in enemyRectangles.iter().enumerate(){
            if rect.does_overlap(&enemy){
                damage_aggregate += enemyDamageValues[index];
            }
        }
        return damage_aggregate;
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn rectange_doesOverlap_otherCompletelyEngulfed_true(){
        let engulfingRectangle = crate::star_shooter_logic::game_objects::rectangle{
                    x: 11.,
                    y: 13.,
                    width: 20.,
                    height: 20.
        };

        let engulfedRectangle = crate::star_shooter_logic::game_objects::rectangle{
                    x: 12.,
                    y: 14.,
                    width: 1.0,
                    height: 1.0
        };

        assert!(engulfingRectangle.does_overlap(&engulfedRectangle));

        assert!(engulfedRectangle.does_overlap(&engulfingRectangle));
    }

    #[test]
    fn rectangle_onlyoverlapson_x_but_not_y_false(){
        let rectOne = crate::star_shooter_logic::game_objects::rectangle{
                    x: 11.,
                    y: 13.,
                    width: 20.,
                    height: 20.
        };

        let rectTwo = crate::star_shooter_logic::game_objects::rectangle{
                    x: 12.,
                    y: 666.,
                    width: 1.0,
                    height: 1.0
        };

        assert!(!rectOne.does_overlap(&rectTwo));

        assert!(!rectTwo.does_overlap(&rectOne));

    }


    #[test]
    fn rectangle_onlyoverlapson_y_but_not_x_false(){
        let rectOne = crate::star_shooter_logic::game_objects::rectangle{
                    x: 11.,
                    y: 13.,
                    width: 20.,
                    height: 20.
        };

        let rectTwo = crate::star_shooter_logic::game_objects::rectangle{
                    x: 666.,
                    y: 14.,
                    width: 1.0,
                    height: 1.0
        };

        assert!(!rectOne.does_overlap(&rectTwo));

        assert!(!rectTwo.does_overlap(&rectOne));

    }

    #[test]
    fn compute_all_collision_damage_test() {
        let projectiles = crate::star_shooter_logic::game_objects::collidable_object_table{
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

        let mut damage_receiver = crate::star_shooter_logic::game_objects::rectangle{
            x: 0.,
            y: 0.,
            width: 5.,
            height: 5.
        };

        {
            let actualResult = crate::star_shooter_logic::compute_all_collision_damage(&damage_receiver,&projectiles.collision_boxes,&projectiles.collisiondamages);

            assert_eq!(actualResult, 5);
        }

        damage_receiver.x = 100.;
        damage_receiver.y = 100.;

        {
            let actualResult = crate::star_shooter_logic::compute_all_collision_damage(&damage_receiver,&projectiles.collision_boxes,&projectiles.collisiondamages);

            assert_eq!(actualResult, 0);

        }
    }

}
