mod trait0 {
    use std::ops::{Deref, Add};
    use crate::variable::Point;

    pub fn test_deref() {
        struct Point<T> {
            x: T,
            y: T
        }
        impl<T> Point<T> {
            pub fn new(x: T, y: T) -> Self {
                Point{x, y}
            }
        }

        impl<T> Deref for Point<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.x
            }
        }

        let pi32 = Point::new(1,2);
        assert_eq!(1, *pi32);
        assert_eq!(1, pi32.x);
    }

    pub fn test_deref2() {
        struct P1<T> {
            x : T
        }

        struct P2<T> {
            p1 : P1<T>
        }

        impl<T> P2<T> {
            pub fn new(t : T) -> Self {
                P2{
                    p1: P1 {
                        x : t
                    }
                }
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
            y: i32
        }

        impl Default for Point {
            fn default() -> Self {
                Point {
                    x:23,y:4
                }
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