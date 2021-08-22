pub fn reply(message: &str) -> &str {
    let reply: &str;
    let msg: &str = message.trim();
    let has_alphanum = msg.contains(|c| char::is_alphanumeric(c));
    let has_alpha = msg.contains(|c| char::is_alphabetic(c));

    if has_alpha && msg.to_uppercase() == msg {
        reply = if msg.ends_with("?") {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    } else if msg.ends_with("?") {
        reply = "Sure.";
    } else if !has_alphanum {
        reply = "Fine. Be that way!";
    } else {
        reply = "Whatever."
    }
    reply
}
