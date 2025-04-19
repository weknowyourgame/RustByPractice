fn main() {
    // A variant is assigned an integer value by default starting from 0.
    enum ExampleArgument {
        North,
        East,
        South,
        West,
    }

    let north = ExampleArgument::North;
    let east = ExampleArgument::East;
    let south = ExampleArgument::South;
    let west = ExampleArgument::West;

    // Enums are namespaced under its identifier, all four variants are the
    // same type 'ExampleArgument'. That allows us to define functions that
    // take 'ExampleArgument' as an argument:
    fn move_to(direction: ExampleArgument) {
        match direction {
            ExampleArgument::North => println!("Move north"),
            ExampleArgument::East => println!("Move east"),
            ExampleArgument::South => println!("Move south"),
            ExampleArgument::West => println!("Move west"),
        }
    }

    // Call the 'move_to' function with the variants of the 'ExampleArgument'
    move_to(north);
    move_to(east);
    move_to(south);
    move_to(west);

    // Enums can also have associated data. This is useful when you want to store additional information about an enum variant.
    enum Animal {
        Dog(String),
        Cat { name: String, age: u8 },
        Bird,
    }

    // To create an instance of the 'Animal' enum we can do
    let dog = Animal::Dog(String::from("Sam"));

    // The 'Cat' variant includes a name and age, so we need to provide values
    let cat = Animal::Cat {
        name: String::from("Tommy"),
        age: 15,
    };

    // The 'Bird' variant does not include any data, so we can create an instance
    let bird = Animal::Bird;

    // Enums can also have methods defined on them.
    impl Animal {
        // A method that returns the name of the animal.
        fn name(&self) -> String {
            match self {
                Animal::Dog(name) => name.clone(),
                Animal::Cat { name, .. } => name.clone(),
                Animal::Bird => String::from("Bird"),
            }
        }

        fn age(&self) -> u8 {
            match self {
                Animal::Cat { age, .. } => *age,
                _ => 0,
            }
        }
    }

    println!("The name of the dog is: {}", dog.name());
    println!(
        "The name of the cat is: {}, and its age is: {}",
        cat.name(),
        cat.age(),
    );
    println!("The name of the bird is: {}", bird.name());
}
