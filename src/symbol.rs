extern crate lazy_static;
use std::collections::HashMap;
use lazy_static::lazy_static;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Symbol {
    pub name: String,
    pub len: usize,
    pub token: Token,
    pub group: Group,
}

#[derive(Debug, PartialEq, Clone,Serialize,Deserialize)]
pub enum Group {
    DataType,
    Function,
    Keyword,
    Operator,   // >, >=, =, !=, <>, <, <=
    Identifier, // t1, a, b
    Delimiter,  // `,`, (, )
}

/// Token includes keywords, functions, and data types
#[derive(Debug, PartialEq, Clone,Serialize,Deserialize)]
pub enum Token {
    /* SQL Keywords */
    Add,
    AddConstraint,
    AlterColumn,
    AlterTable,
    All,
    Any,
    As,
    Asc,
    Between,
    Case,
    Check,
    Column,
    Constraint,
    Create,
    CreateDatabase,
    CreateIndex,
    CreateOrReplaceView,
    CreateTable,
    CreateProcedure,
    CreateUniqueIndex,
    CreateView,
    Database,
    Default,
    Delete,
    Desc,
    Distinct,
    DropColumn,
    DropConstraint,
    DropDatabase,
    DropDefault,
    DropIndex,
    DropTable,
    DropView,
    Exec,
    Exists,
    ForeignKey,
    From,
    FullOuterJoin,
    GroupBy,
    Having,
    In,
    Index,
    InnerJoin,
    InsertInto,
    IsNull,
    IsNotNull,
    LeftJoin,
    Like,
    Limit,
    NotNull,
    On,
    OrderBy,
    Percent,
    PrimaryKey,
    Procedure,
    RightJoin,
    Rownum,
    Select,
    Set,
    Table,
    Top,
    TruncateTable,
    Union,
    UnionAll,
    Unique,
    Update,
    Values,
    View,
    Where,

    /* SQL Function */
    Avg,
    Count,
    Max,
    Min,
    Sum,

    /* SQL Data Type */
    Char,
    Double,
    Float,
    Int,
    Varchar,
    Url,

    /* Operator */
    LT, // <
    LE, // <=
    EQ, // =
    NE, // !=, <>
    GT, // >
    GE, // >=
    AND,
    NOT,
    OR,

    /* Delimiter */
    ParenLeft,  // (
    ParenRight, // )
    Comma,       // ,
    Semicolon,   // ;

    /* Any Identifier */
    Identifier,

}

pub fn symbol(name: &str, token: Token, group: Group) -> Symbol {
    Symbol {
        name: name.to_string(),
        len: name.len(),
        token,
        group,
    }
}

