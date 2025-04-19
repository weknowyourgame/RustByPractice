// Constants are immutable values that are bound to a name and are not allowed to change throughout
// the execution of a program. They can be declared in any scope, including global.
// Declared using const keyword. Naming convention is iper case characters and underscore for spaces

const GLOBAL_CONSTANT: u32 = 100_000;

fn main(){
    println!("{}", GLOBAL_CONSTANT);

    const ONE: u32 = 1;
    println!("{}", ONE);

    const PI: f32 = 3.14159;
    println!("{}", PI);

    const TRUE: bool = true;
    println!("{}", TRUE);

    const CRAB: char = '🦀';
    println!("{}", CRAB);

    const TUPLE: (u32, f32, bool, char) = (ONE, PI, TRUE, CRAB);
    println!("{:?}", TUPLE);

    const ARRAY: [u32; 3] = [ONE, ONE, ONE];
    println!("{:?}", ARRAY);

    const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
    println!("{}", SECONDS_IN_A_DAY);
}
