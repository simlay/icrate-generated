//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLInputElement")]
    #[deprecated]
    pub struct DOMHTMLInputElement;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLInputElement")]
    unsafe impl ClassType for DOMHTMLInputElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLInputElement")]
unsafe impl DOMEventTarget for DOMHTMLInputElement {}

#[cfg(feature = "WebKit_DOMHTMLInputElement")]
unsafe impl NSObjectProtocol for DOMHTMLInputElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLInputElement")]
    unsafe impl DOMHTMLInputElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accept)]
        pub unsafe fn accept(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccept:)]
        pub unsafe fn setAccept(&self, accept: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[method(autofocus)]
        pub unsafe fn autofocus(&self) -> bool;

        #[method(setAutofocus:)]
        pub unsafe fn setAutofocus(&self, autofocus: bool);

        #[method(defaultChecked)]
        pub unsafe fn defaultChecked(&self) -> bool;

        #[method(setDefaultChecked:)]
        pub unsafe fn setDefaultChecked(&self, default_checked: bool);

        #[method(checked)]
        pub unsafe fn checked(&self) -> bool;

        #[method(setChecked:)]
        pub unsafe fn setChecked(&self, checked: bool);

        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "WebKit_DOMHTMLFormElement")]
        #[method_id(@__retain_semantics Other form)]
        pub unsafe fn form(&self) -> Option<Id<DOMHTMLFormElement>>;

        #[cfg(feature = "WebKit_DOMFileList")]
        #[method_id(@__retain_semantics Other files)]
        pub unsafe fn files(&self) -> Option<Id<DOMFileList>>;

        #[cfg(feature = "WebKit_DOMFileList")]
        #[method(setFiles:)]
        pub unsafe fn setFiles(&self, files: Option<&DOMFileList>);

        #[method(indeterminate)]
        pub unsafe fn indeterminate(&self) -> bool;

        #[method(setIndeterminate:)]
        pub unsafe fn setIndeterminate(&self, indeterminate: bool);

        #[method(maxLength)]
        pub unsafe fn maxLength(&self) -> c_int;

        #[method(setMaxLength:)]
        pub unsafe fn setMaxLength(&self, max_length: c_int);

        #[method(multiple)]
        pub unsafe fn multiple(&self) -> bool;

        #[method(setMultiple:)]
        pub unsafe fn setMultiple(&self, multiple: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(readOnly)]
        pub unsafe fn readOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other src)]
        pub unsafe fn src(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSrc:)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultValue)]
        pub unsafe fn defaultValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDefaultValue:)]
        pub unsafe fn setDefaultValue(&self, default_value: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[method(willValidate)]
        pub unsafe fn willValidate(&self) -> bool;

        #[method(selectionStart)]
        pub unsafe fn selectionStart(&self) -> c_int;

        #[method(setSelectionStart:)]
        pub unsafe fn setSelectionStart(&self, selection_start: c_int);

        #[method(selectionEnd)]
        pub unsafe fn selectionEnd(&self) -> c_int;

        #[method(setSelectionEnd:)]
        pub unsafe fn setSelectionEnd(&self, selection_end: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other useMap)]
        pub unsafe fn useMap(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUseMap:)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessKey)]
        pub unsafe fn accessKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other altDisplayString)]
        pub unsafe fn altDisplayString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other absoluteImageURL)]
        pub unsafe fn absoluteImageURL(&self) -> Id<NSURL>;

        #[method(select)]
        pub unsafe fn select(&self);

        #[method(setSelectionRange:end:)]
        pub unsafe fn setSelectionRange_end(&self, start: c_int, end: c_int);

        #[method(click)]
        pub unsafe fn click(&self);
    }
);
