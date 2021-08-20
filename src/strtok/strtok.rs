pub fn strtok<'a>(s: &'a mut &str, p: char) -> &'a str {
    match s.find(p) {
        Some(idx) => {
            let prefix = &s[..idx];
            let suffix = &s[idx + p.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

#[cfg(test)]
mod test {

    use super::strtok;

    #[test]
    fn test() {
        let mut s = "hello world";
        let p = ' ';
        let res = strtok(&mut s, p);
        assert_eq!(res, "hello");
        assert_eq!(s, "world");
    }
}
