//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKContentWorld")]
    /**
      @abstract A WKContentWorld object allows you to separate your application's interaction with content displayed in a WKWebView into different roles that cannot interfere with one another.
    @discussion WKContentWorld objects should be treated as namespaces. This is useful for keeping your application's web content environment separate from the environment of the web page content itself,
    as well as managing multiple different environments within your own application.
    For example:
    - If you have complex scripting logic to bridge your web content to your application but your web content also has complex scripting libraries of its own,
    you avoid possible conflicts by using a client WKContentWorld.
    - If you are writing a general purpose web browser that supports JavaScript extensions, you would use a different client WKContentWorld for each extension.

    Since a WKContentWorld object is a namespace it does not contain any data itself.
    For example:
    - If you store a variable in JavaScript in the scope of a particular WKContentWorld while viewing a particular web page document, after navigating to a new document that variable will be gone.
    - If you store a variable in JavaScript in the scope of a particular WKContentWorld in one WKWebView, that variable will not exist in the same world in another WKWebView.
    */
    pub struct WKContentWorld;

    #[cfg(feature = "WebKit_WKContentWorld")]
    unsafe impl ClassType for WKContentWorld {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKContentWorld")]
/**
  @abstract A WKContentWorld object allows you to separate your application's interaction with content displayed in a WKWebView into different roles that cannot interfere with one another.
@discussion WKContentWorld objects should be treated as namespaces. This is useful for keeping your application's web content environment separate from the environment of the web page content itself,
as well as managing multiple different environments within your own application.
For example:
- If you have complex scripting logic to bridge your web content to your application but your web content also has complex scripting libraries of its own,
you avoid possible conflicts by using a client WKContentWorld.
- If you are writing a general purpose web browser that supports JavaScript extensions, you would use a different client WKContentWorld for each extension.

Since a WKContentWorld object is a namespace it does not contain any data itself.
For example:
- If you store a variable in JavaScript in the scope of a particular WKContentWorld while viewing a particular web page document, after navigating to a new document that variable will be gone.
- If you store a variable in JavaScript in the scope of a particular WKContentWorld in one WKWebView, that variable will not exist in the same world in another WKWebView.
*/
unsafe impl NSObjectProtocol for WKContentWorld {}

extern_methods!(
    /**
      @abstract A WKContentWorld object allows you to separate your application's interaction with content displayed in a WKWebView into different roles that cannot interfere with one another.
    @discussion WKContentWorld objects should be treated as namespaces. This is useful for keeping your application's web content environment separate from the environment of the web page content itself,
    as well as managing multiple different environments within your own application.
    For example:
    - If you have complex scripting logic to bridge your web content to your application but your web content also has complex scripting libraries of its own,
    you avoid possible conflicts by using a client WKContentWorld.
    - If you are writing a general purpose web browser that supports JavaScript extensions, you would use a different client WKContentWorld for each extension.

    Since a WKContentWorld object is a namespace it does not contain any data itself.
    For example:
    - If you store a variable in JavaScript in the scope of a particular WKContentWorld while viewing a particular web page document, after navigating to a new document that variable will be gone.
    - If you store a variable in JavaScript in the scope of a particular WKContentWorld in one WKWebView, that variable will not exist in the same world in another WKWebView.
    */
    #[cfg(feature = "WebKit_WKContentWorld")]
    unsafe impl WKContentWorld {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        /**
          @abstract Retrieve the main world that page content itself uses.
        @discussion When interacting with page content in a WKWebView using the page content world you can disrupt the operation of page content (e.g. by conflicting with variable names in JavaScript set by the web page content itself).
        */
        #[method_id(@__retain_semantics Other pageWorld)]
        pub unsafe fn pageWorld() -> Id<WKContentWorld>;

        /**
          @abstract Retrieve the default world for API client use.
        @discussion When using a content world different from the page content world you can still manipulate the DOM and built-in DOM APIs but without conflicting with other aspects of the page content (e.g. JavaScript from the web page content itself)
        Repeated calls will retrieve the same WKContentWorld instance.
        */
        #[method_id(@__retain_semantics Other defaultClientWorld)]
        pub unsafe fn defaultClientWorld() -> Id<WKContentWorld>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other worldWithName:)]
        pub unsafe fn worldWithName(name: &NSString) -> Id<WKContentWorld>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          @abstract The name of the WKContentWorld
        @discussion The pageWorld and defaultClientWorld instances will have a nil name.
        All other instances will have the non-nil name they were accessed by.
        */
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;
    }
);
