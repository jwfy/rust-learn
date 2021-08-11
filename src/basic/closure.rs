mod closure_test {


    fn test_fn<F>(f : F)
        where F : Fn() {
        f();
        f();
        println!("fn")

        // 凡是Fn的都是&self，可以被多次调用执行
        // 有move的，自由变量 无copy语义的，则会产生所有权转移，后面再使用则会出问题
        // 没move的，自由变量 那就是纯粹的引用关系，
    }

    fn test_fn_p<F>(f : F)
        where F: Fn(String) {
        let s = "hello".to_string();
        f(s);
        // println!("after {}", s);
        // 果然这里会报错，非自由变量同样需要遵循所有权这一套规则，f(s)就是把s的所有权当做参数传递到闭包之中，后面就无法再使用了
    }





    fn test_fn_once<F>(f : F) -> ()
        where F : FnOnce() {
        f();
        println!("fnonce")
    }
    fn test_fn_mut<F>(mut f : F)
        where F : FnMut() {
        f();
        f();
        println!("fnmut")
    }

    pub fn test_fn_nocopy_nomove() {

        #[derive(Clone, Debug)]
        struct Man {
            no : i32,
            age : i32
        }

        let man = Man {
            no: 12,
            age: 56
        };

        test_fn(|| {
            println!("test_fn_nocopy_nomove {:?}", man);
        });
        // 能够跑通再一次说明了Fn其实是&self，引用的关系，并不会获取所有权
        println!("test_fn_nocopy_nomove {:?}", man);
    }

    pub fn test_fn_copy_nomove() {

        #[derive(Clone, Debug, Copy)]
        struct Man {
            no : i32,
            age : i32
        }

        let man = Man {
            no: 12,
            age: 56
        };

        test_fn(|| {
            println!("test_fn_copy_nomove {:?}", man);
        });

        println!("test_fn_copy_nomove {:?}", man);
    }

    pub fn test_fn_copy_move() {

        #[derive(Clone, Debug, Copy)]
        struct Man {
            no : i32,
            age : i32
        }

        let man = Man {
            no: 12,
            age: 56
        };

        test_fn(move || {
            println!("test_fn_copy_move {:?}", man);
        });

        println!("test_fn_copy_move {:?}", man);
    }

    pub fn test_fn_nocopy_move() {

        #[derive(Clone, Debug)]
        struct Man {
            no : i32,
            age : i32
        }

        let man = Man {
            no: 12,
            age: 56
        };

        test_fn(move || {
            println!("test_fn_nocopy_move {:?}", man);
        });

        // println!("test_fn_nocopy_move {:?}", man);
        // 这句话会报错的
    }

    pub fn test_fn_n_n_p() {
        let a = "world".to_string();

        test_fn_p(|s : String| {
            println!("{}", s);
        })
    }

}

pub fn test() {
    // closure_test::test_fn_nocopy_nomove();
    // closure_test::test_fn_copy_nomove();
    // closure_test::test_fn_copy_move();
    // closure_test::test_fn_nocopy_move();
    closure_test::test_fn_n_n_p();
}