//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "CNChangeHistoryEvent.rs"]
mod __CNChangeHistoryEvent;
#[path = "CNChangeHistoryFetchRequest.rs"]
mod __CNChangeHistoryFetchRequest;
#[path = "CNContact.rs"]
mod __CNContact;
#[path = "CNContactFetchRequest.rs"]
mod __CNContactFetchRequest;
#[path = "CNContactFormatter.rs"]
mod __CNContactFormatter;
#[path = "CNContactProperty.rs"]
mod __CNContactProperty;
#[path = "CNContactRelation.rs"]
mod __CNContactRelation;
#[path = "CNContactStore.rs"]
mod __CNContactStore;
#[path = "CNContactVCardSerialization.rs"]
mod __CNContactVCardSerialization;
#[path = "CNContact_NSItemProvider.rs"]
mod __CNContact_NSItemProvider;
#[path = "CNContact_Predicates.rs"]
mod __CNContact_Predicates;
#[path = "CNContactsUserDefaults.rs"]
mod __CNContactsUserDefaults;
#[path = "CNContainer.rs"]
mod __CNContainer;
#[path = "CNContainer_Predicates.rs"]
mod __CNContainer_Predicates;
#[path = "CNError.rs"]
mod __CNError;
#[path = "CNFetchRequest.rs"]
mod __CNFetchRequest;
#[path = "CNFetchResult.rs"]
mod __CNFetchResult;
#[path = "CNGroup.rs"]
mod __CNGroup;
#[path = "CNGroup_Predicates.rs"]
mod __CNGroup_Predicates;
#[path = "CNInstantMessageAddress.rs"]
mod __CNInstantMessageAddress;
#[path = "CNLabeledValue.rs"]
mod __CNLabeledValue;
#[path = "CNMutableContact.rs"]
mod __CNMutableContact;
#[path = "CNMutableGroup.rs"]
mod __CNMutableGroup;
#[path = "CNMutablePostalAddress.rs"]
mod __CNMutablePostalAddress;
#[path = "CNPhoneNumber.rs"]
mod __CNPhoneNumber;
#[path = "CNPostalAddress.rs"]
mod __CNPostalAddress;
#[path = "CNPostalAddressFormatter.rs"]
mod __CNPostalAddressFormatter;
#[path = "CNSaveRequest.rs"]
mod __CNSaveRequest;
#[path = "CNSocialProfile.rs"]
mod __CNSocialProfile;
#[path = "ContactsDefines.rs"]
mod __ContactsDefines;

#[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryAddContactEvent;
#[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryAddGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryAddMemberToGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryAddSubgroupToGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryDeleteContactEvent;
#[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryDeleteGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryDropEverythingEvent;
#[cfg(feature = "Contacts_CNChangeHistoryEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryEvent;
#[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryRemoveMemberFromGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryRemoveSubgroupFromGroupEvent;
#[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryUpdateContactEvent;
#[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
pub use self::__CNChangeHistoryEvent::CNChangeHistoryUpdateGroupEvent;

pub use self::__CNChangeHistoryEvent::CNChangeHistoryEventVisitor;
#[cfg(feature = "Contacts_CNChangeHistoryFetchRequest")]
pub use self::__CNChangeHistoryFetchRequest::CNChangeHistoryFetchRequest;

pub use self::__CNContact::CNContactType;

pub use self::__CNContact::CNContactTypePerson;

pub use self::__CNContact::CNContactTypeOrganization;

pub use self::__CNContact::CNContactSortOrder;

pub use self::__CNContact::CNContactSortOrderNone;

pub use self::__CNContact::CNContactSortOrderUserDefault;

pub use self::__CNContact::CNContactSortOrderGivenName;

pub use self::__CNContact::CNContactSortOrderFamilyName;

#[cfg(feature = "Contacts_CNContact")]
pub use self::__CNContact::CNContact;
pub use self::__CNContact::CNKeyDescriptor;

pub use self::__CNContact::CNContactPropertyNotFetchedExceptionName;

pub use self::__CNContact::CNContactIdentifierKey;

pub use self::__CNContact::CNContactNamePrefixKey;

pub use self::__CNContact::CNContactGivenNameKey;

pub use self::__CNContact::CNContactMiddleNameKey;

pub use self::__CNContact::CNContactFamilyNameKey;

pub use self::__CNContact::CNContactPreviousFamilyNameKey;

