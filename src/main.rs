fn get_title(rust_version: bool) -> String {
    let addition = if rust_version {
        " (Rustic)"
    } else {
        ""
    };
    format!("The Twelve{addition} Days of Christmas\nSung by Rust")
}

fn get_gifts(rust_version :bool) -> [&'static str; 12]  {
    if rust_version { // rust-themed gifts, generated with the help of chat-gpt
        [
            "a borrow checker in a pear tree",
            "two lifetime annotations",
            "three ownership models",
            "four fearless concurrency primitives",
            "five pattern matching arms",
            "six iterators iterating",
            "seven crates a-cargo-ing",
            "eight lifetimes a-lending",
            "nine macros expanding",
            "ten structs a-defining",
            "eleven trait implementations",
            "twelve async tasks a-awaiting",
        ]
    } else{ // regular gifts
        [
            "a partridge in a pear tree",
            "two turtle doves",
            "three French hens",
            "four calling birds",
            "five golden rings",
            "six geese a-laying",
            "seven swans a-swimming",
            "eight maids a-milking",
            "nine ladies dancing",
            "ten lords a-leaping",
            "eleven pipers piping",
            "twelve drummers drumming",
        ]
    }
}

fn sing(rust_version :bool) {
    let days:  [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let title: String = get_title(rust_version);
    let gifts: [&'static str; 12] = get_gifts(rust_version);
    let ndays = days.len();

    println!("{title}");
    for day in 0..ndays{
        println!("");
        let cur_day = days[day];
        let cur_gift = gifts[day];
        let refrain = format!("On the {cur_day} day of Christmas my true love gave to me {cur_gift}");
        if day > 0{
            print!("{refrain}");
            if day > 1 {
                println!{","};
            }
            println!("");
            for inner_day in (1..=day).rev(){
                let inner_gift = gifts[inner_day-1];
                if inner_day > 1 {
                    println!("{inner_gift},");
                } else{
                    println!("and {inner_gift}.");
                }
                
                
            }
        } else{
            println!("{refrain}.");
        }
        
    }
}

fn main() {
    sing(false)
}