use crate::book::Book;
use crate::index::{EntityId, Index};
use crate::metadata::Metadata;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
/// Represents a unit of measurement. Will most commonly represent the minor unit of a currency.
pub struct Unit<T: Metadata> {
    pub(crate) id: EntityId,
    pub(crate) meta: RefCell<T::Unit>,
    pub(crate) index: Rc<Index<T>>,
}
impl<T: Metadata> Unit<T> {
    /// Creates a new unit.
    pub fn new(book: &Book<T>, meta: T::Unit) -> Rc<Self> {
        let unit = Rc::new(Self {
            id: Self::next_id(&book.index),
            index: book.index.clone(),
            meta: RefCell::new(meta),
        });
        Self::register(&unit, &book.index);
        unit
    }
}
impl<T: Metadata> fmt::Debug for Unit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Unit").field("id", &self.id).finish()
    }
}
#[cfg(test)]
mod test {
    use super::Book;
    use super::Unit;
    use crate::metadata::BlankMetadata;
    #[test]
    fn new() {
        use maplit::btreeset;
        let book = Book::<((), (), u8, ())>::new(());
        let unit_a = Unit::new(&book, 50);
        assert_eq!(unit_a.id, 0);
        assert_eq!(unit_a.index, book.index);
        assert_eq!(*unit_a.meta.borrow(), 50);
        let unit_b = Unit::new(&book, 40);
        assert_eq!(unit_b.id, 1);
        assert_eq!(unit_b.index, book.index);
        assert_eq!(*unit_b.meta.borrow(), 40);
        let expected = btreeset! {
            unit_a.clone(),
            unit_b.clone()
        };
        assert_eq!(
            *book.index.units.borrow(),
            expected,
            "Units are in the book"
        );
    }
    #[test]
    fn fmt_debug() {
        let book = Book::<BlankMetadata>::new(());
        let unit = Unit::new(&book, ());
        let actual = format!("{:?}", unit);
        let expected = "Unit { id: 0 }";
        assert_eq!(actual, expected);
        let unit = Unit::new(&book, ());
        let actual = format!("{:?}", unit);
        let expected = "Unit { id: 1 }";
        assert_eq!(actual, expected);
    }
    #[test]
    fn metadata() {
        let book = Book::<((), (), u8, ())>::new(());
        let unit = Unit::new(&book, 3);
        assert_eq!(*unit.get_metadata(), 3);
        unit.set_metadata(9);
        assert_eq!(*unit.get_metadata(), 9);
    }
}