//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSSearchFieldRecentsAutosaveName = NSString;

extern_protocol!(
    pub unsafe trait NSSearchFieldDelegate: NSTextFieldDelegate {
        #[cfg(feature = "AppKit_NSSearchField")]
        #[optional]
        #[method(searchFieldDidStartSearching:)]
        unsafe fn searchFieldDidStartSearching(&self, sender: &NSSearchField);

        #[cfg(feature = "AppKit_NSSearchField")]
        #[optional]
        #[method(searchFieldDidEndSearching:)]
        unsafe fn searchFieldDidEndSearching(&self, sender: &NSSearchField);
    }

    unsafe impl ProtocolType for dyn NSSearchFieldDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSearchField")]
    pub struct NSSearchField;

    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl ClassType for NSSearchField {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
    }
);

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAccessibility for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAccessibilityElementProtocol for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAccessibilityNavigableStaticText for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAccessibilityStaticText for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAnimatablePropertyContainer for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSAppearanceCustomization for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSCoding for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSDraggingDestination for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSObjectProtocol for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSTextContent for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSUserInterfaceItemIdentification for NSSearchField {}

#[cfg(feature = "AppKit_NSSearchField")]
unsafe impl NSUserInterfaceValidations for NSSearchField {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        /**
          The rectangle for the search text within the bounds of the search field. Subclasses can override this method for custom layout purposes.
        */
        #[method(searchTextBounds)]
        pub unsafe fn searchTextBounds(&self) -> NSRect;

        /**
          The rectangle for the search button within the bounds of the search field. Subclasses can override this method for custom layout purposes.
        */
        #[method(searchButtonBounds)]
        pub unsafe fn searchButtonBounds(&self) -> NSRect;

        /**
          The rectangle for the cancel button within the bounds of the search field. Subclasses can override this method for custom layout purposes.
        */
        #[method(cancelButtonBounds)]
        pub unsafe fn cancelButtonBounds(&self) -> NSRect;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          if app wants to do custom search lists. will return empty array if no searches
        */
        #[method_id(@__retain_semantics Other recentSearches)]
        pub unsafe fn recentSearches(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          if app wants to do custom search lists. will return empty array if no searches
        */
        #[method(setRecentSearches:)]
        pub unsafe fn setRecentSearches(&self, recent_searches: &NSArray<NSString>);

        /**
          must be set to use. default is nil which means no autosave.
        */
        #[method_id(@__retain_semantics Other recentsAutosaveName)]
        pub unsafe fn recentsAutosaveName(&self) -> Option<Id<NSSearchFieldRecentsAutosaveName>>;

        /**
          must be set to use. default is nil which means no autosave.
        */
        #[method(setRecentsAutosaveName:)]
        pub unsafe fn setRecentsAutosaveName(
            &self,
            recents_autosave_name: Option<&NSSearchFieldRecentsAutosaveName>,
        );

        #[cfg(feature = "AppKit_NSMenu")]
        /**
          set/get search menu template. Menu can use custom tagged items to indicate special items. this menu isn't actually set but used to construct the dynamic search menu. if cleared, then we don't track recents.
        */
        #[method_id(@__retain_semantics Other searchMenuTemplate)]
        pub unsafe fn searchMenuTemplate(&self) -> Option<Id<NSMenu>>;

        #[cfg(feature = "AppKit_NSMenu")]
        /**
          set/get search menu template. Menu can use custom tagged items to indicate special items. this menu isn't actually set but used to construct the dynamic search menu. if cleared, then we don't track recents.
        */
        #[method(setSearchMenuTemplate:)]
        pub unsafe fn setSearchMenuTemplate(&self, search_menu_template: Option<&NSMenu>);

        /**
          if clear, send action on each key stroke (after sufficient amount of time so we don't interfere with typing). if set, send only on return/enter or clicking magnifying search button
        */
        #[method(sendsWholeSearchString)]
        pub unsafe fn sendsWholeSearchString(&self) -> bool;

        /**
          if clear, send action on each key stroke (after sufficient amount of time so we don't interfere with typing). if set, send only on return/enter or clicking magnifying search button
        */
        #[method(setSendsWholeSearchString:)]
        pub unsafe fn setSendsWholeSearchString(&self, sends_whole_search_string: bool);

        /**
          set/get limit max recents. allowable between 0 and 254. setting -1 will use default.
        */
        #[method(maximumRecents)]
        pub unsafe fn maximumRecents(&self) -> NSInteger;

        /**
          set/get limit max recents. allowable between 0 and 254. setting -1 will use default.
        */
        #[method(setMaximumRecents:)]
        pub unsafe fn setMaximumRecents(&self, maximum_recents: NSInteger);

        #[method(sendsSearchStringImmediately)]
        pub unsafe fn sendsSearchStringImmediately(&self) -> bool;

        #[method(setSendsSearchStringImmediately:)]
        pub unsafe fn setSendsSearchStringImmediately(&self, sends_search_string_immediately: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSearchFieldDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSearchFieldDelegate>>,
        );
    }
);

extern_methods!(
    /// NSSearchField_Deprecated
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        #[deprecated]
        #[method(rectForSearchTextWhenCentered:)]
        pub unsafe fn rectForSearchTextWhenCentered(&self, is_centered: bool) -> NSRect;

        #[deprecated]
        #[method(rectForSearchButtonWhenCentered:)]
        pub unsafe fn rectForSearchButtonWhenCentered(&self, is_centered: bool) -> NSRect;

        #[deprecated]
        #[method(rectForCancelButtonWhenCentered:)]
        pub unsafe fn rectForCancelButtonWhenCentered(&self, is_centered: bool) -> NSRect;

        #[deprecated = "The placeholder centering UI design is no longer available. Setting this property is no-op."]
        #[method(centersPlaceholder)]
        pub unsafe fn centersPlaceholder(&self) -> bool;

        #[deprecated = "The placeholder centering UI design is no longer available. Setting this property is no-op."]
        #[method(setCentersPlaceholder:)]
        pub unsafe fn setCentersPlaceholder(&self, centers_placeholder: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextField`
    ///
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(string_value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(string_value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributed_string_value: &NSAttributedString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(string_value: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSearchField")]
    unsafe impl NSSearchField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
