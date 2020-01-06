use std::env;

// 1) Grab command line numbers
    //    - args[1]: width
    //    - args[2]: height

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!();
    }

    let _width: u32 = args[1].trim().parse()
    .expect("Please type a number!");
    let _height = args[2].trim().parse()
    .expect("Please type a number!");

    // TODO: Remove this, this was testing the terrain generation print order
    // 
    for h in 0.._height {
        print!("h-{}: \t", h);

        for w in 0.._width {
            print!("w-{} ", w);
        }
        
        println!();
    }
    // 2) generate world
}
