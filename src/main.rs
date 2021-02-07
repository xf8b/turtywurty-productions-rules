use std::io;

use rules::create_non_admin_rule;
use rules::create_rule;
use rules::Rule;

mod rules;

fn main() {
    let rules = vec![
        // rules
        create_rule(
            1,
            "Do not promote any illegal acts, breaking of EULAs, breaking of other rules or breaking ToS.",
        ),
        create_rule(
            2,
            "Do not steal other people's work. If you do plan on utilising others things, please get confirmation from them first that it is okay.",
        ),
        create_rule(
            3,
            "Do not spam in any channel.",
        ),
        create_rule(
            4,
            "Please keep everything PG except for #🎊off-topic. No porn is allowed.",
        ),
        create_rule(
            5,
            "No heated discussions will be tolerated. If you witness one, please contact an Administrator.",
        ),
        create_rule(
            6,
            "Learn Java before you ask why your code doesn't work. By not learning Java, you make it harder for both yourself and for us. Explaining how to fix something is significantly more difficult if someone hasn't learned java, so please, just take a week or so out of your time to do that first.",
        ),
        create_rule(
            7,
            "Do not be rude to other users, don't be an asshole and just be nice.",
        ),
        create_rule(
            8,
            "Do not ping whole roles or excessively ping users. Just because you might be fine with it, doesn't mean everyone is, so just keep the pings to a minimum.",
        ),
        create_rule(
            9,
            "English only please. This is because it's very tricky to moderate languages that all our admins can't speak in.",
        ),
        create_rule(
            10,
            "You are only permitted 1 advertisement and 1 collaboration request for a thing, and those should be in #🎭collab-advertise. This rule does not apply for YouTubers and streamers.",
        ),
        create_rule(
            11,
            "Do not be rude or backchat to staff, this will result in you either being muted or kicked. If you have a concern, please DM an admin.",
        ),
        create_non_admin_rule(
            12,
            "No ban evading, alt accounts will be banned. This does not apply for admins.",
        ),
        create_non_admin_rule(
            13,
            "Do not misuse the bots. This includes using their commands in the wrong channel. This rule does not apply for admins related to moderation.",
        ),
        create_rule(
            14,
            "Do not post files in any channel. Always host it online. Direct links to executables are not allowed.",
        ),
        create_rule(
            15,
            "Do not purposely boot into a running conversation asking for help.",
        ),
        create_rule(
            16,
            "No role-playing and no politics.",
        )
    ];

    println!("What would you like to do?");

    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");

    println!();

    match action.to_lowercase().trim() {
        "list" => {
            for rule in &rules {
                println!("{} Number: {}", if rule.is_guideline { "Guideline" } else { "Rule" }, rule.number);
                println!("Definition: {}", rule.definition);
                println!("Applies to admins: {}", rule.applies_to_admins);
                println!();
            }
        }
        "search" => {
            println!("What would you like to search for?");

            let mut search_term = String::new();

            io::stdin()
                .read_line(&mut search_term)
                .expect("Failed to read line");

            println!();

            let result: Vec<&Rule> = rules.iter()
                .filter(|&rule| rule.definition.to_lowercase().contains(search_term.to_lowercase().trim()))
                .collect();

            if !result.is_empty() {
                for rule in &result {
                    println!("{} Number: {}", if rule.is_guideline { "Guideline" } else { "Rule" }, rule.number);
                    println!("Definition: {}", rule.definition);
                    println!("Applies to admins: {}", rule.applies_to_admins);
                    println!();
                }
            } else {
                panic!("error: could not find rule(s) matching {}", search_term)
            }
        }
        _ => panic!("error: invalid action")
    }
}
