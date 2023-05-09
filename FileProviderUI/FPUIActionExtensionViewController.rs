//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::FileProviderUI::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    pub struct FPUIActionExtensionViewController;

    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    unsafe impl ClassType for FPUIActionExtensionViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
unsafe impl NSCoding for FPUIActionExtensionViewController {}

#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
unsafe impl NSEditor for FPUIActionExtensionViewController {}

#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
unsafe impl NSObjectProtocol for FPUIActionExtensionViewController {}

#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
unsafe impl NSSeguePerforming for FPUIActionExtensionViewController {}

#[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
unsafe impl NSUserInterfaceItemIdentification for FPUIActionExtensionViewController {}

extern_methods!(
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    unsafe impl FPUIActionExtensionViewController {
        #[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Id<FPUIActionExtensionContext>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(prepareForError:)]
        pub unsafe fn prepareForError(&self, error: &NSError);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(prepareForActionWithIdentifier:itemIdentifiers:)]
        pub unsafe fn prepareForActionWithIdentifier_itemIdentifiers(
            &self,
            action_identifier: &NSString,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
        );
    }
);

#[cfg(not(any(target_os = "ios")))]
extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    unsafe impl FPUIActionExtensionViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

#[cfg(not(any(target_os = "ios")))]
extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    unsafe impl FPUIActionExtensionViewController {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionViewController")]
    unsafe impl FPUIActionExtensionViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
