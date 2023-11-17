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
    metadata: Map<String, Value>,
    namespace_id: String,         //
    partition: Option<String>,    //
    state: String,
    using: String,
}

impl Index {
  pub fn to_n1ql(&self, bucket: Option<&str>, if_not_exists: Option<bool>, defer_build: Option<bool>) -> Option<String> {

    let mut qry: String = "".to_string();

    if bucket.is_some() && bucket.unwrap() != self.keyspace_id.to_string() {
      return None;
    }

    qry.push_str("CREATE ");

    if self.is_primary.is_some() {
      qry.push_str("PRIMARY ");
    }

    qry.push_str("INDEX ");
    qry.push_str(&self.name.to_string());

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

    if !self.metadata.is_empty() {
      let mut m: Vec<std::string::String> = vec![];
      for (k, v) in self.metadata.iter() {
        m.push(format!("\"{}\": {}", k, v.to_string()));
      }

      if defer_build.is_some() && defer_build.unwrap() {
        m.push(format!("\"{}\": {}", "defer_build", "true"));
      }

      qry.push_str(&format!("\n WITH {{ {} }};\n", m.join(", ")));

    }
    return Some(qry);
  }
}

pub type Indexes = Vec<HashMap<String, Index>>;
