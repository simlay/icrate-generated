//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSFontTraitMask {
        #[cfg(not(any(target_os = "ios")))]
        NSItalicFontMask = 0x00000001,
        #[cfg(not(any(target_os = "ios")))]
        NSBoldFontMask = 0x00000002,
        #[cfg(not(any(target_os = "ios")))]
        NSUnboldFontMask = 0x00000004,
        #[cfg(not(any(target_os = "ios")))]
        NSNonStandardCharacterSetFontMask = 0x00000008,
        #[cfg(not(any(target_os = "ios")))]
        NSNarrowFontMask = 0x00000010,
        #[cfg(not(any(target_os = "ios")))]
        NSExpandedFontMask = 0x00000020,
        #[cfg(not(any(target_os = "ios")))]
        NSCondensedFontMask = 0x00000040,
        #[cfg(not(any(target_os = "ios")))]
        NSSmallCapsFontMask = 0x00000080,
        #[cfg(not(any(target_os = "ios")))]
        NSPosterFontMask = 0x00000100,
        #[cfg(not(any(target_os = "ios")))]
        NSCompressedFontMask = 0x00000200,
        #[cfg(not(any(target_os = "ios")))]
        NSFixedPitchFontMask = 0x00000400,
        #[cfg(not(any(target_os = "ios")))]
        NSUnitalicFontMask = 0x01000000,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSFontCollectionOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSFontCollectionApplicationOnlyMask = 1 << 0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSFontAction {
        #[cfg(not(any(target_os = "ios")))]
        NSNoFontChangeAction = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSViaPanelFontAction = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSAddTraitFontAction = 2,
        #[cfg(not(any(target_os = "ios")))]
        NSSizeUpFontAction = 3,
        #[cfg(not(any(target_os = "ios")))]
        NSSizeDownFontAction = 4,
        #[cfg(not(any(target_os = "ios")))]
        NSHeavierFontAction = 5,
        #[cfg(not(any(target_os = "ios")))]
        NSLighterFontAction = 6,
        #[cfg(not(any(target_os = "ios")))]
        NSRemoveTraitFontAction = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFontManager")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSFontManager;

    #[cfg(feature = "AppKit_NSFontManager")]
    unsafe impl ClassType for NSFontManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSFontManager")]
unsafe impl NSMenuItemValidation for NSFontManager {}

#[cfg(feature = "AppKit_NSFontManager")]
unsafe impl NSObjectProtocol for NSFontManager {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFontManager")]
    unsafe impl NSFontManager {
        #[cfg(not(any(target_os = "ios")))]
        #[method(setFontPanelFactory:)]
        pub unsafe fn setFontPanelFactory(factory_id: Option<&Class>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(setFontManagerFactory:)]
        pub unsafe fn setFontManagerFactory(factory_id: Option<&Class>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other sharedFontManager)]
        pub unsafe fn sharedFontManager() -> Id<NSFontManager>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isMultiple)]
        pub unsafe fn isMultiple(&self) -> bool;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other selectedFont)]
        pub unsafe fn selectedFont(&self) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectedFont:isMultiple:)]
        pub unsafe fn setSelectedFont_isMultiple(&self, font_obj: &NSFont, flag: bool);

        #[cfg(feature = "AppKit_NSMenu")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setFontMenu:)]
        pub unsafe fn setFontMenu(&self, new_menu: &NSMenu);

        #[cfg(feature = "AppKit_NSMenu")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other fontMenu:)]
        pub unsafe fn fontMenu(&self, create: bool) -> Option<Id<NSMenu>>;

        #[cfg(feature = "AppKit_NSFontPanel")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other fontPanel:)]
        pub unsafe fn fontPanel(&self, create: bool) -> Option<Id<NSFontPanel>>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other fontWithFamily:traits:weight:size:)]
        pub unsafe fn fontWithFamily_traits_weight_size(
            &self,
            family: &NSString,
            traits: NSFontTraitMask,
            weight: NSInteger,
            size: CGFloat,
        ) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(traitsOfFont:)]
        pub unsafe fn traitsOfFont(&self, font_obj: &NSFont) -> NSFontTraitMask;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(weightOfFont:)]
        pub unsafe fn weightOfFont(&self, font_obj: &NSFont) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableFonts)]
        pub unsafe fn availableFonts(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableFontFamilies)]
        pub unsafe fn availableFontFamilies(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableMembersOfFontFamily:)]
        pub unsafe fn availableMembersOfFontFamily(
            &self,
            fam: &NSString,
        ) -> Option<Id<NSArray<NSArray>>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:)]
        pub unsafe fn convertFont(&self, font_obj: &NSFont) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:toSize:)]
        pub unsafe fn convertFont_toSize(&self, font_obj: &NSFont, size: CGFloat) -> Id<NSFont>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:toFace:)]
        pub unsafe fn convertFont_toFace(
            &self,
            font_obj: &NSFont,
            typeface: &NSString,
        ) -> Option<Id<NSFont>>;

        #[cfg(all(feature = "AppKit_NSFont", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:toFamily:)]
        pub unsafe fn convertFont_toFamily(
            &self,
            font_obj: &NSFont,
            family: &NSString,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:toHaveTrait:)]
        pub unsafe fn convertFont_toHaveTrait(
            &self,
            font_obj: &NSFont,
            r#trait: NSFontTraitMask,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertFont:toNotHaveTrait:)]
        pub unsafe fn convertFont_toNotHaveTrait(
            &self,
            font_obj: &NSFont,
            r#trait: NSFontTraitMask,
        ) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertWeight:ofFont:)]
        pub unsafe fn convertWeight_ofFont(&self, up_flag: bool, font_obj: &NSFont) -> Id<NSFont>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(action)]
        pub unsafe fn action(&self) -> Sel;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Sel);

        #[deprecated = "NSFontManager doesn't have any delegate method. This property should not be used."]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Object>>;

        #[deprecated = "NSFontManager doesn't have any delegate method. This property should not be used."]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other localizedNameForFamily:face:)]
        pub unsafe fn localizedNameForFamily_face(
            &self,
            family: &NSString,
            face_key: Option<&NSString>,
        ) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectedAttributes:isMultiple:)]
        pub unsafe fn setSelectedAttributes_isMultiple(
            &self,
            attributes: &NSDictionary<NSString, Object>,
            flag: bool,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other convertAttributes:)]
        pub unsafe fn convertAttributes(
            &self,
            attributes: &NSDictionary<NSString, Object>,
        ) -> Id<NSDictionary<NSString, Object>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[deprecated = "Use -[NSFontDescriptor matchingFontDescriptorsWithMandatoryKeys:] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableFontNamesMatchingFontDescriptor:)]
        pub unsafe fn availableFontNamesMatchingFontDescriptor(
            &self,
            descriptor: &NSFontDescriptor,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use +[NSFontCollection allFontCollectionNames] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other collectionNames)]
        pub unsafe fn collectionNames(&self) -> Id<NSArray>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSFontCollection matchingDescriptors] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other fontDescriptorsInCollection:)]
        pub unsafe fn fontDescriptorsInCollection(
            &self,
            collection_names: &NSString,
        ) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +[NSFontCollection showFontCollection:withName:visibility:name:] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method(addCollection:options:)]
        pub unsafe fn addCollection_options(
            &self,
            collection_name: &NSString,
            collection_options: NSFontCollectionOptions,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +[NSFontCollection hideFontCollectionWithName:visibility:error:] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method(removeCollection:)]
        pub unsafe fn removeCollection(&self, collection_name: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSMutableFontCollection addQueryForDescriptors:] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method(addFontDescriptors:toCollection:)]
        pub unsafe fn addFontDescriptors_toCollection(
            &self,
            descriptors: &NSArray,
            collection_name: &NSString,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSString"))]
        #[deprecated = "Use -[NSMutableFontCollection removeQueryForDescriptors:] instead"]
        #[cfg(not(any(target_os = "ios")))]
        #[method(removeFontDescriptor:fromCollection:)]
        pub unsafe fn removeFontDescriptor_fromCollection(
            &self,
            descriptor: &NSFontDescriptor,
            collection: &NSString,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(currentFontAction)]
        pub unsafe fn currentFontAction(&self) -> NSFontAction;

        #[cfg(not(any(target_os = "ios")))]
        #[method(convertFontTraits:)]
        pub unsafe fn convertFontTraits(&self, traits: NSFontTraitMask) -> NSFontTraitMask;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
    }
);

extern_methods!(
    /// NSFontManagerMenuActionMethods
    #[cfg(feature = "AppKit_NSFontManager")]
    unsafe impl NSFontManager {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(fontNamed:hasTraits:)]
        pub unsafe fn fontNamed_hasTraits(
            &self,
            f_name: &NSString,
            some_traits: NSFontTraitMask,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableFontNamesWithTraits:)]
        pub unsafe fn availableFontNamesWithTraits(
            &self,
            some_traits: NSFontTraitMask,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(addFontTrait:)]
        pub unsafe fn addFontTrait(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(removeFontTrait:)]
        pub unsafe fn removeFontTrait(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(modifyFontViaPanel:)]
        pub unsafe fn modifyFontViaPanel(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(modifyFont:)]
        pub unsafe fn modifyFont(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(orderFrontFontPanel:)]
        pub unsafe fn orderFrontFontPanel(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(orderFrontStylesPanel:)]
        pub unsafe fn orderFrontStylesPanel(&self, sender: Option<&Object>);
    }
);
