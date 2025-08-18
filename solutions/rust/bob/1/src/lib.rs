pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    } else if message.trim().ends_with("?")
        && message
            .chars()
            .any(|c| c.is_alphabetic() && c.is_uppercase())
        && message
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
    {
        return "Calm down, I know what I'm doing!";
    } else if message.trim().ends_with("?") {
        return "Sure.";
    } else if message
        .chars()
        .any(|c| c.is_alphabetic() && c.is_uppercase())
        && message
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
    {
        return "Whoa, chill out!";
    }
    "Whatever."
}
