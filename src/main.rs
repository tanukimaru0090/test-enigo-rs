use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use std::ffi::CString;
use std::os::raw::c_void;
use std::thread;
use std::time::Duration;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{FindWindowA, GetForegroundWindow, GetWindowRect};

enum IrisuButton {
    Decision, //決定ボタン
    Return,   //戻るボタン
    End,      //終了ボタン
}

fn press_button(
    enigo: &mut Enigo,
    button_type: IrisuButton,
    start_sleep_time: u64,
    down_sleep_time: u64,
) -> Result<(), String> {
    std::thread::sleep(Duration::from_secs(start_sleep_time));
    match button_type {
        IrisuButton::Decision => {
            enigo.mouse_down(MouseButton::Left);
            std::thread::sleep(Duration::from_secs(down_sleep_time));
            enigo.mouse_up(MouseButton::Left);
        }
        IrisuButton::End => {
            enigo.key_down(Key::Escape);
            std::thread::sleep(Duration::from_secs(down_sleep_time));
            enigo.key_up(Key::Escape);
        }

        IrisuButton::Return => {
            enigo.mouse_down(MouseButton::Right);
            std::thread::sleep(Duration::from_secs(down_sleep_time));
            enigo.mouse_up(MouseButton::Right);
        }
    }
    Ok(())
}

fn play_app(
    enigo: &mut Enigo,
    width: i32,
    height: i32,
    center_x: i32,
    center_y: i32,
) -> Result<(), String> {
    // ここにゲームのロジックを書く
    // タイトル項目の座標
    let mut title_menu: [(i32, i32); 3] = [(0, 0); 3];

    title_menu[0].0 = center_x;
    title_menu[0].1 = center_y + 20;

    title_menu[1].0 = center_x + 100;
    title_menu[1].1 = center_y + 100;

    title_menu[2].0 = center_x + 150;
    title_menu[2].1 = center_y + 150;
    let mut title_menu_on = false;

    // タイトル画面項目のシミュレーションテスト
    /*
    // 項目1
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[0].0,title_menu[0].1);
    // 項目2
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[1].0,title_menu[1].1);
    // 項目3
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[2].0,title_menu[2].1);


    // 項目3
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[2].0,title_menu[2].1);
    // 項目2
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[1].0,title_menu[1].1);
    // 項目1
    std::thread::sleep(Duration::from_secs(1));
    enigo.mouse_move_to(title_menu[0].0,title_menu[0].1);

    */

    // ウィンドウハンドルが無効になったかどうかチェックする
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_null() {
        // ウィンドウハンドルがNULLならエラーを返して終了する
        return Err("ウィンドウハンドルが無効になりました".to_string());
    } else {
        // 項目を変更
        std::thread::sleep(Duration::from_secs(1));
        
        // あるばむに変更
        enigo.mouse_move_to(title_menu[1].0, title_menu[1].1);
        // マウス左を押す
        //std::thread::sleep(Duration::from_secs(2));
        //enigo.mouse_down(MouseButton::Left);
        //std::thread::sleep(Duration::from_secs(1));
        //enigo.mouse_up(MouseButton::Left);
        press_button(&mut *enigo, IrisuButton::Decision, 3, 1)?;
        press_button(&mut *enigo, IrisuButton::Return, 3, 1)?;

        // すこあに変更
        enigo.mouse_move_to(title_menu[2].0, title_menu[2].1);
        press_button(&mut *enigo, IrisuButton::Decision, 3, 1)?;
        press_button(&mut *enigo, IrisuButton::Return, 3, 1)?;
        // すたーとに変更
        enigo.mouse_move_to(title_menu[0].0, title_menu[0].1);
        press_button(&mut *enigo, IrisuButton::Decision, 3, 1)?;
        let mut i = 0;

        // 発射
        loop{
            println!("i:{}",i);
            press_button(&mut *enigo, IrisuButton::Decision, 1, 1)?;
            i+=1;
            if i >3{
                break;
            }
        }
    }
    Ok(())
}

fn init_app(
    hwnd: HWND,
    enigo: &mut Enigo,
    width: &mut i32,
    height: &mut i32,
    center_x: &mut i32,
    center_y: &mut i32,
) -> Result<(), String> {
    // ウィンドウハンドルがNULLでないことを確認する
    if hwnd.is_null() {
        return Err("ウィンドウハンドルの取得に失敗しました".to_string());
    }
    // 矩形サイズを取得する
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let result = unsafe { GetWindowRect(hwnd, &mut rect) };
    // 矩形サイズの取得が成功したことを確認する
    if result == 0 {
        return Err("矩形サイズの取得に失敗しました".to_string());
    }
    // 矩形サイズから中心座標を計算する
    *width = rect.right - rect.left;
    *height = rect.bottom - rect.top;
    *center_x = rect.left + *width / 2;
    *center_y = rect.top + *height / 2;
    // 結果を表示する
    println!(
        "init width:{} height:{} center x:{} y:{}",
        width, height, center_x, center_y
    );
    // 2秒待つ
    thread::sleep(Duration::from_secs(2));
    // マウスを中心に移動する
    enigo.mouse_move_to(*center_x as i32, *center_y as i32);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut width, mut height, mut center_x, mut center_y) = (0, 0, 0, 0);
    let client_window_title = CString::new("irisu syndrome").unwrap();
    // ウィンドウハンドルを取得する
    let mut hwnd = unsafe { FindWindowA(std::ptr::null_mut(), client_window_title.as_ptr()) };
    std::thread::sleep(Duration::from_secs(2));
    // Enigoのインスタンスを作る
    let mut enigo = Enigo::new();
    // init_app関数を呼び出す
    init_app(
        hwnd,
        &mut enigo,
        &mut width,
        &mut height,
        &mut center_x,
        &mut center_y,
    )?;
    // play_app関数を呼び出す
    play_app(&mut enigo, width, height, center_x, center_y)?;
    Ok(())
}
