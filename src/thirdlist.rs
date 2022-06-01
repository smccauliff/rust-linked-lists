// Unlike the previous lists this is a persistent list.

use std::rc::Rc;

type Link<T> = Option<Rc<PersistentListNode<T>>>;

struct PersistentList<T> {
    head : Link<T>,
}

struct PersistentListNode<T> {
    elem : T,
    next : Link<T>,
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
}