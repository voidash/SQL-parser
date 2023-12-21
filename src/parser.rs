use crate::lexer::*;
use crate::symbol::*;



use log::debug;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Symbol>,
}

#[derive(Debug)]
pub enum ParserError {
    CauseByLexer(LexerError),
    TokenLengthZero,
    SyntaxError(String),
}



impl Parser {
    fn new(message: &str) -> Result<Parser,ParserError> {
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

    pub fn parse(&self) -> Result<(), ParserError>{
        let mut iter = self.tokens.iter().peekable();
        match iter.peek() {
            Some(symbol) => match symbol.token {
                    Token::CreateDatabase => {
                        let _ = iter.next(); // "create database"

                        let db_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no db name")))?;
                        check_id(db_name_sym)?;

                        // sql.create_database(&db_name_sym.name)
                        //     .map_err(|e| ParserError::SQLError(e))?;

                        return Ok(());
                    }

                    Token::CreateTable => {
                        debug!("-> create table");
                        let table_name = iter.next().ok_or(ParserError::SyntaxError(String::from("No table Name Given")))?;

                        check_id(table_name)?;
                        


                        // sql.create_table(&table).map_err(|e| ParserError::SQLError(e))?;
                        return Ok(());
                    }
                    Token::InsertInto => {
                        debug!("-> insert into table");
                        // let (table_name, attrs, rows) = parser_insert_into_table(&mut iter)?;
                        // sql.insert_into_table(&table_name, attrs, rows)
                        //     .map_err(|e| ParserError::SQLError(e))?;
                        Ok(())
                    }
                    Token::Select => {
                        debug!("-> select table");
                        // sql.querydata = parse_select(&mut iter)?;
                        // sql.select().map_err(|e| ParserError::SQLError(e))?;
                        Ok(())
                    }
                    Token::DropTable => {
                        debug!("-> drop table");
                        let _ = iter.next(); // "drop table"
                        let tb_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no table name")))?;
                        check_id(tb_name_sym)?;

                        // sql.drop_table(&tb_name_sym.name)
                        //     .map_err(|e| ParserError::SQLError(e))?;
                        Ok(())
                    }
                    Token::DropDatabase => {
                        debug!("-> drop database");
                        let _ = iter.next(); // "drop database"
                        let db_name_sym = iter
                            .next()
                            .ok_or(ParserError::SyntaxError(String::from("no db name")))?;
                        check_id(db_name_sym)?;

                        // sql.drop_database(&db_name_sym.name)
                        //     .map_err(|e| ParserError::SQLError(e))?;
                        Ok(())
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

