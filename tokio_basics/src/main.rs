// Read file synchronously
// -----------------------
// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_file_sync() -> io::Result<String> {
//     let mut file = File::open("example.txt")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }
//
// fn main() {
//     match read_file_sync() {
//         Ok(contents) => println!("Synchronous Read:\n{}", contents),
//         Err(err) => eprintln!("Error reading file: {}", err),
//     }
// }
//
//
// Read file asynchronously
// -----------------------
// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};
//
// async fn read_file_async() -> io::Result<String> {
//     let mut file = File::open("example.txt").await?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).await?;
//     Ok(contents)
// }
//
// #[tokio::main]
// async fn main() {
//     match read_file_async().await {
//         Ok(contents) => println!("Asynchronous Read:\n{}", contents),
//         Err(err) => eprintln!("Error reading file: {}", err),
//     }
// }

// Program Cache System versi KW
// ----------------------------
use std::collections::HashMap;
use tokio::sync::Mutex;

trait AsyncCache<K, V> {
    async fn insert(&self, key: K, value: V);
    async fn get(&self, key: &K) -> Option<V>;
}

struct AsyncInMemoryCache<K, V> {
    cache: Mutex<HashMap<K, V>>,
}

impl<K, V> AsyncInMemoryCache<K, V> {
    fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }
}

impl<K, V> AsyncCache<K, V> for AsyncInMemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Send,
    V: Send + Clone,
{
    async fn insert(&self, key: K, value: V) {
        let mut inner_cache = self.cache.lock().await;
        inner_cache.insert(key, value);
    }

    async fn get(&self, key: &K) -> Option<V> {
        let inner_cache = self.cache.lock().await;
        inner_cache.get(key).cloned()
    }
}

#[tokio::main]
async fn main() {
    let cache = AsyncInMemoryCache::<&str, i32>::new();

    cache.insert("one", 1).await;
    cache.insert("two", 2).await;
    cache.insert("three", 3).await;

    println!("Value for 'one': {:?}", cache.get(&"one").await);
    println!("Value for 'two': {:?}", cache.get(&"two").await);
    println!("Value for 'three': {:?}", cache.get(&"three").await);
}
