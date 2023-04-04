//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "MKAnnotation.rs"]
mod __MKAnnotation;
#[path = "MKAnnotationView.rs"]
mod __MKAnnotationView;
#[path = "MKCircle.rs"]
mod __MKCircle;
#[path = "MKCircleRenderer.rs"]
mod __MKCircleRenderer;
#[path = "MKClusterAnnotation.rs"]
mod __MKClusterAnnotation;
#[path = "MKCompassButton.rs"]
mod __MKCompassButton;
#[path = "MKDirections.rs"]
mod __MKDirections;
#[path = "MKDirectionsRequest.rs"]
mod __MKDirectionsRequest;
#[path = "MKDirectionsResponse.rs"]
mod __MKDirectionsResponse;
#[path = "MKDirectionsTypes.rs"]
mod __MKDirectionsTypes;
#[path = "MKDistanceFormatter.rs"]
mod __MKDistanceFormatter;
#[path = "MKFoundation.rs"]
mod __MKFoundation;
#[path = "MKGeoJSONSerialization.rs"]
mod __MKGeoJSONSerialization;
#[path = "MKGeodesicPolyline.rs"]
mod __MKGeodesicPolyline;
#[path = "MKGeometry.rs"]
mod __MKGeometry;
#[path = "MKGradientPolylineRenderer.rs"]
mod __MKGradientPolylineRenderer;
#[path = "MKHybridMapConfiguration.rs"]
mod __MKHybridMapConfiguration;
#[path = "MKImageryMapConfiguration.rs"]
mod __MKImageryMapConfiguration;
#[path = "MKLocalPointsOfInterestRequest.rs"]
mod __MKLocalPointsOfInterestRequest;
#[path = "MKLocalSearch.rs"]
mod __MKLocalSearch;
#[path = "MKLocalSearchCompleter.rs"]
mod __MKLocalSearchCompleter;
#[path = "MKLocalSearchRequest.rs"]
mod __MKLocalSearchRequest;
#[path = "MKLocalSearchResponse.rs"]
mod __MKLocalSearchResponse;
#[path = "MKLookAroundScene.rs"]
mod __MKLookAroundScene;
#[path = "MKLookAroundSceneRequest.rs"]
mod __MKLookAroundSceneRequest;
#[path = "MKLookAroundSnapshot.rs"]
mod __MKLookAroundSnapshot;
#[path = "MKLookAroundSnapshotOptions.rs"]
mod __MKLookAroundSnapshotOptions;
#[path = "MKLookAroundSnapshotter.rs"]
mod __MKLookAroundSnapshotter;
#[path = "MKLookAroundViewController.rs"]
mod __MKLookAroundViewController;
#[path = "MKMapCamera.rs"]
mod __MKMapCamera;
#[path = "MKMapCameraBoundary.rs"]
mod __MKMapCameraBoundary;
#[path = "MKMapCameraZoomRange.rs"]
mod __MKMapCameraZoomRange;
#[path = "MKMapConfiguration.rs"]
mod __MKMapConfiguration;
#[path = "MKMapItem.rs"]
mod __MKMapItem;
#[path = "MKMapSnapshot.rs"]
mod __MKMapSnapshot;
#[path = "MKMapSnapshotOptions.rs"]
mod __MKMapSnapshotOptions;
#[path = "MKMapSnapshotter.rs"]
mod __MKMapSnapshotter;
#[path = "MKMapView.rs"]
mod __MKMapView;
#[path = "MKMarkerAnnotationView.rs"]
mod __MKMarkerAnnotationView;
#[path = "MKMultiPoint.rs"]
mod __MKMultiPoint;
#[path = "MKMultiPolygon.rs"]
mod __MKMultiPolygon;
#[path = "MKMultiPolygonRenderer.rs"]
mod __MKMultiPolygonRenderer;
#[path = "MKMultiPolyline.rs"]
mod __MKMultiPolyline;
#[path = "MKMultiPolylineRenderer.rs"]
mod __MKMultiPolylineRenderer;
#[path = "MKOverlay.rs"]
mod __MKOverlay;
#[path = "MKOverlayPathRenderer.rs"]
mod __MKOverlayPathRenderer;
#[path = "MKOverlayRenderer.rs"]
mod __MKOverlayRenderer;
#[path = "MKPinAnnotationView.rs"]
mod __MKPinAnnotationView;
#[path = "MKPitchControl.rs"]
mod __MKPitchControl;
#[path = "MKPlacemark.rs"]
mod __MKPlacemark;
#[path = "MKPointAnnotation.rs"]
mod __MKPointAnnotation;
#[path = "MKPointOfInterestCategory.rs"]
mod __MKPointOfInterestCategory;
#[path = "MKPointOfInterestFilter.rs"]
mod __MKPointOfInterestFilter;
#[path = "MKPolygon.rs"]
mod __MKPolygon;
#[path = "MKPolygonRenderer.rs"]
mod __MKPolygonRenderer;
#[path = "MKPolyline.rs"]
mod __MKPolyline;
#[path = "MKPolylineRenderer.rs"]
mod __MKPolylineRenderer;
#[path = "MKShape.rs"]
mod __MKShape;
#[path = "MKStandardMapConfiguration.rs"]
mod __MKStandardMapConfiguration;
#[path = "MKTileOverlay.rs"]
mod __MKTileOverlay;
#[path = "MKTileOverlayRenderer.rs"]
mod __MKTileOverlayRenderer;
#[path = "MKTypes.rs"]
mod __MKTypes;
#[path = "MKUserLocation.rs"]
mod __MKUserLocation;
#[path = "MKUserLocationView.rs"]
mod __MKUserLocationView;
#[path = "MKZoomControl.rs"]
mod __MKZoomControl;
#[path = "NSUserActivity_MKMapItem.rs"]
mod __NSUserActivity_MKMapItem;

