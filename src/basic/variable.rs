#[derive(Clone, Debug)]
pub struct Point{
    x: i32,
    y: i32
}

impl Drop for Point {
    fn drop(&mut self) {
        // 利用析构函数观察和了解其被回收的时候
        println!("{:?} 析构函数回收掉了", self)
    }
}

impl Point {
    pub fn new(x: i32, y : i32) -> Self {
        Point{
            y,x
        }
    }
}

///
/// 下面注释的代码到底哪个是正常有效的呢？
/// let mut a = xxx 含义是a可以不用使用let被重复赋值就可以关联其他指针了，也可以通过他修改指针对应的数据
/// let a = mut xxx 含义是a只是拥有了xxx的可变的功能，它本身不允许再次进行非let的赋值操作
/// let a = b       这相当于把数据从b转移到a上，b的析构函数会被自动调用，后面就无法再次被使用了
///
/// 重点关注一下{@link test_mut_3} 的方法
mod mut_test {
    use crate::variable::Point;

    pub fn test_mut() {
        let mut a = Point::new(1,2);
        let b = &mut a;
        // let b = Point::new(3,4);
        // b = Point::new(3,4);
        println!("{:?}", b);
    }

    pub fn test_mut_1() {
        let mut a = Point::new(11,22);
        let mut b = a;
        // b = Point::new(33,44);
        // let b = Point::new(33,44);
    }

    pub fn test_mut_2() {
        let a = Point::new(111,222);
        let mut b = a;
        // b = Point::new(333,444);
        // let b = Point::new(333, 444);
        println!("{:?}", b);
    }

    pub fn test_mut_3() {
        let v = &mut vec![1,2,3,4];
        v.push(45);
        let v = 1234;

        // v = 123; 会出错
        // v被设置为一个不可变的对象，但是其对应引用的指针内的数据是可以变化的

        let mut vv = vec![1,2,3,4,5];
        vv.push(6);
        vv.push(7);

        vv = vec![4,5,6,7,8];
        // vv = 3; 同样会出错
        // vv是可变的对象，如果对其进行重新赋值，则一定需要确保其类型一致，例如上面都是vec向量，不允许赋值给i32
    }

    pub fn test_mut_4() {
        let mut s1 = "s1".to_string();
        s1.push_str(" add");

        let s2 = "s2".to_string();
        // s1.push_str("sdsd".parse().unwrap());
        // 这句话会提示错误的，因为这里s2是不可变的，所有往不可变的对象追加数据回出现错误
        println!("s1:{}, s2:{}", s1, s2);
    }

}

///
/// fn 方法「不返回数据」等价于「返回（）一般」只是默认不写
/// 函数可以作为返回值和参数使用，在充当返回值的时候最后一定要明确返回函数名称，如果不写则表示返回的是（），而不是默认的方法
/// 下面test2函数的返回值是   「fn() -> Point」
/// 下面test2_2函数的返回值是 「fn()」也就默认是「fn() -> ()」
mod fn_test {
    use crate::variable::Point;

    pub fn test_fn() {
        let a = Point::new(1,2);
        println!("{:?}", a);
        return;
    }

    pub fn test_fn_1() -> () {
        let a = Point::new(11,22);
        println!("{:?}", a);
        return;
    }

    fn test2() -> fn() -> Point {
        fn test() -> Point {
            Point::new(1,2)
        }
        test
        // 这里一定要写test，否则会出现问题
    }

    fn test2_2() -> fn() {
        //
        fn test() -> () {
            println!("test2_2")
        }
        test
    }

    pub fn test_fn_2() {
        let a = test2()();
        // 先调用test2()拿到返回的函数，再执行这个返回的函数
        println!("test_fn_2 test2 {:?}", a);
        test2_2()();
        // 这是属于无参数的情况
    }

    fn test3(f : fn() -> Point) {
        println!("{:?}", f());
    }

    pub fn test_fn_3() {
        test3(test2());
    }
}

///
/// 包含了元祖、结构体、枚举等情况
mod struct_complex_test {
    pub fn test_tuple_1() {
        let a = (1,2,3,4);
        // 元祖是使用括号包围起来的，使用下表从0开始的偏移量获取对应数据
        println!("{}", a.0);
        println!("{}", a.1);
        println!("{}", a.2);
        // println!("{}", a.4);
        // 这个a.4会出错，找不到下标为4的情况
    }

