#![allow(non_upper_case_globals)]

use dioxus_core::prelude::IntoAttributeValue;
use dioxus_core::HasAttributes;
use dioxus_html_internal_macro::impl_extension_attributes;

pub type AttributeDescription = (&'static str, Option<&'static str>, bool);

macro_rules! impl_attribute {
    (
        $element:ident {
            $(#[$attr_method:meta])*
            $fil:ident: $vil:ident (DEFAULT),
        }
    ) => {
        $(#[$attr_method])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($fil), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($fil), ": \"value\"")]
        ///     }
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($fil), ",")]
        ///     }
        /// };
        /// ```
        pub const $fil: AttributeDescription = (stringify!($fil), None, false);
    };

    (
        $element:ident {
            $(#[$attr_method:meta])*
            $fil:ident: $vil:ident ($name:literal),
        }
    ) => {
        $(#[$attr_method])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($fil), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($fil), ": \"value\"")]
        ///     }
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($fil), ",")]
        ///     }
        /// };
        /// ```
        pub const $fil: AttributeDescription = ($name, None, false);
    };

    (
        $element:ident {
            $(#[$attr_method:meta])*
            $fil:ident: $vil:ident (volatile),
        }
    ) => {
        $(#[$attr_method])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($fil), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($fil), ": \"value\"")]
        ///     }
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($fil), ",")]
        ///     }
        /// };
        /// ```
        pub const $fil: AttributeDescription = (stringify!($fil), None, true);
    };

    (
        $element:ident {
            $(#[$attr_method:meta])*
            $fil:ident: $vil:ident (in $ns:literal),
        }
    ) => {
        $(#[$attr_method])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($fil), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($fil), ": \"value\"")]
        ///     }
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($fil), ",")]
        ///     }
        /// };
        /// ```
        pub const $fil: AttributeDescription = (stringify!($fil), Some($ns), false)
    };

    (
        $element:ident {
            $(#[$attr_method:meta])*
            $fil:ident: $vil:ident (in $ns:literal : volatile),
        }
    ) => {
        $(#[$attr_method])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($fil), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($fil), ": \"value\"")]
        ///     }
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($fil), ",")]
        ///     }
        /// };
        /// ```
        pub const $fil: AttributeDescription = (stringify!($fil), Some($ns), true)
    };
}

