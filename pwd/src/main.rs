use std::env;
use std::io::{self, Write};
use std::path;

fn main() -> std::io::Result<()> {
    let binding = env::current_dir().unwrap();
    let temp = binding.as_path();

    match env::args()
        .nth(1)
        .unwrap_or("".to_string())
        .to_lowercase()
        .as_str()
    {
        "--help" => {
            io::stdout().write_all(
                b"Usage: pwd [OPTION]...
Print the full filename of the current working directory.

  -L, --logical   use PWD from environment, even if it contains symlinks
  -P, --physical  avoid all symlinks
      --help        display this help and exit
      --version     output version information and exit

If no option is specified, -L is assumed.",
            )?;
        }
        "-v" | "--version" => {
            io::stdout().write_all(b"The only version")?;
        }
        "-p" | "--physical" => {
            io::stdout().write_all(temp.to_str().unwrap().as_bytes())?;
        }
        "" | "-l" | "--logical" => {
            if path::Path::is_symlink(temp) {
                io::stdout().write_all(
                    path::Path::read_link(temp)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .as_bytes(),
                )?;
            } else {
                io::stdout().write_all(temp.to_str().unwrap().as_bytes())?;
            }
        }
        _ => {
            panic!("Something is wrong");
        }
    };
    Ok(())
}