    pub fn test_tuple_2() {
        let (x, y, z, .., a) = (1,2,3,4,5,6);
        println!("x:{}, y:{}, z:{}, a:{}", x, y, z, a);

        let (a1, a2,a3, a4, a5, .., a6) = (1,2,3,4,5,6);
        println!("a1:{}, a2:{}, a3:{}, a4:{}, a5:{}, a6:{}", a1, a2, a3, a4, a5, a6);

        // 后面的所有不考虑的数据都使用「..」 去隐藏忽略数据，
        // 而且可放在任意位置，最终rust会选择一个最为合适的情况分配参数并执行的
        // 而且可以支持使得两点匹配到0个位置的情况出现

        let (a, b, c, d, _, _) = (1,2,3,4,5,6);
        println!("a:{}, b:{}, c:{}, d:{}", a, b, c, d);
        // 上面这样会出错，「_」 只表示一位的占位符，并不具备忽略多个数据的能力
    }

    pub fn test_struct_1() {
        struct Animal {
            name: String,
            age: i32
        }

        let name = "dog".to_string();

        let a = Animal {
            name,
            // 如果变量名称和结构体内的参数名称一样，可以把「name=name」简写成「name」
            age: 0
        };

        let b = Animal {
            name: "cat".to_string(),
            age : 12
        };
    }

    pub fn test_struct_lifetime() {
        struct Zoo<'a> {
            // 因为原本是&str引用，为保证引用时候的内存安全，需要「明确约定其生命周期的范围」,否则无法感知其引用对象到底在哪里
            // 其实所有包含引用的都需要有其生命周期的管理，只是有的时候可以被忽略掉，不用操心
            name : &'a str,
            age: i32
        }

        impl <'a> Zoo<'a> {
            // 因为明确了生命周期'a 所有也需要明确定义其生命周期的情况
            fn get_age(&self) -> i32 {
                self.age
            }

            fn get_name(&self) -> &'a str {
                // 当然这里的返回值也可写成&str，他会自动的关联上相关的生命周期,如果一定要明确生命周期，则只能写「&'a str」，因为已经定义了生命周期的标识'a
                self.name
            }
        }

        {
            let name = "bird";
            // 这里定义好了name的生命周期的范围就是当前这个大括号内，所以括号外的name输出肯定会报错的
            let zoo = Zoo {
                name,
                age: 0
            };
            println!("name: {}", zoo.name);
        }
        // 这句话肯定报错了，会说「not found in this scope」因为zoo的定义在上面的大括号内，并不能被使用在外面的大括号内，出现生命周期溢出的情况，找不到也就出错了
        // println!("name: {}", zoo.name);
    }

    pub fn test_struct_static_lifetime() {
        struct Zoo {
            // 相比上面的情况'a 特定的生命周期范围，'static 这是属于「全局的生命周期」的范围
            name : &'static str,
            age: i32
        }
        impl Zoo {
            fn get_name(&self) -> &str {
                // 这里就不要写'a static了，因为已经包含全局了，上面的zoo定义也可以忽略
                self.name
            }
        }
        // let name;
        // {
        //     let zoo = Zoo {
        //         name:"zoo",
        //         age:23
        //     };
        //     name = zoo.get_name();
        // }
        // println!("name:{}", name);
    }

    pub fn test_struct_mut() {

        struct Queue<T> {
            older: Vec<T>,
            younger: Vec<T>
        }

        impl <T> Queue<T> {
            pub fn new() -> Self {
                Queue{
                    older:Vec::new(),
                    younger:Vec::new()
                }
            }

            pub fn push(&mut self, c:T) {
                self.younger.push(c);
            }

            pub fn pop(&mut self) -> Option<T> {
                if self.older.is_empty() {
                    if self.younger.is_empty() {
                        return None;
                    }

                    use std::mem::swap;
                    swap(&mut self.older, &mut self.younger);
                    self.older.reverse();
                }
                self.older.pop()
            }
        }

        let mut q = Queue::new();
        q.push("CAD");
        let mut q1 = Queue::new();
        q1.push(123);
        let mut q2 = Queue::<i64>::new();
        q2.push(567);
    }
}

mod option_test {

    pub struct Point(i32, i32);

    pub fn find_store(mobile_os: &Point) -> Option<&Point> {
        match mobile_os {
            Point(10, x) => {
                println!("{}", x);
                Some(&Point(10,2222))},
            _ => None
        }
    }

    pub fn test1() {
        let p = find_store(&Point(10,12));
        {
            let p1 =  find_store(p.unwrap());
        }
        find_store(p.unwrap());
    }

}

fn test_mut() {
    mut_test::test_mut();
    mut_test::test_mut_1();
    mut_test::test_mut_2();
    mut_test::test_mut_4();
}

fn test_fn() {
    fn_test::test_fn_2();
    fn_test::test_fn_3();
}

fn test_struct_complex() {
    // struct_complex_test::test_tuple_1();
    // struct_complex_test::test_tuple_2();
    struct_complex_test::test_struct_lifetime();
}

pub fn test() {
    // test_mut();
    // test_fn();
    // test_struct_complex();
    option_test::test1();
}