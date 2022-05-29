use std::mem::replace;

// This enum requires space for a discriminator integer since Cons binary representation can't
// be all zeros since T is stored there.
// sizeof(Nil) is still size of Cons.
#[derive(Debug)]
pub enum SListV1<T> {
    Cons(T, Box<SListV1<T>>),
    Nil
}
/*
#[derive(Debug)]
struct SListV2Node<T> {
    elem : T,
    next : SListV2<T>,
}

// This enum can use nullpointer optimization since Box<> with nothing in it is all zeros.  Then
// No discriminator field is needed.

#[derive(Debug)]
pub enum SListV2<T> {
    Empty,
    More(Box<SListV2Node<T>>)
}
*/

// ListV2 can't make node public without giving away implementation details.  So this hides the
// Node behind a public type.
#[derive(Debug)]
pub struct SListV3<T : Copy> {
    head : SListV3Link<T>,
}


// Generic type parameter must be specified after the "impl" and then SListV3<T> propogates the unknown type
impl<T : Copy> SListV3<T> {
    // This is a static member function because it does not have a self parameter
    pub fn new() -> Self {
        //TODO:  why do I not need to mention the type parameters here
        SListV3 { head : SListV3Link::Empty,}
    }

    pub fn push(&mut self, elem : T) {
        let new_node_box = Box::new(SListV3Node {
            elem : elem,
            // Can't just assign to self.head because that would be more than one borrow against the same thing?
            //  Need to assign something to self.head so replacing it with empty for now.
            next : replace(&mut self.head, SListV3Link::Empty),
        });

        self.head = SListV3Link::More(new_node_box);
    }

    pub fn pop(&mut self) -> Option<T> {
        // Last statement evaluated is returned
        // self.head would be a value type, but self is a reference
        match replace(&mut self.head, SListV3Link::Empty) {
            SListV3Link::Empty =>  None,
            SListV3Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

// Prevent automatic destructor from running into a stack overflow
impl<T : Copy> Drop for SListV3<T> {
    fn drop(&mut self) {
        let mut cur_link = replace(&mut self.head, SListV3Link::Empty);
        while let SListV3Link::More(mut boxed_node) = cur_link {
            cur_link = replace(&mut boxed_node.next, SListV3Link::Empty);
        }
    }
}

#[derive(Debug)]
enum SListV3Link<T : Copy> {
    Empty,
    More(Box<SListV3Node<T>>),
}

#[derive(Debug)]
struct SListV3Node<T : Copy> {
    elem : T,
    next : SListV3Link<T>,
}

#[cfg(test)]
mod test {
    use crate::firstlist::SListV3;

    #[test]
    fn basics() {
        let mut listv3 = SListV3::new();

        assert_eq!(listv3.pop(), None);

        listv3.push(0);
        listv3.push(1);
        listv3.push(2);

        assert_eq!(listv3.pop(), Some(2));
        assert_eq!(listv3.pop(), Some(1));

        listv3.push(3);
        listv3.push(4);

        assert_eq!(listv3.pop(), Some(4));
        assert_eq!(listv3.pop(), Some(3));
        assert_eq!(listv3.pop(), Some(0));
        assert_eq!(listv3.pop(), None);
    }
}