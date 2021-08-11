// pub fn test() {
//
//     #[derive(Debug, Clone)]
//     pub struct Point {
//         x: i32,
//         y: i32
//     }
//
//     let p1 = Point {
//         x: 1,
//         y: 2
//     };
//
//     let p2 = p1.clone();
//     // 原本以为let p2 = p1 就可以完成拷贝工作，使得不会出现所有权的问题，实际上还是出现了
//     println!("{:?} {:?}", p1, p2);
//
//     let a  = 34;
//     let b = a;
//     println!("{:?}, {:?}", a, b);
//
//     // fn swap((x, y) : (&String, i32)) -> (i32, &String) {
//     //     (y, &"hello".as_string())
//     // }
//
//     // fn swap1<'a, 'b>((x, y) : (&'a String, i32)) -> (i32, &'b String) {
//     //     (y, x)
//     // }
//
//     fn swap2((x, y) : (&String, i32)) -> (i32, &String) {
//         (y, x)
//     }
//
//     fn swap3((x, y) : (&str, i32)) -> (i32, &str) {
//         (y, x)
//     }
//
//     let a = "he".to_string();
//     let (x, y) = swap2((&a, 34));
//     println!("{}", y);
//
// }

mod copy_test {
    use std::fmt::{Pointer, Formatter};

    pub fn test_1() {
        let a = 32;
        let b = a;
        println!("{}, {}", a, b);

        let a1 = "abc";
        let b1 = a1;

        println!("{}, {}", a1, b1);

        let a2 = "aaa".to_string();
        let b2 = a2;
        // println!("{}, {}", a2, b2);
        // 这句话会报错，因为a2是string，没有Copy trait，经历过了所有权转移的操作
        // 经历过后a2就变成初始化值了，b2是唯一所有权的变量了
    }

    pub fn test_2() {

        // 这里添加Copy关键字与否，会影响到下面test_print 函数的执行情况
        // 如果实现了copy，则进行复制操作，不会进行所有权的转移，方法外对p1进行输出可以继续进行
        // 如果没有实现copy，则会进行所有权的转移，方法外的调用会出现错误
        #[derive(Debug, Clone, Copy)]
        pub struct Point {
            x: i32,
            y: i32
        }

        let p1 = Point {
            x: 1,
            y: 2
        };

        fn test_print(point: Point) {
            println!("{:?}", point);
        }

        test_print(p1);
        println!("{:?}", p1);
        // 会出「^^ value borrowed here after move」 错误
    }

    pub fn test_3() {
        #[derive(Debug, Clone, Copy)]
        pub struct Point {
            x: i32,
            y: i32
        }

        let p1 = Point{x:12, y:23};
        let p2 = p1;
        println!("{:?}", p1);
    }

}

pub fn test() {
    // copy_test::test_1();
    // copy_test::test_2();
    copy_test::test_3();
}