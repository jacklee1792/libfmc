use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Copy)]
pub struct Set64<T> {
    bits: u64,
    _marker: PhantomData<T>,
}

#[derive(Default)]
pub struct Set64Iter<T>(Set64<T>);

impl<T> FromIterator<T> for Set64<T>
where
    T: From<u8> + Into<u8>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::default();
        for x in iter {
            ret.insert(x);
        }
        ret
    }
}

impl<T> Iterator for Set64Iter<T>
where
    T: From<u8> + Into<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.bits == 0 {
            None
        } else {
            self.0.pop_first()
        }
    }
}

impl<T> IntoIterator for Set64<T>
where
    T: From<u8> + Into<u8>,
{
    type Item = T;
    type IntoIter = Set64Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Set64Iter(self)
    }
}

impl<T> Clone for Set64<T> {
    fn clone(&self) -> Self {
        Self {
            bits: self.bits,
            _marker: Default::default(),
        }
    }
}

impl<T> Debug for Set64<T>
where
    T: Debug + From<u8> + Into<u8>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .clone()
            .into_iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<_>>();
        write!(f, "{{{}}}", items.join(", "))
    }
}

impl<T> Default for Set64<T> {
    fn default() -> Self {
        Self {
            bits: 0,
            _marker: Default::default(),
        }
    }
}

impl<T> Set64<T>
where
    T: From<u8> + Into<u8>,
{
    pub fn new() -> Self {
        Self {
            bits: 0,
            _marker: Default::default(),
        }
    }

    pub fn from_u64(bits: u64) -> Self {
        Self {
            bits,
            _marker: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn insert(&mut self, item: T) -> bool {
        let prev = self.bits;
        self.bits |= 1 << item.into();
        self.bits != prev
    }

    pub fn remove(&mut self, item: T) -> bool {
        let prev = self.bits;
        self.bits &= !(1 << item.into());
        self.bits != prev
    }

    pub fn contains(&self, item: T) -> bool {
        (self.bits & (1 << item.into())) != 0
    }

    pub fn first(&self) -> Option<T> {
        if self.bits == 0 {
            None
        } else {
            Some((self.bits.trailing_zeros() as u8).into())
        }
    }

    pub fn pop_first(&mut self) -> Option<T> {
        if self.bits == 0 {
            None
        } else {
            let b = self.bits.trailing_zeros() as u8;
            self.bits ^= 1 << b;
            Some(b.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Edge;
    use crate::Set64;

    #[test]
    fn test_set64_edge() {
        use Edge::*;
        let mut s = [UF, UR, FL].into_iter().collect::<Set64<_>>();
        assert_eq!(s.len(), 3);
        assert_eq!(s.contains(UF), true);
        assert_eq!(s.contains(UL), false);
        assert_eq!(s.remove(FL), true);
        assert_eq!(s.len(), 2);
        assert_eq!(s.pop_first().is_some(), true);
        assert_eq!(s.len(), 1);
        assert_eq!(s.pop_first().is_some(), true);
        assert_eq!(s.len(), 0);
        assert_eq!(s.pop_first(), None);
        assert_eq!(s.is_empty(), true);
    }
}
