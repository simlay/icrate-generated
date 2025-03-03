//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CALayerContentsGravity = NSString;
);

typed_enum!(
    pub type CALayerContentsFormat = NSString;
);

typed_enum!(
    pub type CALayerContentsFilter = NSString;
);

typed_enum!(
    pub type CALayerCornerCurve = NSString;
);

ns_options!(
    #[underlying(c_uint)]
    pub enum CAAutoresizingMask {
        kCALayerNotSizable = 0,
        kCALayerMinXMargin = 1 << 0,
        kCALayerWidthSizable = 1 << 1,
        kCALayerMaxXMargin = 1 << 2,
        kCALayerMinYMargin = 1 << 3,
        kCALayerHeightSizable = 1 << 4,
        kCALayerMaxYMargin = 1 << 5,
    }
);

ns_options!(
    #[underlying(c_uint)]
    pub enum CAEdgeAntialiasingMask {
        kCALayerLeftEdge = 1 << 0,
        kCALayerRightEdge = 1 << 1,
        kCALayerBottomEdge = 1 << 2,
        kCALayerTopEdge = 1 << 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CACornerMask {
        kCALayerMinXMinYCorner = 1 << 0,
        kCALayerMaxXMinYCorner = 1 << 1,
        kCALayerMinXMaxYCorner = 1 << 2,
        kCALayerMaxXMaxYCorner = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CALayer")]
    pub struct CALayer;

    #[cfg(feature = "CoreAnimation_CALayer")]
    unsafe impl ClassType for CALayer {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreAnimation_CALayer")]
unsafe impl CAMediaTiming for CALayer {}

#[cfg(feature = "CoreAnimation_CALayer")]
unsafe impl NSCoding for CALayer {}

#[cfg(feature = "CoreAnimation_CALayer")]
unsafe impl NSObjectProtocol for CALayer {}

#[cfg(feature = "CoreAnimation_CALayer")]
unsafe impl NSSecureCoding for CALayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CALayer")]
    unsafe impl CALayer {
        #[method_id(@__retain_semantics Other layer)]
        pub fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &Object) -> Id<Self>;

        #[method_id(@__retain_semantics Other presentationLayer)]
        pub unsafe fn presentationLayer(&self) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other modelLayer)]
        pub unsafe fn modelLayer(&self) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(needsDisplayForKey:)]
        pub unsafe fn needsDisplayForKey(key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[method(bounds)]
        pub fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub fn setBounds(&self, bounds: CGRect);

        #[method(position)]
        pub fn position(&self) -> CGPoint;

        #[method(setPosition:)]
        pub fn setPosition(&self, position: CGPoint);

        #[method(zPosition)]
        pub fn zPosition(&self) -> CGFloat;

        #[method(setZPosition:)]
        pub fn setZPosition(&self, z_position: CGFloat);

        #[method(anchorPoint)]
        pub fn anchorPoint(&self) -> CGPoint;

        #[method(setAnchorPoint:)]
        pub fn setAnchorPoint(&self, anchor_point: CGPoint);

        #[method(anchorPointZ)]
        pub fn anchorPointZ(&self) -> CGFloat;

        #[method(setAnchorPointZ:)]
        pub fn setAnchorPointZ(&self, anchor_point_z: CGFloat);

        #[method(transform)]
        pub fn transform(&self) -> CATransform3D;

        #[method(setTransform:)]
        pub fn setTransform(&self, transform: CATransform3D);

        #[method(frame)]
        pub fn frame(&self) -> CGRect;

        #[method(setFrame:)]
        pub fn setFrame(&self, frame: CGRect);

        #[method(isHidden)]
        pub fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub fn setHidden(&self, hidden: bool);

        #[method(isDoubleSided)]
        pub fn isDoubleSided(&self) -> bool;

        #[method(setDoubleSided:)]
        pub fn setDoubleSided(&self, double_sided: bool);

        #[method(isGeometryFlipped)]
        pub fn isGeometryFlipped(&self) -> bool;

        #[method(setGeometryFlipped:)]
        pub fn setGeometryFlipped(&self, geometry_flipped: bool);

        #[method(contentsAreFlipped)]
        pub fn contentsAreFlipped(&self) -> bool;

        #[method_id(@__retain_semantics Other superlayer)]
        pub fn superlayer(&self) -> Option<Id<CALayer>>;

        #[method(removeFromSuperlayer)]
        pub fn removeFromSuperlayer(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sublayers)]
        pub unsafe fn sublayers(&self) -> Option<Id<NSArray<CALayer>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSublayers:)]
        pub unsafe fn setSublayers(&self, sublayers: Option<&NSArray<CALayer>>);

        #[method(addSublayer:)]
        pub fn addSublayer(&self, layer: &CALayer);

        #[method(insertSublayer:atIndex:)]
        pub fn insertSublayer_atIndex(&self, layer: &CALayer, idx: c_uint);

        #[method(insertSublayer:below:)]
        pub fn insertSublayer_below(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(insertSublayer:above:)]
        pub fn insertSublayer_above(&self, layer: &CALayer, sibling: Option<&CALayer>);

        #[method(replaceSublayer:with:)]
        pub unsafe fn replaceSublayer_with(&self, old_layer: &CALayer, new_layer: &CALayer);

        #[method(sublayerTransform)]
        pub fn sublayerTransform(&self) -> CATransform3D;

        #[method(setSublayerTransform:)]
        pub fn setSublayerTransform(&self, sublayer_transform: CATransform3D);

        #[method_id(@__retain_semantics Other mask)]
        pub fn mask(&self) -> Option<Id<CALayer>>;

        #[method(setMask:)]
        pub unsafe fn setMask(&self, mask: Option<&CALayer>);

        #[method(masksToBounds)]
        pub fn masksToBounds(&self) -> bool;

        #[method(setMasksToBounds:)]
        pub fn setMasksToBounds(&self, masks_to_bounds: bool);

        #[method(convertPoint:fromLayer:)]
        pub fn convertPoint_fromLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertPoint:toLayer:)]
        pub fn convertPoint_toLayer(&self, p: CGPoint, l: Option<&CALayer>) -> CGPoint;

        #[method(convertRect:fromLayer:)]
        pub fn convertRect_fromLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertRect:toLayer:)]
        pub fn convertRect_toLayer(&self, r: CGRect, l: Option<&CALayer>) -> CGRect;

        #[method(convertTime:fromLayer:)]
        pub fn convertTime_fromLayer(
            &self,
            t: CFTimeInterval,
            l: Option<&CALayer>,
        ) -> CFTimeInterval;

        #[method(convertTime:toLayer:)]
        pub fn convertTime_toLayer(&self, t: CFTimeInterval, l: Option<&CALayer>)
            -> CFTimeInterval;

        #[method_id(@__retain_semantics Other hitTest:)]
        pub fn hitTest(&self, p: CGPoint) -> Option<Id<CALayer>>;

        #[method(containsPoint:)]
        pub fn containsPoint(&self, p: CGPoint) -> bool;

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<Object>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&Object>);

        #[method(contentsRect)]
        pub fn contentsRect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub fn setContentsRect(&self, contents_rect: CGRect);

        #[method_id(@__retain_semantics Other contentsGravity)]
        pub fn contentsGravity(&self) -> Id<CALayerContentsGravity>;

        #[method(setContentsGravity:)]
        pub fn setContentsGravity(&self, contents_gravity: &CALayerContentsGravity);

        #[method(contentsScale)]
        pub fn contentsScale(&self) -> CGFloat;

        #[method(setContentsScale:)]
        pub fn setContentsScale(&self, contents_scale: CGFloat);

        #[method(contentsCenter)]
        pub fn contentsCenter(&self) -> CGRect;

        #[method(setContentsCenter:)]
        pub fn setContentsCenter(&self, contents_center: CGRect);

        #[method_id(@__retain_semantics Other contentsFormat)]
        pub fn contentsFormat(&self) -> Id<CALayerContentsFormat>;

        #[method(setContentsFormat:)]
        pub fn setContentsFormat(&self, contents_format: &CALayerContentsFormat);

        #[method_id(@__retain_semantics Other minificationFilter)]
        pub fn minificationFilter(&self) -> Id<CALayerContentsFilter>;

        #[method(setMinificationFilter:)]
        pub fn setMinificationFilter(&self, minification_filter: &CALayerContentsFilter);

        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub fn magnificationFilter(&self) -> Id<CALayerContentsFilter>;

        #[method(setMagnificationFilter:)]
        pub fn setMagnificationFilter(&self, magnification_filter: &CALayerContentsFilter);

        #[method(minificationFilterBias)]
        pub fn minificationFilterBias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub fn setMinificationFilterBias(&self, minification_filter_bias: c_float);

        #[method(isOpaque)]
        pub fn isOpaque(&self) -> bool;

        #[method(setOpaque:)]
        pub fn setOpaque(&self, opaque: bool);

        #[method(display)]
        pub fn display(&self);

        #[method(setNeedsDisplay)]
        pub fn setNeedsDisplay(&self);

        #[method(setNeedsDisplayInRect:)]
        pub fn setNeedsDisplayInRect(&self, r: CGRect);

        #[method(needsDisplay)]
        pub fn needsDisplay(&self) -> bool;

        #[method(displayIfNeeded)]
        pub fn displayIfNeeded(&self);

        #[method(needsDisplayOnBoundsChange)]
        pub fn needsDisplayOnBoundsChange(&self) -> bool;

        #[method(setNeedsDisplayOnBoundsChange:)]
        pub fn setNeedsDisplayOnBoundsChange(&self, needs_display_on_bounds_change: bool);

        #[method(drawsAsynchronously)]
        pub fn drawsAsynchronously(&self) -> bool;

        #[method(setDrawsAsynchronously:)]
        pub fn setDrawsAsynchronously(&self, draws_asynchronously: bool);

        #[method(edgeAntialiasingMask)]
        pub fn edgeAntialiasingMask(&self) -> CAEdgeAntialiasingMask;

        #[method(setEdgeAntialiasingMask:)]
        pub fn setEdgeAntialiasingMask(&self, edge_antialiasing_mask: CAEdgeAntialiasingMask);

        #[method(allowsEdgeAntialiasing)]
        pub fn allowsEdgeAntialiasing(&self) -> bool;

        #[method(setAllowsEdgeAntialiasing:)]
        pub fn setAllowsEdgeAntialiasing(&self, allows_edge_antialiasing: bool);

        #[method(cornerRadius)]
        pub fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub fn setCornerRadius(&self, corner_radius: CGFloat);

        #[method(maskedCorners)]
        pub fn maskedCorners(&self) -> CACornerMask;

        #[method(setMaskedCorners:)]
        pub fn setMaskedCorners(&self, masked_corners: CACornerMask);

        #[method_id(@__retain_semantics Other cornerCurve)]
        pub fn cornerCurve(&self) -> Id<CALayerCornerCurve>;

        #[method(setCornerCurve:)]
        pub fn setCornerCurve(&self, corner_curve: &CALayerCornerCurve);

        #[method(cornerCurveExpansionFactor:)]
        pub fn cornerCurveExpansionFactor(curve: &CALayerCornerCurve) -> CGFloat;

        #[method(borderWidth)]
        pub fn borderWidth(&self) -> CGFloat;

        #[method(setBorderWidth:)]
        pub fn setBorderWidth(&self, border_width: CGFloat);

        #[method(opacity)]
        pub fn opacity(&self) -> c_float;

        #[method(setOpacity:)]
        pub fn setOpacity(&self, opacity: c_float);

        #[method(allowsGroupOpacity)]
        pub fn allowsGroupOpacity(&self) -> bool;

        #[method(setAllowsGroupOpacity:)]
        pub fn setAllowsGroupOpacity(&self, allows_group_opacity: bool);

        #[method_id(@__retain_semantics Other compositingFilter)]
        pub unsafe fn compositingFilter(&self) -> Option<Id<Object>>;

        #[method(setCompositingFilter:)]
        pub unsafe fn setCompositingFilter(&self, compositing_filter: Option<&Object>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other filters)]
        pub unsafe fn filters(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFilters:)]
        pub unsafe fn setFilters(&self, filters: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other backgroundFilters)]
        pub unsafe fn backgroundFilters(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBackgroundFilters:)]
        pub unsafe fn setBackgroundFilters(&self, background_filters: Option<&NSArray>);

        #[method(shouldRasterize)]
        pub fn shouldRasterize(&self) -> bool;

        #[method(setShouldRasterize:)]
        pub fn setShouldRasterize(&self, should_rasterize: bool);

        #[method(rasterizationScale)]
        pub fn rasterizationScale(&self) -> CGFloat;

        #[method(setRasterizationScale:)]
        pub fn setRasterizationScale(&self, rasterization_scale: CGFloat);

        #[method(shadowOpacity)]
        pub fn shadowOpacity(&self) -> c_float;

        #[method(setShadowOpacity:)]
        pub fn setShadowOpacity(&self, shadow_opacity: c_float);

        #[method(shadowOffset)]
        pub fn shadowOffset(&self) -> CGSize;

        #[method(setShadowOffset:)]
        pub fn setShadowOffset(&self, shadow_offset: CGSize);

        #[method(shadowRadius)]
        pub fn shadowRadius(&self) -> CGFloat;

        #[method(setShadowRadius:)]
        pub fn setShadowRadius(&self, shadow_radius: CGFloat);

        #[method(autoresizingMask)]
        pub fn autoresizingMask(&self) -> CAAutoresizingMask;

        #[method(setAutoresizingMask:)]
        pub fn setAutoresizingMask(&self, autoresizing_mask: CAAutoresizingMask);

        #[method_id(@__retain_semantics Other layoutManager)]
        pub fn layoutManager(&self) -> Option<Id<ProtocolObject<dyn CALayoutManager>>>;

        #[method(setLayoutManager:)]
        pub fn setLayoutManager(
            &self,
            layout_manager: Option<&ProtocolObject<dyn CALayoutManager>>,
        );

        #[method(preferredFrameSize)]
        pub fn preferredFrameSize(&self) -> CGSize;

        #[method(setNeedsLayout)]
        pub fn setNeedsLayout(&self);

        #[method(needsLayout)]
        pub fn needsLayout(&self) -> bool;

        #[method(layoutIfNeeded)]
        pub fn layoutIfNeeded(&self);

        #[method(layoutSublayers)]
        pub fn layoutSublayers(&self);

        #[method(resizeSublayersWithOldSize:)]
        pub fn resizeSublayersWithOldSize(&self, size: CGSize);

        #[method(resizeWithOldSuperlayerSize:)]
        pub fn resizeWithOldSuperlayerSize(&self, size: CGSize);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultActionForKey:)]
        pub fn defaultActionForKey(event: &NSString) -> Option<Id<ProtocolObject<dyn CAAction>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionForKey:)]
        pub fn actionForKey(&self, event: &NSString) -> Option<Id<ProtocolObject<dyn CAAction>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other actions)]
        pub fn actions(&self) -> Option<Id<NSDictionary<NSString, ProtocolObject<dyn CAAction>>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setActions:)]
        pub fn setActions(
            &self,
            actions: Option<&NSDictionary<NSString, ProtocolObject<dyn CAAction>>>,
        );

        #[cfg(all(feature = "CoreAnimation_CAAnimation", feature = "Foundation_NSString"))]
        #[method(addAnimation:forKey:)]
        pub fn addAnimation_forKey(&self, anim: &CAAnimation, key: Option<&NSString>);

        #[method(removeAllAnimations)]
        pub fn removeAllAnimations(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeAnimationForKey:)]
        pub fn removeAnimationForKey(&self, key: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other animationKeys)]
        pub fn animationKeys(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "CoreAnimation_CAAnimation", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other animationForKey:)]
        pub unsafe fn animationForKey(&self, key: &NSString) -> Option<Id<CAAnimation>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other delegate)]
        pub fn delegate(&self) -> Option<Id<ProtocolObject<dyn CALayerDelegate>>>;

        #[method(setDelegate:)]
        pub fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn CALayerDelegate>>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);

