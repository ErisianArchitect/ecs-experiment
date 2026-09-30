
use ::core::{
    ptr::{
        NonNull,
    },
    marker::{
        PhantomData,
    },
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawPtr<'a, T: 'a + ?Sized, Marker = ()> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _phantom: PhantomData<(fn(&'a T), Marker)>,
}

impl<'a, T: 'a + ?Sized> RawPtr<'a, T> {
    #[must_use]
    #[inline(always)]
    pub const unsafe fn new_unchecked(raw: *mut T) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(raw) },
            _phantom: PhantomData,
        }
    }
    
    #[must_use]
    #[inline]
    pub const fn new(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            return None;
        }
        Some(unsafe { Self::new_unchecked(raw) })
    }

    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.ptr.as_ptr()
    }
}

pub struct Res<T: Sized> {
    pub(crate) ptr: NonNull<T>,
}

#[derive(Debug, Clone, Copy)]
pub struct ResMut<'a, T: 'static + Sized> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _marker: PhantomData<(&'a mut T,)>,
}

impl<'a, T: 'static + Sized> std::ops::Deref for ResMut<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, 'b, T: 'static + Sized> std::ops::DerefMut for ResMut<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

struct Archetype<F, Marker>
where
    F: 'static,
    // Marker,
    Self: System,
{
    callback: F,
    _marker: PhantomData<(Marker,)>,
}

trait System
where
    Self: 'static,
 {
    fn run(&mut self);
}

trait SameType<T>: Sized {}

impl<T> SameType<T> for T {}

struct World;

trait SystemParam: Sized {
    type Resolved<'a>;
    fn resolve<'w>(world: &'w World) -> Self::Resolved<'w>;
}

impl<'a, T> SystemParam for ResMut<'a, T> {
    type Resolved<'b> = ResMut<'b, T>;
    fn resolve<'w>(_world: &'w World) -> Self::Resolved<'w> {
        todo!()
    }
}

impl<F> System for Archetype<F, ()>
where
    F: Fn(),
{
    fn run(&mut self) {}
}

impl<F, T0> System for Archetype<F, (T0,)>
where
    T0: SystemParam + 'static,
    F: Fn(T0),
    F: for<'t0> Fn(T0::Resolved<'t0>),
{
    fn run(&mut self) {}
}

impl<F, T0, T1> System for Archetype<F, (T0, T1)>
where
    T0: SystemParam + 'static,
    T1: SystemParam + 'static,
    F: Fn(T0, T1),
    F: for<'t0, 't1> Fn(T0::Resolved<'t0>, T1::Resolved<'t1>),
{
    fn run(&mut self) {}
}

pub struct SystemMarker<Marker> {
    _marker: PhantomData<(fn(Marker),)>,
}

trait IntoSystem<Marker>: 'static
where
    Self::System: System,
{
    type System;
    
    fn into_system(self) -> Self::System;
}

impl<F, Args> IntoSystem<SystemMarker<(F, Args)>> for F
where
    Archetype<F, Args>: System,
{
    type System = Archetype<F, Args>;
    
    fn into_system(self) -> Self::System {
        Archetype {
            callback: self,
            _marker: PhantomData,
        }
    }
}

fn to_system<Marker, F>(_f: F)
where
    F: 'static,
    F: IntoSystem<Marker>,
{
    todo!()
}

struct Outbox<'a> {
    a: &'a u64,
}



fn foo() {
    to_system(|| {});
    to_system(|_a: ResMut<u64>| {});
    to_system(|_a: ResMut<u64>, _b: ResMut<u64>| {});
}
