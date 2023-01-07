#![warn(clippy::all, clippy::pedantic)]
// echoe - echo enhanced
// Copyright 2023 by Jim Lawless
// X11/MIT license
// See Github repo for full license

// This tiny utility was inspired by an old C program I had written called echof. 
// echof  had the ability to embed hex-encoded bytes in the output from an echo command
// workalike.  I thought I'd try an alternate approach with a few escape sequences
// that would permit the inclusion of some common escape sequences for tabs,
// carriage-returns, and newlines.  

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();    
    if args.len()==1 {
        print!("\nechoe - An enhanced echo command by Jim Lawless\n");
        print!("\nYou may include \\n, \\r, \\t, and \\\\ in arguments to echoe.\n");
        print!("You must add \\n at the end of the invocation if you want to end the output with a newline.\n");
        print!("\nExample:\n\techoe \"This\\tis\\na\\ttest.\\n\"\n\n");
        print!("Please see https://github.com/jimlawless/echoe/\n");
        return;
    }
    for val in args[1..].iter() {
        let mut esc = false;
        for c in val.chars() {
            if esc {
                match c {
                    'n' => print!("\n"),
                    'r' => print!("\r"),
                    't' => print!("\t"),
                    '\\' => print!("\\"),
                    _ => print!("\\{}",c),
                }                                
                esc=false;
            }
            else {
                match c {
                    '\\' => esc=true,
                    _ => print!("{}",c),
                }
            }
        }
    }
}