extern_protocol!(
    pub unsafe trait CALayoutManager: NSObjectProtocol {
        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(preferredSizeOfLayer:)]
        unsafe fn preferredSizeOfLayer(&self, layer: &CALayer) -> CGSize;

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(invalidateLayoutOfLayer:)]
        unsafe fn invalidateLayoutOfLayer(&self, layer: &CALayer);

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);
    }

    unsafe impl ProtocolType for dyn CALayoutManager {}
);

extern_protocol!(
    pub unsafe trait CAAction {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(runActionForKey:object:arguments:)]
        unsafe fn runActionForKey_object_arguments(
            &self,
            event: &NSString,
            an_object: &Object,
            dict: Option<&NSDictionary>,
        );
    }

    unsafe impl ProtocolType for dyn CAAction {}
);

extern_methods!(
    /// CAActionAdditions
    #[cfg(feature = "Foundation_NSNull")]
    unsafe impl NSNull {}
);

#[cfg(feature = "Foundation_NSNull")]
unsafe impl CAAction for NSNull {}

extern_protocol!(
    pub unsafe trait CALayerDelegate: NSObjectProtocol {
        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(displayLayer:)]
        unsafe fn displayLayer(&self, layer: &CALayer);

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(layerWillDraw:)]
        unsafe fn layerWillDraw(&self, layer: &CALayer);

        #[cfg(feature = "CoreAnimation_CALayer")]
        #[optional]
        #[method(layoutSublayersOfLayer:)]
        unsafe fn layoutSublayersOfLayer(&self, layer: &CALayer);

        #[cfg(all(feature = "CoreAnimation_CALayer", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other actionForLayer:forKey:)]
        unsafe fn actionForLayer_forKey(
            &self,
            layer: &CALayer,
            event: &NSString,
        ) -> Option<Id<ProtocolObject<dyn CAAction>>>;
    }

    unsafe impl ProtocolType for dyn CALayerDelegate {}
);

