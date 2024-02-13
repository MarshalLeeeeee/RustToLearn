pub struct module2_struct {
    pub x: u32,
    y: u32,
}

impl module2_struct {
    pub fn init(x: u32, y: u32) -> module2_struct {
        module2_struct { x, y }
    }
    pub fn print_self(&self) {
        println!("module2 struct x: {}, y: {}", self.x, self.y);
    }
}

pub enum module2_enum {
    X(u32),
    Y(u32),
}

impl module2_enum {
    pub fn print_self(&self) {
        match self {
            module2_enum::X(x) => println!("module2 enum x: {}", x),
            module2_enum::Y(y) => println!("module2 enum y: {}", y),
        }
    }
}