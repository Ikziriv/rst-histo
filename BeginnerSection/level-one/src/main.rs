fn main() {
    // Variable
    let x: i8 = 88;
    let mut y = 99;
    /*
        Defined variable using let and specification (Data type variable) the variable
        like typescript if interger i8, f8, bool or sting, float etc.
    */

    // Comment on Rust
    println!("Hello, world {} or {}", x, y);
    /*
        Block Comment for the detail
        And bug can place this section.
    */

    // Condition IF ELSE
    let n: i8 = 11;
    if n == 20 {
        println!("Nilai n {} anda adalah 20", n);
    } else if n > 8 {
        println!("Nilai n {} anda kurang dari standar 8", n);
    } else {
        println!("Nilai n {} anda kurang dari 11", n);
    }

    // Looping using loop, while, for
    let mut n = 0;
    loop {
        n += 1;
        if n == 6 {
            continue;
        }
        if n > 10 {
            break;
        }
        println!("nilai n {} perulangan anda", n);
    }
    
    while n <= 8 {
        // If n is multiple of 6
        if n % 6 == 0 {
            println!("nilai n {} perulangan anda", n);
        }

        n += 1;
    }

    let number = 30..51;
    let animals = vec!["Cat", "Bird", "Dog"];
    for j in number {
        println!("The number you have {}", j);
    }
    
    for (index, a) in animals.iter().enumerate() {
        println!("Nomer Urut {} The number you have {}", index, a);
    }

    // Enum Types
    enum Direction {
        Up, Down, Left, Right
    }

    // let player_direction::Direction = Direction::Up;
    // match player_direction {
    //     Direction::Up => println!("Heading Up!"),
    //     Direction::Down => println!("Heading Down!"),
    //     Direction::Left => println!("Heading Left!"),
    //     Direction::Right => println!("Heading Right!"),
    // }
}
