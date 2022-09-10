use regex::Regex;

pub fn reply(message: &str) -> &str {
    let cap_re = Regex::new(r"[^A-Z]").unwrap();
    let question = message.trim().ends_with('?');
    let nothing = message.trim().is_empty() || message.trim().replace('\r', "").is_empty();
    let all_cap = message.to_uppercase() == message && !cap_re.replace_all(message, "").is_empty();

    if nothing {
        return "Fine. Be that way!"
    }

    if question && all_cap {
        return "Calm down, I know what I'm doing!"
    }

    if question {
        return "Sure."
    }

    if all_cap {
        return "Whoa, chill out!"
    }

    return "Whatever."

}
