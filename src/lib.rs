pub mod star_shooter_logic {
    pub mod game_objects {

        enum CollidableObjectState{
            Inactive,
            Spawning,
            Idle,
            StandardAttack,
            SpecialAttack,
            Dying
        }

        pub struct Rectangle{
            pub x: f32,
            pub y: f32,
            pub width: f32,
            pub height: f32
        }

        impl Rectangle{
            fn does_overlap(&self, other: &Rectangle) -> bool {
                fn number_within_segment(number_to_test: f32, beginn_of_segment: f32, segment_lenght: f32) -> bool{
                    (number_to_test >= beginn_of_segment) && (number_to_test <= (beginn_of_segment+ segment_lenght))
                }
                (number_within_segment(self.x, other.x,other.width) || number_within_segment(other.x,self.x,self.width))
                &&
                (number_within_segment(self.y, other.y,other.height) || number_within_segment(other.y,self.y,self.height))
            }

        }

        pub fn compute_all_collision_damage(rect: &crate::star_shooter_logic::game_objects::Rectangle, enemy_rectangles: &Vec<crate::star_shooter_logic::game_objects::Rectangle>, enemy_damage_values: &Vec<u32>) -> u32 {
            let mut damage_aggregate =0;
            for (index, enemy) in enemy_rectangles.iter().enumerate(){
                if rect.does_overlap(&enemy){
                    damage_aggregate += enemy_damage_values[index];
                }
            }
            return damage_aggregate;
        }

        #[test]
        fn rectange_does_overlap_other_completely_engulfed_true(){
            let engulfing_rectangle = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 11.,
                        y: 13.,
                        width: 20.,
                        height: 20.
            };

            let engulfed_rectangle = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 12.,
                        y: 14.,
                        width: 1.0,
                        height: 1.0
            };

            assert!(engulfing_rectangle.does_overlap(&engulfed_rectangle));

            assert!(engulfed_rectangle.does_overlap(&engulfing_rectangle));
        }

        #[test]
        fn rectangle_onlyoverlapson_y_but_not_x_false(){
            let rect_one = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 11.,
                        y: 13.,
                        width: 20.,
                        height: 20.
            };

            let rect_two = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 666.,
                        y: 14.,
                        width: 1.0,
                        height: 1.0
            };

            assert!(!rect_one.does_overlap(&rect_two));

            assert!(!rect_two.does_overlap(&rect_one));

        }

        // cont here: 
        #[test]
        fn rectangle_onlyoverlapson_x_but_not_y_false(){
            let rect_one = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 11.,
                        y: 13.,
                        width: 20.,
                        height: 20.
            };

            let rect_two = crate::star_shooter_logic::game_objects::Rectangle{
                        x: 12.,
                        y: 666.,
                        width: 1.0,
                        height: 1.0
            };

            assert!(!rect_one.does_overlap(&rect_two));

            assert!(!rect_two.does_overlap(&rect_one));

        }

    }

    mod ui_objects {
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
    }



}