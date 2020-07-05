enum CollidableObjectState{
    Inactive,
    Spawning,
    Idle,
    StandardAttack,
    SpecialAttack,
    Dying
}

pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Rectangle{
    pub position : Point,
    pub width: f32,
    pub height: f32
}

fn does_overlap(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    fn number_within_segment(number_to_test: f32, beginn_of_segment: f32, segment_lenght: f32) -> bool{
        (number_to_test >= beginn_of_segment) && (number_to_test <= (beginn_of_segment + segment_lenght))
    }
    (number_within_segment(rect1.position.x, rect2.position.x,rect2.width) || number_within_segment(rect2.position.x,rect1.position.x,rect1.width))
    &&
    (number_within_segment(rect1.position.y, rect2.position.y,rect2.height) || number_within_segment(rect2.position.y,rect1.position.y,rect1.height))
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

    assert!(does_overlap(&engulfing_rectangle, &engulfed_rectangle));

    assert!(does_overlap(&engulfed_rectangle, &engulfing_rectangle));
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

    assert!(!does_overlap(&rect_one ,&rect_two));

    assert!(!does_overlap(&rect_two ,&rect_one));

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

    assert!(!does_overlap(&rect_one, &rect_two));

    assert!(!does_overlap(&rect_two, &rect_one));

}