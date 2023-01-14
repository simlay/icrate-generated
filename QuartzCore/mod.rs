//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "CAAnimation.rs"]
mod __CAAnimation;
#[path = "CABase.rs"]
mod __CABase;
#[path = "CAConstraintLayoutManager.rs"]
mod __CAConstraintLayoutManager;
#[path = "CADisplayLink.rs"]
mod __CADisplayLink;
#[path = "CAEDRMetadata.rs"]
mod __CAEDRMetadata;
#[path = "CAEmitterCell.rs"]
mod __CAEmitterCell;
#[path = "CAEmitterLayer.rs"]
mod __CAEmitterLayer;
#[path = "CAFrameRateRange.rs"]
mod __CAFrameRateRange;
#[path = "CAGradientLayer.rs"]
mod __CAGradientLayer;
#[path = "CALayer.rs"]
mod __CALayer;
#[path = "CAMediaTiming.rs"]
mod __CAMediaTiming;
#[path = "CAMediaTimingFunction.rs"]
mod __CAMediaTimingFunction;
#[path = "CAMetalLayer.rs"]
mod __CAMetalLayer;
#[path = "CAOpenGLLayer.rs"]
mod __CAOpenGLLayer;
#[path = "CARemoteLayerClient.rs"]
mod __CARemoteLayerClient;
#[path = "CARemoteLayerServer.rs"]
mod __CARemoteLayerServer;
#[path = "CARenderer.rs"]
mod __CARenderer;
#[path = "CAReplicatorLayer.rs"]
mod __CAReplicatorLayer;
#[path = "CAScrollLayer.rs"]
mod __CAScrollLayer;
#[path = "CAShapeLayer.rs"]
mod __CAShapeLayer;
#[path = "CATextLayer.rs"]
mod __CATextLayer;
#[path = "CATiledLayer.rs"]
mod __CATiledLayer;
#[path = "CATransaction.rs"]
mod __CATransaction;
#[path = "CATransform3D.rs"]
mod __CATransform3D;
#[path = "CATransformLayer.rs"]
mod __CATransformLayer;
#[path = "CAValueFunction.rs"]
mod __CAValueFunction;

