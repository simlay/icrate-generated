//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLConnection")]
    pub struct NSURLConnection;

    #[cfg(feature = "Foundation_NSURLConnection")]
    unsafe impl ClassType for NSURLConnection {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSURLConnection")]
unsafe impl NSObjectProtocol for NSURLConnection {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLConnection")]
    unsafe impl NSURLConnection {
        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:startImmediately:)]
        pub unsafe fn initWithRequest_delegate_startImmediately(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&Object>,
            start_immediately: bool,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated = "Use NSURLSession (see NSURLSession.h)"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other connectionWithRequest:delegate:)]
        pub unsafe fn connectionWithRequest_delegate(
            request: &NSURLRequest,
            delegate: Option<&Object>,
        ) -> Option<Id<NSURLConnection>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Id<NSURLRequest>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other currentRequest)]
        pub unsafe fn currentRequest(&self) -> Id<NSURLRequest>;

        #[method(start)]
        pub unsafe fn start(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(unscheduleFromRunLoop:forMode:)]
        pub unsafe fn unscheduleFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setDelegateQueue:)]
        pub unsafe fn setDelegateQueue(&self, queue: Option<&NSOperationQueue>);

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method(canHandleRequest:)]
        pub unsafe fn canHandleRequest(request: &NSURLRequest) -> bool;
    }
);

extern_protocol!(
    pub unsafe trait NSURLConnectionDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURLConnection"))]
        #[optional]
        #[method(connection:didFailWithError:)]
        unsafe fn connection_didFailWithError(&self, connection: &NSURLConnection, error: &NSError);

        #[cfg(feature = "Foundation_NSURLConnection")]
        #[optional]
        #[method(connectionShouldUseCredentialStorage:)]
        unsafe fn connectionShouldUseCredentialStorage(&self, connection: &NSURLConnection)
            -> bool;

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLConnection"
        ))]
        #[optional]
        #[method(connection:willSendRequestForAuthenticationChallenge:)]
        unsafe fn connection_willSendRequestForAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(all(
            feature = "Foundation_NSURLConnection",
            feature = "Foundation_NSURLProtectionSpace"
        ))]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:canAuthenticateAgainstProtectionSpace:)]
        unsafe fn connection_canAuthenticateAgainstProtectionSpace(
            &self,
            connection: &NSURLConnection,
            protection_space: &NSURLProtectionSpace,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLConnection"
        ))]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:didReceiveAuthenticationChallenge:)]
        unsafe fn connection_didReceiveAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLConnection"
        ))]
        #[deprecated = "Use -connection:willSendRequestForAuthenticationChallenge: instead."]
        #[optional]
        #[method(connection:didCancelAuthenticationChallenge:)]
        unsafe fn connection_didCancelAuthenticationChallenge(
            &self,
            connection: &NSURLConnection,
            challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDelegate {}
);

