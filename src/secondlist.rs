// This cleans up the V4 implementation by just using Option instead of having a separate Link enum
#[derive(Debug)]
pub struct SListV4<T> {
    head : Option<Box<SListV4Node<T>>>,
}

// This is a tuple struct which is being used as a wrapper around SListV4.
// IMO: I don't like tuples.  Code is more readable without them.
//  IntoIter appears to be like a C++ concept.  A type which should exist, but we have no way of
// actually enforcing its existance (before C++-20 anyways).
// IntoIter takes a value and not a reference to a value.  When this is returned from a method
// ownership will be moved to iterator.  It is an *into* iterator which is destructive so I guess
// that makes sense
pub struct IntoIter<T>(SListV4<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// Iter is valid over some lifetime
pub struct Iter<'a, T> {
    next : Option<&'a SListV4Node<T>>,
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
    next : Option<&'a mut SListV4Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // This needs to "take", which will swap the mutable reference next with Option::None
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> SListV4<T> {
    // This is a static member function because it does not have a self parameter
    pub fn new() -> Self {
        SListV4 { head : None,}
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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

    // Since this takes self (a value).   This converts self into an iterator.  The previous variable
    // owning self will no longer be valid.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Iter requires a lifetime, but it appears this can be elided since the default rule,
    // returned object has a lifetime of self, would appear to be good enough here.
    pub fn iter(&self) -> Iter<T> {
        Iter{ next : self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{ next : self.head.as_deref_mut() }
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

    #[test]
    pub fn peek_test() {
        let mut list_v4 = SListV4::new();
        list_v4.push(0);

        // | pattern match|
        list_v4.peak_mut().map(| value| {
            *value = 7;
        });

        assert_eq!(list_v4.peek(), Some(&7));
    }

    #[test]
    fn into_iter() {
        let mut list_v4 = SListV4::new();
        list_v4.push(1); list_v4.push(2); list_v4.push(3);

        let mut iter = list_v4.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        // Uncomment this to validate this won't compile because list_v4 is now invalid.
       // assert_eq!(list_v4.peek(), None);
    }

    #[test]
    fn iter() {
        let mut list_v4 = SListV4::new();
        list_v4.push(1);
        list_v4.push(2);

        for x in list_v4.iter() {
            println!("{}", x);
        }

        let mut iter = list_v4.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list_v4 = SListV4::new();
        let mut iter_mut = list_v4.iter_mut();
        assert_eq!(iter_mut.next(), None);
        list_v4.push(1);
        list_v4.push(2);

        let mut iter_mut2 = list_v4.iter_mut();
        assert_eq!(iter_mut2.next(), Some(&mut 2));
        assert_eq!(iter_mut2.next(), Some(&mut 1));
        assert_eq!(iter_mut2.next(), None);
    }
}

