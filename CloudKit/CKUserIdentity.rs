//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKUserIdentity")]
    pub struct CKUserIdentity;

    #[cfg(feature = "CloudKit_CKUserIdentity")]
    unsafe impl ClassType for CKUserIdentity {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSCoding for CKUserIdentity {}

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSObjectProtocol for CKUserIdentity {}

#[cfg(feature = "CloudKit_CKUserIdentity")]
unsafe impl NSSecureCoding for CKUserIdentity {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKUserIdentity")]
    unsafe impl CKUserIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKUserIdentityLookupInfo")]
        /**
          This is the @c lookupInfo you passed in to @c CKDiscoverUserIdentitiesOperation or @c CKFetchShareParticipantsOperation
        */
        #[method_id(@__retain_semantics Other lookupInfo)]
        pub unsafe fn lookupInfo(&self) -> Option<Id<CKUserIdentityLookupInfo>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other nameComponents)]
        pub unsafe fn nameComponents(&self) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "CloudKit_CKRecordID")]
        #[method_id(@__retain_semantics Other userRecordID)]
        pub unsafe fn userRecordID(&self) -> Option<Id<CKRecordID>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          @abstract Link to the Contacts database.

          @discussion Identities discovered via @c CKDiscoverAllUserIdentitiesOperation correspond to entries in the local Contacts database.  These identities will have @c contactIdentifiers filled out, which your app may use to get additional information about the contacts that were discovered.  Multiple @c contactIdentifiers may exist for a single discovered user, as multiple contacts may contain the same email addresses or phone numbers.

          @return individual, non-unified contacts.

          @discussion To transform these identifiers into an array of unified contact identifiers, pass a @c CNContact.predicateForContacts(withIdentifiers:) predicate into @c CNContactStore.unifiedContacts(matching:keysToFetch:)

          @see Contacts.framework and CNContact.identifier
        */
        #[method_id(@__retain_semantics Other contactIdentifiers)]
        pub unsafe fn contactIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[method(hasiCloudAccount)]
        pub unsafe fn hasiCloudAccount(&self) -> bool;
    }
);