pub use self::__CNContact::CNContactNameSuffixKey;

pub use self::__CNContact::CNContactNicknameKey;

pub use self::__CNContact::CNContactOrganizationNameKey;

pub use self::__CNContact::CNContactDepartmentNameKey;

pub use self::__CNContact::CNContactJobTitleKey;

pub use self::__CNContact::CNContactPhoneticGivenNameKey;

pub use self::__CNContact::CNContactPhoneticMiddleNameKey;

pub use self::__CNContact::CNContactPhoneticFamilyNameKey;

pub use self::__CNContact::CNContactPhoneticOrganizationNameKey;

pub use self::__CNContact::CNContactBirthdayKey;

pub use self::__CNContact::CNContactNonGregorianBirthdayKey;

pub use self::__CNContact::CNContactNoteKey;

pub use self::__CNContact::CNContactImageDataKey;

pub use self::__CNContact::CNContactThumbnailImageDataKey;

pub use self::__CNContact::CNContactImageDataAvailableKey;

pub use self::__CNContact::CNContactTypeKey;

pub use self::__CNContact::CNContactPhoneNumbersKey;

pub use self::__CNContact::CNContactEmailAddressesKey;

pub use self::__CNContact::CNContactPostalAddressesKey;

pub use self::__CNContact::CNContactDatesKey;

pub use self::__CNContact::CNContactUrlAddressesKey;

pub use self::__CNContact::CNContactRelationsKey;

pub use self::__CNContact::CNContactSocialProfilesKey;

pub use self::__CNContact::CNContactInstantMessageAddressesKey;
#[cfg(feature = "Contacts_CNContactFetchRequest")]
pub use self::__CNContactFetchRequest::CNContactFetchRequest;

pub use self::__CNContactFormatter::CNContactFormatterStyle;

pub use self::__CNContactFormatter::CNContactFormatterStyleFullName;

pub use self::__CNContactFormatter::CNContactFormatterStylePhoneticFullName;

pub use self::__CNContactFormatter::CNContactDisplayNameOrder;

pub use self::__CNContactFormatter::CNContactDisplayNameOrderUserDefault;

pub use self::__CNContactFormatter::CNContactDisplayNameOrderGivenNameFirst;

pub use self::__CNContactFormatter::CNContactDisplayNameOrderFamilyNameFirst;
#[cfg(feature = "Contacts_CNContactFormatter")]
pub use self::__CNContactFormatter::CNContactFormatter;

pub use self::__CNContactFormatter::CNContactPropertyAttribute;
#[cfg(feature = "Contacts_CNContactProperty")]
pub use self::__CNContactProperty::CNContactProperty;
#[cfg(feature = "Contacts_CNContactRelation")]
pub use self::__CNContactRelation::CNContactRelation;

pub use self::__CNContactRelation::CNLabelContactRelationAssistant;

pub use self::__CNContactRelation::CNLabelContactRelationManager;

pub use self::__CNContactRelation::CNLabelContactRelationColleague;

pub use self::__CNContactRelation::CNLabelContactRelationTeacher;

pub use self::__CNContactRelation::CNLabelContactRelationSibling;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerSibling;

pub use self::__CNContactRelation::CNLabelContactRelationElderSibling;

pub use self::__CNContactRelation::CNLabelContactRelationSister;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerSister;

pub use self::__CNContactRelation::CNLabelContactRelationYoungestSister;

pub use self::__CNContactRelation::CNLabelContactRelationElderSister;

pub use self::__CNContactRelation::CNLabelContactRelationEldestSister;

pub use self::__CNContactRelation::CNLabelContactRelationBrother;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerBrother;

pub use self::__CNContactRelation::CNLabelContactRelationYoungestBrother;

pub use self::__CNContactRelation::CNLabelContactRelationElderBrother;

pub use self::__CNContactRelation::CNLabelContactRelationEldestBrother;

pub use self::__CNContactRelation::CNLabelContactRelationFriend;

pub use self::__CNContactRelation::CNLabelContactRelationMaleFriend;

pub use self::__CNContactRelation::CNLabelContactRelationFemaleFriend;

pub use self::__CNContactRelation::CNLabelContactRelationSpouse;

pub use self::__CNContactRelation::CNLabelContactRelationWife;

pub use self::__CNContactRelation::CNLabelContactRelationHusband;

pub use self::__CNContactRelation::CNLabelContactRelationPartner;

