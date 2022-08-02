use std::env;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main(){
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");
    
    let x = input.len();
    let comment = "###";
    let mut y = "=============================="; 
    let mut z = "==============================";
   
    // cleaner if just one line.  Need to clean up.
    let output = format!(
           "{}{}{}\n{}{}{}{}\n{}{}{}",
           &comment,
           " ",
           &y,
           &comment,
           " ",
           (0..(32 - input.len()) / 2).map(|_| " ").collect::<String>(),
           input.to_uppercase(),
           &comment,
           " ",
           &z
        );

    println!("{}", output);

    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
    ctx.set_contents(output.to_owned()).unwrap();

    assert_eq!(ctx.get_contents().unwrap(), output);
}