pub use self::__CAAnimation::kCAAnimationCubic;
pub use self::__CAAnimation::kCAAnimationCubicPaced;
pub use self::__CAAnimation::kCAAnimationDiscrete;
pub use self::__CAAnimation::kCAAnimationLinear;
pub use self::__CAAnimation::kCAAnimationPaced;
pub use self::__CAAnimation::kCAAnimationRotateAuto;
pub use self::__CAAnimation::kCAAnimationRotateAutoReverse;
pub use self::__CAAnimation::kCATransitionFade;
pub use self::__CAAnimation::kCATransitionFromBottom;
pub use self::__CAAnimation::kCATransitionFromLeft;
pub use self::__CAAnimation::kCATransitionFromRight;
pub use self::__CAAnimation::kCATransitionFromTop;
pub use self::__CAAnimation::kCATransitionMoveIn;
pub use self::__CAAnimation::kCATransitionPush;
pub use self::__CAAnimation::kCATransitionReveal;
#[cfg(feature = "CoreAnimation_CAAnimation")]
pub use self::__CAAnimation::CAAnimation;
pub use self::__CAAnimation::CAAnimationCalculationMode;
pub use self::__CAAnimation::CAAnimationDelegate;
#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
pub use self::__CAAnimation::CAAnimationGroup;
pub use self::__CAAnimation::CAAnimationRotationMode;
#[cfg(feature = "CoreAnimation_CABasicAnimation")]
pub use self::__CAAnimation::CABasicAnimation;
#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
pub use self::__CAAnimation::CAKeyframeAnimation;
#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
pub use self::__CAAnimation::CAPropertyAnimation;
#[cfg(feature = "CoreAnimation_CASpringAnimation")]
pub use self::__CAAnimation::CASpringAnimation;
#[cfg(feature = "CoreAnimation_CATransition")]
pub use self::__CAAnimation::CATransition;
pub use self::__CAAnimation::CATransitionSubtype;
pub use self::__CAAnimation::CATransitionType;
pub use self::__CABase::CACurrentMediaTime;
#[cfg(feature = "CoreAnimation_CAConstraint")]
pub use self::__CAConstraintLayoutManager::CAConstraint;
#[cfg(feature = "CoreAnimation_CAConstraintLayoutManager")]
pub use self::__CAConstraintLayoutManager::CAConstraintLayoutManager;
pub use self::__CAConstraintLayoutManager::{
    kCAConstraintHeight, kCAConstraintMaxX, kCAConstraintMaxY, kCAConstraintMidX,
    kCAConstraintMidY, kCAConstraintMinX, kCAConstraintMinY, kCAConstraintWidth,
    CAConstraintAttribute,
};
#[cfg(feature = "CoreAnimation_CADisplayLink")]
pub use self::__CADisplayLink::CADisplayLink;
#[cfg(feature = "CoreAnimation_CAEDRMetadata")]
pub use self::__CAEDRMetadata::CAEDRMetadata;
#[cfg(feature = "CoreAnimation_CAEmitterCell")]
pub use self::__CAEmitterCell::CAEmitterCell;
pub use self::__CAEmitterLayer::kCAEmitterLayerAdditive;
pub use self::__CAEmitterLayer::kCAEmitterLayerBackToFront;
pub use self::__CAEmitterLayer::kCAEmitterLayerCircle;
pub use self::__CAEmitterLayer::kCAEmitterLayerCuboid;
pub use self::__CAEmitterLayer::kCAEmitterLayerLine;
pub use self::__CAEmitterLayer::kCAEmitterLayerOldestFirst;
pub use self::__CAEmitterLayer::kCAEmitterLayerOldestLast;
pub use self::__CAEmitterLayer::kCAEmitterLayerOutline;
pub use self::__CAEmitterLayer::kCAEmitterLayerPoint;
pub use self::__CAEmitterLayer::kCAEmitterLayerPoints;
pub use self::__CAEmitterLayer::kCAEmitterLayerRectangle;
pub use self::__CAEmitterLayer::kCAEmitterLayerSphere;
pub use self::__CAEmitterLayer::kCAEmitterLayerSurface;
pub use self::__CAEmitterLayer::kCAEmitterLayerUnordered;
pub use self::__CAEmitterLayer::kCAEmitterLayerVolume;
#[cfg(feature = "CoreAnimation_CAEmitterLayer")]
pub use self::__CAEmitterLayer::CAEmitterLayer;
pub use self::__CAEmitterLayer::CAEmitterLayerEmitterMode;
pub use self::__CAEmitterLayer::CAEmitterLayerEmitterShape;
pub use self::__CAEmitterLayer::CAEmitterLayerRenderMode;
pub use self::__CAFrameRateRange::CAFrameRateRange;
pub use self::__CAFrameRateRange::CAFrameRateRangeDefault;
pub use self::__CAFrameRateRange::CAFrameRateRangeIsEqualToRange;
pub use self::__CAFrameRateRange::CAFrameRateRangeMake;
pub use self::__CAGradientLayer::kCAGradientLayerAxial;
pub use self::__CAGradientLayer::kCAGradientLayerConic;
pub use self::__CAGradientLayer::kCAGradientLayerRadial;
#[cfg(feature = "CoreAnimation_CAGradientLayer")]
pub use self::__CAGradientLayer::CAGradientLayer;
pub use self::__CAGradientLayer::CAGradientLayerType;
pub use self::__CALayer::kCAContentsFormatGray8Uint;
pub use self::__CALayer::kCAContentsFormatRGBA16Float;
pub use self::__CALayer::kCAContentsFormatRGBA8Uint;
pub use self::__CALayer::kCACornerCurveCircular;
pub use self::__CALayer::kCACornerCurveContinuous;
pub use self::__CALayer::kCAFilterLinear;
pub use self::__CALayer::kCAFilterNearest;
pub use self::__CALayer::kCAFilterTrilinear;
pub use self::__CALayer::kCAGravityBottom;
pub use self::__CALayer::kCAGravityBottomLeft;
pub use self::__CALayer::kCAGravityBottomRight;
pub use self::__CALayer::kCAGravityCenter;
pub use self::__CALayer::kCAGravityLeft;
pub use self::__CALayer::kCAGravityResize;
pub use self::__CALayer::kCAGravityResizeAspect;
pub use self::__CALayer::kCAGravityResizeAspectFill;
pub use self::__CALayer::kCAGravityRight;
pub use self::__CALayer::kCAGravityTop;
pub use self::__CALayer::kCAGravityTopLeft;
pub use self::__CALayer::kCAGravityTopRight;
pub use self::__CALayer::kCAOnOrderIn;
pub use self::__CALayer::kCAOnOrderOut;
pub use self::__CALayer::kCATransition;
pub use self::__CALayer::CAAction;
#[cfg(feature = "CoreAnimation_CALayer")]
pub use self::__CALayer::CALayer;
pub use self::__CALayer::CALayerContentsFilter;
pub use self::__CALayer::CALayerContentsFormat;
pub use self::__CALayer::CALayerContentsGravity;
pub use self::__CALayer::CALayerCornerCurve;
pub use self::__CALayer::CALayerDelegate;
pub use self::__CALayer::CALayoutManager;
pub use self::__CALayer::{
    kCALayerBottomEdge, kCALayerLeftEdge, kCALayerRightEdge, kCALayerTopEdge,
    CAEdgeAntialiasingMask,
};
pub use self::__CALayer::{
    kCALayerHeightSizable, kCALayerMaxXMargin, kCALayerMaxYMargin, kCALayerMinXMargin,
    kCALayerMinYMargin, kCALayerNotSizable, kCALayerWidthSizable, CAAutoresizingMask,
};
pub use self::__CALayer::{
    kCALayerMaxXMaxYCorner, kCALayerMaxXMinYCorner, kCALayerMinXMaxYCorner, kCALayerMinXMinYCorner,
    CACornerMask,
};
pub use self::__CAMediaTiming::kCAFillModeBackwards;
pub use self::__CAMediaTiming::kCAFillModeBoth;
pub use self::__CAMediaTiming::kCAFillModeForwards;
pub use self::__CAMediaTiming::kCAFillModeRemoved;
pub use self::__CAMediaTiming::CAMediaTiming;
pub use self::__CAMediaTiming::CAMediaTimingFillMode;
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionDefault;
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseIn;
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseInEaseOut;
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseOut;
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionLinear;
#[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::CAMediaTimingFunction;
pub use self::__CAMediaTimingFunction::CAMediaTimingFunctionName;
#[cfg(feature = "CoreAnimation_CARemoteLayerClient")]
pub use self::__CARemoteLayerClient::CARemoteLayerClient;
#[cfg(feature = "CoreAnimation_CARemoteLayerServer")]
pub use self::__CARemoteLayerServer::CARemoteLayerServer;
pub use self::__CARenderer::kCARendererColorSpace;
pub use self::__CARenderer::kCARendererMetalCommandQueue;
#[cfg(feature = "CoreAnimation_CARenderer")]
pub use self::__CARenderer::CARenderer;
#[cfg(feature = "CoreAnimation_CAReplicatorLayer")]
pub use self::__CAReplicatorLayer::CAReplicatorLayer;
pub use self::__CAScrollLayer::kCAScrollBoth;
pub use self::__CAScrollLayer::kCAScrollHorizontally;
pub use self::__CAScrollLayer::kCAScrollNone;
pub use self::__CAScrollLayer::kCAScrollVertically;
#[cfg(feature = "CoreAnimation_CAScrollLayer")]
pub use self::__CAScrollLayer::CAScrollLayer;
pub use self::__CAScrollLayer::CAScrollLayerScrollMode;
pub use self::__CAShapeLayer::kCAFillRuleEvenOdd;
pub use self::__CAShapeLayer::kCAFillRuleNonZero;
pub use self::__CAShapeLayer::kCALineCapButt;
pub use self::__CAShapeLayer::kCALineCapRound;
pub use self::__CAShapeLayer::kCALineCapSquare;
pub use self::__CAShapeLayer::kCALineJoinBevel;
pub use self::__CAShapeLayer::kCALineJoinMiter;
pub use self::__CAShapeLayer::kCALineJoinRound;
#[cfg(feature = "CoreAnimation_CAShapeLayer")]
pub use self::__CAShapeLayer::CAShapeLayer;
pub use self::__CAShapeLayer::CAShapeLayerFillRule;
pub use self::__CAShapeLayer::CAShapeLayerLineCap;
pub use self::__CAShapeLayer::CAShapeLayerLineJoin;
pub use self::__CATextLayer::kCAAlignmentCenter;
pub use self::__CATextLayer::kCAAlignmentJustified;
pub use self::__CATextLayer::kCAAlignmentLeft;
pub use self::__CATextLayer::kCAAlignmentNatural;
pub use self::__CATextLayer::kCAAlignmentRight;
pub use self::__CATextLayer::kCATruncationEnd;
pub use self::__CATextLayer::kCATruncationMiddle;
pub use self::__CATextLayer::kCATruncationNone;
pub use self::__CATextLayer::kCATruncationStart;
#[cfg(feature = "CoreAnimation_CATextLayer")]
pub use self::__CATextLayer::CATextLayer;
pub use self::__CATextLayer::CATextLayerAlignmentMode;
pub use self::__CATextLayer::CATextLayerTruncationMode;
#[cfg(feature = "CoreAnimation_CATiledLayer")]
pub use self::__CATiledLayer::CATiledLayer;
pub use self::__CATransaction::kCATransactionAnimationDuration;
pub use self::__CATransaction::kCATransactionAnimationTimingFunction;
pub use self::__CATransaction::kCATransactionCompletionBlock;
pub use self::__CATransaction::kCATransactionDisableActions;
#[cfg(feature = "CoreAnimation_CATransaction")]
pub use self::__CATransaction::CATransaction;
pub use self::__CATransform3D::CATransform3D;
pub use self::__CATransform3D::CATransform3DConcat;
pub use self::__CATransform3D::CATransform3DEqualToTransform;
pub use self::__CATransform3D::CATransform3DIdentity;
pub use self::__CATransform3D::CATransform3DInvert;
pub use self::__CATransform3D::CATransform3DIsAffine;
pub use self::__CATransform3D::CATransform3DIsIdentity;
pub use self::__CATransform3D::CATransform3DMakeRotation;
pub use self::__CATransform3D::CATransform3DMakeScale;
pub use self::__CATransform3D::CATransform3DMakeTranslation;
pub use self::__CATransform3D::CATransform3DRotate;
pub use self::__CATransform3D::CATransform3DScale;
pub use self::__CATransform3D::CATransform3DTranslate;
#[cfg(feature = "CoreAnimation_CATransformLayer")]
pub use self::__CATransformLayer::CATransformLayer;
pub use self::__CAValueFunction::kCAValueFunctionRotateX;
pub use self::__CAValueFunction::kCAValueFunctionRotateY;
pub use self::__CAValueFunction::kCAValueFunctionRotateZ;
pub use self::__CAValueFunction::kCAValueFunctionScale;
pub use self::__CAValueFunction::kCAValueFunctionScaleX;
pub use self::__CAValueFunction::kCAValueFunctionScaleY;
pub use self::__CAValueFunction::kCAValueFunctionScaleZ;
pub use self::__CAValueFunction::kCAValueFunctionTranslate;
pub use self::__CAValueFunction::kCAValueFunctionTranslateX;
pub use self::__CAValueFunction::kCAValueFunctionTranslateY;
pub use self::__CAValueFunction::kCAValueFunctionTranslateZ;
#[cfg(feature = "CoreAnimation_CAValueFunction")]
pub use self::__CAValueFunction::CAValueFunction;
pub use self::__CAValueFunction::CAValueFunctionName;
