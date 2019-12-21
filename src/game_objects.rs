enum CollidableObjectState{
    Inactive,
    Spawning,
    Idle,
    StandardAttack,
    SpecialAttack,
    Dying
}

pub struct Point<T> {
    pub x: T,
    pub y: T
}

pub struct Rectangle{
    pub position : Point<f32>,
    pub width: f32,
    pub height: f32
}

impl Rectangle{
    fn does_overlap(&self, other: &Rectangle) -> bool {
        fn number_within_segment(number_to_test: f32, beginn_of_segment: f32, segment_lenght: f32) -> bool{
            (number_to_test >= beginn_of_segment) && (number_to_test <= (beginn_of_segment+ segment_lenght))
        }
        (number_within_segment(self.position.x, other.position.x,other.width) || number_within_segment(other.position.x,self.position.x,self.width))
        &&
        (number_within_segment(self.position.y, other.position.y,other.height) || number_within_segment(other.position.y,self.position.y,self.height))
    }

}

pub fn compute_all_collision_damage(rect: Rectangle, enemy_rectangles: &Vec<Rectangle>, enemy_damage_values: &Vec<u32>) -> u32 {
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
    let engulfing_rectangle = Rectangle{
                position: Point{
                    x: 11.,
                    y: 13.
                },
                width: 20.,
                height: 20.
    };

    let engulfed_rectangle = Rectangle{
                position: Point{
                    x: 12.,
                    y: 14.
                },
                width: 1.0,
                height: 1.0
    };

    assert!(engulfing_rectangle.does_overlap(&engulfed_rectangle));

    assert!(engulfed_rectangle.does_overlap(&engulfing_rectangle));
}

#[test]
fn rectangle_onlyoverlapson_y_but_not_x_false(){
    let rect_one = Rectangle{
                position: Point{
                    x: 11.,
                    y: 13.
                },
                width: 20.,
                height: 20.
    };

    let rect_two = Rectangle{
                position: Point {
                    x: 666.,
                    y: 14.
                },
                width: 1.0,
                height: 1.0
    };

    assert!(!rect_one.does_overlap(&rect_two));

    assert!(!rect_two.does_overlap(&rect_one));

}

// cont here: 
#[test]
fn rectangle_onlyoverlapson_x_but_not_y_false(){
    let rect_one = Rectangle{
                position: Point{
                    x: 11.,
                    y: 13.
                },
                width: 20.,
                height: 20.
    };

    let rect_two = Rectangle{
                position: Point{
                    x: 12.,
                    y: 666.
                },
                width: 1.0,
                height: 1.0
    };

    assert!(!rect_one.does_overlap(&rect_two));

    assert!(!rect_two.does_overlap(&rect_one));

}