use thiserror::Error;

#[derive(Clone, Debug)]
pub struct NonEmptyVec<T: Clone>(T, Vec<T>);
#[derive(Debug, Error)]
pub enum NonEmptyVecCreateErr {
    #[error("Cannot create a NonEmptyVec from an empty vector")]
    EmptyVec,
}
impl<T: Clone> NonEmptyVec<T> {
    pub fn new(vec: Vec<T>) -> Result<NonEmptyVec<T>, NonEmptyVecCreateErr> {
        let (first, rest) = vec.split_first().ok_or(NonEmptyVecCreateErr::EmptyVec)?;
        Ok(NonEmptyVec(first.clone(), rest.to_vec()))
    }
    pub fn get_all(self) -> Vec<T> {
        let first = vec![self.0];
        let rest = self.1;
        [first, rest].concat()
    }
    pub fn sort_by_key<F, K>(&mut self, key_fn: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut vec = self.clone().get_all();
        vec.sort_by_key(key_fn);

        *self =
            Self::new(vec).expect("NonEmptyVec::sort_by_key should never return an empty vector");
    }
    pub fn get(self) -> (T, Vec<T>) {
        (self.0, self.1)
    }
}