lazy_static! {
    /// A static struct of token hashmap storing all tokens
    pub static ref SYMBOLS: HashMap<&'static str, Symbol> = {
        let mut m = HashMap::new();

        // ChatGPT 

        /* SQL Keywords */
        m.insert("add", symbol("add", Token::Add, Group::Keyword));
        m.insert("add constraint", symbol("add constraint", Token::AddConstraint, Group::Keyword));
        m.insert("alter column", symbol("alter column", Token::AlterColumn, Group::Keyword));
        m.insert("alter table", symbol("alter table", Token::AlterTable, Group::Keyword));
        m.insert("all", symbol("all", Token::All, Group::Keyword));
        m.insert("any", symbol("any", Token::Any, Group::Keyword));
        m.insert("as", symbol("as", Token::As, Group::Keyword));
        m.insert("asc", symbol("asc", Token::Asc, Group::Keyword));
        m.insert("between", symbol("between", Token::Between, Group::Keyword));
        m.insert("case", symbol("case", Token::Case, Group::Keyword));
        m.insert("check", symbol("check", Token::Check, Group::Keyword));
        m.insert("column", symbol("column", Token::Column, Group::Keyword));
        m.insert("constraint", symbol("constraint", Token::Constraint, Group::Keyword));
        m.insert("create", symbol("create", Token::Create, Group::Keyword));
        m.insert("create database", symbol("create database", Token::CreateDatabase, Group::Keyword));
        m.insert("create index", symbol("create index", Token::CreateIndex, Group::Keyword));
        m.insert("create or replace view", symbol("create or replace view", Token::CreateOrReplaceView, Group::Keyword));
        m.insert("create table", symbol("create table", Token::CreateTable, Group::Keyword));
        m.insert("create procedure", symbol("create procedure", Token::CreateProcedure, Group::Keyword));
        m.insert("create unique index", symbol("create unique index", Token::CreateUniqueIndex, Group::Keyword));
        m.insert("create view", symbol("create view", Token::CreateView, Group::Keyword));
        m.insert("database", symbol("database", Token::Database, Group::Keyword));
        m.insert("default", symbol("default", Token::Default, Group::Keyword));
        m.insert("delete", symbol("delete", Token::Delete, Group::Keyword));
        m.insert("desc", symbol("desc", Token::Desc, Group::Keyword));
        m.insert("distinct", symbol("distinct", Token::Distinct, Group::Keyword));
        m.insert("drop column", symbol("drop column", Token::DropColumn, Group::Keyword));
        m.insert("drop constraint", symbol("drop constraint", Token::DropConstraint, Group::Keyword));
        m.insert("drop database", symbol("drop database", Token::DropDatabase, Group::Keyword));
        m.insert("drop default", symbol("drop default", Token::DropDefault, Group::Keyword));
        m.insert("drop index", symbol("drop index", Token::DropIndex, Group::Keyword));
        m.insert("drop table", symbol("drop table", Token::DropTable, Group::Keyword));
        m.insert("drop view", symbol("drop view", Token::DropView, Group::Keyword));
        m.insert("exec", symbol("exec", Token::Exec, Group::Keyword));
        m.insert("exists", symbol("exists", Token::Exists, Group::Keyword));
        m.insert("foreign key", symbol("foreign key", Token::ForeignKey, Group::Keyword));
        m.insert("from", symbol("from", Token::From, Group::Keyword));
        m.insert("full outer join", symbol("full outer join", Token::FullOuterJoin, Group::Keyword));
        m.insert("group by", symbol("group by", Token::GroupBy, Group::Keyword));
        m.insert("having", symbol("having", Token::Having, Group::Keyword));
        m.insert("in", symbol("in", Token::In, Group::Keyword));
        m.insert("index", symbol("index", Token::Index, Group::Keyword));
        m.insert("inner join", symbol("inner join", Token::InnerJoin, Group::Keyword));
        m.insert("insert into", symbol("insert into", Token::InsertInto, Group::Keyword));
        m.insert("is null", symbol("is null", Token::IsNull, Group::Keyword));
        m.insert("is not null", symbol("is not null", Token::IsNotNull, Group::Keyword));
        m.insert("left join", symbol("left join", Token::LeftJoin, Group::Keyword));
        m.insert("like", symbol("like", Token::Like, Group::Keyword));
        m.insert("limit", symbol("limit", Token::Limit, Group::Keyword));
        m.insert("not null", symbol("not null", Token::NotNull, Group::Keyword));
        m.insert("on", symbol("on", Token::On, Group::Keyword));
        m.insert("order by", symbol("order by", Token::OrderBy, Group::Keyword));
        m.insert("percent", symbol("percent", Token::Percent, Group::Keyword));
        m.insert("primary key", symbol("primary key", Token::PrimaryKey, Group::Keyword));
        m.insert("procedure", symbol("procedure", Token::Procedure, Group::Keyword));
        m.insert("right join", symbol("right join", Token::RightJoin, Group::Keyword));
        m.insert("rownum", symbol("rownum", Token::Rownum, Group::Keyword));
        m.insert("select", symbol("select", Token::Select, Group::Keyword));
        m.insert("set", symbol("set", Token::Set, Group::Keyword));
        m.insert("table", symbol("table", Token::Table, Group::Keyword));
        m.insert("top", symbol("top", Token::Top, Group::Keyword));
        m.insert("truncate table", symbol("truncate table", Token::TruncateTable, Group::Keyword));
        m.insert("union", symbol("union", Token::Union, Group::Keyword));
        m.insert("union all", symbol("union all", Token::UnionAll, Group::Keyword));
        m.insert("unique", symbol("unique", Token::Unique, Group::Keyword));
        m.insert("update", symbol("update", Token::Update, Group::Keyword));
        m.insert("values", symbol("values", Token::Values, Group::Keyword));
        m.insert("view", symbol("view", Token::View, Group::Keyword));
        m.insert("where", symbol("where", Token::Where, Group::Keyword));

        /* SQL Function */
        m.insert("avg", symbol("avg", Token::Avg, Group::Function));
        m.insert("count", symbol("count", Token::Count, Group::Function));
        m.insert("max", symbol("max", Token::Max, Group::Function));
        m.insert("min", symbol("min", Token::Min, Group::Function));
        m.insert("sum", symbol("sum", Token::Sum, Group::Function));

        /* SQL Data Type */
        m.insert("char", symbol("char", Token::Char, Group::DataType));
        m.insert("double", symbol("double", Token::Double, Group::DataType));
        m.insert("float", symbol("float", Token::Float, Group::DataType));
        m.insert("int", symbol("int", Token::Int, Group::DataType));
        m.insert("varchar", symbol("varchar", Token::Varchar, Group::DataType));
        m.insert("url", symbol("url", Token::Url, Group::DataType));

        /* Operator */
        m.insert(">", symbol(">", Token::GT, Group::Operator));
        m.insert(">=", symbol(">=", Token::GE, Group::Operator));
        m.insert("=", symbol("=", Token::EQ, Group::Operator));
        m.insert("!=", symbol("!=", Token::NE, Group::Operator));
        m.insert("<>", symbol("<>", Token::NE, Group::Operator));
        m.insert("<", symbol("<", Token::LT, Group::Operator));
        m.insert("<=", symbol("<=", Token::LE, Group::Operator));
        m.insert("and", symbol("and", Token::AND, Group::Operator));
        m.insert("not", symbol("not", Token::NOT, Group::Operator));
        m.insert("or", symbol("or", Token::OR, Group::Operator));

        m //return m
    };
}

