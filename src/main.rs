use core::mem;
use windows::Win32::Foundation;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, VK_LWIN, VK_TAB, KEYEVENTF_KEYUP
};
use std::thread;
use std::time::Duration;
use windows::Win32::UI::WindowsAndMessaging;

fn main() {
    let duration = Duration::from_millis(400);
    let longduration = Duration::from_millis(2000);
    unsafe {

        // Press Windows key
        let mut p = Foundation::POINT { x: 0, y: 0 };
        let d = Foundation::POINT { x: 0, y: 0 };
        let wk = KEYBDINPUT {
            wVk: VK_LWIN,
            wScan: 0,
            dwFlags: KEYBD_EVENT_FLAGS(0),
            time: 0,
            dwExtraInfo: 0,
        };
        let mut input_wk: INPUT = mem::zeroed();
        input_wk.r#type = INPUT_KEYBOARD;
        input_wk.Anonymous.ki = wk;
        
        // Press Tab Key
        let tk = KEYBDINPUT {
            wVk: VK_TAB,
            wScan: 0,
            dwFlags: KEYBD_EVENT_FLAGS(0),
            time: 0,
            dwExtraInfo: 0,
        };
        let mut input_tk: INPUT = mem::zeroed();
        input_tk.r#type = INPUT_KEYBOARD;
        input_tk.Anonymous.ki = tk;

        // Release Windows Key
        let r_wk = KEYBDINPUT {
            wVk: VK_LWIN,
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };
        let mut r_input_wk: INPUT = mem::zeroed();
        r_input_wk.r#type = INPUT_KEYBOARD;
        r_input_wk.Anonymous.ki = r_wk;

        // Release Tab Key
        let r_tk = KEYBDINPUT {
            wVk: VK_TAB,
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };
        let mut r_input_tk: INPUT = mem::zeroed();
        r_input_tk.r#type = INPUT_KEYBOARD;
        r_input_tk.Anonymous.ki = r_tk;
        
        loop {
            WindowsAndMessaging::GetCursorPos(&mut p).expect("cant do it");
            thread::sleep(duration);
            if p == d {
                SendInput(&[input_wk, input_tk], mem::size_of::<INPUT>() as i32);
                SendInput(&[r_input_wk, r_input_tk], mem::size_of::<INPUT>() as i32);
                thread::sleep(longduration);
            }
        }
    }
}
