//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::SoundAnalysis::*;

extern_protocol!(
    /**
     @brief The base protocol to which analysis requests conform
    @discussion An analysis request is a configuration that defines the analysis the client wishes to perform on the audio stream. Each request has a corresponding result type, which contains information describing the analysis results. This protocol is designed for all requests provided by the framework to conform to, and shouldn't be conformed to by client objects.
    */
    pub unsafe trait SNRequest: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn SNRequest {}
);
