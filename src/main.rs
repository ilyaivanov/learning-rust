use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Commands:");
        println!("  ○ add _Name_ to _Department_");
        println!("  ○ print _Department_ (if no dep all company will be printed)");
        println!("  ○ stop");

        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);

        let command = command.trim();
        if command.starts_with("add") {
            let words: Vec<&str> = command.split_whitespace().collect();

            match (words.get(1), words.get(3)) {
                (Some(person), Some(dep)) => {
                    let dep_ar = company.get_mut(*dep);
                    match dep_ar {
                        Some(dep_ar) => dep_ar.push(person.to_string()),
                        _ => {
                            let mut list: Vec<String> = Vec::new();
                            list.push(person.to_string());
                            company.insert(dep.to_string(), list);
                        }
                    }
                    println!("Added '{}' into department '{}'.", person, dep);
                }
                _ => {
                    println!("Unrecognized add command. Please enter name and department. Like 'add Johny to Finance'");
                }
            }
        } else if command.starts_with("print") {
            let dep = command.split_whitespace().nth(1);
            match dep {
                Some(dep) => {
                    println!("{} {:?}", dep, company.get(dep).unwrap());
                }
                _ => {
                    println!("{:?}", company);
                }
            }
        } else if command.starts_with("stop") {
            break;
        } else {
            println!("Unknown command")
        }
    }
}
