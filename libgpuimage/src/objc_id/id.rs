use std::any::Any;
use std::fmt;
use std::hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


use objc::Message;
use objc::rc::{StrongPtr, WeakPtr};
use objc::runtime::Object;


pub enum Owned{}
pub enum Shared{}
pub trait Ownership : Any {}
impl Ownership for Owned{}
impl Ownership for Shared{}


pub struct Id<T, O = Owned> {
    ptr: StrongPtr,
    item: PhantomData<T>,
    own: PhantomData<O>,
}

impl <T, O> Id<T, O>
    where T:Message, O: Ownership {
    unsafe fn new(ptr: StrongPtr) -> Id<T,O>{
        Id{ptr:ptr,item:PhantomData,own:PhantomData}
    }

    pub unsafe fn from_ptr(ptr: *mut T) -> Id<T, O>{
        assert!(!ptr.is_null(),"Attempted to construct an Id from a null pointer");
        Id::new(StrongPtr::retain(ptr as *mut Object))
    }

    pub unsafe fn from_retained_ptr(ptr: *mut T) -> Id<T,O>{
        assert!(!ptr.is_null(),"Attempted to construct an Id from a null pointer");
        Id::new(StrongPtr::new(ptr as *mut Object))
    }
}

impl<T> Id<T, Owned> where T: Message{
    pub fn share(self) -> ShareId<T> {
        let Id{ptr,..} = self;
        unsafe {Id::new(ptr)}
    }
}

impl<T> Clone for Id<T, Shared> where T: Message {
    fn clone(&self) -> ShareId<T> {
        unsafe {
            Id::new(self.ptr.clone())
        }
    }
}

unsafe impl<T, O> Sync for Id<T, O> where T:Sync{}
unsafe impl<T> Send for Id<T,Owned> where T: Send{}
unsafe impl<T> Send for Id<T,Shared> where T:Sync{}

impl<T, O> Deref for Id<T, O> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {&*(*self.ptr as *mut T)}
    }
}

impl<T> DerefMut for Id<T, Owned> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *(*self.ptr as *mut T)}
    }
}

impl<T, O> PartialEq for Id<T, O> where T:PartialEq {
    fn eq(&self, other: &Id<T,O>) -> bool {
        self.deref() == other.deref()
    }
    fn ne(&self, other: &Id<T, O>) -> bool {
        self.deref() != other.deref()
    }
}

impl<T,O> Eq for Id<T,O> where T:Eq {}

impl<T,O> hash::Hash for Id<T,O> where T:hash::Hash {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.deref().hash(state)
    }
}

impl<T,O> fmt::Debug for Id<T,O> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T,O> fmt::Pointer for Id<T,O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

pub type ShareId<T> = Id<T, Shared>;
pub struct WeakId<T>{
    ptr: WeakPtr,
    item: PhantomData<T>
}

impl<T> WeakId<T> where T:Message{
    pub fn new(obj: &ShareId<T>) -> WeakId<T>{
        WeakId{ptr:obj.ptr.weak(),item:PhantomData}
    }

    pub fn load(&self) -> Option<ShareId<T>>{
        let obj = self.ptr.load();
        if obj.is_null(){
            None
        }else {
            Some(unsafe {Id::new(obj)})
        }
    }
}

unsafe impl<T> Sync for WeakId<T> where T:Sync{}
unsafe impl<T> Send for WeakId<T> where T:Sync{}


