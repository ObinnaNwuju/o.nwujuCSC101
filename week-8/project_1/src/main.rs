use std::io;


fn main() {
    
    let aps1_2 = vec!["Intern", "Paralegal", "placement"];
    let aps3_5 = vec!["Administrator", "Reasearch Assistant", "Junior Associate", "Classroom Teacher"];
    let aps5_8 = vec!["Senior Administrator", "Phd Candidate", "Associate", "Snr Teacher"];
    let el18_10 = vec!["Office manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el210_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    println!("Enter staff job Title");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");
    let title = title.trim();

    println!("Enter staff years of work experience:");
    let mut years = String::new();
    io::stdin().read_line(&mut years).expect("Failed to read input");
    let year: u32 = years.trim().parse().unwrap_or(0);

    let aps_level = if aps1_2.contains(&title) {
        "APS 1-2"
    } else if aps3_5.contains(&title) {
        "APS 3-5"
    } else if aps5_8.contains(&title) {
        "APS 5-8"
    } else if el18_10.contains(&title) {
        "APS 8-10"
    } else if el210_13.contains(&title) {
        "APS 10-13"
    } else if ses.contains(&title) {
        "SES level"
    } 
    else {
        "Unknown job Title"
    };

    println!("\n--- Result ---");
    println!("Staff Job Title: {}", title);
    println!("experience: {} years", years);
    println!("Matched APS level: {}", aps_level);


}