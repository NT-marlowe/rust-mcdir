use std::env;
use std::fs;

fn main() {
    println!("mkdir and cd");
    
    let argv: Vec<String> = env::args().collect();
    assert!(argv.len() >= 2, "mcdir: missing operand");
    // println!("len = {}", argv.len());

    let dir_name = &argv[1];
    match fs::create_dir(dir_name) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }   
    
    println!("commandline args = {:?}" , argv);
}

