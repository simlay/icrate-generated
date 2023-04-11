//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_struct!(
    #[encoding_name("_MTLPackedFloat4x3")]
    pub struct MTLPackedFloat4x3 {
        pub columns: [MTLPackedFloat3; 4],
    }
);

extern_struct!(
    #[encoding_name("_MTLAxisAlignedBoundingBox")]
    /**
      @brief An axis aligned bounding box with a min and max point
    */
    pub struct MTLAxisAlignedBoundingBox {
        pub min: MTLPackedFloat3,
        pub max: MTLPackedFloat3,
    }
);
