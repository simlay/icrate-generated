//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_protocol!(
    pub unsafe trait MKGeoJSONObject: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn MKGeoJSONObject {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKGeoJSONDecoder")]
    pub struct MKGeoJSONDecoder;

    #[cfg(feature = "MapKit_MKGeoJSONDecoder")]
    unsafe impl ClassType for MKGeoJSONDecoder {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKGeoJSONDecoder")]
unsafe impl NSObjectProtocol for MKGeoJSONDecoder {}

extern_methods!(
    #[cfg(feature = "MapKit_MKGeoJSONDecoder")]
    unsafe impl MKGeoJSONDecoder {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other geoJSONObjectsWithData:error:_)]
        pub unsafe fn geoJSONObjectsWithData_error(
            &self,
            data: &NSData,
        ) -> Result<Id<NSArray<ProtocolObject<dyn MKGeoJSONObject>>>, Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKGeoJSONFeature")]
    pub struct MKGeoJSONFeature;

    #[cfg(feature = "MapKit_MKGeoJSONFeature")]
    unsafe impl ClassType for MKGeoJSONFeature {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKGeoJSONFeature")]
unsafe impl MKGeoJSONObject for MKGeoJSONFeature {}

#[cfg(feature = "MapKit_MKGeoJSONFeature")]
unsafe impl NSObjectProtocol for MKGeoJSONFeature {}

extern_methods!(
    #[cfg(feature = "MapKit_MKGeoJSONFeature")]
    unsafe impl MKGeoJSONFeature {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKShape"))]
        #[method_id(@__retain_semantics Other geometry)]
        pub unsafe fn geometry(&self) -> Id<NSArray<MKShape>>;
    }
);

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKPointAnnotation")]
    unsafe impl MKPointAnnotation {}
);

#[cfg(feature = "MapKit_MKPointAnnotation")]
unsafe impl MKGeoJSONObject for MKPointAnnotation {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKMultiPoint")]
    unsafe impl MKMultiPoint {}
);

#[cfg(feature = "MapKit_MKMultiPoint")]
unsafe impl MKGeoJSONObject for MKMultiPoint {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKMultiPolyline")]
    unsafe impl MKMultiPolyline {}
);

#[cfg(feature = "MapKit_MKMultiPolyline")]
unsafe impl MKGeoJSONObject for MKMultiPolyline {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    unsafe impl MKMultiPolygon {}
);

#[cfg(feature = "MapKit_MKMultiPolygon")]
unsafe impl MKGeoJSONObject for MKMultiPolygon {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKPolyline")]
    unsafe impl MKPolyline {}
);

#[cfg(feature = "MapKit_MKPolyline")]
unsafe impl MKGeoJSONObject for MKPolyline {}

extern_methods!(
    /// MKGeoJSONSerialization
    #[cfg(feature = "MapKit_MKPolygon")]
    unsafe impl MKPolygon {}
);

#[cfg(feature = "MapKit_MKPolygon")]
unsafe impl MKGeoJSONObject for MKPolygon {}
