extern crate getopts;
use std::env;
use std::path::Path;
use std::io;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let level = level(&args);
    print_files(Path::new("./"), "", "", level).unwrap();
}

fn level(args: &Vec<String>) -> Option<u32> {
    let mut opts = Options::new();
    opts.optopt("L", "", "Number of levels to print", "LEVELS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    matches.opt_str("L").and_then(|level| level.parse().ok())
}

// TODO
//   don't show hidden files, -a option to show everything
//   option to only list directories
//   print usage
//   allow directory to be specified, default to .
//   check level option works the same as UNIX tree
//   put on github
//   documentation comments
fn print_files(path: &Path, indent: &str, child_indent: &str, level: Option<u32>) -> io::Result<()> {
    match path.file_name().and_then(|os_str| os_str.to_str()) {
        Some(name) => println!("{}{}", indent, name),
        None => println!("./"),
    }
    if path.is_dir() {
        // Print the children if no level was specified or if the remaining level is greater than zero
        let print_children = level.map(|l| l > 0).unwrap_or(true);
        if print_children {
            let mut children = try!(path.read_dir()).peekable();
            // Can't use a for loop because the loop borrows the iterator mutably which prevents the call to peek()
            loop {
                match children.next() {
                    Some(entry) => {
                        let (new_indent, new_child_indent) = if children.peek().is_none() {
                            (child_indent.to_string() + " `-- ", child_indent.to_string() + "    ")
                        } else {
                            (child_indent.to_string() + " |-- ", child_indent.to_string() + " |  ")
                        };
                        let file = try!(entry);
                        // Subtract one from the remaining level before passing it to the children
                        let next_level = level.map(|l| l - 1);
                        try!(print_files(&file.path(), &new_indent, &new_child_indent, next_level));
                    },
                    None => break,
                }
            }
        } else {
            println!("{}{}", child_indent, " `-- ...");
        }
    }
    Ok(())
}