pub use self::__MKAnnotation::MKAnnotation;

pub use self::__MKAnnotationView::MKAnnotationCalloutInfoDidChangeNotification;
#[cfg(feature = "MapKit_MKAnnotationView")]
pub use self::__MKAnnotationView::MKAnnotationView;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewCollisionMode;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewCollisionModeCircle;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewCollisionModeNone;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewCollisionModeRectangle;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragState;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragStateCanceling;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragStateDragging;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragStateEnding;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragStateNone;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewDragStateStarting;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewZPriority;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityDefaultSelected;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityDefaultUnselected;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityMax;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKAnnotationViewZPriorityMin;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKFeatureDisplayPriority;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityDefaultHigh;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityDefaultLow;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKAnnotationView::MKFeatureDisplayPriorityRequired;
#[cfg(feature = "MapKit_MKCircle")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKCircle::MKCircle;
#[cfg(feature = "MapKit_MKCircleRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKCircleRenderer::MKCircleRenderer;
#[cfg(feature = "MapKit_MKClusterAnnotation")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKClusterAnnotation::MKClusterAnnotation;
#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKCompassButton::MKCompassButton;

pub use self::__MKDirections::MKDirectionsHandler;

#[cfg(feature = "MapKit_MKDirections")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirections::MKDirections;
pub use self::__MKDirections::MKETAHandler;

pub use self::__MKDirectionsRequest::MKDirectionsRoutePreference;

pub use self::__MKDirectionsRequest::MKDirectionsRoutePreferenceAny;

#[cfg(feature = "MapKit_MKDirectionsRequest")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsRequest::MKDirectionsRequest;
pub use self::__MKDirectionsRequest::MKDirectionsRoutePreferenceAvoid;
#[cfg(feature = "MapKit_MKDirectionsResponse")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsResponse::MKDirectionsResponse;
#[cfg(feature = "MapKit_MKETAResponse")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsResponse::MKETAResponse;
#[cfg(feature = "MapKit_MKRoute")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsResponse::MKRoute;
#[cfg(feature = "MapKit_MKRouteStep")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsResponse::MKRouteStep;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsTypes::MKDirectionsTransportType;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsTypes::MKDirectionsTransportTypeAny;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsTypes::MKDirectionsTransportTypeAutomobile;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsTypes::MKDirectionsTransportTypeTransit;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKDirectionsTypes::MKDirectionsTransportTypeWalking;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnits;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitsDefault;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitsMetric;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitsImperial;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitsImperialWithYards;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitStyle;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitStyleDefault;

pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitStyleAbbreviated;

#[cfg(feature = "MapKit_MKDistanceFormatter")]
pub use self::__MKDistanceFormatter::MKDistanceFormatter;
pub use self::__MKDistanceFormatter::MKDistanceFormatterUnitStyleFull;
#[cfg(feature = "MapKit_MKGeoJSONDecoder")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKGeoJSONSerialization::MKGeoJSONDecoder;
#[cfg(feature = "MapKit_MKGeoJSONFeature")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKGeoJSONSerialization::MKGeoJSONFeature;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKGeoJSONSerialization::MKGeoJSONObject;
#[cfg(feature = "MapKit_MKGeodesicPolyline")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKGeodesicPolyline::MKGeodesicPolyline;

pub use self::__MKGeometry::MKCoordinateSpan;

pub use self::__MKGeometry::MKCoordinateRegion;

pub use self::__MKGeometry::MKCoordinateRegionMakeWithDistance;

pub use self::__MKGeometry::MKMapPoint;

pub use self::__MKGeometry::MKMapSize;

pub use self::__MKGeometry::MKMapRect;

pub use self::__MKGeometry::MKZoomScale;

pub use self::__MKGeometry::MKMapSizeWorld;

pub use self::__MKGeometry::MKMapRectWorld;

pub use self::__MKGeometry::MKMapPointForCoordinate;

pub use self::__MKGeometry::MKCoordinateForMapPoint;

pub use self::__MKGeometry::MKMetersPerMapPointAtLatitude;

pub use self::__MKGeometry::MKMapPointsPerMeterAtLatitude;

pub use self::__MKGeometry::MKMetersBetweenMapPoints;

pub use self::__MKGeometry::MKMapRectNull;

pub use self::__MKGeometry::MKMapRectUnion;

pub use self::__MKGeometry::MKMapRectIntersection;

pub use self::__MKGeometry::MKMapRectInset;

pub use self::__MKGeometry::MKMapRectOffset;

pub use self::__MKGeometry::MKMapRectContainsPoint;

pub use self::__MKGeometry::MKMapRectContainsRect;

pub use self::__MKGeometry::MKMapRectIntersectsRect;

pub use self::__MKGeometry::MKCoordinateRegionForMapRect;

pub use self::__MKGeometry::MKMapRectSpans180thMeridian;

pub use self::__MKGeometry::MKMapRectRemainder;
#[cfg(feature = "MapKit_MKGradientPolylineRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKGradientPolylineRenderer::MKGradientPolylineRenderer;
#[cfg(feature = "MapKit_MKHybridMapConfiguration")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKHybridMapConfiguration::MKHybridMapConfiguration;
#[cfg(feature = "MapKit_MKImageryMapConfiguration")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKImageryMapConfiguration::MKImageryMapConfiguration;
#[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalPointsOfInterestRequest::MKLocalPointsOfInterestRequest;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalPointsOfInterestRequest::MKPointsOfInterestRequestMaxRadius;

