use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::collections::HashMap;


#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    is_primary: Option<bool>,     // done
    name: String,                 // done
    condition: Option<String>,    //
    datastore_id: String,
    id: String,
    index_key: Vec<String>,       //
    keyspace_id: String,          //
    metadata: Option<Map<String, Value>>,
    namespace_id: String,         //
    partition: Option<String>,    //
    state: String,
    using: String,
}

impl Index {
  pub fn name(&self) -> String {
    return self.name.to_string()
  }

  pub fn to_n1ql(&self, bucket: Option<&std::string::String>, if_not_exists: Option<bool>, 
    defer_build: Option<bool>, num_replica: Option<u8>) -> Option<String> {

    let mut qry: String = "".to_string();

    if bucket.is_some() && bucket.unwrap() != &self.keyspace_id.to_string() {
      return None;
    }

    qry.push_str("CREATE ");

    if self.is_primary.is_some() {
      qry.push_str("PRIMARY ");
    }

    qry.push_str(&format!("INDEX `{}`", self.name.to_string()));

    let mut ne = "";
    if if_not_exists.is_some() && if_not_exists.unwrap() {
      ne = "IF NOT EXISTS";
    }


    qry.push_str(&format!(" {} \nON `{}` ", ne, self.keyspace_id.to_string()));
    

    if self.index_key.len()>0 {
      qry.push_str(&format!("\n ({}) ",self.index_key.join(",")));
    }

    if self.partition.is_some() {
      qry.push_str(&format!("\n PARTITION BY {}", self.partition.as_ref().unwrap()));
    }

    if self.condition.is_some() {
      qry.push_str(&format!("\n WHERE {} ",self.condition.as_ref().unwrap()));
    }

    let mut with: HashMap<std::string::String,std::string::String> = HashMap::new();
    if self.metadata.is_some() {
      for (k, v) in self.metadata.as_ref().unwrap().iter() {
        with.insert(k.to_string(), v.to_string());
      }
    }

    if defer_build.is_some() && defer_build.unwrap() {
      with.insert("defer_build".to_string(), "true".to_string());

    }

    //num_replica
    if num_replica.is_some() {
      with.insert("num_replica".to_string(), num_replica.unwrap().to_string());
    }

    // Iterate over everything.
    let mut with_vec: Vec::<String>= vec![];
    for (k, v) in &with {
      with_vec.push(format!("\"{k}\": {v}"));
    }

    if with.len() > 0 {
      qry.push_str(&format!("\n WITH {{ {} }};", with_vec.join(", ")));
    }

    return Some(qry);
  }
}

pub type Indexes = Vec<HashMap<String, Index>>;
