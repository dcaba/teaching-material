use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backup_file_path = "db.json";
    let backup_file = File::open(backup_file_path);
    let mut programs: Vec<semver::Program> = vec![];
    match backup_file {
        Ok(backup_file) => programs = from_backup(backup_file)?,
        Err(_) => from_release_txt("releases.txt", &mut programs),
    }

    let programs_export = serde_json::to_string_pretty(&programs)?;
    println!("{}", programs_export);

    let mut backup_file = File::create(backup_file_path)?;
    backup_file.write_all(programs_export.as_bytes())?;
    Ok(())
}

fn from_backup(backup_file: File) -> Result<Vec<semver::Program>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(backup_file);
    let programs = serde_json::from_reader(reader)?;
    Ok(programs)
}

fn from_release_txt(file_path: &str, programs: &mut Vec<semver::Program>) {
    // open "releases.txt", bail on error
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
        let versions: Vec<semver::SemVer> = parts.map(|version_str| version_str.into()).collect();
        // we're still in iterator land - time to collect and push the result to our program vec
        programs.push(semver::Program::new(program_name, versions));

        line_number += 1;
    }
}
