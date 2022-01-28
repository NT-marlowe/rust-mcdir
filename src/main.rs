use std::fs;

fn main() {
    println!("mkdir and cd");
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }   
    

    
}

