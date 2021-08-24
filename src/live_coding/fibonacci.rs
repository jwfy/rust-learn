pub struct Fibonacci {
    a: u64,
    b: u64,
    cur: u8,
    total: u8,
}

impl Fibonacci {
    pub fn new(total: u8) -> Self {
        Self {
            a: 0,
            b: 0,
            cur: 0,
            total,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    // 这个是特质的一个特点，提供给下面的next方法返回的结果类型

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.total {
            return None;
        }
        if self.a == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let c = self.a + self.b;
            self.a = self.b;
            self.b = c;
        }
        self.cur += 1;
        Some(self.a)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let fb = Fibonacci::new(2);
        for v in fb.into_iter() {
            // into_iter 返回的是迭代器
            println!("{}", v);
        }
    }
}