macro_rules! impl_element {
    (
        $(#[$attr:meta])*
        $name:ident None {
            $(
                $(#[$attr_method:meta])*
                $fil:ident: $vil:ident $extra:tt,
            )*
        }
    ) => {
        #[allow(non_camel_case_types)]
        $(#[$attr])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        /// # let attributes = vec![];
        /// # fn ChildComponent() -> Element { unimplemented!() }
        /// # let raw_expression: Element = rsx! {};
        /// rsx! {
        ///     // Elements are followed by braces that surround any attributes and children for that element
        #[doc = concat!("    ", stringify!($name), " {")]
        ///         // Add any attributes first
        ///         class: "my-class",
        ///         "custom-attribute-name": "value",
        ///         // Then add any attributes you are spreading into this element
        ///         ..attributes,
        ///         // Then add any children elements, components, text nodes, or raw expressions
        ///         div {}
        ///         ChildComponent {}
        ///         "child text"
        ///         {raw_expression}
        ///     }
        /// };
        /// ```
        pub mod $name {
            #[allow(unused)]
            use super::*;
            pub use crate::attribute_groups::global_attributes::*;

            pub const TAG_NAME: &'static str = stringify!($name);
            pub const NAME_SPACE: Option<&'static str> = None;

            $(
                impl_attribute!(
                    $name {
                        $(#[$attr_method])*
                        $fil: $vil ($extra),
                    }
                );
            )*
        }
    };

    (
        $(#[$attr:meta])*
        $name:ident $namespace:literal {
            $(
                $(#[$attr_method:meta])*
                $fil:ident: $vil:ident $extra:tt,
            )*
        }
    ) => {
        $(#[$attr])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        /// # let attributes = vec![];
        /// # fn ChildComponent() -> Element { unimplemented!() }
        /// # let raw_expression: Element = rsx! {};
        /// rsx! {
        ///     // Elements are followed by braces that surround any attributes and children for that element
        #[doc = concat!("    ", stringify!($name), " {")]
        ///         // Add any attributes first
        ///         color: "red",
        ///         "custom-attribute-name": "value",
        ///         // Then add any attributes you are spreading into this element
        ///         ..attributes,
        ///         // Then add any children elements, components, text nodes, or raw expressions
        ///         circle { cx: "10", cy: "10", r: "2", fill: "red" }
        ///         ChildComponent {}
        ///         "child text"
        ///         {raw_expression}
        ///     }
        /// };
        /// ```
        pub mod $name {
            #[allow(unused)]
            use super::*;

            pub const TAG_NAME: &'static str = stringify!($name);
            pub const NAME_SPACE: Option<&'static str> = Some($namespace);

            $(
                impl_attribute!(
                    $name {
                        $(#[$attr_method])*
                        $fil: $vil ($extra),
                    }
                );
            )*
        }
    };

    (
        $(#[$attr:meta])*
        $element:ident [$name:literal, $namespace:tt] {
            $(
                $(#[$attr_method:meta])*
                $fil:ident: $vil:ident $extra:tt,
            )*
        }
    ) => {
        #[allow(non_camel_case_types)]
        $(#[$attr])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, no_run
        /// # use dioxus::prelude::*;
        /// # let attributes = vec![];
        /// # fn ChildComponent() -> Element { unimplemented!() }
        /// # let raw_expression: Element = rsx! {};
        /// rsx! {
        ///     // Elements are followed by braces that surround any attributes and children for that element
        #[doc = concat!("    ", stringify!($element), " {")]
        ///         // Add any attributes first
        ///         color: "red",
        ///         "custom-attribute-name": "value",
        ///         // Then add any attributes you are spreading into this element
        ///         ..attributes,
        ///         // Then add any children elements, components, text nodes, or raw expressions
        ///         circle { cx: "10", cy: "10", r: "2", fill: "red" }
        ///         ChildComponent {}
        ///         "child text"
        ///         {raw_expression}
        ///     }
        /// };
        /// ```
        pub mod $element {
            #[allow(unused)]
            use super::*;

            pub const TAG_NAME: &'static str = $name;
            pub const NAME_SPACE: Option<&'static str> = Some($namespace);

            $(
                impl_attribute!(
                    $element {
                        $(#[$attr_method])*
                        $fil: $vil ($extra),
                    }
                );
            )*
        }
    }
}

macro_rules! builder_constructors {
    (
        $(
            $(#[$attr:meta])*
            $name:ident $namespace:tt {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident: $vil:ident $extra:tt,
                )*
            };
         )*
        ) => {

        $(
            impl_element!(
                $(#[$attr])*
                $name $namespace {
                    $(
                        $(#[$attr_method])*
                        $fil: $vil $extra,
                    )*
                }
            );
        )*

        /// This module contains helpers for rust analyzer autocompletion
        #[doc(hidden)]
        pub mod completions {
            /// This helper tells rust analyzer that it should autocomplete the element name with braces.
            #[allow(non_camel_case_types)]
            pub enum CompleteWithBraces {
                $(
                    $(#[$attr])*
                    ///
                    /// ## Usage in rsx
                    ///
                    /// ```rust, no_run
                    /// # use dioxus::prelude::*;
                    /// # let attributes = vec![];
                    /// # fn ChildComponent() -> Element { unimplemented!() }
                    /// # let raw_expression: Element = rsx! {};
                    /// rsx! {
                    ///     // Elements are followed by braces that surround any attributes and children for that element
                    #[doc = concat!("    ", stringify!($name), " {")]
                    ///         // Add any attributes first
                    ///         class: "my-class",
                    ///         "custom-attribute-name": "value",
                    ///         // Then add any attributes you are spreading into this element
                    ///         ..attributes,
                    ///         // Then add any children elements, components, text nodes, or raw expressions
                    ///         div {}
                    ///         ChildComponent {}
                    ///         "child text"
                    ///         {raw_expression}
                    ///     }
                    /// };
                    /// ```
                    $name {}
                ),*
            }
        }

        pub(crate) mod extensions {
            use super::*;
            $(
                impl_extension_attributes![$name { $($fil,)* }];
            )*
        }
    };
}

// Organized in the same order as
// https://developer.mozilla.org/en-US/docs/Web/HTML/Element
//
// Does not include obsolete elements.
//
// This namespace represents a collection of modern HTML-5 compatible elements.
//
// This list does not include obsolete, deprecated, experimental, or poorly supported elements.
builder_constructors! {
    /// Build a
    /// [`<div>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div)
    /// element.
    ///
    /// Part of the HTML namespace. Only works in HTML-compatible renderers
    ///
    /// ## Definition and Usage
    /// - The `<div>` tag defines a division or a section in an HTML document.
    /// - The `<div>` tag is used as a container for HTML elements - which is then styled with CSS or manipulated with  JavaScript.
    /// - The `<div>` tag is easily styled by using the class or id attribute.
    /// - Any sort of content can be put inside the `<div>` tag!
    ///
    /// Note: By default, browsers always place a line break before and after the <div> element.
    ///
    /// ## References:
    /// - <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/div>
    /// - <https://www.w3schools.com/tags/tag_div.asp>
    div None {};

    /// Build a
    /// [`<img>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    /// element.
    img None {
        alt: String DEFAULT,
        crossorigin: CrossOrigin DEFAULT,
        decoding: ImageDecoding DEFAULT,
        height: usize DEFAULT,
        ismap: Bool DEFAULT,
        loading: String DEFAULT,
        src: Uri DEFAULT,
        srcset: String DEFAULT, // FIXME this is much more complicated
        usemap: String DEFAULT, // FIXME should be a fragment starting with '#'
        width: usize DEFAULT,
        referrerpolicy: String DEFAULT,
        // sizes: SpacedList<String>, // FIXME it's not really just a string
    };
}
