pub use general_storage::*;
use wasm_bindgen::prelude::*;

pub struct LocalStorage {
    local_storage: web_sys::Storage,
}

impl LocalStorage {
    pub fn new() -> Self {
        Self {
            local_storage: web_sys::window().unwrap().local_storage().unwrap().unwrap(),
        }
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct JsError(pub JsValue);

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match js_sys::JSON::stringify(&self.0) {
            Ok(s) => write!(f, "{:?}", s),
            Err(e) => write!(f, "Failed to parse error: {:?}", e),
        }
    }
}

impl std::error::Error for JsError {}

impl Storage for LocalStorage {
    fn exists<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        self.local_storage.get_item(key.as_ref()).unwrap().is_some()
    }

    fn clear(&mut self) {
        self.local_storage.clear().unwrap()
    }

    fn remove<K>(&mut self, key: K) -> Result<(), RemoveError>
    where
        K: AsRef<str>,
    {
        self.local_storage
            .remove_item(key.as_ref())
            .map_err(|e| RemoveError::IoError(Box::new(StringError(format!("{:?}", e)))))
    }

    fn load_raw<K>(&self, key: K) -> Result<Vec<u8>, LoadRawError>
    where
        K: AsRef<str>,
    {
        log::info!("loading from localstorage with key '{}'", key.as_ref());
        let maybe_string = self
            .local_storage
            .get_item(key.as_ref())
            .map_err(|e| LoadRawError::IoError(Box::new(StringError(format!("{:?}", e)))))?;
        let string = maybe_string.ok_or(LoadRawError::NoSuchKey)?;
        serde_json::from_str(&string)
            .map_err(|e| LoadRawError::IoError(Box::new(StringError(format!("{:?}", e)))))
    }

    fn store_raw<K, V>(&mut self, key: K, value: V) -> Result<(), StoreRawError>
    where
        K: AsRef<str>,
        V: AsRef<[u8]>,
    {
        let string = serde_json::to_string(value.as_ref())
            .map_err(|e| StoreRawError::IoError(Box::new(e)))?;
        log::info!("storing to localstorage with key '{}'", key.as_ref());
        self.local_storage
            .set_item(key.as_ref(), &string)
            .map_err(|e| StoreRawError::IoError(Box::new(JsError(e))))?;
        Ok(())
    }
}