pub use self::__CNContactRelation::CNLabelContactRelationMalePartner;

pub use self::__CNContactRelation::CNLabelContactRelationFemalePartner;

pub use self::__CNContactRelation::CNLabelContactRelationGirlfriendOrBoyfriend;

pub use self::__CNContactRelation::CNLabelContactRelationGirlfriend;

pub use self::__CNContactRelation::CNLabelContactRelationBoyfriend;

pub use self::__CNContactRelation::CNLabelContactRelationParent;

pub use self::__CNContactRelation::CNLabelContactRelationMother;

pub use self::__CNContactRelation::CNLabelContactRelationFather;

pub use self::__CNContactRelation::CNLabelContactRelationChild;

pub use self::__CNContactRelation::CNLabelContactRelationDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationSon;

pub use self::__CNContactRelation::CNLabelContactRelationGrandparent;

pub use self::__CNContactRelation::CNLabelContactRelationGrandmother;

pub use self::__CNContactRelation::CNLabelContactRelationGrandmotherMothersMother;

pub use self::__CNContactRelation::CNLabelContactRelationGrandmotherFathersMother;

pub use self::__CNContactRelation::CNLabelContactRelationGrandfather;

pub use self::__CNContactRelation::CNLabelContactRelationGrandfatherMothersFather;

pub use self::__CNContactRelation::CNLabelContactRelationGrandfatherFathersFather;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandparent;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandmother;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandfather;

pub use self::__CNContactRelation::CNLabelContactRelationGrandchild;

pub use self::__CNContactRelation::CNLabelContactRelationGranddaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGranddaughterDaughtersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGranddaughterSonsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGrandson;

pub use self::__CNContactRelation::CNLabelContactRelationGrandsonDaughtersSon;

pub use self::__CNContactRelation::CNLabelContactRelationGrandsonSonsSon;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandchild;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGranddaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandson;

pub use self::__CNContactRelation::CNLabelContactRelationParentInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationMotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationMotherInLawWifesMother;

pub use self::__CNContactRelation::CNLabelContactRelationMotherInLawHusbandsMother;

pub use self::__CNContactRelation::CNLabelContactRelationFatherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationFatherInLawWifesFather;

pub use self::__CNContactRelation::CNLabelContactRelationFatherInLawHusbandsFather;

pub use self::__CNContactRelation::CNLabelContactRelationCoParentInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationCoMotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationCoFatherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationSiblingInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerSiblingInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationElderSiblingInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerSisterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationElderSisterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawSpousesSister;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawWifesSister;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawHusbandsSister;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawYoungerBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawElderBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerBrotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationElderBrotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawSpousesBrother;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawHusbandsBrother;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawWifesBrother;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawYoungerSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawElderSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawWifesBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationSisterInLawHusbandsBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawWifesSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationBrotherInLawHusbandsSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationCoSiblingInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationCoSisterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationCoBrotherInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationChildInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationDaughterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationSonInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationCousin;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousin;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousin;

pub use self::__CNContactRelation::CNLabelContactRelationMaleCousin;

pub use self::__CNContactRelation::CNLabelContactRelationFemaleCousin;

pub use self::__CNContactRelation::CNLabelContactRelationCousinParentsSiblingsChild;

pub use self::__CNContactRelation::CNLabelContactRelationCousinParentsSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinParentsSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinParentsSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationCousinParentsSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinParentsSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinParentsSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinMothersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinMothersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationCousinMothersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinMothersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationCousinFathersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinFathersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinFathersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinFathersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinFathersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinFathersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationCousinFathersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinFathersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinFathersBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinFathersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinFathersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinFathersBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationCousinGrandparentsSiblingsChild;

pub use self::__CNContactRelation::CNLabelContactRelationCousinGrandparentsSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationCousinGrandparentsSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersSiblingsSonOrFathersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersSiblingsSonOrFathersSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationYoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationElderCousinMothersSiblingsDaughterOrFathersSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsYoungerSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsElderSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingMothersSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingMothersYoungerSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingMothersElderSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingFathersSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingFathersYoungerSibling;

pub use self::__CNContactRelation::CNLabelContactRelationParentsSiblingFathersElderSibling;

pub use self::__CNContactRelation::CNLabelContactRelationAunt;

