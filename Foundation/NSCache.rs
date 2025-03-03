//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSCache")]
    pub struct NSCache<
        KeyType: Message = Object,
        ObjectType: Message = Object,
        KeyTypeOwnership: Ownership = Shared,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (KeyType, KeyTypeOwnership)>,
        _inner1: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSCache")]
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > ClassType for NSCache<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSCache")]
unsafe impl<
        KeyType: Message,
        ObjectType: Message,
        KeyTypeOwnership: Ownership,
        ObjectTypeOwnership: Ownership,
    > NSObjectProtocol for NSCache<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
{
}

extern_methods!(
    #[cfg(feature = "Foundation_NSCache")]
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSCache<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSCacheDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSCacheDelegate>>);

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            key: &KeyType,
        ) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, obj: &ObjectType, key: &KeyType);

        #[method(setObject:forKey:cost:)]
        pub unsafe fn setObject_forKey_cost(&self, obj: &ObjectType, key: &KeyType, g: NSUInteger);

        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, key: &KeyType);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method(totalCostLimit)]
        pub unsafe fn totalCostLimit(&self) -> NSUInteger;

        #[method(setTotalCostLimit:)]
        pub unsafe fn setTotalCostLimit(&self, total_cost_limit: NSUInteger);

        #[method(countLimit)]
        pub unsafe fn countLimit(&self) -> NSUInteger;

        #[method(setCountLimit:)]
        pub unsafe fn setCountLimit(&self, count_limit: NSUInteger);

        #[method(evictsObjectsWithDiscardedContent)]
        pub unsafe fn evictsObjectsWithDiscardedContent(&self) -> bool;

        #[method(setEvictsObjectsWithDiscardedContent:)]
        pub unsafe fn setEvictsObjectsWithDiscardedContent(
            &self,
            evicts_objects_with_discarded_content: bool,
        );
    }
);

extern_protocol!(
    pub unsafe trait NSCacheDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSCache")]
        #[optional]
        #[method(cache:willEvictObject:)]
        unsafe fn cache_willEvictObject(&self, cache: &NSCache, obj: &Object);
    }

    unsafe impl ProtocolType for dyn NSCacheDelegate {}
);
