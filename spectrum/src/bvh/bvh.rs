use std::sync::Arc;
use crate::base::ray::Ray;

//bounding volume hierarchy node
pub trait BVHNode {
    fn intersect(&self, ray :&Ray) -> bool;
}

//
pub struct BVHTree{
    pub left: Option<Arc<BVHTree>>,
    pub right: Option<Arc<BVHTree>>, 
}