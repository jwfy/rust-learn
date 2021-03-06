mod threads {
    use std::cell::RefCell;
    use std::io::Read;
    use std::thread;
    use std::thread::spawn;

    pub fn test1() {
        thread_local! {
            static Foo : RefCell<u32> = RefCell::new(1)
        };
        // 会生成一个thread::LocalKey的Foo实例

        Foo.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
            // 可以传入一个闭包，然后操作线程本地存储的数据
        });

        thread::spawn(|| {
            Foo.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            })
        });

        Foo.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });
    }

    // pub fn test2() {
    //     use std::sync::mpsc::channel;
    //     use std::path::PathBuf;
    //     use std::fs::File;

    //     let documents: Vec<PathBuf> = Vec::new();

    //     let (sender, receiver) = channel();
    //     let handle = spawn(move || {
    //         for filename in documents {
    //             let mut f = File::open(filename).unwrap();
    //             let mut text = String::new();
    //             f.read_to_string(&mut text).unwrap();

    //             if sender.send(text).is_err() {
    //                 break;
    //             }
    //         }
    //         Ok(())
    //     });

    // }
}

pub fn test() {
    threads::test1();
}

#[cfg(test)]
mod test {
    use std::thread;

    #[test]
    pub fn test1() {
        let mut v = vec![1, 2, 3, 4];
        thread::spawn(move || {
            v.push(5);
        });
        // v.push(6);
        // 上面这个v的所有权经过转移操作了，现在v不再和vec有关联关系了，所以会报错
    }

    #[test]
    pub fn test2() {
        let s = "hello world".to_string();
        let s1 = "hello world";
        thread::spawn(move || {
            println!("{}", s);
            // 这里s和s1都可以使用，前提是必须使用move，把所有权转移到闭包内部
            // 如果仔细看线程spawn方法，也可以发现
        });

        let si = 1000;
        thread::spawn(move || {
            println!("{}", si);
        });
    }
}
