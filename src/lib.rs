use std::ops::Not;

use wasm_bindgen::prelude::*;
use web_sys::{console, KeyboardEvent};
use substring::Substring;

#[wasm_bindgen]
pub struct Scanner {
    keyboard_evt: Closure<dyn FnMut(KeyboardEvent)>
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum EventEnum {
  Data = "data",
}

#[wasm_bindgen(typescript_custom_section)]
const Event_Enum: &'static str = r#"
enum EventEnum {
    Data = "data"
}
"#;

static mut EVENT: Option<js_sys::Function> = None;
static mut SCAN: String = String::new();

#[wasm_bindgen]
impl Scanner {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Scanner {
    console_error_panic_hook::set_once();

    Scanner { 
      keyboard_evt: Closure::<dyn FnMut(KeyboardEvent)>::new(move | evt: KeyboardEvent | unsafe {
        if evt.char_code() == 123 { SCAN = evt.key(); } //Start Logging
        else if evt.char_code() == 13 { Scanner::emit_event(SCAN.clone()); SCAN = String::new(); } //End Logging
        else if String::is_empty(&SCAN).not() && SCAN.substring(0, 1) == "{" { console::log_1(&evt.key().into()); SCAN.push_str(&evt.key()); } //Prepend Scan
      })
    }
  }

  #[wasm_bindgen(skip)]
  pub unsafe fn emit_event(data: String) {
    let scan = data.clone();
    let f = EVENT.clone().unwrap_or_default();
    f.call1(&JsValue::null(), &JsValue::from(scan));
  }

  #[wasm_bindgen]
  pub unsafe fn start(&self, f: &js_sys::Function) -> Result<(), JsValue> {
    //Check If Already Running
    if EVENT.is_some() { return Err(JsValue::from("There is already a Scanner instance running.")); }

    //Set Callback Function
    if f.is_undefined() { return Err(JsValue::from("No return Function has been passed for a successful scan.")); }
    else { EVENT = Some(f.clone()); }

    //Register Keyboard Event Listener
    let win = web_sys::window().unwrap();
    win.add_event_listener_with_callback( &"keypress", self.keyboard_evt.as_ref().unchecked_ref())?;
    Ok(())
  }

  #[wasm_bindgen]
  pub unsafe fn stop(&self) -> Result<(), JsValue>{
    EVENT = None;

    let win = web_sys::window().unwrap();
    win.remove_event_listener_with_callback(&"keypress", self.keyboard_evt.as_ref().unchecked_ref())?;
    Ok(())
  }




}