pub use self::__CNContactRelation::CNLabelContactRelationAuntParentsSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntParentsYoungerSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntParentsElderSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersYoungerSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersElderSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersYoungerBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationAuntFathersElderBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationAuntMothersSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntMothersYoungerSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntMothersElderSister;

pub use self::__CNContactRelation::CNLabelContactRelationAuntMothersBrothersWife;

pub use self::__CNContactRelation::CNLabelContactRelationGrandaunt;

pub use self::__CNContactRelation::CNLabelContactRelationUncle;

pub use self::__CNContactRelation::CNLabelContactRelationUncleParentsBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleParentsYoungerBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleParentsElderBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleMothersBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleMothersYoungerBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleMothersElderBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleMothersSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersYoungerBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersElderBrother;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersYoungerSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationUncleFathersElderSistersHusband;

pub use self::__CNContactRelation::CNLabelContactRelationGranduncle;

pub use self::__CNContactRelation::CNLabelContactRelationSiblingsChild;

pub use self::__CNContactRelation::CNLabelContactRelationNiece;

pub use self::__CNContactRelation::CNLabelContactRelationNieceSistersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationNieceBrothersDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationNieceSistersDaughterOrWifesSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationNieceBrothersDaughterOrHusbandsSiblingsDaughter;

pub use self::__CNContactRelation::CNLabelContactRelationNephew;

pub use self::__CNContactRelation::CNLabelContactRelationNephewSistersSon;

pub use self::__CNContactRelation::CNLabelContactRelationNephewBrothersSon;

pub use self::__CNContactRelation::CNLabelContactRelationNephewBrothersSonOrHusbandsSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationNephewSistersSonOrWifesSiblingsSon;

pub use self::__CNContactRelation::CNLabelContactRelationGrandniece;

pub use self::__CNContactRelation::CNLabelContactRelationGrandnieceSistersGranddaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGrandnieceBrothersGranddaughter;

pub use self::__CNContactRelation::CNLabelContactRelationGrandnephew;

pub use self::__CNContactRelation::CNLabelContactRelationGrandnephewSistersGrandson;

pub use self::__CNContactRelation::CNLabelContactRelationGrandnephewBrothersGrandson;

pub use self::__CNContactRelation::CNLabelContactRelationStepparent;

pub use self::__CNContactRelation::CNLabelContactRelationStepmother;

pub use self::__CNContactRelation::CNLabelContactRelationStepfather;

pub use self::__CNContactRelation::CNLabelContactRelationStepchild;

pub use self::__CNContactRelation::CNLabelContactRelationStepdaughter;

pub use self::__CNContactRelation::CNLabelContactRelationStepson;

pub use self::__CNContactRelation::CNLabelContactRelationStepsister;

pub use self::__CNContactRelation::CNLabelContactRelationStepbrother;

pub use self::__CNContactRelation::CNLabelContactRelationMotherInLawOrStepmother;

pub use self::__CNContactRelation::CNLabelContactRelationFatherInLawOrStepfather;

pub use self::__CNContactRelation::CNLabelContactRelationDaughterInLawOrStepdaughter;

pub use self::__CNContactRelation::CNLabelContactRelationSonInLawOrStepson;

pub use self::__CNContactRelation::CNLabelContactRelationCousinOrSiblingsChild;

pub use self::__CNContactRelation::CNLabelContactRelationNieceOrCousin;

pub use self::__CNContactRelation::CNLabelContactRelationNephewOrCousin;

pub use self::__CNContactRelation::CNLabelContactRelationGrandchildOrSiblingsChild;

pub use self::__CNContactRelation::CNLabelContactRelationGranddaughterOrNiece;

pub use self::__CNContactRelation::CNLabelContactRelationGrandsonOrNephew;

pub use self::__CNContactRelation::CNLabelContactRelationGreatGrandchildOrSiblingsGrandchild;

pub use self::__CNContactRelation::CNLabelContactRelationDaughterInLawOrSisterInLaw;

pub use self::__CNContactRelation::CNLabelContactRelationSonInLawOrBrotherInLaw;

pub use self::__CNContactStore::CNEntityType;

pub use self::__CNContactStore::CNEntityTypeContacts;

pub use self::__CNContactStore::CNAuthorizationStatus;

pub use self::__CNContactStore::CNAuthorizationStatusNotDetermined;

pub use self::__CNContactStore::CNAuthorizationStatusRestricted;

