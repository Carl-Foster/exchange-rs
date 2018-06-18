use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub trait Store: Serialize + DeserializeOwned + GetID {
  const PATH: &'static str;
  fn save_state(&self) -> io::Result<serde_json::Result<()>> {
    let filename = Path::new("store").join(Self::PATH).join(self.get_id());
    File::create(&filename).map(|file| serde_json::to_writer(file, self))
  }

  fn init_from_store(id: String) -> Option<Self> {
    let filename = Path::new("store").join(Self::PATH).join(id);
    let contents = File::open(&filename).and_then(|mut file| {
      let mut s = String::new();
      file.read_to_string(&mut s)?;
      Ok(s)
    });
    match contents {
      Ok(s) => serde_json::from_str(&s).ok(),
      Err(_) => None,
    }
  }
}

pub trait GetID {
  fn get_id(&self) -> String;
}
