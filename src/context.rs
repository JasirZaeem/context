use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
struct ContextData {
    context: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Context {
    data: ContextData,
    config_path: PathBuf,
    pwd: PathBuf,
}

impl ContextData {
    fn new() -> Self {
        Self {
            context: HashMap::new()
        }
    }
}

impl Context {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr_path = Some(self.pwd.as_path());

        let mut paths = Vec::new();
        while let Some(path) = curr_path {
            paths.push(path);
            curr_path = path.parent();
        }

        let mut result = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.context.get(path) {
                result.extend(map.iter());
            }
        }

        result
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr_path = Some(self.pwd.as_path());
        let mut result = None;
        while let Some(path) = curr_path {
            if let Some(path_values) = self.data.context.get(path) {
                if let Some(value) = path_values.get(key) {
                    result = Some(value);
                    break;
                }
            }

            curr_path = path.parent();
        }

        result
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.context.entry(self.pwd.clone()).or_default().insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.context.entry(self.pwd.clone()).or_default().remove(key);
    }
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn from_config_props(config_path: PathBuf, pwd: PathBuf) -> Self {
        if std::fs::metadata(&config_path).is_ok() {
            let file_contents = std::fs::read_to_string(&config_path).unwrap_or(String::from("{\"context\": {}}"));
            let data: ContextData = serde_json::from_str(&file_contents).unwrap_or(ContextData::new());
            return Self {
                data,
                config_path,
                pwd,
            };
        }

        Self {
            data: ContextData::new(),
            config_path,
            pwd,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};
    use crate::context::Context;
    use crate::config::ContextConfig;
    use crate::config::Operation::Print;
    use crate::context::ContextData;

    fn get_config(pwd: PathBuf) -> ContextConfig {
        ContextConfig {
            operation: Print(None),
            config: PathBuf::from("/context_config.json"),
            pwd,
        }
    }

    fn get_context_data() -> ContextData {
        let mut data_map = HashMap::new();
        data_map.insert(PathBuf::from("/"), {
            let mut map = HashMap::new();
            map.insert(String::from("key1"), String::from("value1"));
            map.insert(String::from("key2"), String::from("value2"));
            map
        });

        data_map.insert(PathBuf::from("/dir1"), {
            let mut map = HashMap::new();
            map.insert(String::from("key3"), String::from("value3"));
            map.insert(String::from("key1"), String::from("value4"));
            map
        });

        data_map.insert(PathBuf::from("/dir1/dir2"), {
            let mut map = HashMap::new();
            map.insert(String::from("key4"), String::from("value5"));
            map.insert(String::from("key1"), String::from("value6"));
            map
        });

        data_map.insert(PathBuf::from("/dir1/dir2/dir3"), {
            let mut map = HashMap::new();
            map.insert(String::from("key4"), String::from("value5"));
            map.insert(String::from("key1"), String::from("value8"));
            map
        });

        ContextData {
            context: data_map,
        }
    }

    fn get_context(pwd: PathBuf) -> Context {
        Context {
            data: get_context_data(),
            config_path: PathBuf::from("/context_config.json"),
            pwd,
        }
    }

    #[test]
    fn test_get_value_all() {
        let context = get_context(PathBuf::from("/dir1/dir2/dir3"));

        let k1 = String::from("key1");
        let v1 = String::from("value8");
        let k2 = String::from("key2");
        let v2 = String::from("value2");
        let k3 = String::from("key3");
        let v3 = String::from("value3");
        let k4 = String::from("key4");
        let v4 = String::from("value5");

        assert_eq!(context.get_value_all().len(), 4);
        assert_eq!(context.get_value_all().get(&k1).unwrap().to_owned(), &v1);
        assert_eq!(context.get_value_all().get(&k2).unwrap().to_owned(), &v2);
        assert_eq!(context.get_value_all().get(&k3).unwrap().to_owned(), &v3);
        assert_eq!(context.get_value_all().get(&k4).unwrap().to_owned(), &v4);
    }

    #[test]
    fn test_get_value() {
        let context = get_context(PathBuf::from("/dir1/dir2/dir3"));
        assert_eq!(context.get_value("key1").unwrap(), "value8");
        assert_eq!(context.get_value("key2").unwrap(), "value2");
    }

    #[test]
    fn test_set_value() {
        let mut context = get_context(PathBuf::from("/dir1/dir2/dir3"));
        context.set_value(String::from("key5"), String::from("value9"));
        assert_eq!(context.get_value("key5").unwrap(), "value9");

        assert_eq!(context.get_value("key2").unwrap(), "value2");
        context.set_value(String::from("key2"), String::from("value10"));
        assert_eq!(context.get_value("key2").unwrap(), "value10");
    }

    #[test]
    fn test_remove_value() {
        let mut context = get_context(PathBuf::from("/dir1/dir2/dir3"));
        assert_eq!(context.get_value("key1").unwrap(), "value8");
        context.remove_value("key1");
        assert_eq!(context.get_value("key1").unwrap(), "value6");
    }
}