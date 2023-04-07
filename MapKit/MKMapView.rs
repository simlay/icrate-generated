//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "watchos")))]
    pub enum MKUserTrackingMode {
        #[cfg(not(any(target_os = "watchos")))]
        MKUserTrackingModeNone = 0,
        #[cfg(not(any(target_os = "watchos")))]
        MKUserTrackingModeFollow = 1,
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        MKUserTrackingModeFollowWithHeading = 2,
    }
);

extern_static!(MKMapViewDefaultAnnotationViewReuseIdentifier: &'static NSString);

extern_static!(MKMapViewDefaultClusterAnnotationViewReuseIdentifier: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapView")]
    pub struct MKMapView;

    #[cfg(feature = "MapKit_MKMapView")]
    unsafe impl ClassType for MKMapView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAccessibility for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAccessibilityElementProtocol for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAnimatablePropertyContainer for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAppearanceCustomization for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSCoding for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSDraggingDestination for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSObjectProtocol for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSUserInterfaceItemIdentification for MKMapView {}

extern_methods!(
    #[cfg(feature = "MapKit_MKMapView")]
    unsafe impl MKMapView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn MKMapViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn MKMapViewDelegate>>);

        #[deprecated]
        #[method(mapType)]
        pub unsafe fn mapType(&self) -> MKMapType;

        #[deprecated]
        #[method(setMapType:)]
        pub unsafe fn setMapType(&self, map_type: MKMapType);

        #[cfg(feature = "MapKit_MKMapConfiguration")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other preferredConfiguration)]
        pub unsafe fn preferredConfiguration(&self) -> Id<MKMapConfiguration>;

        #[cfg(feature = "MapKit_MKMapConfiguration")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setPreferredConfiguration:)]
        pub unsafe fn setPreferredConfiguration(
            &self,
            preferred_configuration: &MKMapConfiguration,
        );

        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[method(setRegion:animated:)]
        pub unsafe fn setRegion_animated(&self, region: MKCoordinateRegion, animated: bool);

        #[method(centerCoordinate)]
        pub unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D;

        #[method(setCenterCoordinate:)]
        pub unsafe fn setCenterCoordinate(&self, center_coordinate: CLLocationCoordinate2D);

        #[method(setCenterCoordinate:animated:)]
        pub unsafe fn setCenterCoordinate_animated(
            &self,
            coordinate: CLLocationCoordinate2D,
            animated: bool,
        );

        #[method(regionThatFits:)]
        pub unsafe fn regionThatFits(&self, region: MKCoordinateRegion) -> MKCoordinateRegion;

        #[method(visibleMapRect)]
        pub unsafe fn visibleMapRect(&self) -> MKMapRect;

        #[method(setVisibleMapRect:)]
        pub unsafe fn setVisibleMapRect(&self, visible_map_rect: MKMapRect);

        #[method(setVisibleMapRect:animated:)]
        pub unsafe fn setVisibleMapRect_animated(&self, map_rect: MKMapRect, animate: bool);

        #[method(mapRectThatFits:)]
        pub unsafe fn mapRectThatFits(&self, map_rect: MKMapRect) -> MKMapRect;

        #[method(setVisibleMapRect:edgePadding:animated:)]
        pub unsafe fn setVisibleMapRect_edgePadding_animated(
            &self,
            map_rect: MKMapRect,
            insets: NSEdgeInsets,
            animate: bool,
        );

        #[method(mapRectThatFits:edgePadding:)]
        pub unsafe fn mapRectThatFits_edgePadding(
            &self,
            map_rect: MKMapRect,
            insets: NSEdgeInsets,
        ) -> MKMapRect;

        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Id<MKMapCamera>;

        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: &MKMapCamera);

        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method(setCamera:animated:)]
        pub unsafe fn setCamera_animated(&self, camera: &MKMapCamera, animated: bool);

        #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other cameraZoomRange)]
        pub unsafe fn cameraZoomRange(&self) -> Id<MKMapCameraZoomRange>;

        #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setCameraZoomRange:)]
        pub unsafe fn setCameraZoomRange(&self, camera_zoom_range: Option<&MKMapCameraZoomRange>);

        #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setCameraZoomRange:animated:)]
        pub unsafe fn setCameraZoomRange_animated(
            &self,
            camera_zoom_range: Option<&MKMapCameraZoomRange>,
            animated: bool,
        );

        #[cfg(feature = "MapKit_MKMapCameraBoundary")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other cameraBoundary)]
        pub unsafe fn cameraBoundary(&self) -> Option<Id<MKMapCameraBoundary>>;

        #[cfg(feature = "MapKit_MKMapCameraBoundary")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setCameraBoundary:)]
        pub unsafe fn setCameraBoundary(&self, camera_boundary: Option<&MKMapCameraBoundary>);

        #[cfg(feature = "MapKit_MKMapCameraBoundary")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setCameraBoundary:animated:)]
        pub unsafe fn setCameraBoundary_animated(
            &self,
            camera_boundary: Option<&MKMapCameraBoundary>,
            animated: bool,
        );

        #[method(convertCoordinate:toPointToView:)]
        pub unsafe fn convertCoordinate_toPointToView(
            &self,
            coordinate: CLLocationCoordinate2D,
            view: Option<&NSView>,
        ) -> CGPoint;

        #[method(convertPoint:toCoordinateFromView:)]
        pub unsafe fn convertPoint_toCoordinateFromView(
            &self,
            point: CGPoint,
            view: Option<&NSView>,
        ) -> CLLocationCoordinate2D;

        #[method(convertRegion:toRectToView:)]
        pub unsafe fn convertRegion_toRectToView(
            &self,
            region: MKCoordinateRegion,
            view: Option<&NSView>,
        ) -> CGRect;

        #[method(convertRect:toRegionFromView:)]
        pub unsafe fn convertRect_toRegionFromView(
            &self,
            rect: CGRect,
            view: Option<&NSView>,
        ) -> MKCoordinateRegion;

        #[method(isZoomEnabled)]
        pub unsafe fn isZoomEnabled(&self) -> bool;

        #[method(setZoomEnabled:)]
        pub unsafe fn setZoomEnabled(&self, zoom_enabled: bool);

        #[method(isScrollEnabled)]
        pub unsafe fn isScrollEnabled(&self) -> bool;

        #[method(setScrollEnabled:)]
        pub unsafe fn setScrollEnabled(&self, scroll_enabled: bool);

        #[cfg(not(any(target_os = "tvos")))]
        #[method(isRotateEnabled)]
        pub unsafe fn isRotateEnabled(&self) -> bool;

        #[cfg(not(any(target_os = "tvos")))]
        #[method(setRotateEnabled:)]
        pub unsafe fn setRotateEnabled(&self, rotate_enabled: bool);

        #[cfg(not(any(target_os = "tvos")))]
        #[method(isPitchEnabled)]
        pub unsafe fn isPitchEnabled(&self) -> bool;

        #[cfg(not(any(target_os = "tvos")))]
        #[method(setPitchEnabled:)]
        pub unsafe fn setPitchEnabled(&self, pitch_enabled: bool);

        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(showsPitchControl)]
        pub unsafe fn showsPitchControl(&self) -> bool;

        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(setShowsPitchControl:)]
        pub unsafe fn setShowsPitchControl(&self, shows_pitch_control: bool);

        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(showsZoomControls)]
        pub unsafe fn showsZoomControls(&self) -> bool;

        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(setShowsZoomControls:)]
        pub unsafe fn setShowsZoomControls(&self, shows_zoom_controls: bool);

        #[cfg(not(any(target_os = "tvos")))]
        #[method(showsCompass)]
        pub unsafe fn showsCompass(&self) -> bool;

        #[cfg(not(any(target_os = "tvos")))]
        #[method(setShowsCompass:)]
        pub unsafe fn setShowsCompass(&self, shows_compass: bool);

        #[method(showsScale)]
        pub unsafe fn showsScale(&self) -> bool;

        #[method(setShowsScale:)]
        pub unsafe fn setShowsScale(&self, shows_scale: bool);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[deprecated]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[deprecated]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(showsPointsOfInterest)]
        pub unsafe fn showsPointsOfInterest(&self) -> bool;

        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setShowsPointsOfInterest:)]
        pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

        #[deprecated = "None"]
        #[method(showsBuildings)]
        pub unsafe fn showsBuildings(&self) -> bool;

        #[deprecated = "None"]
        #[method(setShowsBuildings:)]
        pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

        #[deprecated]
        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        #[deprecated]
        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);

        #[method(showsUserLocation)]
        pub unsafe fn showsUserLocation(&self) -> bool;

        #[method(setShowsUserLocation:)]
        pub unsafe fn setShowsUserLocation(&self, shows_user_location: bool);

        #[cfg(feature = "MapKit_MKUserLocation")]
        #[method_id(@__retain_semantics Other userLocation)]
        pub unsafe fn userLocation(&self) -> Id<MKUserLocation>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(userTrackingMode)]
        pub unsafe fn userTrackingMode(&self) -> MKUserTrackingMode;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setUserTrackingMode:)]
        pub unsafe fn setUserTrackingMode(&self, user_tracking_mode: MKUserTrackingMode);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setUserTrackingMode:animated:)]
        pub unsafe fn setUserTrackingMode_animated(&self, mode: MKUserTrackingMode, animated: bool);

        #[method(isUserLocationVisible)]
        pub unsafe fn isUserLocationVisible(&self) -> bool;

        #[method(addAnnotation:)]
        pub unsafe fn addAnnotation(&self, annotation: &ProtocolObject<dyn MKAnnotation>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addAnnotations:)]
        pub unsafe fn addAnnotations(
            &self,
            annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        );

        #[method(removeAnnotation:)]
        pub unsafe fn removeAnnotation(&self, annotation: &ProtocolObject<dyn MKAnnotation>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(removeAnnotations:)]
        pub unsafe fn removeAnnotations(
            &self,
            annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other annotations)]
        pub unsafe fn annotations(&self) -> Id<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other annotationsInMapRect:)]
        pub unsafe fn annotationsInMapRect(
            &self,
            map_rect: MKMapRect,
        ) -> Id<NSSet<ProtocolObject<dyn MKAnnotation>>>;

        #[cfg(feature = "MapKit_MKAnnotationView")]
        #[method_id(@__retain_semantics Other viewForAnnotation:)]
        pub unsafe fn viewForAnnotation(
            &self,
            annotation: &ProtocolObject<dyn MKAnnotation>,
        ) -> Option<Id<MKAnnotationView>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "MapKit_MKAnnotationView"))]
        #[method_id(@__retain_semantics Other dequeueReusableAnnotationViewWithIdentifier:)]
        pub unsafe fn dequeueReusableAnnotationViewWithIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Id<MKAnnotationView>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "MapKit_MKAnnotationView"))]
        #[method_id(@__retain_semantics Other dequeueReusableAnnotationViewWithIdentifier:forAnnotation:)]
        pub unsafe fn dequeueReusableAnnotationViewWithIdentifier_forAnnotation(
            &self,
            identifier: &NSString,
            annotation: &ProtocolObject<dyn MKAnnotation>,
        ) -> Id<MKAnnotationView>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerClass:forAnnotationViewWithReuseIdentifier:)]
        pub unsafe fn registerClass_forAnnotationViewWithReuseIdentifier(
            &self,
            view_class: Option<&Class>,
            identifier: &NSString,
        );

        #[method(selectAnnotation:animated:)]
        pub unsafe fn selectAnnotation_animated(
            &self,
            annotation: &ProtocolObject<dyn MKAnnotation>,
            animated: bool,
        );

        #[method(deselectAnnotation:animated:)]
        pub unsafe fn deselectAnnotation_animated(
            &self,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            animated: bool,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other selectedAnnotations)]
        pub unsafe fn selectedAnnotations(&self) -> Id<NSArray<ProtocolObject<dyn MKAnnotation>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSelectedAnnotations:)]
        pub unsafe fn setSelectedAnnotations(
            &self,
            selected_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        );

        #[method(annotationVisibleRect)]
        pub unsafe fn annotationVisibleRect(&self) -> CGRect;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(showAnnotations:animated:)]
        pub unsafe fn showAnnotations_animated(
            &self,
            annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
            animated: bool,
        );
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "watchos")))]
    pub enum MKOverlayLevel {
        #[cfg(not(any(target_os = "watchos")))]
        MKOverlayLevelAboveRoads = 0,
        #[cfg(not(any(target_os = "watchos")))]
        MKOverlayLevelAboveLabels = 1,
    }
);