impl Symbol {
    pub fn match_delimiter(ch: char) -> Option<Symbol> {
        match ch {
            '(' => Some(symbol("(", Token::ParenLeft, Group::Delimiter)),
            ')' => Some(symbol(")", Token::ParenRight, Group::Delimiter)),
            ',' => Some(symbol(",", Token::Comma, Group::Delimiter)),
            ';' => Some(symbol(";", Token::Semicolon, Group::Delimiter)),
            _ => None,
        }
    }
}

pub fn check_multi_keywords_front(s: &str) -> Option<Vec<u32>> {
    match s {
        "add" => Some(vec![2]),
        "alter" => Some(vec![2]),
        "create" => Some(vec![2, 3, 4]),
        "drop" => Some(vec![2]),
        "foreign" => Some(vec![2]),
        "full" => Some(vec![2]),
        "group" => Some(vec![2]),
        "inner" => Some(vec![2]),
        "insert" => Some(vec![2]),
        "is" => Some(vec![2, 3]),
        "left" => Some(vec![2]),
        "not" => Some(vec![2]),
        "order" => Some(vec![2]),
        "outer" => Some(vec![2]),
        "primary" => Some(vec![2]),
        "right" => Some(vec![2]),
        "select" => Some(vec![2]),
        "truncate" => Some(vec![2]),
        "union" => Some(vec![2]),
        _ => return None,
    }
}
