use std::ops::Not;
use wasm_bindgen::prelude::*;
use web_sys::{console, KeyboardEvent};
use substring::Substring;


#[derive(Default)]
struct Scanner {
  keyboard_evt: Option<JsValue>,
  pub scan: String,
  pub event: ScannerEvents
}

#[derive(Default)]
struct ScannerEvents {
  data: Option<js_sys::Function>,
  log: Option<js_sys::Function>
}

static mut SCANNER: Scanner = Scanner{
  keyboard_evt: None,
  scan: String::new(),
  event: ScannerEvents { 
    data: None,
    log: None
  }
};

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum EVENTS {
  DATA = "data",
  LOG = "log"
}

pub unsafe fn emit_event(evt: EVENTS, data: String) {
  match evt {
    EVENTS::DATA => {
      let scan = data.clone();
      let f = SCANNER.event.data.clone().unwrap_or_default();
      let _ = f.call1(&JsValue::null(), &JsValue::from(scan));
    },
    EVENTS::LOG => {
      let scan = data.clone();
      let f = SCANNER.event.log.clone().unwrap_or_default();
      let _ = f.call1(&JsValue::null(), &JsValue::from(scan));
    },
    EVENTS::__Nonexhaustive => todo!(),
  }
}

#[wasm_bindgen]
pub unsafe fn start(f: &js_sys::Function) -> Result<(), JsValue> {
  //Check If Already Running
  if SCANNER.event.data.is_some() { return Err(JsValue::from("There is already a Scanner instance running.")); }

  //Set Callback Function
  if f.is_undefined() { return Err(JsValue::from("No return Function has been passed for a successful scan.")); }
  else { SCANNER.event.data = Some(f.clone()); }

  let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move | evt: KeyboardEvent | unsafe {
    if evt.char_code() == 123 { //Start Logging
      SCANNER.scan = evt.key(); 
      emit_event(EVENTS::LOG, SCANNER.scan.clone());
    } 
    else if evt.char_code() == 13 { //End Logging
      emit_event(EVENTS::LOG, SCANNER.scan.clone());
      emit_event(EVENTS::DATA, SCANNER.scan.clone()); 
      SCANNER.scan = String::new(); 
    } 
    else if String::is_empty(&SCANNER.scan).not() && SCANNER.scan.substring(0, 1) == "{" { //Prepend Scan
      SCANNER.scan.push_str(&evt.key()); 
      emit_event(EVENTS::LOG, SCANNER.scan.clone());
    } 
  });

  let win = web_sys::window().unwrap();
  let _evt = win.add_event_listener_with_callback( &"keypress", closure.as_ref().unchecked_ref());

  let _r = closure.as_ref();
  SCANNER.keyboard_evt = Some(closure.as_ref().into());
  closure.forget();
  Ok(())
}

#[wasm_bindgen]
pub unsafe fn stop() -> Result<(), JsValue>{
  SCANNER.event.data = None;

  let win = web_sys::window().unwrap();

  if SCANNER.keyboard_evt.is_some() {
    win.remove_event_listener_with_callback(&"keypress", SCANNER.keyboard_evt.clone().unwrap().unchecked_ref())?; 
    return Ok(());
  } else {
    return Err(JsValue::from("Unable to stop as Event has not been started."));
  }
}

#[wasm_bindgen(js_name="addListener")]
pub unsafe fn add_listener(evt: EVENTS, f: &js_sys::Function) -> Result<(), JsValue>{
  match evt {
    EVENTS::DATA => {
      //Check is Scanner is running
      if SCANNER.event.data.is_some() { //Replace Existing Return Function
        SCANNER.event.data = Some(f.clone())
      }
    },
    EVENTS::LOG => SCANNER.event.log = Some(f.clone()),
    EVENTS::__Nonexhaustive => todo!()
  }

  Ok(())
}

#[wasm_bindgen(js_name="removeListener")]
pub unsafe fn remove_listener(evt: EVENTS) -> Result<(), JsValue>{
  if evt == EVENTS::DATA && SCANNER.event.data.is_some() { let _ =stop(); } //Check is Scanner is running
  else if evt == EVENTS::LOG { SCANNER.event.log = None; }
  
  Ok(())
}

#[wasm_bindgen(typescript_custom_section)]
const TS_EVENTS: &'static str = r#"
export enum Events {
    DATA = "data",
    LOG = "log"
}
"#;