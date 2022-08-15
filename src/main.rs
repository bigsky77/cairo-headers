use std::env;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;
use structopt::clap::arg_enum;

arg_enum! {
#[derive(Debug, PartialEq)]
    enum SubCommand {
        H,
        F,
        L,
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(case_insensitive = true)]
    arg: SubCommand,

    #[structopt(default_value="")]
    header: String,

    #[structopt(default_value="")]
    notice: String,

    #[structopt(default_value="")]
    dev: String,

    #[structopt(default_value="")]
    param_1: String,

    #[structopt(default_value="")]
    param_2: String,

    #[structopt(default_value="")]
    param_3: String,

    #[structopt(default_value="")]
    param_4: String,
}

fn main() {
    let opt = Opt::from_args();

    if opt.arg == SubCommand::F {
        let notice = opt.notice;
        let dev = opt.dev;
        let p_1 = opt.param_1;
        let p_2 = opt.param_2;
        let p_3 = opt.param_3;
        let p_4 = opt.param_4;

        let output = format!(
            "{}{}\n{}{}\n{}{}",
            "# notice:",
            notice,
            "# dev:",
            dev,
            "# param:",
            p_1,
            );

        println!("{}", output);

        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
        ctx.set_contents(output.to_owned()).unwrap();
    
    } else if opt.arg == SubCommand::L {
        let input = opt.header;  
       
        if (input.len()%2)==0 {
        let output = format!(
            "{}{}{}{}{}{}{}",
            "###",
            " ",
            (0..(32 - input.len()) / 2).map(|_| "=").collect::<String>(),
            " ",
            input,
            " ",
            (0..(32 - input.len()) / 2).map(|_| "=").collect::<String>(),
            );

        println!("{}", output);

        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
        ctx.set_contents(output.to_owned()).unwrap();
        }
        else {
        let output = format!(
            "{}{}{}{}{}{}{}",
            "###",
            " ",
            (0..(32 - input.len()) / 2).map(|_| "=").collect::<String>(),
            " ",
            input,
            " ",
            (0..((32 - input.len()) / 2) + 1).map(|_| "=").collect::<String>(),
            );

        println!("{}", output);

        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
        ctx.set_contents(output.to_owned()).unwrap();
        };

    } else {
        let input = opt.header;

        let output = format!(
        "{}{}{}\n{}{}{}{}\n{}{}{}",
        "###",
        " ",
        "==================================",
        "###",
        " ",
        (0..(34 - input.len()) / 2).map(|_| " ").collect::<String>(),
        input.to_uppercase(),
        "###",
        " ",
        "==================================",
        );
        
        println!("{}", output);

        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    
        ctx.set_contents(output.to_owned()).unwrap();
    }


}
