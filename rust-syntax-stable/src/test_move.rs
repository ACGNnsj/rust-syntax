struct Moved{
    name: String,
    age: u32,
}

impl Moved{
    fn finish(mut self)->u32{
        self.name = String::from("Finished");
        println!("Moved");
        self.age
    }
}

#[test]
fn test_move(){
    let mut moved =  Moved{
        name: String::from("Moved"),
        age: 20,
    };
    moved.finish();
    // print!("{} {}", moved.name, moved.age);
    // moved.finish(); // value used here after move
    let never  = panic!("Never");
    let n={return (never)};
}