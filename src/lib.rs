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

    pub struct space_objects{
        x_coordinates: Vec<f32>,
        y_coordinates: Vec<f32>,
        widths: Vec<f32>,
        heights: Vec<f32>,
        texture_ids: Vec<u32>
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
