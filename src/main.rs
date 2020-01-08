use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!();
    }

    let _width: u32 = args[1].trim().parse()
    .expect("Please type a number!");
    let _height: u32 = args[2].trim().parse()
    .expect("Please type a number!");
        
    //     println!();
    // }
    // 2) generate world
    //  a. ) Build background 
    //  b. ) Add clouds
    //  c. ) Add ground layer
    //  d. ) Add grass
    //  e. ) Add objets

    let ground_twenty_percent: f64 = (_height as f64 * 0.2).ceil();
    println!("{}", ground_twenty_percent);
    
}   
