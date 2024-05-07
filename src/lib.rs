use colored::Colorize;
use std::io::{stdout, Write};
use std::ops::Div;
use std::thread;
use std::time::Duration;
pub mod args;
pub mod check;
pub mod connect;
pub mod format;

/// function animation
pub trait Animation: AsRef<str> {
    fn animation(&self);
}
impl<T: AsRef<str>> Animation for T {
    fn animation(&self) {
        for caracter in self.as_ref().chars() {
            let colored_text =
                caracter
                    .to_string()
                    .truecolor(0xff, random_number(), random_number());
            print!("{}", colored_text);

            stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(0x1));
        }
        println!();
    }
}

fn random_number() -> u8 {
    let rand = rand::random::<u8>();

    rand
}

pub fn ArtProgram() {
    r#"
  ______   __                            __              ________          __  __
 /      \ /  |                          /  |            /        |        /  |/  |
/$$$$$$  |$$ |____    ______    _______ $$ |   __       $$$$$$$$/__    __ $$ |$$ |
$$ |  $$/ $$      \  /      \  /       |$$ |  /  |      $$ |__  /  |  /  |$$ |$$ |
$$ |      $$$$$$$  |/$$$$$$  |/$$$$$$$/ $$ |_/$$/       $$    | $$ |  $$ |$$ |$$ |
$$ |   __ $$ |  $$ |$$    $$ |$$ |      $$   $$<        $$$$$/  $$ |  $$ |$$ |$$ |
$$ \__/  |$$ |  $$ |$$$$$$$$/ $$ \_____ $$$$$$  \       $$ |    $$ \__$$ |$$ |$$ |
$$    $$/ $$ |  $$ |$$       |$$       |$$ | $$  |      $$ |    $$    $$/ $$ |$$ |
 $$$$$$/  $$/   $$/  $$$$$$$/  $$$$$$$/ $$/   $$/       $$/      $$$$$$/  $$/ $$/

    "#
    .animation();
    let ok = "Create By ".bright_green();
    Writecolor("000 - Exit\n");
    Writecolor("Bypass All Domain [100]\n");
    Writecolor("Create By Lanbyshell\n");
}
pub fn Writecolor<T: AsRef<str>>(write: T) {
    let write = write.as_ref();
    match random_number() {
        n if n >= 0xa && n < 0x14 => {
            println!("{}", write.bright_purple());
        }
        n if n < 0xa => {
            println!("{}", write.bright_magenta());
        }
        n if n >= 0x14 && n < 0x32 => {
            println!("{}", write.bright_blue());
        }
        n if n >= 0x32 && n < 0x50 => {
            println!("{}", write.bright_cyan());
        }
        n if n >= 0x50 && n < 0x78 => {
            println!("{}", write.bright_yellow());
        }
        n if n >= 0x78 && n < 0xA0 => {
            println!("{}", write.bright_red());
        }
        n if n >= 0xA0 && n < 0xC8 => {
            println!("{}", write.bright_white());
        }
        n if n >= 0xC8 && n <= 0xFF => {
            println!("{}", write.bright_green());
        }
        _ => {
            println!("{}", write.bright_green());
        }
    }
}
