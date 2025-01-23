use std::{thread, time::Duration};
use tts::Tts;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn speech(text: &str, voice_index: usize, rate: f32, volume: f32) {
  // 初始化TTS引擎
  let mut tts = Tts::default().expect("初始化TTS引擎失败");

  // 设置语音速度（可选）
  tts.set_rate(rate).expect("设置语速失败");

  // 设置音量（可选）
  tts.set_volume(volume).expect("设置音量失败");

  // 设置音色（可选）
  if let Some(voice) = tts.voices().expect("获取音色失败").get(voice_index) {
      tts.set_voice(voice).expect("设置音色失败");
  }

  // 朗读文本
  tts.speak(text, false).expect("朗读失败");

  // 等待语音播放完成（假设播放时间为3秒）
  thread::sleep(Duration::from_secs(5));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,speech])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