extern_methods!(
    /// OverlaysAPI
    #[cfg(feature = "MapKit_MKMapView")]
    unsafe impl MKMapView {
        #[method(addOverlay:level:)]
        pub unsafe fn addOverlay_level(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
            level: MKOverlayLevel,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addOverlays:level:)]
        pub unsafe fn addOverlays_level(
            &self,
            overlays: &NSArray<ProtocolObject<dyn MKOverlay>>,
            level: MKOverlayLevel,
        );

        #[method(removeOverlay:)]
        pub unsafe fn removeOverlay(&self, overlay: &ProtocolObject<dyn MKOverlay>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(removeOverlays:)]
        pub unsafe fn removeOverlays(&self, overlays: &NSArray<ProtocolObject<dyn MKOverlay>>);

        #[method(insertOverlay:atIndex:level:)]
        pub unsafe fn insertOverlay_atIndex_level(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
            index: NSUInteger,
            level: MKOverlayLevel,
        );

        #[method(insertOverlay:aboveOverlay:)]
        pub unsafe fn insertOverlay_aboveOverlay(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
            sibling: &ProtocolObject<dyn MKOverlay>,
        );

        #[method(insertOverlay:belowOverlay:)]
        pub unsafe fn insertOverlay_belowOverlay(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
            sibling: &ProtocolObject<dyn MKOverlay>,
        );

        #[method(exchangeOverlay:withOverlay:)]
        pub unsafe fn exchangeOverlay_withOverlay(
            &self,
            overlay1: &ProtocolObject<dyn MKOverlay>,
            overlay2: &ProtocolObject<dyn MKOverlay>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other overlays)]
        pub unsafe fn overlays(&self) -> Id<NSArray<ProtocolObject<dyn MKOverlay>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other overlaysInLevel:)]
        pub unsafe fn overlaysInLevel(
            &self,
            level: MKOverlayLevel,
        ) -> Id<NSArray<ProtocolObject<dyn MKOverlay>>>;

        #[cfg(feature = "MapKit_MKOverlayRenderer")]
        #[method_id(@__retain_semantics Other rendererForOverlay:)]
        pub unsafe fn rendererForOverlay(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Option<Id<MKOverlayRenderer>>;

        #[method(addOverlay:)]
        pub unsafe fn addOverlay(&self, overlay: &ProtocolObject<dyn MKOverlay>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addOverlays:)]
        pub unsafe fn addOverlays(&self, overlays: &NSArray<ProtocolObject<dyn MKOverlay>>);

        #[method(insertOverlay:atIndex:)]
        pub unsafe fn insertOverlay_atIndex(
            &self,
            overlay: &ProtocolObject<dyn MKOverlay>,
            index: NSUInteger,
        );

        #[method(exchangeOverlayAtIndex:withOverlayAtIndex:)]
        pub unsafe fn exchangeOverlayAtIndex_withOverlayAtIndex(
            &self,
            index1: NSUInteger,
            index2: NSUInteger,
        );
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "watchos")))]
    pub unsafe trait MKMapViewDelegate: NSObjectProtocol {
        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapView:regionWillChangeAnimated:)]
        unsafe fn mapView_regionWillChangeAnimated(&self, map_view: &MKMapView, animated: bool);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapView:regionDidChangeAnimated:)]
        unsafe fn mapView_regionDidChangeAnimated(&self, map_view: &MKMapView, animated: bool);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewDidChangeVisibleRegion:)]
        unsafe fn mapViewDidChangeVisibleRegion(&self, map_view: &MKMapView);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewWillStartLoadingMap:)]
        unsafe fn mapViewWillStartLoadingMap(&self, map_view: &MKMapView);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewDidFinishLoadingMap:)]
        unsafe fn mapViewDidFinishLoadingMap(&self, map_view: &MKMapView);

        #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKMapView"))]
        #[optional]
        #[method(mapViewDidFailLoadingMap:withError:)]
        unsafe fn mapViewDidFailLoadingMap_withError(&self, map_view: &MKMapView, error: &NSError);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewWillStartRenderingMap:)]
        unsafe fn mapViewWillStartRenderingMap(&self, map_view: &MKMapView);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewDidFinishRenderingMap:fullyRendered:)]
        unsafe fn mapViewDidFinishRenderingMap_fullyRendered(
            &self,
            map_view: &MKMapView,
            fully_rendered: bool,
        );

        #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
        #[optional]
        #[method_id(@__retain_semantics Other mapView:viewForAnnotation:)]
        unsafe fn mapView_viewForAnnotation(
            &self,
            map_view: &MKMapView,
            annotation: &ProtocolObject<dyn MKAnnotation>,
        ) -> Option<Id<MKAnnotationView>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MapKit_MKAnnotationView",
            feature = "MapKit_MKMapView"
        ))]
        #[optional]
        #[method(mapView:didAddAnnotationViews:)]
        unsafe fn mapView_didAddAnnotationViews(
            &self,
            map_view: &MKMapView,
            views: &NSArray<MKAnnotationView>,
        );

        #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
        #[optional]
        #[method(mapView:didSelectAnnotationView:)]
        unsafe fn mapView_didSelectAnnotationView(
            &self,
            map_view: &MKMapView,
            view: &MKAnnotationView,
        );

        #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
        #[optional]
        #[method(mapView:didDeselectAnnotationView:)]
        unsafe fn mapView_didDeselectAnnotationView(
            &self,
            map_view: &MKMapView,
            view: &MKAnnotationView,
        );

        #[cfg(feature = "MapKit_MKMapView")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(mapView:didSelectAnnotation:)]
        unsafe fn mapView_didSelectAnnotation(
            &self,
            map_view: &MKMapView,
            annotation: &ProtocolObject<dyn MKAnnotation>,
        );

        #[cfg(feature = "MapKit_MKMapView")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(mapView:didDeselectAnnotation:)]
        unsafe fn mapView_didDeselectAnnotation(
            &self,
            map_view: &MKMapView,
            annotation: &ProtocolObject<dyn MKAnnotation>,
        );

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewWillStartLocatingUser:)]
        unsafe fn mapViewWillStartLocatingUser(&self, map_view: &MKMapView);

        #[cfg(feature = "MapKit_MKMapView")]
        #[optional]
        #[method(mapViewDidStopLocatingUser:)]
        unsafe fn mapViewDidStopLocatingUser(&self, map_view: &MKMapView);

        #[cfg(all(feature = "MapKit_MKMapView", feature = "MapKit_MKUserLocation"))]
        #[optional]
        #[method(mapView:didUpdateUserLocation:)]
        unsafe fn mapView_didUpdateUserLocation(
            &self,
            map_view: &MKMapView,
            user_location: &MKUserLocation,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKMapView"))]
        #[optional]
        #[method(mapView:didFailToLocateUserWithError:)]
        unsafe fn mapView_didFailToLocateUserWithError(
            &self,
            map_view: &MKMapView,
            error: &NSError,
        );

        #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
        #[cfg(not(any(target_os = "tvos")))]
        #[optional]
        #[method(mapView:annotationView:didChangeDragState:fromOldState:)]
        unsafe fn mapView_annotationView_didChangeDragState_fromOldState(
            &self,
            map_view: &MKMapView,
            view: &MKAnnotationView,
            new_state: MKAnnotationViewDragState,
            old_state: MKAnnotationViewDragState,
        );

        #[cfg(feature = "MapKit_MKMapView")]
        #[cfg(not(any(target_os = "watchos")))]
        #[optional]
        #[method(mapView:didChangeUserTrackingMode:animated:)]
        unsafe fn mapView_didChangeUserTrackingMode_animated(
            &self,
            map_view: &MKMapView,
            mode: MKUserTrackingMode,
            animated: bool,
        );

        #[cfg(all(feature = "MapKit_MKMapView", feature = "MapKit_MKOverlayRenderer"))]
        #[optional]
        #[method_id(@__retain_semantics Other mapView:rendererForOverlay:)]
        unsafe fn mapView_rendererForOverlay(
            &self,
            map_view: &MKMapView,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<MKOverlayRenderer>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MapKit_MKMapView",
            feature = "MapKit_MKOverlayRenderer"
        ))]
        #[optional]
        #[method(mapView:didAddOverlayRenderers:)]
        unsafe fn mapView_didAddOverlayRenderers(
            &self,
            map_view: &MKMapView,
            renderers: &NSArray<MKOverlayRenderer>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MapKit_MKClusterAnnotation",
            feature = "MapKit_MKMapView"
        ))]
        #[cfg(not(any(target_os = "watchos")))]
        #[optional]
        #[method_id(@__retain_semantics Other mapView:clusterAnnotationForMemberAnnotations:)]
        unsafe fn mapView_clusterAnnotationForMemberAnnotations(
            &self,
            map_view: &MKMapView,
            member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        ) -> Id<MKClusterAnnotation>;
    }

    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl ProtocolType for dyn MKMapViewDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKMapView")]
    unsafe impl MKMapView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
