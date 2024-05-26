#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, elem: T) -> &mut Self {
        let new_node = Box::new(Node {
            elem,
            //使用option就可以不使用replace来实现不转移所有权的修改获取数据而是使用take
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self
    }
    pub fn pop(&mut self) -> Option<T> {
        //这里也是使用take来代替
        match self.head.take() {
            None => None,
            Some(head_node) => {
                self.head = head_node.next;
                Some(head_node.elem)
            }
        }
    }
    //实现获取链表开头的元素的引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }
    //实现获取链表开头元素的可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|node| &mut node.elem)
    }
}
pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        //这里也是使用take老代替
        let mut cur_link_node = self.head.take();
        while let Some(mut boxed_node) = cur_link_node {
            cur_link_node = boxed_node.next.take();
        }
    }
}
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Link, List, Node};

    #[test]
    fn create_test() {
        let list: List<i32> = List::new();
        let target_list: List<i32> = List { head: None };
        assert_eq!(list, target_list);
    }
    #[test]
    fn push_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        let target_list: List<i32> = List {
            head: Some(Box::new(Node {
                elem: 88,
                next: Some(Box::new(Node {
                    elem: 66,
                    next: Some(Box::new(Node {
                        elem: 55,
                        next: Link::None,
                    })),
                })),
            })),
        };
        assert_eq!(list, target_list);
    }
    #[test]
    fn pop_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        assert_eq!(list.pop(), Some(88));
        assert_eq!(list.pop(), Some(66));
        assert_eq!(list.pop(), Some(55));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        let peek_ref = list.peek();
        assert_eq!(peek_ref, Some(&88));
        let peek_ref_mut = list.peek_mut();
        assert_eq!(peek_ref_mut, Some(&mut 88));
    }
    #[test]
    fn into_iter_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        for (index, val) in list.into_iter().enumerate() {
            match index {
                0 => {
                    assert_eq!(val, 88);
                }
                1 => {
                    assert_eq!(val, 66);
                }
                2 => {
                    assert_eq!(val, 55);
                }
                _ => {
                    panic!()
                }
            }
        }
    }
    #[test]
    fn iter_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        for (index, val) in list.iter().enumerate() {
            match index {
                0 => {
                    assert_eq!(val, &88);
                }
                1 => {
                    assert_eq!(val, &66);
                }
                2 => {
                    assert_eq!(val, &55);
                }
                _ => {
                    panic!()
                }
            }
        }
    }
    #[test]
    fn mut_iter_test() {
        let mut list = List::new();
        list.push(55).push(66).push(88);
        for (index, val) in list.iter_mut().enumerate() {
            match index {
                0 => {
                    assert_eq!(val, &mut 88);
                }
                1 => {
                    assert_eq!(val, &mut 66);
                }
                2 => {
                    assert_eq!(val, &mut 55);
                }
                _ => {
                    panic!()
                }
            }
        }
    }
}
