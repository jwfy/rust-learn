use std::fmt::{Debug, Display};

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // 这里就直接给self.count += 1，主要是因为上面的&mut 参数数据

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


fn piter<U, T>(t : T)
where 
    T : IntoIterator<Item = U>, 
    U : Debug + Display {
        // 这个方法可作为一个for循环遍历的操作的一个泛型方法
        for u in t {
            println!("{:?}", u);
        }
}




#[cfg(test)]
mod test {

    use super::{Counter, piter};

    #[test]
    fn test0() {
        let iter = Counter::new();
        for v in iter {
            println!("v==={}", v);
        }
    }

    #[test]
    fn test() {
        let v1 = vec![1,2,3,4];
        let iter = v1.iter();
        let iter = &v1.iter();
        let iter = &mut v1.iter();
        // 这分为三种不同的迭代操作
        for i in iter {
            println!("{}", i);
        }
    }

    #[test]
    fn test2() {
        let text = " dssdsd\n 
         sfs sdfsd sdfs \n 
         sfsd sd ";
         let v : Vec<&str> = text.lines().map(str::trim).filter(|s| *s != "ignore").collect();
         // 这里必须强制定义v的数据类型
         piter(&v);
         println!("{}", v[1]);
         // 需要时刻注意借用和所有权的所属关系

    }
}