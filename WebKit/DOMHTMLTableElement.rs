//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLTableElement")]
    #[deprecated]
    pub struct DOMHTMLTableElement;

    #[cfg(feature = "WebKit_DOMHTMLTableElement")]
    unsafe impl ClassType for DOMHTMLTableElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLTableElement")]
unsafe impl DOMEventTarget for DOMHTMLTableElement {}

#[cfg(feature = "WebKit_DOMHTMLTableElement")]
unsafe impl NSObjectProtocol for DOMHTMLTableElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLTableElement")]
    unsafe impl DOMHTMLTableElement {
        #[cfg(feature = "WebKit_DOMHTMLTableCaptionElement")]
        #[method_id(@__retain_semantics Other caption)]
        pub unsafe fn caption(&self) -> Option<Id<DOMHTMLTableCaptionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableCaptionElement")]
        #[method(setCaption:)]
        pub unsafe fn setCaption(&self, caption: Option<&DOMHTMLTableCaptionElement>);

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[method_id(@__retain_semantics Other tHead)]
        pub unsafe fn tHead(&self) -> Option<Id<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[method(setTHead:)]
        pub unsafe fn setTHead(&self, t_head: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[method_id(@__retain_semantics Other tFoot)]
        pub unsafe fn tFoot(&self) -> Option<Id<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[method(setTFoot:)]
        pub unsafe fn setTFoot(&self, t_foot: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other rows)]
        pub unsafe fn rows(&self) -> Option<Id<DOMHTMLCollection>>;

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other tBodies)]
        pub unsafe fn tBodies(&self) -> Option<Id<DOMHTMLCollection>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other border)]
        pub unsafe fn border(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBorder:)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cellPadding)]
        pub unsafe fn cellPadding(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCellPadding:)]
        pub unsafe fn setCellPadding(&self, cell_padding: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cellSpacing)]
        pub unsafe fn cellSpacing(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCellSpacing:)]
        pub unsafe fn setCellSpacing(&self, cell_spacing: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other frameBorders)]
        pub unsafe fn frameBorders(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFrameBorders:)]
        pub unsafe fn setFrameBorders(&self, frame_borders: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rules)]
        pub unsafe fn rules(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRules:)]
        pub unsafe fn setRules(&self, rules: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other summary)]
        pub unsafe fn summary(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSummary:)]
        pub unsafe fn setSummary(&self, summary: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[method_id(@__retain_semantics Other createTHead)]
        pub unsafe fn createTHead(&self) -> Option<Id<DOMHTMLElement>>;

        #[method(deleteTHead)]
        pub unsafe fn deleteTHead(&self);

        #[method_id(@__retain_semantics Other createTFoot)]
        pub unsafe fn createTFoot(&self) -> Option<Id<DOMHTMLElement>>;

        #[method(deleteTFoot)]
        pub unsafe fn deleteTFoot(&self);

        #[method_id(@__retain_semantics Other createCaption)]
        pub unsafe fn createCaption(&self) -> Option<Id<DOMHTMLElement>>;

        #[method(deleteCaption)]
        pub unsafe fn deleteCaption(&self);

        #[method_id(@__retain_semantics Other insertRow:)]
        pub unsafe fn insertRow(&self, index: c_int) -> Option<Id<DOMHTMLElement>>;

        #[method(deleteRow:)]
        pub unsafe fn deleteRow(&self, index: c_int);
    }
);
