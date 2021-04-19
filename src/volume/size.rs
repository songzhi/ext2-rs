use core::fmt::{self, Display};
use core::cmp::Ordering;

use sector::{Address, SectorSize};

#[derive(Clone, Copy, Debug)]
pub enum Size<S: SectorSize> {
    Unbounded,
    Bounded(Address<S>),
}

impl<S: SectorSize> Size<S> {
    pub fn try_len(&self) -> Option<Address<S>> {
        match *self {
            Size::Unbounded => None,
            Size::Bounded(n) => Some(n),
        }
    }

    /// Returns `true` if the size is [`Bounded`].
    pub fn is_bounded(&self) -> bool {
        matches!(self, Self::Bounded(..))
    }
}

impl<S: SectorSize> Display for Size<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Size::Unbounded => write!(f, "Unbounded"),
            Size::Bounded(n) => write!(f, "Bounded({})", n),
        }
    }
}

impl<S: SectorSize> PartialEq for Size<S> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Size::Unbounded, _) => false,
            (_, &Size::Unbounded) => false,
            (&Size::Bounded(ref a), &Size::Bounded(ref b)) => a.eq(b),
        }
    }
}

impl<S: SectorSize> PartialEq<Address<S>> for Size<S> {
    fn eq(&self, rhs: &Address<S>) -> bool {
        match *self {
            Size::Unbounded => false,
            Size::Bounded(ref n) => n.eq(rhs),
        }
    }
}

impl<S: SectorSize> PartialOrd for Size<S> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match (self, rhs) {
            (&Size::Unbounded, &Size::Unbounded) => None,
            (&Size::Unbounded, _) => Some(Ordering::Greater),
            (_, &Size::Unbounded) => Some(Ordering::Less),
            (&Size::Bounded(ref a), &Size::Bounded(ref b)) => a.partial_cmp(b),
        }
    }
}

impl<S: SectorSize> PartialOrd<Address<S>> for Size<S> {
    fn partial_cmp(&self, rhs: &Address<S>) -> Option<Ordering> {
        match *self {
            Size::Unbounded => Some(Ordering::Greater),
            Size::Bounded(ref n) => n.partial_cmp(rhs),
        }
    }
}
