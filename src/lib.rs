use std::ops::Not;
use wasm_bindgen::prelude::*;
use web_sys::{console, KeyboardEvent};
use substring::Substring;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum EventEnum {
  Data = "data",
}

#[derive(Default)]
struct Scanner {
  keyboard_evt: Option<JsValue>,
  pub scan: String,
  pub event: Option<js_sys::Function>
}

static mut SCANNER: Scanner = Scanner{
  keyboard_evt: None,
  scan: String::new(),
  event: None
};

#[wasm_bindgen(skip)]
pub unsafe fn emit_event(data: String) {
  let scan = data.clone();
  let f = SCANNER.event.clone().unwrap_or_default();
  let _ = f.call1(&JsValue::null(), &JsValue::from(scan));
}

#[wasm_bindgen]
pub unsafe fn start(f: &js_sys::Function) -> Result<(), JsValue> {
  //Check If Already Running
  if SCANNER.event.is_some() { return Err(JsValue::from("There is already a Scanner instance running.")); }

  //Set Callback Function
  if f.is_undefined() { return Err(JsValue::from("No return Function has been passed for a successful scan.")); }
  else { SCANNER.event = Some(f.clone()); }

  let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move | evt: KeyboardEvent | unsafe {
    if evt.char_code() == 123 { SCANNER.scan = evt.key(); } //Start Logging
    else if evt.char_code() == 13 { emit_event(SCANNER.scan.clone()); SCANNER.scan = String::new(); } //End Logging
    else if String::is_empty(&SCANNER.scan).not() && SCANNER.scan.substring(0, 1) == "{" { console::log_1(&evt.key().into()); SCANNER.scan.push_str(&evt.key()); } //Prepend Scan
  });

  let win = web_sys::window().unwrap();
  let evt = win.add_event_listener_with_callback( &"keypress", closure.as_ref().unchecked_ref());

  let r = closure.as_ref();
  SCANNER.keyboard_evt = Some(closure.as_ref().into());
  closure.forget();
  Ok(())
}

#[wasm_bindgen]
pub unsafe fn stop() -> Result<(), JsValue>{
  SCANNER.event = None;

  let win = web_sys::window().unwrap();

  if SCANNER.keyboard_evt.is_some() {
    win.remove_event_listener_with_callback(&"keypress", SCANNER.keyboard_evt.clone().unwrap().unchecked_ref())?; 
    return Ok(());
  } else {
    return Err(JsValue::from("Unable to stop as Event has not been started."));
  }
}