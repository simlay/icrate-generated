//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKTileOverlay")]
    /**
      MKTileOverlay represents a data source for raster image tiles in the spherical mercator projection (EPSG:3857).
    */
    pub struct MKTileOverlay;

    #[cfg(feature = "MapKit_MKTileOverlay")]
    unsafe impl ClassType for MKTileOverlay {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKTileOverlay")]
/**
  MKTileOverlay represents a data source for raster image tiles in the spherical mercator projection (EPSG:3857).
*/
unsafe impl MKAnnotation for MKTileOverlay {}

#[cfg(feature = "MapKit_MKTileOverlay")]
/**
  MKTileOverlay represents a data source for raster image tiles in the spherical mercator projection (EPSG:3857).
*/
unsafe impl MKOverlay for MKTileOverlay {}

#[cfg(feature = "MapKit_MKTileOverlay")]
/**
  MKTileOverlay represents a data source for raster image tiles in the spherical mercator projection (EPSG:3857).
*/
unsafe impl NSObjectProtocol for MKTileOverlay {}

extern_methods!(
    /**
      MKTileOverlay represents a data source for raster image tiles in the spherical mercator projection (EPSG:3857).
    */
    #[cfg(feature = "MapKit_MKTileOverlay")]
    unsafe impl MKTileOverlay {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithURLTemplate:)]
        pub unsafe fn initWithURLTemplate(
            this: Option<Allocated<Self>>,
            url_template: Option<&NSString>,
        ) -> Id<Self>;

        /**
          default is 256x256
        */
        #[method(tileSize)]
        pub unsafe fn tileSize(&self) -> CGSize;

        /**
          default is 256x256
        */
        #[method(setTileSize:)]
        pub unsafe fn setTileSize(&self, tile_size: CGSize);

        /**
          Default is NO. If NO, a tile at x=0, y=0 is the upper left, otherwise it is in the lower left.
        */
        #[method(isGeometryFlipped)]
        pub unsafe fn isGeometryFlipped(&self) -> bool;

        /**
          Default is NO. If NO, a tile at x=0, y=0 is the upper left, otherwise it is in the lower left.
        */
        #[method(setGeometryFlipped:)]
        pub unsafe fn setGeometryFlipped(&self, geometry_flipped: bool);

        /**
          The minimum/maximum zoom level at which tile data is available for this overlay. A tile at level 0 covers the entire world, at level 1 it covers 1/4th of the world, at level 2 it covers 1/16th of the world, and so on.
        */
        #[method(minimumZ)]
        pub unsafe fn minimumZ(&self) -> NSInteger;

        /**
          The minimum/maximum zoom level at which tile data is available for this overlay. A tile at level 0 covers the entire world, at level 1 it covers 1/4th of the world, at level 2 it covers 1/16th of the world, and so on.
        */
        #[method(setMinimumZ:)]
        pub unsafe fn setMinimumZ(&self, minimum_z: NSInteger);

        #[method(maximumZ)]
        pub unsafe fn maximumZ(&self) -> NSInteger;

        #[method(setMaximumZ:)]
        pub unsafe fn setMaximumZ(&self, maximum_z: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other URLTemplate)]
        pub unsafe fn URLTemplate(&self) -> Option<Id<NSString>>;

        #[method(canReplaceMapContent)]
        pub unsafe fn canReplaceMapContent(&self) -> bool;

        #[method(setCanReplaceMapContent:)]
        pub unsafe fn setCanReplaceMapContent(&self, can_replace_map_content: bool);
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MKTileOverlayPath {
        pub x: NSInteger,
        pub y: NSInteger,
        pub z: NSInteger,
        pub contentScaleFactor: CGFloat,
    }
);

extern_methods!(
    /**
      Subclassers may override these methods to customize the loading behavior of MKTileOverlay
    */
    /// CustomLoading
    #[cfg(feature = "MapKit_MKTileOverlay")]
    unsafe impl MKTileOverlay {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URLForTilePath:)]
        pub unsafe fn URLForTilePath(&self, path: MKTileOverlayPath) -> Id<NSURL>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(loadTileAtPath:result:)]
        pub unsafe fn loadTileAtPath_result(
            &self,
            path: MKTileOverlayPath,
            result: &Block<(*mut NSData, *mut NSError), ()>,
        );
    }
);
