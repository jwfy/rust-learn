mod trait0 {
    use std::ops::Deref;

    pub fn test_deref() {
        struct Point<T> {
            x: T,
            y: T,
        }
        impl<T> Point<T> {
            pub fn new(x: T, y: T) -> Self {
                Point { x, y }
            }
        }

        impl<T> Deref for Point<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.x
            }
        }

        let pi32 = Point::new(1, 2);
        assert_eq!(1, *pi32);
        assert_eq!(1, pi32.x);
    }

    pub fn test_deref2() {
        struct P1<T> {
            x: T,
        }

        struct P2<T> {
            p1: P1<T>,
        }

        impl<T> P2<T> {
            pub fn new(t: T) -> Self {
                P2 { p1: P1 { x: t } }
            }
        }

        impl<T> Deref for P2<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.p1.x
                // 这个就是表示*P2的返回值是什么，以及P2.之后的类型是什么
            }
        }

        let p = P2::new(23);
        let p1 = *p;
        assert_eq!(24, p1);
    }

    pub fn test_default() {
        struct Point {
            x: i32,
            y: i32,
        }

        impl Default for Point {
            fn default() -> Self {
                Point { x: 23, y: 4 }
            }
        }

        let p = Point::default();
        assert_eq!(223, p.x);
    }
}

pub fn test() {
    // trait0::test_deref();
    // trait0::test_deref2();
    trait0::test_default();
}

mod dynTrait {

    pub trait Formatter {
        fn format(&self, input: &String) -> bool;
    }

    pub struct MarkdownFormatter;
    impl Formatter for MarkdownFormatter {
        fn format(&self, input: &String) -> bool {
            println!("markdown format:{}", input);
            true
        }
    }

    pub struct RustFormatter;
    impl Formatter for RustFormatter {
        fn format(&self, input: &String) -> bool {
            println!("rust fromat:{}", input);
            false
        }
    }

    pub struct JavaFormatter;
    impl Formatter for JavaFormatter {
        fn format(&self, input: &String) -> bool {
            println!("java format:{}", input);
            true
        }
    }

    pub fn format(input: &String, formatter: Box<dyn Formatter>) -> bool {
        // 因为这里需要传入相关特质功能的实体，但是rust又不是类似java那样的可用指定传入接口，所以需要使用dyn参数
        // 又dyn + trait 是没有具体大小的，这点也违背rust的基础要求，所以还得用Box包装数据
        formatter.format(input)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::io::Write;

    #[test]
    pub fn test() {
        let mut buf: Vec<u8> = vec![];
        // let write: Write = buf;
        // 不能给相关的特质trait赋值一个对象信息
        let write1: &mut Write = &mut buf;
        // 但是这个可以，因为这只是一个引用对象，胖指针而已
    }

    #[test]
    pub fn test_dyn() {
        let java_formatter = dynTrait::JavaFormatter;
        let res = dynTrait::format(&"哈哈".to_string(), Box::new(java_formatter));
        println!("{}", res);
    }
}
