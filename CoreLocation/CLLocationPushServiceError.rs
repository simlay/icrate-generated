//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_static!(CLLocationPushServiceErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    /**
       CLLocationPushServiceError

      Discussion:
        Error returned as code to NSError from -[CLLocationManager startMonitoringLocationPushesWithCompletion:].
    */
    pub enum CLLocationPushServiceError {
        CLLocationPushServiceErrorUnknown = 0,
        CLLocationPushServiceErrorMissingPushExtension = 1,
        CLLocationPushServiceErrorMissingPushServerEnvironment = 2,
        CLLocationPushServiceErrorMissingEntitlement = 3,
    }
);
