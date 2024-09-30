
pub trait TryFromIterator<T>: Sized {
    type Error;

    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Result<Self, Self::Error>;
}
