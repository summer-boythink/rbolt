use quickcheck::{Arbitrary, Gen};
use std::fmt;

use super::Pgid;

/// Pgids 结构体，表示一组页 ID
#[derive(Debug, Default, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Pgids(pub Vec<Pgid>);

impl Pgids {
    /// 合并并排序两个 Pgids 的合集
    pub fn merge(&self, other: &Pgids) -> Pgids {
        let mut merged = self.0.clone();
        merged.extend(&other.0);
        merged.sort();
        Pgids(merged)
    }

    /// 获取 Pgids 的长度
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 对 Pgids 进行排序
    pub fn sort(&mut self) {
        self.0.sort();
    }
}

impl Arbitrary for Pgids {
    fn arbitrary(g: &mut Gen) -> Self {
        let vec = Vec::<Pgid>::arbitrary(g);
        Pgids(vec)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(Pgids))
    }
}

impl fmt::Display for Pgids {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pgids({:?})", self.0)
    }
}
