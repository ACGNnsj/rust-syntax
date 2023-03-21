trait Greeting {
    fn greeting(&self) -> String;
}

struct Cat {
    name: String,
}

impl Cat {
    fn new(name: &str) -> Cat {
        Cat {
            name: name.to_string(),
        }
    }
}

impl Greeting for Cat {
    fn greeting(&self) -> String {
        "Meow!".to_string() + &self.name
    }
}

struct Mouse {
    name: String,
}

impl Greeting for Mouse {
    fn greeting(&self) -> String {
        "Squeak!".to_string() + &self.name
    }
}

struct Dog;

impl Greeting for Dog {
    fn greeting(&self) -> String {
        "Woof!".to_string()
    }
}

fn print_greeting_static<G: Greeting>(g: G) {
    println!("{}", g.greeting());
}

fn print_greeting_dynamic(g: Box<dyn Greeting>) {
    println!("{}", g.greeting());
}

fn print_greeting_dynamic_alter(g: &dyn Greeting) {
    println!("{}", g.greeting());
}

fn print_greeting_vec_dynamic(v: Vec<Box<dyn Greeting>>) {
    for g in v {
        println!("{}", g.greeting());
    }
}

#[test]
fn test_trait_object() {
    let cat: Cat = Cat { name: "Tom".to_string() };
    let dog: Dog = Dog;
    let mouse: Mouse = Mouse { name: "Jerry".to_string() };
    print_greeting_static(Cat { name: "Tom".parse().unwrap() });
    print_greeting_static(Dog);
    print_greeting_static(Mouse { name: "Jerry".parse().unwrap() });
    print_greeting_dynamic(Box::new(Cat { name: "Tom".to_string() }));
    print_greeting_dynamic(Box::new(Dog));
    print_greeting_dynamic(Box::new(Mouse { name: "Jerry".to_string() }));
    print_greeting_dynamic_alter(&cat);
    print_greeting_dynamic_alter(&dog);
    print_greeting_dynamic_alter(&mouse);
    let c: &dyn Greeting = &Cat::new("Tom");
    let d: &dyn Greeting = &Dog;
    print_greeting_dynamic_alter(c);
    print_greeting_static(Cat::new("t"));
    let v: Vec<Box<dyn Greeting>> = vec![Box::new(Cat::new("Tom")), Box::new(Dog), Box::new(Mouse { name: "Jerry".to_string() })];
    print_greeting_vec_dynamic(v);
}