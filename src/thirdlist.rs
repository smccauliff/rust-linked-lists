// Unlike the previous lists this is a persistent list.

use std::rc::Rc;

// Ideally this would have a parameterized Rc so we could do Rc or Arc.  But this turns out to
// be difficult and would need to build different packages.   See
// https://github.com/bodil/im-rs and
// https://users.rust-lang.org/t/support-both-rc-and-arc-in-a-library/40563

type Link<T> = Option<Rc<PersistentListNode<T>>>;

struct PersistentList<T> {
    head : Link<T>,
}

struct PersistentListNode<T> {
    elem : T,
    next : Link<T>,
}

pub struct Iter<'a, T> {
    next : Option<&'a PersistentListNode<T>>,
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


impl<T> PersistentList<T> {
    pub fn new() -> PersistentList<T> {
        PersistentList { head: None, }
    }

    pub fn prepend(&self, elem : T) -> PersistentList<T> {
        PersistentList{ head : Some(Rc::new(PersistentListNode{
            elem : elem,
            next : self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> PersistentList<T> {
        PersistentList{ head : self.head.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next : self.head.as_deref() }
    }
}

impl<T> Drop for PersistentList<T> {
    fn drop(&mut self) {
        // Losing the reference to head drops head on the next iteration.
        // This function won't be called if the head was shared by someone so its safe to move the
        // initial head.
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                // not self.head because self.head is going to go away.
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::thirdlist::PersistentList;

    #[test]
    fn basic() {
        let plist = PersistentList::new();
        let plist_with_7 = plist.prepend(7);
        assert_eq!(plist_with_7.head(), Some(&7));
        let tailed = plist_with_7.tail();
        assert_eq!(tailed.head(), None);
    }

    #[test]
    fn iter() {
        let empty_list : PersistentList<i32> = PersistentList::new();
        assert_eq!(empty_list.iter().next(), None);

        let plist = PersistentList::new().prepend(0).prepend(1).prepend(2);

        let mut iter = plist.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }
}