trait Speaker {
    fn speak(&self);
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Speaker for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

struct Human;
impl Speaker for Human {
    fn speak(&self) {
        println!("Damn!");
    }
}

fn test_dyn_trait() {
    let mut speakers: Vec<Box<dyn Speaker>> = Vec::new();
    speakers.push(Box::new(Dog));
    speakers.push(Box::new(Cat));
    speakers.push(Box::new(Human));
    for speaker in speakers {
        speaker.speak();
    }
}

fn main() {
    test_dyn_trait();
}
