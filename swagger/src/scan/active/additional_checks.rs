use std::collections::hash_set;

use super::utils::create_payload;
///use super::utils::create_payload_for_get;
use super::*;
use colored::*;
use reqwest::{self, Url};
use serde::ser::Error;
use serde_json::json;
use utils;

impl<T: OAS + Serialize> ActiveScan<T> {
    pub fn change_payload(orig: &Value, path: &[String], new_val: Value) -> Value {
        let mut change = &mut json!(null);
        let mut ret = orig.clone();
        for path_part in path.iter() {
            change = &mut ret[path_part];
        }
        *change = new_val;
        ret.clone()
    }
    pub async fn func_test(&self, _auth: &Authorization) -> CheckRetVal {
        let values_path = self.path_params.clone();
        let mut ret_val = CheckRetVal::default();
        for (path, item) in &self.oas.get_paths() {
            for (m, op) in item.get_ops().iter() {
                self.oas.servers();
                // create_payload(&self.oas_value, op);

                dbg!(create_payload(&self.oas_value, op, &values_path, None));
            }
        }

        ret_val
    }
 
            }