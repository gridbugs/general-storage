pub use general_storage::*;

#[cfg(any(
    not(any(feature = "general_storage_file", feature = "general_storage_web")),
    all(feature = "general_storage_file", feature = "general_storage_web"),
))]
mod implementation {
    use super::*;
    pub mod backend {}
    pub struct StaticStorage(());

    const WARNING: &str = "using null implementation of StaticStorage";

    impl StaticStorage {
        pub fn exists<K>(&self, _key: K) -> bool
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        pub fn clear(&mut self) {
            unimplemented!()
        }

        pub fn remove<K>(&mut self, _key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        pub fn load_raw<K>(&self, _key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        pub fn store_raw<K, V>(&mut self, _key: K, _value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            unimplemented!()
        }
        pub fn load<K, T, F>(
            &self,
            _key: K,
            _format: F,
        ) -> Result<T, LoadError<F::DeserializeError>>
        where
            K: AsRef<str>,
            T: DeserializeOwned,
            F: StorageFormat,
        {
            unimplemented!()
        }
        pub fn store<K, T, F>(
            &mut self,
            _key: K,
            _value: &T,
            _format: F,
        ) -> Result<(), StoreError<F::SerializeError>>
        where
            K: AsRef<str>,
            T: Serialize,
            F: StorageFormat,
        {
            unimplemented!()
        }
    }

    impl Storage for StaticStorage {
        fn exists<K>(&self, _key: K) -> bool
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        fn clear(&mut self) {
            unimplemented!()
        }

        fn remove<K>(&mut self, _key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        fn load_raw<K>(&self, _key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            unimplemented!()
        }

        fn store_raw<K, V>(&mut self, _key: K, _value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            unimplemented!()
        }
    }
}

#[cfg(all(feature = "general_storage_file", not(feature = "general_storage_web")))]
mod implementation {
    use super::*;
    use backend::FileStorage;
    pub use general_storage_file as backend;
    pub struct StaticStorage(FileStorage);

    impl StaticStorage {
        pub fn new(inner: FileStorage) -> Self {
            Self(inner)
        }
        pub fn exists<K>(&self, key: K) -> bool
        where
            K: AsRef<str>,
        {
            self.0.exists(key)
        }

        pub fn clear(&mut self) {
            self.0.clear()
        }

        pub fn remove<K>(&mut self, key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            self.0.remove(key)
        }

        pub fn load_raw<K>(&self, key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            self.0.load_raw(key)
        }

        pub fn store_raw<K, V>(&mut self, key: K, value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            self.0.store_raw(key, value)
        }
        pub fn load<K, T, F>(&self, key: K, format: F) -> Result<T, LoadError<F::DeserializeError>>
        where
            K: AsRef<str>,
            T: DeserializeOwned,
            F: StorageFormat,
        {
            self.0.load(key, format)
        }
        pub fn store<K, T, F>(
            &mut self,
            key: K,
            value: &T,
            format: F,
        ) -> Result<(), StoreError<F::SerializeError>>
        where
            K: AsRef<str>,
            T: Serialize,
            F: StorageFormat,
        {
            self.0.store(key, value, format)
        }
    }

    impl Storage for StaticStorage {
        fn exists<K>(&self, key: K) -> bool
        where
            K: AsRef<str>,
        {
            self.0.exists(key)
        }

        fn clear(&mut self) {
            self.0.clear()
        }

        fn remove<K>(&mut self, key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            self.0.remove(key)
        }

        fn load_raw<K>(&self, key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            self.0.load_raw(key)
        }

        fn store_raw<K, V>(&mut self, key: K, value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            self.0.store_raw(key, value)
        }
    }
}

#[cfg(all(feature = "general_storage_web", not(feature = "general_storage_file")))]
mod implementation {
    use super::*;
    use backend::LocalStorage;
    pub use general_storage_web as backend;
    pub struct StaticStorage(LocalStorage);

    impl StaticStorage {
        pub fn new(inner: LocalStorage) -> Self {
            Self(inner)
        }
        pub fn exists<K>(&self, key: K) -> bool
        where
            K: AsRef<str>,
        {
            self.0.exists(key)
        }

        pub fn clear(&mut self) {
            self.0.clear()
        }

        pub fn remove<K>(&mut self, key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            self.0.remove(key)
        }

        pub fn load_raw<K>(&self, key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            self.0.load_raw(key)
        }

        pub fn store_raw<K, V>(&mut self, key: K, value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            self.0.store_raw(key, value)
        }
        pub fn load<K, T, F>(&self, key: K, format: F) -> Result<T, LoadError<F::DeserializeError>>
        where
            K: AsRef<str>,
            T: DeserializeOwned,
            F: StorageFormat,
        {
            self.0.load(key, format)
        }
        pub fn store<K, T, F>(
            &mut self,
            key: K,
            value: &T,
            format: F,
        ) -> Result<(), StoreError<F::SerializeError>>
        where
            K: AsRef<str>,
            T: Serialize,
            F: StorageFormat,
        {
            self.0.store(key, value, format)
        }
    }

    impl Storage for StaticStorage {
        fn exists<K>(&self, key: K) -> bool
        where
            K: AsRef<str>,
        {
            self.0.exists(key)
        }

        fn clear(&mut self) {
            self.0.clear()
        }

        fn remove<K>(&mut self, key: K) -> Result<(), RemoveError>
        where
            K: AsRef<str>,
        {
            self.0.remove(key)
        }

        fn load_raw<K>(&self, key: K) -> Result<Vec<u8>, LoadRawError>
        where
            K: AsRef<str>,
        {
            self.0.load_raw(key)
        }

        fn store_raw<K, V>(&mut self, key: K, value: V) -> Result<(), StoreRawError>
        where
            K: AsRef<str>,
            V: AsRef<[u8]>,
        {
            self.0.store_raw(key, value)
        }
    }
}

pub use implementation::*;
