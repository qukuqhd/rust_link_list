use std::mem::replace;

//定义的链表节点的基本架构包含了值和下一个节点的连接
#[derive(PartialEq, Eq, Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
//下个节点的连接设计为枚举可能是空也可能是有数据为more包含了下一个节点的box指针
#[derive(PartialEq, Eq, Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}
//链表结构体包含头的链
#[derive(PartialEq, Eq, Debug)]
pub struct List<T> {
    head: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        //创建一个空的链表
        Self { head: Link::Empty }
    }
    //实现入栈
    pub fn push(&mut self, elem: T) {
        //把当前链表作为新的头链的之后的链表，这里使用了replace来实现不进行所有权的就获取了头节点数据
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    //实现出栈
    pub fn pop(&mut self) -> Option<T> {
        //也使用了replace来获取头节点的数据而没有发生转移所有权
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let result = Some(node.elem);
                self.head = node.next;
                result
            }
        }
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link_node = replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link_node {
            cur_link_node = replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
#[cfg(test)]
mod test {
    use super::{Link, List, Node};

    #[test]
    fn create_test() {
        let list: List<i32> = List::new();
        let target_list: List<i32> = List { head: Link::Empty };
        assert_eq!(list, target_list);
    }
    #[test]
    fn push_test() {
        let mut list = List::new();
        list.push(55);
        list.push(66);
        list.push(88);
        let target_list: List<i32> = List {
            head: Link::More(Box::new(Node {
                elem: 88,
                next: Link::More(Box::new(Node {
                    elem: 66,
                    next: Link::More(Box::new(Node {
                        elem: 55,
                        next: Link::Empty,
                    })),
                })),
            })),
        };
        assert_eq!(list, target_list);
    }
    #[test]
    fn pop_test() {
        let mut list = List::new();
        list.push(55);
        list.push(66);
        list.push(88);
        assert_eq!(list.pop(), Some(88));
        assert_eq!(list.pop(), Some(66));
        assert_eq!(list.pop(), Some(55));
        assert_eq!(list.pop(), None);
    }
}
