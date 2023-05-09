//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSBundleExecutableArchitectureI386 = 0x00000007,
        NSBundleExecutableArchitecturePPC = 0x00000012,
        NSBundleExecutableArchitectureX86_64 = 0x01000007,
        NSBundleExecutableArchitecturePPC64 = 0x01000012,
        NSBundleExecutableArchitectureARM64 = 0x0100000c,
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSBundle")]
    pub struct NSBundle;

    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl ClassType for NSBundle {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSBundle")]
unsafe impl NSObjectProtocol for NSBundle {}

extern_methods!(
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[method_id(@__retain_semantics Other mainBundle)]
        pub fn mainBundle() -> Id<NSBundle>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundleWithPath:)]
        pub unsafe fn bundleWithPath(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other bundleWithURL:)]
        pub unsafe fn bundleWithURL(url: &NSURL) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other bundleForClass:)]
        pub unsafe fn bundleForClass(a_class: &Class) -> Id<NSBundle>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundleWithIdentifier:)]
        pub unsafe fn bundleWithIdentifier(identifier: &NSString) -> Option<Id<NSBundle>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allBundles)]
        pub unsafe fn allBundles() -> Id<NSArray<NSBundle>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allFrameworks)]
        pub unsafe fn allFrameworks() -> Id<NSArray<NSBundle>>;

        #[method(load)]
        pub unsafe fn load(&self) -> bool;

        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[method(unload)]
        pub unsafe fn unload(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method(preflightAndReturnError:_)]
        pub unsafe fn preflightAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(loadAndReturnError:_)]
        pub unsafe fn loadAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other bundleURL)]
        pub unsafe fn bundleURL(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other resourceURL)]
        pub unsafe fn resourceURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForAuxiliaryExecutable:)]
        pub unsafe fn URLForAuxiliaryExecutable(
            &self,
            executable_name: &NSString,
        ) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other privateFrameworksURL)]
        pub unsafe fn privateFrameworksURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other sharedFrameworksURL)]
        pub unsafe fn sharedFrameworksURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other sharedSupportURL)]
        pub unsafe fn sharedSupportURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other builtInPlugInsURL)]
        pub unsafe fn builtInPlugInsURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other appStoreReceiptURL)]
        pub unsafe fn appStoreReceiptURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundlePath)]
        pub unsafe fn bundlePath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other resourcePath)]
        pub unsafe fn resourcePath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other executablePath)]
        pub unsafe fn executablePath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForAuxiliaryExecutable:)]
        pub unsafe fn pathForAuxiliaryExecutable(
            &self,
            executable_name: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other privateFrameworksPath)]
        pub unsafe fn privateFrameworksPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sharedFrameworksPath)]
        pub unsafe fn sharedFrameworksPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sharedSupportPath)]
        pub unsafe fn sharedSupportPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other builtInPlugInsPath)]
        pub unsafe fn builtInPlugInsPath(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:inBundleWithURL:)]
        pub unsafe fn URLForResource_withExtension_subdirectory_inBundleWithURL(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundle_url: &NSURL,
        ) -> Option<Id<NSURL>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:inBundleWithURL:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_inBundleWithURL(
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            bundle_url: &NSURL,
        ) -> Option<Id<NSArray<NSURL>>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:)]
        pub unsafe fn URLForResource_withExtension(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Id<NSURL>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:)]
        pub unsafe fn URLForResource_withExtension_subdirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSURL>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForResource:withExtension:subdirectory:localization:)]
        pub unsafe fn URLForResource_withExtension_subdirectory_localization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Id<NSURL>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSArray<NSURL>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other URLsForResourcesWithExtension:subdirectory:localization:)]
        pub unsafe fn URLsForResourcesWithExtension_subdirectory_localization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Id<NSArray<NSURL>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:)]
        pub unsafe fn pathForResource_ofType_inDirectory_class(
            name: Option<&NSString>,
            ext: Option<&NSString>,
            bundle_path: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory_class(
            ext: Option<&NSString>,
            bundle_path: &NSString,
        ) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:)]
        pub unsafe fn pathForResource_ofType(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:)]
        pub unsafe fn pathForResource_ofType_inDirectory(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForResource:ofType:inDirectory:forLocalization:)]
        pub unsafe fn pathForResource_ofType_inDirectory_forLocalization(
            &self,
            name: Option<&NSString>,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pathsForResourcesOfType:inDirectory:forLocalization:)]
        pub unsafe fn pathsForResourcesOfType_inDirectory_forLocalization(
            &self,
            ext: Option<&NSString>,
            subpath: Option<&NSString>,
            localization_name: Option<&NSString>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:value:table:)]
        pub unsafe fn localizedStringForKey_value_table(
            &self,
            key: &NSString,
            value: Option<&NSString>,
            table_name: Option<&NSString>,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other infoDictionary)]
        pub fn infoDictionary(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedInfoDictionary)]
        pub unsafe fn localizedInfoDictionary(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForInfoDictionaryKey:)]
        pub unsafe fn objectForInfoDictionaryKey(&self, key: &NSString) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(classNamed:)]
        pub unsafe fn classNamed(&self, class_name: &NSString) -> Option<&'static Class>;

        #[method(principalClass)]
        pub unsafe fn principalClass(&self) -> Option<&'static Class>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizations)]
        pub unsafe fn preferredLocalizations(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizations)]
        pub unsafe fn localizations(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other developmentLocalization)]
        pub unsafe fn developmentLocalization(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizationsFromArray:)]
        pub unsafe fn preferredLocalizationsFromArray(
            localizations_array: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preferredLocalizationsFromArray:forPreferences:)]
        pub unsafe fn preferredLocalizationsFromArray_forPreferences(
            localizations_array: &NSArray<NSString>,
            preferences_array: Option<&NSArray<NSString>>,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other executableArchitectures)]
        pub unsafe fn executableArchitectures(&self) -> Option<Id<NSArray<NSNumber>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSBundleExtensionMethods
    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other variantFittingPresentationWidth:)]
        pub unsafe fn variantFittingPresentationWidth(&self, width: NSInteger) -> Id<NSString>;
    }
);

