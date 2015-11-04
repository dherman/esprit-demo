// esprit parser
extern crate esprit;

// JS AST definitions
extern crate easter;

use esprit::script;
use easter::prog::Script;

use std::{env, io, thread};
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut result));
    Ok(result)
}

fn count_lines(script: &Script) -> usize {
    script.value.body.len()
}

const STACK_SIZE: usize = 4 * 1024 * 1024;

// because it's recursive descent, esprit requires a bigger stack than the default stack size
fn with_big_stack<F>(f: F)
    where F: FnOnce(),
          F: Send + 'static
{
    thread::Builder::new().stack_size(STACK_SIZE).spawn(f).unwrap().join().unwrap();
}

fn main() {
    with_big_stack(|| {
        for arg in env::args().skip(1) {
            match read_file(&arg[..]) {
                Err(_)  => { println!("error: failed to read {}", arg); }
                Ok(src) => {
                    match script(&src[..]) {
                        Err(_) => { println!("error: failed to parse {}", arg); }
                        Ok(s)  => {
                            let lines = count_lines(&s);
                            println!("{}: {} line{}", arg, lines, if lines == 1 { "" } else { "s" });
                        }
                    }
                }
            }
        }
    });
}
