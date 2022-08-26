#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::atomic::AtomicI32;

#[tauri::command]
fn list_quizes() -> Vec<String> {
    let paths = {
        if !Path::new("./quizes").exists() {
            match std::fs::create_dir("./quizes") {
                Err(why) => panic!("couldn't create ./quizes: {}", why),
                Ok(_) => {}
            }
        }
        std::fs::read_dir("./quizes").unwrap()
    };

    paths
        .filter(|path| {
            if let Ok(ent) = path {
                if let Ok(filetype) = ent.file_type() {
                    return filetype.is_dir();
                }
            }
            false
        })
        .map(|path| {
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect()
}

#[derive(Deserialize, Serialize, Clone)]
struct Quiz {
    id: usize,
    word: String,
    choices: Vec<String>,
    correct: usize,
}

#[derive(Deserialize, Serialize)]
struct WordData {
    word: String,
    compl: String,
    meanings: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Words(Vec<WordData>);

#[tauri::command]
fn generate_quiz(name: &str, num: usize) -> Vec<Quiz> {
    let mut rng = rand::thread_rng();
    let words: Words = {
        let pathstr = "./quizes/".to_string() + name + "/data.yaml";
        let path = Path::new(&pathstr);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => serde_yaml::from_str(&s).unwrap(),
        }
    };

    let res = rand::seq::index::sample(&mut rng, words.0.len(), num)
        .iter()
        .map(|i| {
            let correct = rng.gen_range(0..4);
            let mut choices: Vec<_> = rand::seq::index::sample(&mut rng, words.0.len() - 1, 3)
                .iter()
                .map(|sel| if sel >= i { sel + 1 } else { sel })
                .map(|sel| {
                    let meanings = words.0.get(sel).unwrap().meanings.clone();
                    let mi = rng.gen_range(0..meanings.len());
                    meanings.get(mi).unwrap().clone()
                })
                .collect();
            choices.insert(correct, {
                let mi = rng.gen_range(0..words.0.get(i).unwrap().meanings.len());
                words.0.get(i).unwrap().meanings.get(mi).unwrap().clone()
            });
            Quiz {
                id: i,
                word: words.0.get(i).unwrap().word.clone(),
                choices,
                correct,
            }
        })
        .collect();
    res
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Playrecord {
    time: chrono::NaiveDateTime,
    results: Vec<(usize, String)>,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Playrecords(Vec<Playrecord>);

#[derive(Debug, Serialize, Deserialize)]
struct QuizInfo {
    title: String,
}

async fn open_db(name: &str) -> sqlx::Pool<sqlx::Sqlite> {
    let pathstr = "./quizes/".to_string() + name + "/playdata.db";
    let path = Path::new(&pathstr);
    let display = path.display();
    let is_new = !path.exists();
    if is_new {
        match File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(_) => {}
        }
    }
    let pool = sqlx::SqlitePool::connect(&("sqlite://".to_owned() + &pathstr))
        .await
        .unwrap();
    if is_new {
        sqlx::query("create table play(id integer primary key autoincrement, datetime text not null, mode integer not null);").execute(&pool).await.unwrap();
        sqlx::query("create table result(id integer primary key autoincrement, play_id integer not null, quiz_id integer not null, is_correct integer not null);").execute(&pool).await.unwrap();
    }
    pool
}

#[tauri::command]
async fn save_play(
    name: &str,
    datetime: &str,
    mode: i32,
    results: Vec<(usize, bool)>,
) -> Result<(), ()> {
    let pool = open_db(name).await;

    sqlx::query("insert into play(datetime, mode) values(?, ?);")
        .bind(datetime)
        .bind(mode)
        .execute(&pool)
        .await
        .unwrap();

    let play_id: u32 = sqlx::query("select max(id) from play;")
        .fetch_one(&pool)
        .await
        .unwrap()
        .try_get("max(id)")
        .unwrap();

    for res in results {
        sqlx::query("insert into result(play_id, quiz_id, is_correct) values(?, ?, ?);")
            .bind(play_id)
            .bind(res.0 as u32)
            .bind(res.1)
            .execute(&pool)
            .await
            .unwrap();
    }

    Ok(())
}

#[tauri::command]
async fn get_lastplay(name: &str) -> Result<String, ()> {
    let pool = open_db(name).await;

    let datetime: String =
        match sqlx::query("select datetime from play where id = (select max(id) from play);")
            .fetch_all(&pool)
            .await
            .unwrap()
            .get(0)
        {
            None => "".to_owned(),
            Some(row) => match row.try_get("datetime") {
                Err(_) => "".to_owned(),
                Ok(dat) => dat,
            },
        };

    Ok(datetime)
}

#[tauri::command]
fn get_quizinfo(name: &str) -> QuizInfo {
    let pathstr = "./quizes/".to_string() + name + "/info.yaml";
    let path = Path::new(&pathstr);
    let display = path.display();
    let mut file = {
        if path.exists() {
            match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            }
        } else {
            File::create(&pathstr).unwrap()
        }
    };
    let mut s = String::new();
    let info: QuizInfo = match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => match serde_yaml::from_str(&s) {
            Err(_) => panic!("format is invalid."),
            Ok(data) => data,
        },
    };
    info
}
#[async_std::main]
async fn main() {
    tauri::Builder::default()
        .manage(AtomicI32::new(0))
        .invoke_handler(tauri::generate_handler![
            generate_quiz,
            save_play,
            get_quizinfo,
            get_lastplay,
            list_quizes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