pub use self::__CNContactStore::CNAuthorizationStatusDenied;

pub use self::__CNContactStore::CNAuthorizationStatusAuthorized;
#[cfg(feature = "Contacts_CNContactStore")]
pub use self::__CNContactStore::CNContactStore;

pub use self::__CNContactStore::CNContactStoreDidChangeNotification;
#[cfg(feature = "Contacts_CNContactVCardSerialization")]
pub use self::__CNContactVCardSerialization::CNContactVCardSerialization;
#[cfg(feature = "Contacts_CNContactsUserDefaults")]
pub use self::__CNContactsUserDefaults::CNContactsUserDefaults;

pub use self::__CNContainer::CNContainerType;

pub use self::__CNContainer::CNContainerTypeUnassigned;

pub use self::__CNContainer::CNContainerTypeLocal;

pub use self::__CNContainer::CNContainerTypeExchange;

#[cfg(feature = "Contacts_CNContainer")]
pub use self::__CNContainer::CNContainer;
pub use self::__CNContainer::CNContainerTypeCardDAV;

pub use self::__CNContainer::CNContainerIdentifierKey;

pub use self::__CNContainer::CNContainerNameKey;

pub use self::__CNContainer::CNContainerTypeKey;

pub use self::__CNError::CNErrorDomain;

pub use self::__CNError::CNErrorCode;

pub use self::__CNError::CNErrorCodeCommunicationError;

pub use self::__CNError::CNErrorCodeDataAccessError;

pub use self::__CNError::CNErrorCodeAuthorizationDenied;

pub use self::__CNError::CNErrorCodeNoAccessableWritableContainers;

pub use self::__CNError::CNErrorCodeUnauthorizedKeys;

pub use self::__CNError::CNErrorCodeFeatureDisabledByUser;

pub use self::__CNError::CNErrorCodeRecordDoesNotExist;

pub use self::__CNError::CNErrorCodeInsertedRecordAlreadyExists;

pub use self::__CNError::CNErrorCodeContainmentCycle;

pub use self::__CNError::CNErrorCodeContainmentScope;

pub use self::__CNError::CNErrorCodeParentRecordDoesNotExist;

pub use self::__CNError::CNErrorCodeRecordIdentifierInvalid;

pub use self::__CNError::CNErrorCodeRecordNotWritable;

pub use self::__CNError::CNErrorCodeParentContainerNotWritable;

pub use self::__CNError::CNErrorCodeValidationMultipleErrors;

pub use self::__CNError::CNErrorCodeValidationTypeMismatch;

pub use self::__CNError::CNErrorCodeValidationConfigurationError;

pub use self::__CNError::CNErrorCodePredicateInvalid;

pub use self::__CNError::CNErrorCodePolicyViolation;

pub use self::__CNError::CNErrorCodeClientIdentifierInvalid;

pub use self::__CNError::CNErrorCodeClientIdentifierDoesNotExist;

pub use self::__CNError::CNErrorCodeClientIdentifierCollision;

pub use self::__CNError::CNErrorCodeChangeHistoryExpired;

pub use self::__CNError::CNErrorCodeChangeHistoryInvalidAnchor;

pub use self::__CNError::CNErrorCodeChangeHistoryInvalidFetchRequest;

pub use self::__CNError::CNErrorCodeVCardMalformed;

pub use self::__CNError::CNErrorCodeVCardSummarizationError;

pub use self::__CNError::CNErrorUserInfoAffectedRecordsKey;

pub use self::__CNError::CNErrorUserInfoAffectedRecordIdentifiersKey;

pub use self::__CNError::CNErrorUserInfoValidationErrorsKey;

pub use self::__CNError::CNErrorUserInfoKeyPathsKey;
#[cfg(feature = "Contacts_CNFetchRequest")]
pub use self::__CNFetchRequest::CNFetchRequest;
#[cfg(feature = "Contacts_CNFetchResult")]
pub use self::__CNFetchResult::CNFetchResult;
#[cfg(feature = "Contacts_CNGroup")]
pub use self::__CNGroup::CNGroup;

pub use self::__CNGroup::CNGroupIdentifierKey;

pub use self::__CNGroup::CNGroupNameKey;
#[cfg(feature = "Contacts_CNInstantMessageAddress")]
pub use self::__CNInstantMessageAddress::CNInstantMessageAddress;

