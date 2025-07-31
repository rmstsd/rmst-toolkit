use ::image::{ImageFormat, ImageReader};
use std::io::Cursor;
use tauri::{image::Image, Manager};

use crate::window;

#[tauri::command]
pub async fn setIcon(app: tauri::AppHandle) {
  dbg!(&"set icon start");
  let ww = app.get_webview_window(window::WIN_LABEL_SETTING).unwrap();

  // 从字节创建Image对象
  // let image = Image::from_path("resources/g.png");

  let img = ImageReader::open("resources/test.png")
    .unwrap()
    .decode()
    .unwrap();

  let mut bytes: Vec<u8> = Vec::new();
  img
    .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
    .unwrap();

  ww.set_icon(Image::from_bytes(&bytes).unwrap()).unwrap();

  dbg!(&"set icon end");
  // match image {
  //   Ok(image) => ww.set_icon(image).unwrap(),
  //   Err(err) => {
  //     dbg!(&err);
  //   }
  // }
}
