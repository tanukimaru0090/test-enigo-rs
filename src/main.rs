use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use log::{error, info, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::ffi::CString;
use std::os::raw::c_void;
use std::thread;
use std::env::args;
use std::process::Command;
use std::time::Duration;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{FindWindowA, GetForegroundWindow, GetWindowRect};

enum IrisuButton {
    LClick, //決定ボタン
    RClick, //戻るボタン
    Esc,    //終了ボタン
}

fn press_button(
    enigo: &mut Enigo,
    button_type: IrisuButton,
    start_sleep_time: u64,
    down_sleep_time: u64,
) -> Result<(), String> {
    std::thread::sleep(Duration::from_secs(start_sleep_time));
    match button_type {
        IrisuButton::LClick => {
            info!("press button:LClick");
            enigo.mouse_down(MouseButton::Left);
            std::thread::sleep(Duration::from_secs(down_sleep_time));
            enigo.mouse_up(MouseButton::Left);
        }
        IrisuButton::Esc => {
            info!("press button:Esc");
            enigo.key_down(Key::Escape);
            std::thread::sleep(Duration::from_secs(down_sleep_time));
            enigo.key_up(Key::Escape);
        }

        IrisuButton::RClick => {
            info!("press button:RClick");
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

    //let mut title_menu_on = false;

    // タイトル項目の座標
    let title_menu: [(i32, i32); 4] = [
        (center_x, center_y + 20),
        (center_x + 100, center_y + 100),
        (center_x + 150, center_y + 150),
        (center_x, center_y - 150),
    ];

    let album_menu: [(i32, i32); 5] = [
        (center_x - 250, center_y + 200),
        ((center_x - 250) + 150, center_y + 200),
        ((center_x - 250) + 250, center_y + 200),
        ((center_x - 250) + 350, center_y + 200),
        ((center_x - 250) + 450, center_y + 200),
    ];

    // ウィンドウハンドルが無効になったかどうかチェックする
    unsafe {
        loop {
            let client_window_title = CString::new("irisu syndrome").unwrap();
            // ウィンドウハンドルを取得する
            let mut hwnd =
                unsafe { FindWindowA(std::ptr::null_mut(), client_window_title.as_ptr()) };

            // ウィンドウハンドルがNULLならエラーを返して終了する
            if hwnd.is_null() {
                warn!("シミュレーション中にウィンドウハンドルが無効になりました。プログラムを終了します。");
                return Err(String::from(""));
            } else {
                // 最初にBGMを二回変える
                std::thread::sleep(Duration::from_secs(2));
                enigo.mouse_move_to(title_menu[3].0, title_menu[3].1);
                press_button(&mut *enigo, IrisuButton::LClick, 3, 1)?;
                press_button(&mut *enigo, IrisuButton::LClick, 3, 1)?;
                
                // タイトル画面項目のシミュレーションテスト
                for (i, title) in title_menu.iter().enumerate() {
                    if i >2{
                        break;
                    }
                    // ウィンドウハンドルを取得する
                    hwnd =
                        unsafe { FindWindowA(std::ptr::null_mut(), client_window_title.as_ptr()) };

                    // ウィンドウハンドルがNULLならエラーを返して終了する
                    if hwnd.is_null() {
                        warn!("シミュレーション中にウィンドウハンドルが無効になりました。プログラムを終了します。");
                        return Err(String::from(""));
                    }
                    std::thread::sleep(Duration::from_secs(2));
                    enigo.mouse_move_to(title.0, title.1);
                }



                std::thread::sleep(Duration::from_secs(2));
                // あるばむにカーソルを合わせる
                enigo.mouse_move_to(title_menu[1].0, title_menu[1].1);

                //あるばむに変更
                press_button(&mut *enigo, IrisuButton::LClick, 3, 1)?;

                // あるばむのメニュー1~5にカーソルを合わせる
                for (i, album) in album_menu.iter().enumerate() {
                    // ウィンドウハンドルを取得する
                    hwnd =
                        unsafe { FindWindowA(std::ptr::null_mut(), client_window_title.as_ptr()) };

                    // ウィンドウハンドルがNULLならエラーを返して終了する
                    if hwnd.is_null() {
                        warn!("シミュレーション中にウィンドウハンドルが無効になりました。プログラムを終了します。");

                        return Err(String::from(""));
                    }
                    std::thread::sleep(Duration::from_secs(2));
                    enigo.mouse_move_to(album.0, album.1);
                    press_button(&mut *enigo, IrisuButton::LClick, 3, 15)?;
                    press_button(&mut *enigo, IrisuButton::RClick, 3, 3)?;
                }

                press_button(&mut *enigo, IrisuButton::RClick, 3, 3)?;
                
                // すたーとに変更
                std::thread::sleep(Duration::from_secs(2));
                enigo.mouse_move_to(title_menu[0].0, title_menu[0].1);
                press_button(&mut *enigo, IrisuButton::RClick, 3, 1)?;
                let mut i = 0;

                // 発射
                loop {
                    hwnd =
                        unsafe { FindWindowA(std::ptr::null_mut(), client_window_title.as_ptr()) };

                    // ウィンドウハンドルがNULLならエラーを返して終了する
                    if hwnd.is_null() {
                        warn!("シミュレーション中にウィンドウハンドルが無効になりました。プログラムを終了します。");
                        return Err(String::from(""));
                    }

                    let (mut x, mut y) = enigo.mouse_location();
                    info!("mouse x:{} y:{}", x, y);
                    info!("i:{}", i);

                    if x < 650 && y < 513 && !(x < 0) && !(y<0) {
                        std::thread::sleep(Duration::from_secs(2));
                        enigo.mouse_move_to(x + i * 4, y+i*4);
                    }else if x > 650&& y > 513{
                        x = center_x;
                        y = center_y;
                    }
                    press_button(&mut *enigo, IrisuButton::LClick, 1, 1)?;
                    i += 1;

                    if i > 50 {
                        break;
                    }
                }
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
    info!(
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
    //let mut args = args();
    //let mut args:Vec<String> = args.skip(1).collect();
    let default_program = "/Users/daruma/Downloads/irisu203/irisu203/irisu.exe";

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    //info!("{}",args[0]);
    let  cmd = Command::new(default_program).spawn().unwrap();
    
    thread::sleep(Duration::from_secs(2));

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