pub use self::__CNInstantMessageAddress::CNInstantMessageAddressUsernameKey;

pub use self::__CNInstantMessageAddress::CNInstantMessageAddressServiceKey;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceAIM;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceFacebook;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceGaduGadu;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceGoogleTalk;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceICQ;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceJabber;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceMSN;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceQQ;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceSkype;

pub use self::__CNInstantMessageAddress::CNInstantMessageServiceYahoo;
#[cfg(feature = "Contacts_CNLabeledValue")]
pub use self::__CNLabeledValue::CNLabeledValue;

pub use self::__CNLabeledValue::CNLabelHome;

pub use self::__CNLabeledValue::CNLabelWork;

pub use self::__CNLabeledValue::CNLabelSchool;

pub use self::__CNLabeledValue::CNLabelOther;

pub use self::__CNLabeledValue::CNLabelEmailiCloud;

pub use self::__CNLabeledValue::CNLabelURLAddressHomePage;

pub use self::__CNLabeledValue::CNLabelDateAnniversary;
#[cfg(feature = "Contacts_CNMutableContact")]
pub use self::__CNMutableContact::CNMutableContact;
#[cfg(feature = "Contacts_CNMutableGroup")]
pub use self::__CNMutableGroup::CNMutableGroup;
#[cfg(feature = "Contacts_CNMutablePostalAddress")]
pub use self::__CNMutablePostalAddress::CNMutablePostalAddress;
#[cfg(feature = "Contacts_CNPhoneNumber")]
pub use self::__CNPhoneNumber::CNPhoneNumber;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberiPhone;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberAppleWatch;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberMobile;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberMain;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberHomeFax;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberWorkFax;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberOtherFax;

pub use self::__CNPhoneNumber::CNLabelPhoneNumberPager;
#[cfg(feature = "Contacts_CNPostalAddress")]
pub use self::__CNPostalAddress::CNPostalAddress;

pub use self::__CNPostalAddress::CNPostalAddressStreetKey;

pub use self::__CNPostalAddress::CNPostalAddressSubLocalityKey;

pub use self::__CNPostalAddress::CNPostalAddressCityKey;

pub use self::__CNPostalAddress::CNPostalAddressSubAdministrativeAreaKey;

pub use self::__CNPostalAddress::CNPostalAddressStateKey;

pub use self::__CNPostalAddress::CNPostalAddressPostalCodeKey;

pub use self::__CNPostalAddress::CNPostalAddressCountryKey;

pub use self::__CNPostalAddress::CNPostalAddressISOCountryCodeKey;

pub use self::__CNPostalAddressFormatter::CNPostalAddressFormatterStyle;

#[cfg(feature = "Contacts_CNPostalAddressFormatter")]
pub use self::__CNPostalAddressFormatter::CNPostalAddressFormatter;
pub use self::__CNPostalAddressFormatter::CNPostalAddressFormatterStyleMailingAddress;

pub use self::__CNPostalAddressFormatter::CNPostalAddressPropertyAttribute;

pub use self::__CNPostalAddressFormatter::CNPostalAddressLocalizedPropertyNameAttribute;
#[cfg(feature = "Contacts_CNSaveRequest")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__CNSaveRequest::CNSaveRequest;
#[cfg(feature = "Contacts_CNSocialProfile")]
pub use self::__CNSocialProfile::CNSocialProfile;

pub use self::__CNSocialProfile::CNSocialProfileURLStringKey;

pub use self::__CNSocialProfile::CNSocialProfileUsernameKey;

pub use self::__CNSocialProfile::CNSocialProfileUserIdentifierKey;

pub use self::__CNSocialProfile::CNSocialProfileServiceKey;

pub use self::__CNSocialProfile::CNSocialProfileServiceFacebook;

pub use self::__CNSocialProfile::CNSocialProfileServiceFlickr;

pub use self::__CNSocialProfile::CNSocialProfileServiceLinkedIn;

pub use self::__CNSocialProfile::CNSocialProfileServiceMySpace;

pub use self::__CNSocialProfile::CNSocialProfileServiceSinaWeibo;

pub use self::__CNSocialProfile::CNSocialProfileServiceTencentWeibo;

pub use self::__CNSocialProfile::CNSocialProfileServiceTwitter;

pub use self::__CNSocialProfile::CNSocialProfileServiceYelp;

pub use self::__CNSocialProfile::CNSocialProfileServiceGameCenter;
