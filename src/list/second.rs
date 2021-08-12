///
/// 相比first更加高端的，高级的一个list表功能，添加了
/// 1、泛型
/// 2、生命周期
/// 3、迭代器
/// 4、Option
// 在之前first.rs 文件中，Link只是个枚举类型，包含了2类数据，有和没有，和option是一样的，可以进行替换操作
// enum Link {
//     Empty,
//     More(Box<Node>),
//   }

pub struct List<T> {
    head: Link<T>,
}

// 利用type关键字可以对类型重新定义，有种宏或者alias的作用
// 后面没有数值直接就是None,而不再是Link::Empty了，下面也都进行了修改操作
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    // 接着开始考虑泛型了，支持更多的数据类型
    next: Link<T>,
    // 这个next就是Option类型的数据
}

impl <T> List <T>{
    // 这需要从impl List 变成当前的模样，都需要添加<T>
    pub fn new() -> Self {
        List { 
            head: None 
            // 这里也从Empty换成了None
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: mem::replace(&mut self.head, None),
            next: self.head.take()
            // 这里把mem.replace 给换成了这个take操作
            // 点进这个take方法，可以看出来背后调用的也是mem::take 操作，本质上和注释的代码一致，稍微好看些罢了
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }

        // 在这里还可以继续完善这个代码逻辑，在match option {
        //     None => Node,
        //     Some(x) => Some(y)
        // }
        // 重新返回一个option
        // 类似这种代码可以替换成option.map 类似一种重新映射的关系,传入的参数是一个FnOnce闭包
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // self.head.map(|node| {
        //     &node.elem
        // })
        // 这样写应该不对劲，这会影响这个self.head的所有权情况，所以只能引用这个self.head
        self.head.as_ref().map(|node| {
            &node.elem
        })
        // 这个option的as_ref 就相当于多出来一个self.head的借用对象，不会出现所有权修改问题
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // 这个是可变的，返回一个可变对象
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl <T> Drop for List<T> {
    // 这个也从 impl Drop for List 改成当前模样
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test{

    #[test]
    fn test_peek() {

    } 
}