extern_static!(kCAGravityCenter: &'static CALayerContentsGravity);

extern_static!(kCAGravityTop: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottom: &'static CALayerContentsGravity);

extern_static!(kCAGravityLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityTopLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityTopRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottomLeft: &'static CALayerContentsGravity);

extern_static!(kCAGravityBottomRight: &'static CALayerContentsGravity);

extern_static!(kCAGravityResize: &'static CALayerContentsGravity);

extern_static!(kCAGravityResizeAspect: &'static CALayerContentsGravity);

extern_static!(kCAGravityResizeAspectFill: &'static CALayerContentsGravity);

extern_static!(kCAContentsFormatRGBA8Uint: &'static CALayerContentsFormat);

extern_static!(kCAContentsFormatRGBA16Float: &'static CALayerContentsFormat);

extern_static!(kCAContentsFormatGray8Uint: &'static CALayerContentsFormat);

extern_static!(kCAFilterNearest: &'static CALayerContentsFilter);

extern_static!(kCAFilterLinear: &'static CALayerContentsFilter);

extern_static!(kCAFilterTrilinear: &'static CALayerContentsFilter);

extern_static!(kCACornerCurveCircular: &'static CALayerCornerCurve);

extern_static!(kCACornerCurveContinuous: &'static CALayerCornerCurve);

extern_static!(kCAOnOrderIn: &'static NSString);

extern_static!(kCAOnOrderOut: &'static NSString);

extern_static!(kCATransition: &'static NSString);