#[cfg(feature = "MapKit_MKLocalSearch")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearch::MKLocalSearch;
pub use self::__MKLocalSearch::MKLocalSearchCompletionHandler;
#[cfg(feature = "MapKit_MKLocalSearchCompleter")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleter;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterDelegate;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterResultType;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterResultTypeAddress;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterResultTypePointOfInterest;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompleterResultTypeQuery;
#[cfg(feature = "MapKit_MKLocalSearchCompletion")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKLocalSearchCompletion;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKSearchCompletionFilterType;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKSearchCompletionFilterTypeLocationsAndQueries;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchCompleter::MKSearchCompletionFilterTypeLocationsOnly;
#[cfg(feature = "MapKit_MKLocalSearchRequest")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchRequest::MKLocalSearchRequest;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchRequest::MKLocalSearchResultType;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchRequest::MKLocalSearchResultTypeAddress;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchRequest::MKLocalSearchResultTypePointOfInterest;
#[cfg(feature = "MapKit_MKLocalSearchResponse")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKLocalSearchResponse::MKLocalSearchResponse;
#[cfg(feature = "MapKit_MKLookAroundScene")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundScene::MKLookAroundScene;
#[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundSceneRequest::MKLookAroundSceneRequest;
#[cfg(feature = "MapKit_MKLookAroundSnapshot")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundSnapshot::MKLookAroundSnapshot;
#[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundSnapshotOptions::MKLookAroundSnapshotOptions;
#[cfg(feature = "MapKit_MKLookAroundSnapshotter")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundSnapshotter::MKLookAroundSnapshotter;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundBadgePosition;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundBadgePositionBottomTrailing;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundBadgePositionTopLeading;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundBadgePositionTopTrailing;
#[cfg(feature = "MapKit_MKLookAroundViewController")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundViewController;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKLookAroundViewController::MKLookAroundViewControllerDelegate;
#[cfg(feature = "MapKit_MKMapCamera")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapCamera::MKMapCamera;
#[cfg(feature = "MapKit_MKMapCameraBoundary")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapCameraBoundary::MKMapCameraBoundary;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapCameraZoomRange::MKMapCameraZoomDefault;
#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapCameraZoomRange::MKMapCameraZoomRange;

pub use self::__MKMapConfiguration::MKMapElevationStyle;

pub use self::__MKMapConfiguration::MKMapElevationStyleFlat;

#[cfg(feature = "MapKit_MKMapConfiguration")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapConfiguration::MKMapConfiguration;
pub use self::__MKMapConfiguration::MKMapElevationStyleRealistic;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsCameraKey;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeDefault;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeDriving;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeKey;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeTransit;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsDirectionsModeWalking;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsMapCenterKey;
#[cfg(not(any(target_os = "tvos")))]
pub use self::__MKMapItem::MKLaunchOptionsMapSpanKey;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKMapItem::MKLaunchOptionsMapTypeKey;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKMapItem::MKLaunchOptionsShowsTrafficKey;
#[cfg(feature = "MapKit_MKMapItem")]
pub use self::__MKMapItem::MKMapItem;

pub use self::__MKMapItem::MKMapItemTypeIdentifier;
#[cfg(feature = "MapKit_MKMapSnapshot")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapSnapshot::MKMapSnapshot;
#[cfg(feature = "MapKit_MKMapSnapshotOptions")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapSnapshotOptions::MKMapSnapshotOptions;

pub use self::__MKMapSnapshotter::MKMapSnapshotCompletionHandler;
#[cfg(feature = "MapKit_MKMapSnapshotter")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapSnapshotter::MKMapSnapshotter;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKUserTrackingMode;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKUserTrackingModeFollow;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKUserTrackingModeFollowWithHeading;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKUserTrackingModeNone;

pub use self::__MKMapView::MKMapViewDefaultAnnotationViewReuseIdentifier;

