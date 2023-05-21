//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSConnection")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSConnection;

    #[deprecated = "Use NSXPCConnection instead"]
    #[cfg(feature = "Foundation_NSConnection")]
    unsafe impl ClassType for NSConnection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSConnection")]
unsafe impl NSObjectProtocol for NSConnection {}

extern_methods!(
    #[cfg(feature = "Foundation_NSConnection")]
    unsafe impl NSConnection {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other statistics)]
        pub unsafe fn statistics(&self) -> Id<NSDictionary<NSString, NSNumber>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allConnections)]
        pub unsafe fn allConnections() -> Id<NSArray<NSConnection>>;

        #[deprecated]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other defaultConnection)]
        pub unsafe fn defaultConnection() -> Id<NSConnection>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:)]
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Id<NSDistantObject>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<NSDistantObject>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:usingNameServer:)]
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &Object,
            server: &NSPortNameServer,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:)]
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &Object,
        ) -> Option<Id<Self>>;

        #[method(requestTimeout)]
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval;

        #[method(setRequestTimeout:)]
        pub unsafe fn setRequestTimeout(&self, request_timeout: NSTimeInterval);

        #[method(replyTimeout)]
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval;

        #[method(setReplyTimeout:)]
        pub unsafe fn setReplyTimeout(&self, reply_timeout: NSTimeInterval);

        #[method_id(@__retain_semantics Other rootObject)]
        pub unsafe fn rootObject(&self) -> Option<Id<Object>>;

        #[method(setRootObject:)]
        pub unsafe fn setRootObject(&self, root_object: Option<&Object>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSConnectionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSConnectionDelegate>>,
        );

        #[method(independentConversationQueueing)]
        pub unsafe fn independentConversationQueueing(&self) -> bool;

        #[method(setIndependentConversationQueueing:)]
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independent_conversation_queueing: bool,
        );

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[cfg(feature = "Foundation_NSDistantObject")]
        #[method_id(@__retain_semantics Other rootProxy)]
        pub unsafe fn rootProxy(&self) -> Id<NSDistantObject>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addRequestMode:)]
        pub unsafe fn addRequestMode(&self, rmode: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeRequestMode:)]
        pub unsafe fn removeRequestMode(&self, rmode: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requestModes)]
        pub unsafe fn requestModes(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerName:)]
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method(registerName:withNameServer:)]
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other connectionWithReceivePort:sendPort:)]
        pub unsafe fn connectionWithReceivePort_sendPort(
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other currentConversation)]
        pub unsafe fn currentConversation() -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Init initWithReceivePort:sendPort:)]
        pub unsafe fn initWithReceivePort_sendPort(
            this: Option<Allocated<Self>>,
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Id<NSPort>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Id<NSPort>;

        #[method(enableMultipleThreads)]
        pub unsafe fn enableMultipleThreads(&self);

        #[method(multipleThreadsEnabled)]
        pub unsafe fn multipleThreadsEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(addRunLoop:)]
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeRunLoop:)]
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop);

        #[method(runInNewThread)]
        pub unsafe fn runInNewThread(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other remoteObjects)]
        pub unsafe fn remoteObjects(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other localObjects)]
        pub unsafe fn localObjects(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(dispatchWithComponents:)]
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSConnection")]
    unsafe impl NSConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSConnectionReplyMode: &'static NSString);

extern_static!(NSConnectionDidDieNotification: &'static NSString);

extern_protocol!(
    pub unsafe trait NSConnectionDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSConnection")]
        #[optional]
        #[method(makeNewConnection:sender:)]
        unsafe fn makeNewConnection_sender(
            &self,
            conn: &NSConnection,
            ancestor: &NSConnection,
        ) -> bool;

        #[cfg(feature = "Foundation_NSConnection")]
        #[optional]
        #[method(connection:shouldMakeNewConnection:)]
        unsafe fn connection_shouldMakeNewConnection(
            &self,
            ancestor: &NSConnection,
            conn: &NSConnection,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[optional]
        #[method_id(@__retain_semantics Other authenticationDataForComponents:)]
        unsafe fn authenticationDataForComponents(&self, components: &NSArray) -> Id<NSData>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[optional]
        #[method(authenticateComponents:withData:)]
        unsafe fn authenticateComponents_withData(
            &self,
            components: &NSArray,
            signature: &NSData,
        ) -> bool;

        #[cfg(feature = "Foundation_NSConnection")]
        #[optional]
        #[method_id(@__retain_semantics Other createConversationForConnection:)]
        unsafe fn createConversationForConnection(&self, conn: &NSConnection) -> Id<Object>;

        #[cfg(all(
            feature = "Foundation_NSConnection",
            feature = "Foundation_NSDistantObjectRequest"
        ))]
        #[optional]
        #[method(connection:handleRequest:)]
        unsafe fn connection_handleRequest(
            &self,
            connection: &NSConnection,
            doreq: &NSDistantObjectRequest,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSConnectionDelegate {}
);

extern_static!(NSFailedAuthenticationException: &'static NSString);

extern_static!(NSConnectionDidInitializeNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDistantObjectRequest")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSDistantObjectRequest;

    #[deprecated = "Use NSXPCConnection instead"]
    #[cfg(feature = "Foundation_NSDistantObjectRequest")]
    unsafe impl ClassType for NSDistantObjectRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSDistantObjectRequest")]
unsafe impl NSObjectProtocol for NSDistantObjectRequest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDistantObjectRequest")]
    unsafe impl NSDistantObjectRequest {
        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Id<NSConnection>;

        #[method_id(@__retain_semantics Other conversation)]
        pub unsafe fn conversation(&self) -> Id<Object>;

        #[cfg(feature = "Foundation_NSException")]
        #[method(replyWithException:)]
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSDistantObjectRequest")]
    unsafe impl NSDistantObjectRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
