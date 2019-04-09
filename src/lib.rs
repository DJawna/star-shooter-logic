pub mod star_shooter_objects {

    pub struct imagefiles{
        id: Vec<u32>,
        paths: Vec<String>
    }

    pub struct textures{
        id: Vec<u32>,
        imageids: Vec<u32>,
        xcoordinates: Vec<u32>,
        ycoordinates: Vec<u32>,
        widths: Vec<u32>,
        heights: Vec<u32>
    }

    /***  game objects contain the positions of the gameobjects ***/
    pub struct game_objects{
        id: Vec<u32>,
        x_coordinates: Vec<f32>,
        y_coordinates: Vec<f32>,
        widths: Vec<f32>,
        heights: Vec<f32>,
        texture_ids: Vec<u32>
    }

    /*** saves the destructables ***/
    pub struct destructables{
        game_object_ids: Vec<u32>,
        hitpoints: Vec<u32>,
        x_collboxoffsets: Vec<f32>,
        y_collboxoffsets: Vec<f32>,
        widthoffsets: Vec<f32>,
        heightoffsets: Vec<f32>
    }

    pub enum movement_input{
        move_up,
        move_down,
        move_left,
        move_right,
        primary_fire,
        select,
        esc        
    }

    pub enum screen{
        start_screen,
        continue_screen,
        options_screen,
        confirm_resolution_screen,
        highscore_screen,
        level_screen,
        pause_screen
    }

    pub struct level_state{
        
    }


    pub fn bar()-> u32{
        42
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }



}
