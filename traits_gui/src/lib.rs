

pub trait Draw {
    fn draw(&self);
}


//* A struct that receives a vec of the *SAME* generic types which implements Draw */
//* we can only create a vector of the same component bcause compiler needs to know the size(stack) of the component(struct) */
pub struct ScreenD<T: Draw> {
     pub components: Vec<T>
 }

 impl<T> ScreenD<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
 } 


 /** A struct that receives a vec of the *SAME* generic types which implements Draw */
 /** we wrap it inside a Box with dynamic types,  */

 pub struct Screen {
     pub components: Vec<Box<dyn Draw>>
 }

 impl Screen {
     pub fn run(&self) {
         for component in self.components.iter() {
             component.draw();
         }
     }
 }


 //** GUI Components */

 pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button {} {} {}", self.width, self.height, self.label)
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing Button {} {} {:?}", self.width, self.height, self.options)
    }
}