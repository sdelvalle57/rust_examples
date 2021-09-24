
use traits_gui::{Button, Screen, ScreenD, SelectBox};

 /** Create a vector of components like Button, SelectBox, Dropdown.... */



fn main() {

    let select_box_1 = SelectBox {
        height: 20,
        width: 70,
        options: vec![]
    };

    let select_box_2 = SelectBox {
        height: 20,
        width: 70,
        options: vec![]
    };

    let screen = ScreenD {
        components: vec![select_box_1, select_box_2]
    };

    screen.run();



    let select_box = Box::new(
        SelectBox {
            height: 20,
            width: 70,
            options: vec![]
        }
    );

    let button = Box::new(
        Button {
            height: 19,
            width: 60,
            label: String::from("label")
        }
    );

    let screen = Screen {
        components: vec![select_box, button]
    };

    screen.run();


}
