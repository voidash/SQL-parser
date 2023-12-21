use std::collections::HashSet;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryData {
    pub fields: Vec<String>,
    pub tables: Vec<String>,
    pub joins: Vec<Join>,
    pub is_distinct: bool,
    pub top: u32,
    pub condition: Vec<WhereClause>,
}
impl QueryData {
    pub fn new() -> QueryData {
        QueryData {
            fields: vec![],
            tables: vec![],
            joins: vec![],
            is_distinct: false,
            top: 0,
            condition: vec![]
        }
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Join {
    pub join_type: JoinType,
    pub table: String,
}

impl Join {
    pub fn new(name: &str) -> Join {
        Join {
            join_type: JoinType::get(name).unwrap(),
            table: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JoinType {
    InnerJoin,
    FullOuterJoin,
    RightJoin,
    LeftJoin,
}

impl JoinType {
    fn get(name: &str) -> Option<JoinType> {
        let t = match name {
            "inner join" => JoinType::InnerJoin,
            "full outer join" => JoinType::FullOuterJoin,
            "left join" => JoinType::LeftJoin,
            "right join" => JoinType::RightJoin,
            _ => return None,
        };
        Some(t)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum BinOp {
    Another(Box<WhereClause>),
    Assigned(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhereClause {
    pub left: BinOp, 
    pub right: BinOp,
    pub operation: String,
}

impl WhereClause {
    pub fn new() -> Self{
       Self {
          left: BinOp::Assigned("".into()),
          right: BinOp::Assigned("".into()),
          operation: "".into()
       } 
    }
}

