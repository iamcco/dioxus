#![doc = include_str!("../../docs/event_handlers.md")]

use std::any::Any;
use std::sync::RwLock;

macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident $(: $js_name:literal)?
        )*
    ) => {
        $(
            $( #[$attr] )*
            /// <details open>
            /// <summary>General Event Handler Information</summary>
            ///
            #[doc = include_str!("../../docs/event_handlers.md")]
            ///
            /// </details>
            ///
            #[doc = include_str!("../../docs/common_event_handler_errors.md")]
            $(
                #[doc(alias = $js_name)]
            )?
            #[inline]
            pub fn $name<__Marker>(mut _f: impl ::dioxus_core::prelude::SuperInto<::dioxus_core::prelude::EventHandler<::dioxus_core::Event<$data>>, __Marker>) -> ::dioxus_core::Attribute {
                // super into will make a closure that is owned by the current owner (either the child component or the parent component).
                // We can't change that behavior in a minor version because it would cause issues with Components that accept event handlers.
                // Instead we run super into with an owner that is moved into the listener closure so it will be dropped when the closure is dropped.
                let owner = <::generational_box::UnsyncStorage as ::generational_box::AnyStorage>::owner();
                let event_handler = ::dioxus_core::prelude::with_owner(owner.clone(), || _f.super_into());
                ::dioxus_core::Attribute::new(
                    impl_event!(@name $name $($js_name)?),
                    ::dioxus_core::AttributeValue::listener(move |e: ::dioxus_core::Event<crate::PlatformEventData>| {
                        // Force the owner to be moved into the event handler
                        _ = &owner;
                        event_handler.call(e.map(|e| e.into()));
                    }),
                    None,
                    false,
                ).into()
            }

            #[doc(hidden)]
            $( #[$attr] )*
            pub mod $name {
                use super::*;

                // When expanding the macro, we use this version of the function if we see an inline closure to give better type inference
                $( #[$attr] )*
                pub fn call_with_explicit_closure<
                    __Marker,
                    Return: ::dioxus_core::SpawnIfAsync<__Marker> + 'static,
                >(
                    event_handler: impl FnMut(::dioxus_core::Event<$data>) -> Return + 'static,
                ) -> ::dioxus_core::Attribute {
                    #[allow(deprecated)]
                    super::$name(event_handler)
                }
            }
        )*
    };

    (@name $name:ident $js_name:literal) => {
        $js_name
    };
    (@name $name:ident) => {
        stringify!($name)
    };
}

static EVENT_CONVERTER: RwLock<Option<Box<dyn HtmlEventConverter>>> = RwLock::new(None);

#[inline]
pub fn set_event_converter(converter: Box<dyn HtmlEventConverter>) {
    *EVENT_CONVERTER.write().unwrap() = Some(converter);
}

#[inline]
pub(crate) fn with_event_converter<F, R>(f: F) -> R
where
    F: FnOnce(&dyn HtmlEventConverter) -> R,
{
    let converter = EVENT_CONVERTER.read().unwrap();
    f(converter.as_ref().unwrap().as_ref())
}

/// A platform specific event.
pub struct PlatformEventData {
    event: Box<dyn Any>,
}

impl PlatformEventData {
    pub fn new(event: Box<dyn Any>) -> Self {
        Self { event }
    }

    pub fn inner(&self) -> &Box<dyn Any> {
        &self.event
    }

    pub fn downcast<T: 'static>(&self) -> Option<&T> {
        self.event.downcast_ref::<T>()
    }

    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.event.downcast_mut::<T>()
    }

    pub fn into_inner<T: 'static>(self) -> Option<T> {
        self.event.downcast::<T>().ok().map(|e| *e)
    }
}

/// A converter between a platform specific event and a general event. All code in a renderer that has a large binary size should be placed in this trait. Each of these functions should be snipped in high levels of optimization.
pub trait HtmlEventConverter: Send + Sync {
    /// Convert a general event to a keyboard data event
    fn convert_keyboard_data(&self, event: &PlatformEventData) -> KeyboardData;
    /// Convert a general event to a mouse data event
    fn convert_mouse_data(&self, event: &PlatformEventData) -> MouseData;
    /// Convert a general event to a pointer data event
    fn convert_pointer_data(&self, event: &PlatformEventData) -> PointerData;
}

impl From<&PlatformEventData> for KeyboardData {
    fn from(val: &PlatformEventData) -> Self {
        with_event_converter(|c| c.convert_keyboard_data(val))
    }
}

impl From<&PlatformEventData> for MouseData {
    fn from(val: &PlatformEventData) -> Self {
        with_event_converter(|c| c.convert_mouse_data(val))
    }
}

impl From<&PlatformEventData> for PointerData {
    fn from(val: &PlatformEventData) -> Self {
        with_event_converter(|c| c.convert_pointer_data(val))
    }
}

mod keyboard;
mod mouse;
mod pointer;

pub use keyboard::*;
pub use mouse::*;
pub use pointer::*;
