#![allow(non_upper_case_globals)]
#![allow(deprecated)]

use dioxus_core::prelude::IntoAttributeValue;
use dioxus_core::HasAttributes;
use dioxus_html_internal_macro::impl_extension_attributes;

use crate::AttributeDescription;

macro_rules! mod_methods {
    (
        @base
        $(#[$mod_attr:meta])*
        $mod:ident;
        $fn:ident;
        $fn_html_to_rsx:ident;
        $(
            $(#[$attr:meta])*
            $name:ident $(: $(no-$alias:ident)? $js_name:literal)? $(in $ns:literal)?;
        )+
    ) => {
        $(#[$mod_attr])*
        pub mod $mod {
            use super::*;
            $(
                mod_methods! {
                    @attr
                    $(#[$attr])*
                    $name $(: $(no-$alias)? $js_name)? $(in $ns)?;
                }
            )+
        }

        impl_extension_attributes![$mod { $($name,)* }];
    };

    (
        @attr
        $(#[$attr:meta])*
        $name:ident $(: no-alias $js_name:literal)? $(in $ns:literal)?;
    ) => {
        $(#[$attr])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, ignore
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($name), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        ///     div {
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($name), ": \"value\"")]
        ///     }
        ///     div {
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($name), ",")]
        ///     }
        /// };
        /// ```
        pub const $name: AttributeDescription = mod_methods! { $name $(: $js_name)? $(in $ns)?; };
    };

    (
        @attr
        $(#[$attr:meta])*
        $name:ident $(: $js_name:literal)? $(in $ns:literal)?;
    ) => {
        $(#[$attr])*
        ///
        /// ## Usage in rsx
        ///
        /// ```rust, ignore
        /// # use dioxus::prelude::*;
        #[doc = concat!("let ", stringify!($name), " = \"value\";")]
        ///
        /// rsx! {
        ///     // Attributes need to be under the element they modify
        ///     div {
        ///         // Attributes are followed by a colon and then the value of the attribute
        #[doc = concat!("        ", stringify!($name), ": \"value\"")]
        ///     }
        ///     div {
        ///         // Or you can use the shorthand syntax if you have a variable in scope that has the same name as the attribute
        #[doc = concat!("        ", stringify!($name), ",")]
        ///     }
        /// };
        /// ```
        $(
            #[doc(alias = $js_name)]
        )?
        pub const $name: AttributeDescription = mod_methods! { $name $(: $js_name)? $(in $ns)?; };
    };

    // Rename the incoming ident and apply a custom namespace
    ( $name:ident: $lit:literal in $ns:literal; ) => { ($lit, Some($ns), false) };

    // Custom namespace
    ( $name:ident in $ns:literal; ) => { (stringify!($name), Some($ns), false) };

    // Rename the incoming ident
    ( $name:ident: $lit:literal; ) => { ($lit, None, false ) };

    // Don't rename the incoming ident
    ( $name:ident; ) => { (stringify!($name), None, false) };
}

mod_methods! {
    @base

    global_attributes;
    map_global_attributes;
    map_html_global_attributes_to_rsx;

    /// The HTML class attribute is used to specify a class for an HTML element.
    ///
    /// ## Details
    /// Multiple HTML elements can share the same class.
    ///
    /// The class global attribute is a space-separated list of the case-sensitive classes of the element.
    /// Classes allow CSS and Javascript to select and access specific elements via the class selectors or
    /// functions like the DOM method document.getElementsByClassName.
    ///
    /// ## Multiple Classes
    ///
    /// If you include multiple classes in a single element dioxus will automatically join them with a space.
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// rsx! {
    ///     div {
    ///         class: "my-class",
    ///         class: "my-other-class"
    ///     }
    /// };
    /// ```
    ///
    /// ## Optional Classes
    ///
    /// You can include optional attributes with an unterminated if statement as the value of the attribute. This is very useful for conditionally applying css classes:
    ///
    /// ```rust
    /// # use dioxus::prelude::*;
    /// rsx! {
    ///     div {
    ///         class: if true {
    ///             "my-class"
    ///         },
    ///         class: if false {
    ///             "my-other-class"
    ///         }
    ///     }
    /// };
    /// ```
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class>
    class;

    /// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden>
    hidden;

    /// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/style>
    style;

    /// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate>
    translate;

    // This macro creates an explicit method call for each of the style attributes.
    //
    // The left token specifies the name of the attribute in the rsx! macro, and the right string literal specifies the
    // actual name of the attribute generated.
    //
    // This roughly follows the html spec

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
    align_content: "align-content" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    align_items: "align-items" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    align_self: "align-self" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/alignment-adjust>
    alignment_adjust: "alignment-adjust" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background>
    background in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-color>
    background_color: "background-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-image>
    background_image: "background-image" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin>
    background_origin: "background-origin" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-position>
    background_position: "background-position" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-repeat>
    background_repeat: "background-repeat" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-size>
    background_size: "background-size" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border>
    border in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-color>
    border_color: "border-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-style>
    border_style: "border-style" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    border_width: "border-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom>
    border_bottom: "border-bottom" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-color>
    border_bottom_color: "border-bottom-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-style>
    border_bottom_style: "border-bottom-style" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-width>
    border_bottom_width: "border-bottom-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left>
    border_left: "border-left" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-color>
    border_left_color: "border-left-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-style>
    border_left_style: "border-left-style" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-width>
    border_left_width: "border-left-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right>
    border_right: "border-right" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-color>
    border_right_color: "border-right-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-style>
    border_right_style: "border-right-style" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-width>
    border_right_width: "border-right-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top>
    border_top: "border-top" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-color>
    border_top_color: "border-top-color" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-style>
    border_top_style: "border-top-style" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-width>
    border_top_width: "border-top-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
    border_radius: "border-radius" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    border_bottom_left_radius: "border-bottom-left-radius" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    border_bottom_right_radius: "border-bottom-right-radius" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    border_top_left_radius: "border-top-left-radius" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    border_top_right_radius: "border-top-right-radius" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-spacing>
    border_spacing: "border-spacing" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
    bottom in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
    box_sizing: "box-sizing" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/color>
    color in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/contain>
    contain in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/content>
    content in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    cursor in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    display in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display-inside>
    display_inside: "display-inside" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display-outside>
    display_outside: "display-outside" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display-extras>
    display_extras: "display-extras" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display-box>
    display_box: "display-box" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex>
    flex in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>
    flex_basis: "flex-basis" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow>
    flex_grow: "flex-grow" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink>
    flex_shrink: "flex-shrink" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-flow>
    flex_flow: "flex-flow" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    flex_direction: "flex-direction" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap>
    flex_wrap: "flex-wrap" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font>
    font in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
    font_family: "font-family" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-size>
    font_size: "font-size" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    font_weight: "font-weight" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid>
    grid in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    grid_auto_flow: "grid-auto-flow" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    grid_auto_columns: "grid-auto-columns" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    grid_auto_rows: "grid-auto-rows" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template>
    grid_template: "grid-template" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas>
    grid_template_areas: "grid-template-areas" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
    grid_template_columns: "grid-template-columns" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
    grid_template_rows: "grid-template-rows" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-area>
    grid_area: "grid-area" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    grid_column: "grid-column" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-start>
    grid_column_start: "grid-column-start" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-end>
    grid_column_end: "grid-column-end" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    grid_row: "grid-row" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start>
    grid_row_start: "grid-row-start" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-end>
    grid_row_end: "grid-row-end" in "style";
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/height>
    height in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    justify_content: "justify-content" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
    justify_items: "justify-items" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    justify_self: "justify-self" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
    left in "style";
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>
    margin in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-bottom>
    margin_bottom: "margin-bottom" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-left>
    margin_left: "margin-left" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-right>
    margin_right: "margin-right" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-top>
    margin_top: "margin-top" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>
    max_height: "max-height" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-lines>
    max_lines: "max-lines" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>
    max_width: "max-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>
    min_height: "min-height" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>
    min_width: "min-width" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/opacity>
    opacity in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    overflow in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    overflow_x: "overflow-x" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    overflow_y: "overflow-y" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>
    padding in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>
    padding_bottom: "padding-bottom" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-left>
    padding_left: "padding-left" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>
    padding_right: "padding-right" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>
    padding_top: "padding-top" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events>
    pointer_events: "pointer-events" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    position in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
    right in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/size>
    size in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    text_align: "text-align" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-height>
    text_height: "text-height" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-wrap>
    text_wrap: "text-wrap" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
    top in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/visibility>
    visibility in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    white_space: "white-space" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/width>
    width in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
    word_break: "word-break" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-spacing>
    word_spacing: "word-spacing" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-wrap>
    word_wrap: "word-wrap" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/wrap-flow>
    wrap_flow: "wrap-flow" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/wrap-through>
    wrap_through: "wrap-through" in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>
    gap in "style";

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index>
    z_index: "z-index" in "style";
}
