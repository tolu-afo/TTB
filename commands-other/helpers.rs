pub fn parse_username(username: Option<&str>) -> Option<&str> {
    if let Some(name) = username {
        return match name.chars().nth(0) {
            Some('@') if name.len() > 1 => Some(&name[1..]),
            Some('@') => None,
            _ => Some(name),
        };
    } else {
        return None;
    }
}
