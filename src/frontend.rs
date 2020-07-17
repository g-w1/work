fn are_u_sure() -> bool {
    println!("Are you sure? (Y/n) ");
    let mut sure: String = String::new();
    stdin().read_line(&mut sure).unwrap();
    let sure = sure.to_lowercase().trim().to_owned();
    if sure == String::from("y") || sure == String::from("yes") {
        true
    } else {
        false
    }
}

fn format_finished(finished_or_not: bool, in_20th_century: bool) -> String {
    if finished_or_not && in_20th_century {
        String::from("✅")
    } else if in_20th_century {
        String::from("❌")
    } else if finished_or_not {
        String::from("[X]")
    } else {
        String::from("[ ]")
    }
}
