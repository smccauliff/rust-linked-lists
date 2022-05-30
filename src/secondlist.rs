// This cleans up the V4 implementation by just using Option instead of having a separate Link enum
#[derive(Debug)]
pub struct SListV4<T> {
    head : Option<Box<SListV4Node<T>>>,
}

// Generic type parameter must be specified after the "impl" and then SListV4<T> propagates the unknown type
impl<T> SListV4<T> {
    // This is a static member function because it does not have a self parameter
    pub fn new() -> Self {
        //TODO:  why do I not need to mention the type parameters here
        SListV4 { head : None,}
    }

    pub fn push(&mut self, elem : T) {
        let new_node_box = Box::new(SListV4Node {
            elem : elem,
            // Can't just assign to self.head because that would be more than one borrow against the same thing?
            //  Need to assign something to self.head so replacing it with empty for now.
            next : self.head.take(),
        });

        self.head = Some(new_node_box);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

// Prevent automatic destructor from running into a stack overflow
impl<T> Drop for SListV4<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[derive(Debug)]
struct SListV4Node<T> {
    elem : T,
    next : Option<Box<SListV4Node<T>>>,
}

#[cfg(test)]
mod test {
    use crate::secondlist::SListV4;

    #[test]
    fn basics() {
        let mut list_v4 = SListV4::new();

        assert_eq!(list_v4.pop(), None);

        list_v4.push(0);
        list_v4.push(1);
        list_v4.push(2);

        assert_eq!(list_v4.pop(), Some(2));
        assert_eq!(list_v4.pop(), Some(1));

        list_v4.push(3);
        list_v4.push(4);

        assert_eq!(list_v4.pop(), Some(4));
        assert_eq!(list_v4.pop(), Some(3));
        assert_eq!(list_v4.pop(), Some(0));
        assert_eq!(list_v4.pop(), None);
    }
}