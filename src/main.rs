use std::env;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main(){
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");
    
    let output = format!(
           "{}{}{}{}{}",
           "### ============",
           " ",
           (0..(64 - input.len()) / 2).map(|_| "").collect::<String>(),
           input.to_uppercase(),
           " ============"
        );

    println!("{}", output);

    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
    ctx.set_contents(output.to_owned()).unwrap();

    assert_eq!(ctx.get_contents().unwrap(), output);
}


