pub fn strtok<'a>(s: &'a mut &str, p: char) -> &'a str {
    match s.find(p) {
        Some(idx) => {
            let prefix = &s[..idx];
            let suffix = &s[idx + p.len_utf8()..];
            // 这个len_utf8主要是因为其长度在不同环境下是可变的，所以不能贸然的+1操作
            // &s[..idx] 其实就是类似于字符串字面量的切面而已
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
