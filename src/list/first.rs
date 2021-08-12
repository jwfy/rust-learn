use std::{mem};


// pub enum List {
//   Empty,
//   Elem(i32, List),
//   // 这就直接报错了，因为Rust不允许未定义大小的结构出现，这个一般情况下都是定义成Box
// }
// error[E0072]: recursive type `List` has infinite size
//  --> src/list/first.rs:2:1
//   |
// 2 | pub enum List {
//   | ^^^^^^^^^^^^^ recursive type has infinite size
// 3 |   Empty,
// 4 |   Elem(i32, List),
//   |             ---- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
//   |
// 4 |   Elem(i32, Box<List>),

// pub enum List {
//     Empty,
//     Elem(i32, Box<List>)
//     // 这个box是强制把数据存储到堆上的操作，而Box本身只有指针数据，
//     // 所以可以解决上面那个出现的「recursive type `List` has infinite size」问题
// }

// 但是这种写法太不行了，无法保证下一个节点是什么样子，还有个EMPTY节点不在堆里面

// 来个推荐的样式代码

pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    // 构造函数
    List { head: Link::Empty }
  }

  pub fn push(&mut self, elem:i32) {
    // let new_node = Node{
    //     elem,
    //     next: self.head
    // };
    // self.head = Link::More(Box::new(new_node));
    // 这样肯定会报错的，因为移动的是self.head，move会出现问题，因为移动到了node里面self本身的生命周期就结束了，没法玩啊

    // 可以学习利用mem::replace
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
      // 这个操作是把self.head变成后面的empty，而self.head本身就被移动到next字段中
    });

    self.head = Link::More(new_node);
    // 这里self是mut可变的，所以这样赋值是没有问题的
  }

  pub fn pop(&mut self) -> Option<i32> {
    // match self.head {
    //   Link::Empty => {
    //     Option::None
    //   },
    //   Link::More(node) => {
    //     Option::Some(node.elem)
    //   }
    // }
    match mem::replace(&mut self.head, Link::Empty) {
      // 把head数据全部移出和替换操作，然后赋值为empty默认数据
      Link::Empty => None,
        // 如果head本身就是empty，那就无须考虑了
      Link::More(node) => {
        let node = *node; // 这类似于拆包，解引用的操作
        self.head = node.next;
        Some(node.elem)
      }                        
    }
  }
}

impl Drop for List {
  fn drop(&mut self) {
      let mut cur_link = mem::replace(&mut self.head, Link::Empty);
      while let Link::More(mut boxed_node) = cur_link {
          cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
      }
  }
}


// 这个是单测，可以利用cargo test 批量执行各个具体的单元测试
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
      let mut list = List::new();
      list.push(1);
      list.push(2);
      list.push(3);

      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), None);
    }
}