use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    fn new_short(major: u16) -> SemVer {
        Self::new(major, 0, 0)
    }

    // fn info(&self) {
    //     println!(
    //         "hi, I'm a semver: {}.{}.{}",
    //         self.major, self.minor, self.patch
    //     )
    // }

    // fn patch(&mut self) -> &mut u16 {
    //     &mut self.patch
    // }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::new_short(1)
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<&str> for SemVer {
    fn from(s: &str) -> Self {
        let vs: Vec<u16> = s.split(".").filter_map(|item| item.parse().ok()).collect();
        assert!(vs.len() == 3);
        SemVer {
            major: vs[0],
            minor: vs[1],
            patch: vs[2],
        }
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    release_history: Vec<SemVer>,
}

fn main() -> Result<(), std::io::Error> {
    // create a `Vec` to hold the list of programs
    let mut programs: Vec<Program> = vec![];

    // open "releases.txt", bail on error
    let file_path = "releases.txt";

    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

    // use a `BufReader` to iterate over the lines of the file handle
    let mut line_number = 1;
    for line in BufReader::new(file).lines() {
        // if the line can be read (it might be invalid data), split it on ","
        let line = match line {
            Ok(line) => line,
            Err(why) => panic!("error reading {} line {}: {}", file_path, line_number, why),
        };
        let mut parts = line.split(",");
        // take the first element of your split - that's the name
        let program_name = match parts.next() {
            Some(value) => value.to_string(),
            None => panic!(
                "error reading {}: line {} did not contain the first element",
                file_path, line_number
            ),
        };
        // the rest is a list of &str slices that each can be MAPPED INTO a SemVer!
        let versions: Vec<SemVer> = parts.map(|version_str| version_str.into()).collect();
        // we're still in iterator land - time to collect and push the result to our program vec
        programs.push(Program {
            name: program_name,
            release_history: versions,
        });

        line_number += 1;
    }

    // finally, print the program vec.
    for program in programs {
        let header = format!("Program {} release history", program.name);
        println!("{}", "-".repeat(header.len()));
        println!("{}", header);
        println!("{}", "-".repeat(header.len()));
        for version in program.release_history {
            println!("{}", version)
        }
    }

    Ok(())
}
