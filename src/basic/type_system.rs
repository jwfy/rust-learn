mod trait_test {

    pub struct Point {
        x: i32,
        y: i32,
    }

    trait PointTrait {
        fn get_total(&self) -> i32;
    }

    impl PointTrait for Point {
        fn get_total(&self) -> i32 {
            self.y + self.x
        }
    }

    fn dynaimic_dispatch(point_trait: &PointTrait) -> i32 {
        // 动态分发引用
        point_trait.get_total()
    }

    fn static_dispatch<T>(t: &T) -> i32
    where
        T: PointTrait,
    {
        // 这就是0成本抽象了，不会消耗资源进行查找操作
        t.get_total()
    }

    fn static_dispatch_1<T: PointTrait>(t: T) -> i32 {
        // 这是上面静态分发的又一种写法，把具体的类型约束写在泛型里，但是这样会导致观赏性降低，推荐使用where
        t.get_total()
    }

    pub fn test_1() {
        let point = Point { x: 3, y: 5 };
        assert_eq!(static_dispatch(&point), dynaimic_dispatch(&point));
    }
}

mod drop_test {

    #[derive(Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Drop for Point {
        fn drop(&mut self) {
            println!("{:p} {:?} 被回收", self, self)
        }
    }

    pub fn test_1() {
        let p1 = Point { x: 11, y: 22 };
        println!("haha");
        let p2 = p1;
        println!("hehe");
        // 这里只有1次析构操作，因为p1和p2指向的指针实际上是同一个，所以析构操作也只有1次
    }

    pub fn test_2() {
        let p1 = Point { x: 11, y: 22 };
        {
            let p2 = Point { x: 44, y: 33 };
            println!("haha");
            // 这里p2自动调用析构函数
        }
        println!("hehe");
    }

    pub fn test_3() {
        let mut p1 = Point { x: 11, y: 22 };
        {
            p1;
            // 利用花括号，显示的设置p1对象的适用范围，进行析构操作
            println!("haha");
        };
        println!("hehe");
        // p1.x = 23;
    }

    pub fn test_4() {
        let p1 = Point { x: 1, y: 2 };
        println!("haha");
        let p1 = Point { x: 3, y: 4 };
        println!("hehe");

        // 变量遮蔽，shadow，不会主动的进行析构操作，例如第一个p1不会主动进行，而是等到函数结束后，进行析构操作，
        // 尽管第一个Point已经被丢弃了
        // 所以都是和其生命周期结束的时候进行析构操作
    }
}

mod box_test {
    use std::rc::Rc;

    fn test_box() {
        type NodePrt<T> = Option<Box<Node<T>>>;
        struct Node<T> {
            data: T,
            next: NodePrt<T>,
        }

        let mut first = Box::new(Node {
            data: 1,
            next: None,
        });

        let mut second = Box::new(Node {
            data: 2,
            next: None,
        });

        first.next = Some(second);
        // second.next = Some(first);
        // 上面的这个second.next会直接报错，因为second的所有权已经被转移到first中去了，就无法再次使用second.next操作了
    }

    // fn test_rc_box() {
    //
    //     type NodePrt<T> = Option<Rc<Node<T>>>;
    //     struct Node<T> {
    //         data:T,
    //         next: NodePrt<T>
    //     }
    //
    //     let mut first = Box::new(Node {
    //         data: 1,
    //         next: None
    //     });
    //
    //     let mut second = Box::new(Node {
    //         data: 2,
    //         next: Some(Rc::from(first.clone()))
    //     });
    //
    //     first.next = Some(Rc::from(second.clone()));
    //     // second.next = Some(first);
    //     // 上面的这个second.next会直接报错，因为second的所有权已经被转移到first中去了，就无法再次使用second.next操作了
    // }

    pub fn point() {
        #[derive(Debug)]
        pub struct Point {
            x: i32,
            y: i32,
        }

        impl Drop for Point {
            fn drop(&mut self) {
                println!("{:p} {:?} 被回收", self, self)
            }
        }

        fn get<'a>() -> &'a i32 {
            let pp = Point { x: 1, y: 1 };
            &pp;
            println!("马上要被析构了 {:?}", pp);
            &34534
            // 贸然的写返回&pp，会报错，因为pp在这里被析构回收的，然后还是返回pp的引用指针
            // 这就出现了内存已经被回收，但是对象引用却在外面，出现「悬挂指针」也就是野指针的情况
            // rust会发现的
        }
        println!("开始了",);
        let x = get();
        println!("结束了",);
        println!("{:?}", x);
    }
}

pub fn test_trait() {
    trait_test::test_1();
}

pub fn test_drop() {
    // drop_test::test_1();
    // drop_test::test_2();
    // drop_test::test_3();
    // drop_test::test_4();
}

pub fn test_box() {
    box_test::point();
}

pub fn test() {
    // test_trait();
    // test_drop();
    test_box();
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let s = "abc";
        let s1 = s;
        println!("{}, {}", s, s1);
        // 这个例子能够跑过说明，&str 就等同于基本类型，同时也有一个名称「字符串字面量」
        let a = 12;
        let a1 = a;
        println!("{}, {}", a, a1);
        let mut ss = "abv";
        let mut ss1 = ss;
        ss = "abcdef";
        ss1 = "aaaa";
        println!("{}, {}", ss, ss1);
    }

    #[test]
    fn test1() {}
}
