use crate::lexer::*;
use crate::query::*;
use crate::symbol::*;
use std::iter::Peekable;
use std::slice::Iter;
use log::debug;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser {
    tokens: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParserError {
    CauseByLexer(LexerError),
    TokenLengthZero,
    SyntaxError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParserOutput {
    Select(QueryData),
    TooLazyToImplement
} 


impl Parser {
    pub fn new(message: &str) -> Result<Parser,ParserError> {
        let mut s: Scanner = Scanner::new(message);
        match s.scan_tokens() {
            Ok(tokens) => {
                if tokens.len() == 0 {
                    return Err(ParserError::TokenLengthZero);
                }
                 Ok(Parser { tokens })
            }
            Err(e) => Err(ParserError::CauseByLexer(e)),
        }
    }

    pub fn parse(&self) -> Result<ParserOutput, ParserError>{
        let mut iter = self.tokens.iter().peekable();
        match iter.peek() {
            Some(symbol) => match symbol.token {
                    Token::CreateDatabase => {
                        let _ = iter.next(); // "create database"

                        let db_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no db name")))?;
                        check_id(db_name_sym)?;


                        return Ok(ParserOutput::TooLazyToImplement);
                    }

                    Token::CreateTable => {
                        debug!("-> create table");
                        let table_name = iter.next().ok_or(ParserError::SyntaxError(String::from("No table Name Given")))?;

                        check_id(table_name)?;
                        

                        
                        return Ok(ParserOutput::TooLazyToImplement);
                    }
                    Token::InsertInto => {
                        debug!("-> insert into table");
                        
                        return Ok(ParserOutput::TooLazyToImplement);
                    }
                    Token::Select => {
                        debug!("-> select table");
                        let table = parse_select(&mut iter)?;


                       
                        Ok(ParserOutput::Select(table))
                    }
                    Token::DropTable => {
                        debug!("-> drop table");
                        let _ = iter.next(); // "drop table"
                        let tb_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no table name")))?;
                        check_id(tb_name_sym)?;

                        return Ok(ParserOutput::TooLazyToImplement);
                    }
                    Token::DropDatabase => {
                        debug!("-> drop database");
                        let _ = iter.next(); // "drop database"
                        let db_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no db name")))?;
                        check_id(db_name_sym)?;

                        return Ok(ParserOutput::TooLazyToImplement);
                    }
                    _ => {
                        return Err(ParserError::SyntaxError(String::from("unknown keyword")));
                    }
                },
                None => {
                    return Err(ParserError::SyntaxError(String::from("miss query")));
                }
        } 
    }
}

fn parse_select(iter: &mut Peekable<Iter<Symbol>>) -> Result<QueryData, ParserError> {
    let _ = iter.next(); // select

    let mut query_data = QueryData::new();

    if check_token(iter.peek(), Token::Distinct) {
        iter.next();
        query_data.is_distinct = true;
    }

    if check_token(iter.peek(), Token::Top) {
        iter.next(); // top

        let top_spec = match iter.next() {
            Some(s) => s.name.clone(),
            None => return Err(ParserError::SyntaxError(String::from("invalid select top syntax"))),
        };

        query_data.top = top_spec
                .parse::<u32>()
                .map_err(|_| ParserError::SyntaxError(String::from("invalid select top syntax")))?;
    }

    query_data.fields = get_id_list(iter, false)?;

    assert_token(iter.next(), Token::From)?;

    query_data.tables = get_id_list(iter, false)?;

    loop {
        match check_token(iter.peek(), Token::InnerJoin)
            || check_token(iter.peek(), Token::FullOuterJoin)
            || check_token(iter.peek(), Token::LeftJoin)
            || check_token(iter.peek(), Token::RightJoin)
        {
            true => {
                println!("testing");
                let mut join = Join::new(&iter.next().unwrap().name);
                match iter.next() {
                    Some(s) if s.token == Token::Identifier => join.table = s.name.clone(),
                    Some(_) | None => return Err(ParserError::SyntaxError(String::from("invalid select join syntax"))),
                }

                assert_token(iter.next(), Token::On)?;

                assert_token(iter.next(), Token::Identifier)?;
                query_data.joins.push(join);
            }
            false => break,
        }
    }

    if check_token(iter.peek(), Token::Where) {
        loop {
            assert_token(iter.next(), Token::Where)?;

            let mut where_clause: WhereClause = WhereClause::new();
            if check_token(iter.peek(), Token::Identifier) {
                where_clause.left = BinOp::Assigned(iter.next().unwrap().name.clone());

                if check_token(iter.peek(), Token::LT) ||
                   check_token(iter.peek(), Token::LE) ||
                   check_token(iter.peek(), Token::EQ) ||
                   check_token(iter.peek(), Token::NE) ||
                   check_token(iter.peek(), Token::GT) ||
                   check_token(iter.peek(), Token::GE){
                        where_clause.operation = iter.next().unwrap().name.clone(); 
                   } else {
                        return Err(ParserError::SyntaxError(String::from("WHERE is malformed")));
                   }
                
                if check_token(iter.peek(), Token::Identifier) {
                    where_clause.right = BinOp::Assigned(iter.next().unwrap().name.clone())
                }else{
                        return Err(ParserError::SyntaxError(String::from("WHERE is malformed")));
                }

                }else{
                    return Err(ParserError::SyntaxError(String::from("WHERE is malformed")));
                }

                query_data.condition.push(where_clause);

                if !check_token(iter.peek(), Token::Where) {
                    break;
                }
        }

    }
    assert_token(iter.next(), Token::Semicolon)?;
    Ok(query_data)

}

/// Check if the symbol is an identifier
#[inline]
fn check_id(sym: &Symbol) -> Result<(), ParserError> {
    if sym.group != Group::Identifier {
        return Err(ParserError::SyntaxError(format!("{} is not an", &sym.name)));
    }
    Ok(())
}

/// Check if the next symbol is the expected token.
#[inline]
fn check_token(sym: Option<&&Symbol>, token: Token) -> bool {
    match sym {
        Some(s) if s.token == token => true,
        Some(_) | None => false,
    }
}

/// Assert the symbol is the expected token.
#[inline]
fn assert_token(sym: Option<&Symbol>, token: Token) -> Result<(), ParserError> {
    if sym
        .ok_or(ParserError::SyntaxError(String::from("invalid syntax")))?
        .token
        != token
    {
        return Err(ParserError::SyntaxError(String::from("invalid syntax")));
    }
    Ok(())
}

fn get_id_list(iter: &mut Peekable<Iter<Symbol>>, is_parent: bool) -> Result<Vec<String>, ParserError> {
    let mut v = vec![];
    if is_parent {
        assert_token(iter.next(), Token::ParenLeft)?;
    }
    loop {
        match iter.next() {
            Some(s) if s.token == Token::Identifier => {
                v.push(s.name.clone());
                match iter.peek() {
                    Some(s) if s.token == Token::Comma => {
                        iter.next();
                        continue;
                    }
                    Some(_) | None => break,
                }
            }
            Some(_) | None => return Err(ParserError::SyntaxError(String::from("invalid syntax"))),
        }
    }
    if is_parent {
        assert_token(iter.next(), Token::ParenRight)?;
    }
    Ok(v)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn check_parse() {
        let parsed_values = Parser::new(r#"
            select * from something left join A on B where A = "EASE";
        "#).unwrap();

        println!("{:?}",&parsed_values.parse().unwrap());

    }
    }