extern_protocol!(
    pub unsafe trait NSURLConnectionDataDelegate: NSURLConnectionDelegate {
        #[cfg(all(
            feature = "Foundation_NSURLConnection",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other connection:willSendRequest:redirectResponse:)]
        unsafe fn connection_willSendRequest_redirectResponse(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
            response: Option<&NSURLResponse>,
        ) -> Option<Id<NSURLRequest>>;

        #[cfg(all(
            feature = "Foundation_NSURLConnection",
            feature = "Foundation_NSURLResponse"
        ))]
        #[optional]
        #[method(connection:didReceiveResponse:)]
        unsafe fn connection_didReceiveResponse(
            &self,
            connection: &NSURLConnection,
            response: &NSURLResponse,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSURLConnection"))]
        #[optional]
        #[method(connection:didReceiveData:)]
        unsafe fn connection_didReceiveData(&self, connection: &NSURLConnection, data: &NSData);

        #[cfg(all(
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSURLConnection",
            feature = "Foundation_NSURLRequest"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other connection:needNewBodyStream:)]
        unsafe fn connection_needNewBodyStream(
            &self,
            connection: &NSURLConnection,
            request: &NSURLRequest,
        ) -> Option<Id<NSInputStream>>;

        #[cfg(feature = "Foundation_NSURLConnection")]
        #[optional]
        #[method(connection:didSendBodyData:totalBytesWritten:totalBytesExpectedToWrite:)]
        unsafe fn connection_didSendBodyData_totalBytesWritten_totalBytesExpectedToWrite(
            &self,
            connection: &NSURLConnection,
            bytes_written: NSInteger,
            total_bytes_written: NSInteger,
            total_bytes_expected_to_write: NSInteger,
        );

        #[cfg(all(
            feature = "Foundation_NSCachedURLResponse",
            feature = "Foundation_NSURLConnection"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other connection:willCacheResponse:)]
        unsafe fn connection_willCacheResponse(
            &self,
            connection: &NSURLConnection,
            cached_response: &NSCachedURLResponse,
        ) -> Option<Id<NSCachedURLResponse>>;

        #[cfg(feature = "Foundation_NSURLConnection")]
        #[optional]
        #[method(connectionDidFinishLoading:)]
        unsafe fn connectionDidFinishLoading(&self, connection: &NSURLConnection);
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDataDelegate {}
);

extern_protocol!(
    pub unsafe trait NSURLConnectionDownloadDelegate: NSURLConnectionDelegate {
        #[cfg(feature = "Foundation_NSURLConnection")]
        #[optional]
        #[method(connection:didWriteData:totalBytesWritten:expectedTotalBytes:)]
        unsafe fn connection_didWriteData_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            bytes_written: c_longlong,
            total_bytes_written: c_longlong,
            expected_total_bytes: c_longlong,
        );

        #[cfg(feature = "Foundation_NSURLConnection")]
        #[optional]
        #[method(connectionDidResumeDownloading:totalBytesWritten:expectedTotalBytes:)]
        unsafe fn connectionDidResumeDownloading_totalBytesWritten_expectedTotalBytes(
            &self,
            connection: &NSURLConnection,
            total_bytes_written: c_longlong,
            expected_total_bytes: c_longlong,
        );

        #[cfg(all(feature = "Foundation_NSURL", feature = "Foundation_NSURLConnection"))]
        #[method(connectionDidFinishDownloading:destinationURL:)]
        unsafe fn connectionDidFinishDownloading_destinationURL(
            &self,
            connection: &NSURLConnection,
            destination_url: &NSURL,
        );
    }

    unsafe impl ProtocolType for dyn NSURLConnectionDownloadDelegate {}
);

extern_methods!(
    /// NSURLConnectionSynchronousLoading
    #[cfg(feature = "Foundation_NSURLConnection")]
    unsafe impl NSURLConnection {
        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse"
        ))]
        #[deprecated = "Use [NSURLSession dataTaskWithRequest:completionHandler:] (see NSURLSession.h"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other sendSynchronousRequest:returningResponse:error:_)]
        pub unsafe fn sendSynchronousRequest_returningResponse_error(
            request: &NSURLRequest,
            response: Option<&mut Option<Id<NSURLResponse>>>,
        ) -> Result<Id<NSData>, Id<NSError>>;
    }
);

extern_methods!(
    /// NSURLConnectionQueuedLoading
    #[cfg(feature = "Foundation_NSURLConnection")]
    unsafe impl NSURLConnection {
        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSOperationQueue",
            feature = "Foundation_NSURLRequest",
            feature = "Foundation_NSURLResponse"
        ))]
        #[deprecated = "Use [NSURLSession dataTaskWithRequest:completionHandler:] (see NSURLSession.h"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(sendAsynchronousRequest:queue:completionHandler:)]
        pub unsafe fn sendAsynchronousRequest_queue_completionHandler(
            request: &NSURLRequest,
            queue: &NSOperationQueue,
            handler: &Block<(*mut NSURLResponse, *mut NSData, *mut NSError), ()>,
        );
    }
);
