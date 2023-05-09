use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use thiserror::Error;

pub struct NonZeroU32(u32);
impl NonZeroU32 {
    pub fn new(number: u32) -> Result<Self, String> {
        if number > 0 {
            Ok(NonZeroU32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

pub struct NonZeroF32(f32);
impl NonZeroF32 {
    pub fn new(number: f32) -> Result<Self, String> {
        if number > 0.0 {
            Ok(NonZeroF32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

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
    pub fn first(self) -> T {
        self.0
    }
    pub fn rest(self) -> Vec<T> {
        self.1
    }
    pub fn sort_by_key<F, K>(&self, key_fn: F) -> Self where
    F: FnMut(&T) -> K,
    K: Ord,  {
        let mut vec = self.get_all();
        vec.sort_by_key(key_fn);

        Self::new(vec).expect("NonEmptyVec::sort_by_key should never return an empty vector")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_personal_token: Option<String>,
    pub github_owner: String,
    pub github_repo: String,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}