extern_static!(NSBundleDidLoadNotification: &'static NSNotificationName);

extern_static!(NSLoadedClasses: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct NSBundleResourceRequest;

    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    unsafe impl ClassType for NSBundleResourceRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSBundleResourceRequest")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for NSBundleResourceRequest {}

#[cfg(feature = "Foundation_NSBundleResourceRequest")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSProgressReporting for NSBundleResourceRequest {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    unsafe impl NSBundleResourceRequest {
        #[cfg(not(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos"
        )))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithTags:)]
        pub unsafe fn initWithTags(
            this: Option<Allocated<Self>>,
            tags: &NSSet<NSString>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSBundle",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithTags:bundle:)]
        pub unsafe fn initWithTags_bundle(
            this: Option<Allocated<Self>>,
            tags: &NSSet<NSString>,
            bundle: &NSBundle,
        ) -> Id<Self>;

        #[method(loadingPriority)]
        pub unsafe fn loadingPriority(&self) -> c_double;

        #[method(setLoadingPriority:)]
        pub unsafe fn setLoadingPriority(&self, loading_priority: c_double);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other tags)]
        pub unsafe fn tags(&self) -> Id<NSSet<NSString>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Id<NSBundle>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(beginAccessingResourcesWithCompletionHandler:)]
        pub unsafe fn beginAccessingResourcesWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(conditionallyBeginAccessingResourcesWithCompletionHandler:)]
        pub unsafe fn conditionallyBeginAccessingResourcesWithCompletionHandler(
            &self,
            completion_handler: &Block<(Bool,), ()>,
        );

        #[method(endAccessingResources)]
        pub unsafe fn endAccessingResources(&self);

        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSBundleResourceRequest")]
    unsafe impl NSBundleResourceRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSBundleResourceRequestAdditions
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "macos")))]
        #[method(setPreservationPriority:forTags:)]
        pub unsafe fn setPreservationPriority_forTags(
            &self,
            priority: c_double,
            tags: &NSSet<NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "macos")))]
        #[method(preservationPriorityForTag:)]
        pub unsafe fn preservationPriorityForTag(&self, tag: &NSString) -> c_double;
    }
);

#[cfg(not(any(target_os = "macos")))]
extern_static!(NSBundleResourceRequestLowDiskSpaceNotification: &'static NSNotificationName);

#[cfg(not(any(target_os = "macos")))]
extern_static!(NSBundleResourceRequestLoadingPriorityUrgent: c_double);
