//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSHashTableStrongMemory: NSPointerFunctionsOptions = NSPointerFunctionsStrongMemory);

#[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
extern_static!(
    NSHashTableZeroingWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsZeroingWeakMemory
);

extern_static!(NSHashTableCopyIn: NSPointerFunctionsOptions = NSPointerFunctionsCopyIn);

extern_static!(
    NSHashTableObjectPointerPersonality: NSPointerFunctionsOptions =
        NSPointerFunctionsObjectPointerPersonality
);

extern_static!(NSHashTableWeakMemory: NSPointerFunctionsOptions = NSPointerFunctionsWeakMemory);

pub type NSHashTableOptions = NSUInteger;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSHashTable")]
    pub struct NSHashTable<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSHashTable")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSHashTable<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSHashTable")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSCoding
    for NSHashTable<ObjectType, ObjectTypeOwnership>
{
}

#[cfg(feature = "Foundation_NSHashTable")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSFastEnumeration
    for NSHashTable<ObjectType, ObjectTypeOwnership>
{
}

#[cfg(feature = "Foundation_NSHashTable")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSObjectProtocol
    for NSHashTable<ObjectType, ObjectTypeOwnership>
{
}

#[cfg(feature = "Foundation_NSHashTable")]
unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> NSSecureCoding
    for NSHashTable<ObjectType, ObjectTypeOwnership>
{
}

extern_methods!(
    #[cfg(feature = "Foundation_NSHashTable")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSHashTable<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithOptions:capacity:)]
        pub unsafe fn initWithOptions_capacity(
            this: Option<Allocated<Self>>,
            options: NSPointerFunctionsOptions,
            initial_capacity: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithPointerFunctions:capacity:)]
        pub unsafe fn initWithPointerFunctions_capacity(
            this: Option<Allocated<Self>>,
            functions: &NSPointerFunctions,
            initial_capacity: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other hashTableWithOptions:)]
        pub unsafe fn hashTableWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Id<NSHashTable<ObjectType>>;

        #[deprecated = "GC no longer supported"]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other hashTableWithWeakObjects)]
        pub unsafe fn hashTableWithWeakObjects() -> Id<Object>;

        #[method_id(@__retain_semantics Other weakObjectsHashTable)]
        pub unsafe fn weakObjectsHashTable() -> Id<NSHashTable<ObjectType>>;

        #[cfg(feature = "Foundation_NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Id<NSPointerFunctions>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other member:)]
        pub unsafe fn member(
            &self,
            object: Option<&ObjectType>,
        ) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: Option<&ObjectType>);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: Option<&ObjectType>);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, an_object: Option<&ObjectType>) -> bool;

        #[method(intersectsHashTable:)]
        pub unsafe fn intersectsHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isEqualToHashTable:)]
        pub unsafe fn isEqualToHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isSubsetOfHashTable:)]
        pub unsafe fn isSubsetOfHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(intersectHashTable:)]
        pub unsafe fn intersectHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(unionHashTable:)]
        pub unsafe fn unionHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(minusHashTable:)]
        pub unsafe fn minusHashTable(&self, other: &NSHashTable<ObjectType>);

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other setRepresentation)]
        pub unsafe fn setRepresentation(&self) -> Id<NSSet<ObjectType>>;
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSHashEnumerator {
        _pi: NSUInteger,
        _si: NSUInteger,
        _bs: *mut c_void,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSFreeHashTable(table: &NSHashTable);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSResetHashTable(table: &NSHashTable);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSCompareHashTables(table1: &NSHashTable, table2: &NSHashTable) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSCopyHashTableWithZone(
        table: &NSHashTable,
        zone: *mut NSZone,
    ) -> NonNull<NSHashTable>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSHashGet(table: &NSHashTable, pointer: *mut c_void) -> NonNull<c_void>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSHashInsert(table: &NSHashTable, pointer: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSHashInsertKnownAbsent(table: &NSHashTable, pointer: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSHashInsertIfAbsent(table: &NSHashTable, pointer: *mut c_void) -> *mut c_void;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSHashRemove(table: &NSHashTable, pointer: *mut c_void);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSEnumerateHashTable(table: &NSHashTable) -> NSHashEnumerator;
);

extern_fn!(
    pub unsafe fn NSNextHashEnumeratorItem(enumerator: NonNull<NSHashEnumerator>) -> *mut c_void;
);

extern_fn!(
    pub unsafe fn NSEndHashTableEnumeration(enumerator: NonNull<NSHashEnumerator>);
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSCountHashTable(table: &NSHashTable) -> NSUInteger;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSHashTable", feature = "Foundation_NSString"))]
    pub unsafe fn NSStringFromHashTable(table: &NSHashTable) -> NonNull<NSString>;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSHashTable"))]
    pub unsafe fn NSAllHashTableObjects(table: &NSHashTable) -> NonNull<NSArray>;
);

extern_struct!(
    #[encoding_name("?")]
    pub struct NSHashTableCallBacks {
        pub hash: Option<unsafe extern "C" fn(NonNull<NSHashTable>, NonNull<c_void>) -> NSUInteger>,
        pub isEqual: Option<
            unsafe extern "C" fn(NonNull<NSHashTable>, NonNull<c_void>, NonNull<c_void>) -> Bool,
        >,
        pub retain: Option<unsafe extern "C" fn(NonNull<NSHashTable>, NonNull<c_void>)>,
        pub release: Option<unsafe extern "C" fn(NonNull<NSHashTable>, NonNull<c_void>)>,
        pub describe:
            Option<unsafe extern "C" fn(NonNull<NSHashTable>, NonNull<c_void>) -> *mut NSString>,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSCreateHashTableWithZone(
        call_backs: NSHashTableCallBacks,
        capacity: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<NSHashTable>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSHashTable")]
    pub unsafe fn NSCreateHashTable(
        call_backs: NSHashTableCallBacks,
        capacity: NSUInteger,
    ) -> NonNull<NSHashTable>;
);

#[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
extern_static!(NSIntegerHashCallBacks: NSHashTableCallBacks);

extern_static!(NSNonOwnedPointerHashCallBacks: NSHashTableCallBacks);

extern_static!(NSNonRetainedObjectHashCallBacks: NSHashTableCallBacks);

extern_static!(NSObjectHashCallBacks: NSHashTableCallBacks);

extern_static!(NSOwnedObjectIdentityHashCallBacks: NSHashTableCallBacks);

extern_static!(NSOwnedPointerHashCallBacks: NSHashTableCallBacks);

extern_static!(NSPointerToStructHashCallBacks: NSHashTableCallBacks);

#[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
extern_static!(NSIntHashCallBacks: NSHashTableCallBacks);
