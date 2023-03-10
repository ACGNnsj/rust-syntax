use std::sync::atomic::Ordering::AcqRel;

pub(in crate::test_orphan_rule) trait AnyTrait {
    fn any_trait(&self) -> usize;
}

pub(in super) trait LocalTrait {
    fn my_trait(&self) -> usize;
}