#[cfg(feature = "MapKit_MKMapView")]
pub use self::__MKMapView::MKMapView;
pub use self::__MKMapView::MKMapViewDefaultClusterAnnotationViewReuseIdentifier;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKMapViewDelegate;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKOverlayLevel;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKOverlayLevelAboveLabels;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMapView::MKOverlayLevelAboveRoads;
#[cfg(feature = "MapKit_MKMarkerAnnotationView")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMarkerAnnotationView::MKMarkerAnnotationView;
#[cfg(feature = "MapKit_MKMultiPoint")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMultiPoint::MKMultiPoint;
#[cfg(feature = "MapKit_MKMultiPolygon")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMultiPolygon::MKMultiPolygon;
#[cfg(feature = "MapKit_MKMultiPolygonRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMultiPolygonRenderer::MKMultiPolygonRenderer;
#[cfg(feature = "MapKit_MKMultiPolyline")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMultiPolyline::MKMultiPolyline;
#[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKMultiPolylineRenderer::MKMultiPolylineRenderer;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKOverlay::MKOverlay;
#[cfg(feature = "MapKit_MKOverlayPathRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKOverlayPathRenderer::MKOverlayPathRenderer;
#[cfg(feature = "MapKit_MKOverlayRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKOverlayRenderer::MKOverlayRenderer;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKOverlayRenderer::MKRoadWidthAtZoomScale;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKPinAnnotationView::MKPinAnnotationColor;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKPinAnnotationView::MKPinAnnotationColorGreen;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKPinAnnotationView::MKPinAnnotationColorPurple;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__MKPinAnnotationView::MKPinAnnotationColorRed;
#[cfg(feature = "MapKit_MKPinAnnotationView")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPinAnnotationView::MKPinAnnotationView;
#[cfg(feature = "MapKit_MKPitchControl")]
#[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
pub use self::__MKPitchControl::MKPitchControl;
#[cfg(feature = "MapKit_MKPlacemark")]
pub use self::__MKPlacemark::MKPlacemark;
#[cfg(feature = "MapKit_MKPointAnnotation")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointAnnotation::MKPointAnnotation;

pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategory;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryATM;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAirport;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAmusementPark;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryAquarium;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBakery;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBank;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBeach;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryBrewery;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCafe;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCampground;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryCarRental;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryEVCharger;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFireStation;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFitnessCenter;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryFoodMarket;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryGasStation;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryHospital;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryHotel;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryLaundry;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryLibrary;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMarina;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMovieTheater;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryMuseum;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryNationalPark;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryNightlife;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPark;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryParking;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPharmacy;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPolice;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPostOffice;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryPublicTransport;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryRestaurant;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryRestroom;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategorySchool;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryStadium;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryStore;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryTheater;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryUniversity;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryWinery;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestCategory::MKPointOfInterestCategoryZoo;
#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPointOfInterestFilter::MKPointOfInterestFilter;
#[cfg(feature = "MapKit_MKPolygon")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPolygon::MKPolygon;
#[cfg(feature = "MapKit_MKPolygonRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPolygonRenderer::MKPolygonRenderer;
#[cfg(feature = "MapKit_MKPolyline")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPolyline::MKPolyline;
#[cfg(feature = "MapKit_MKPolylineRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKPolylineRenderer::MKPolylineRenderer;
#[cfg(feature = "MapKit_MKShape")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKShape::MKShape;

pub use self::__MKStandardMapConfiguration::MKStandardMapEmphasisStyle;

pub use self::__MKStandardMapConfiguration::MKStandardMapEmphasisStyleDefault;

#[cfg(feature = "MapKit_MKStandardMapConfiguration")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKStandardMapConfiguration::MKStandardMapConfiguration;
pub use self::__MKStandardMapConfiguration::MKStandardMapEmphasisStyleMuted;
#[cfg(feature = "MapKit_MKTileOverlay")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTileOverlay::MKTileOverlay;

pub use self::__MKTileOverlay::MKTileOverlayPath;
#[cfg(feature = "MapKit_MKTileOverlayRenderer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTileOverlayRenderer::MKTileOverlayRenderer;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapType;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeHybrid;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeHybridFlyover;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeMutedStandard;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeSatellite;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeSatelliteFlyover;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKMapTypeStandard;

#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorCode;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorDecodingFailed;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorDirectionsNotFound;
pub use self::__MKTypes::MKErrorDomain;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorLoadingThrottled;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorPlacemarkNotFound;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorServerFailure;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKErrorUnknown;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKFeatureVisibility;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKFeatureVisibilityAdaptive;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKFeatureVisibilityHidden;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKTypes::MKFeatureVisibilityVisible;
#[cfg(feature = "MapKit_MKUserLocation")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKUserLocation::MKUserLocation;
#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__MKUserLocationView::MKUserLocationView;
#[cfg(feature = "MapKit_MKZoomControl")]
#[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
pub use self::__MKZoomControl::MKZoomControl;
