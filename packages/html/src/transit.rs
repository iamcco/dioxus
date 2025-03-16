use std::{any::Any, rc::Rc};

use crate::events::*;
use dioxus_core::ElementId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "serialize")]
#[derive(Serialize, Debug, PartialEq)]
pub struct HtmlEvent {
    pub element: ElementId,
    pub name: String,
    pub bubbles: bool,
    pub data: EventData,
}

#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for HtmlEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug, Clone)]
        struct Inner {
            element: ElementId,
            name: String,
            bubbles: bool,
            data: serde_json::Value,
        }

        let Inner {
            element,
            name,
            bubbles,
            data,
        } = Inner::deserialize(deserializer)?;

        // in debug mode let's try and be helpful as to why the deserialization failed
        let data = deserialize_raw(&name, &data).map_err(|e| {
            serde::de::Error::custom(format!(
                "Failed to deserialize event data for event {}:  {:#?}\n'{:#?}'",
                name, e, data,
            ))
        })?;

        Ok(HtmlEvent {
            data,
            element,
            bubbles,
            name,
        })
    }
}

#[cfg(feature = "serialize")]
fn deserialize_raw(name: &str, data: &serde_json::Value) -> Result<EventData, serde_json::Error> {
    use EventData::*;

    // a little macro-esque thing to make the code below more readable
    #[inline]
    fn de<'de, F>(f: &'de serde_json::Value) -> Result<F, serde_json::Error>
    where
        F: Deserialize<'de>,
    {
        F::deserialize(f)
    }

    let data = match name {
        // Mouse
        "click" | "contextmenu" | "dblclick" | "doubleclick" | "mousedown" | "mouseenter"
        | "mouseleave" | "mousemove" | "mouseout" | "mouseover" | "mouseup" => Mouse(de(data)?),

        // Keyboard
        "keydown" | "keypress" | "keyup" => Keyboard(de(data)?),

        // Pointer
        "pointerlockchange" | "pointerlockerror" | "pointerdown" | "pointermove" | "pointerup"
        | "pointerover" | "pointerout" | "pointerenter" | "pointerleave" | "gotpointercapture"
        | "lostpointercapture" => Pointer(de(data)?),

        other => {
            return Err(serde::de::Error::custom(format!(
                "Unknown event type: {other}"
            )))
        }
    };

    Ok(data)
}

#[cfg(feature = "serialize")]
impl HtmlEvent {
    pub fn bubbles(&self) -> bool {
        self.bubbles
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum EventData {
    Mouse(SerializedMouseData),
    Keyboard(SerializedKeyboardData),
    Pointer(SerializedPointerData),
}

impl EventData {
    pub fn into_any(self) -> Rc<dyn Any> {
        match self {
            EventData::Mouse(data) => {
                Rc::new(PlatformEventData::new(Box::new(data))) as Rc<dyn Any>
            }
            EventData::Keyboard(data) => {
                Rc::new(PlatformEventData::new(Box::new(data))) as Rc<dyn Any>
            }
            EventData::Pointer(data) => {
                Rc::new(PlatformEventData::new(Box::new(data))) as Rc<dyn Any>
            }
        }
    }
}

#[test]
fn test_back_and_forth() {
    let data = HtmlEvent {
        element: ElementId(0),
        data: EventData::Mouse(SerializedMouseData::default()),
        name: "click".to_string(),
        bubbles: true,
    };

    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    let o = r#"
{
  "element": 0,
  "name": "click",
  "bubbles": true,
  "data": {
    "alt_key": false,
    "button": 0,
    "buttons": 0,
    "client_x": 0,
    "client_y": 0,
    "ctrl_key": false,
    "meta_key": false,
    "offset_x": 0,
    "offset_y": 0,
    "page_x": 0,
    "page_y": 0,
    "screen_x": 0,
    "screen_y": 0,
    "shift_key": false
  }
}
    "#;

    let p: HtmlEvent = serde_json::from_str(o).unwrap();

    assert_eq!(data, p);
}

/// A trait for converting from a serialized event to a concrete event type.
pub struct SerializedHtmlEventConverter;

impl HtmlEventConverter for SerializedHtmlEventConverter {
    fn convert_keyboard_data(&self, event: &PlatformEventData) -> KeyboardData {
        event
            .downcast::<SerializedKeyboardData>()
            .cloned()
            .unwrap()
            .into()
    }

    fn convert_mouse_data(&self, event: &PlatformEventData) -> MouseData {
        event
            .downcast::<SerializedMouseData>()
            .cloned()
            .unwrap()
            .into()
    }
    fn convert_pointer_data(&self, event: &PlatformEventData) -> PointerData {
        event
            .downcast::<SerializedPointerData>()
            .cloned()
            .unwrap()
            .into()
    }
}
