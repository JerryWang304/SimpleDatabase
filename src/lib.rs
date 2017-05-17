#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::io::Error;
use std::io::ErrorKind;
// too ugly using &str
// fortunately: str => Into<Vec<u8>>
pub struct SimpleDB {
    data: RwLock<HashMap<Vec<u8>, Vec<u8>>>,

}
impl SimpleDB {
    // create a new database
    pub fn new() -> Result<SimpleDB, Error> {
        Ok(

            SimpleDB {
                data: RwLock::new(HashMap::new())
            }

        )
    }
    // get the associated value. Return an option 
    pub fn get<T: Into<Vec<u8>>>(&self, key: T) ->  Option<String> {
        // data.read => LockResult<RwLockGuard<HashMap>>
        match self.data.read().ok() {
            // ok() => Option<RwLockGuard<HashMap>>
            None => None, 
            Some(ref guard) => {
                // guard: & RwLockGuard<HashMap>
                // guard.get() => Option<&>
                match guard.get(&key.into()) {
                    None => None,
                    // value is reference to HashMap[key]
                    Some(value) => {
                        // conver Vec<u8> to a string
                        Some(String::from_utf8(value.clone()).unwrap())
                    }
                }

            }
        }
    }
    // set a (key,value)
    // return a result: success or failure
    pub fn set<T: Into<Vec<u8>>>(&self, key:T, value: T) -> Result<(), Error> {
        // write().ok() => Option<RwLockWriteGuard<HashMap>>
        match self.data.write().ok() {
            None => Err(Error::new(ErrorKind::Other,"No!")),
            Some(ref mut guard) => {
                guard.insert(key.into(), value.into());
                Ok(())
            }
        }
    }
    // delete a (key, value) given key
    pub fn delete<T: Into<Vec<u8>>>(&self, key: T) -> Result<(), Error> {
        match self.data.write().ok() {
            None => Err(Error::new(ErrorKind::Other,"No!")),

            Some(ref mut guard) => {

                match guard.remove(&key.into()) {
                    None => Err(Error::new(ErrorKind::Other,"No!")),
                    Some(_) => Ok(())
                }

            }
        }
    }
}
unsafe impl Sync for SimpleDB {}
unsafe impl Send for SimpleDB {}

#[cfg(test)]
mod tests {
    //use std::collections::HashMap;
    #[test]
    #[allow(dead_code)]
    fn it_works() {
        // let mut data: HashMap<&str, &str> = HashMap::new();
        // data.insert("foo","bar");
        // assert_eq!(data["fo"], "bar");
    }
}
