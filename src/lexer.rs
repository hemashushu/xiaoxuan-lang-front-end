/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use std::char;

use crate::error::Error;
use crate::token::Location;
use crate::token::Token;
use crate::token::TokenDetail;

pub fn tokenize(text: &str) -> Result<Vec<TokenDetail>, Error> {
    let vec_char: Vec<char> = text.chars().collect();

    let mut chars = &vec_char[..];
    let mut token_details: Vec<TokenDetail> = vec![];

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    ' ' | '\t' => {
                        // whitespace
                        rest
                    }

                    '\r' => {
                        // new line
                        if is_char('\n', rest) {
                            add_token_detail(&mut token_details, new_token_detail(Token::NewLine));
                            move_forword(rest, 1)
                        } else {
                            add_token_detail(&mut token_details, new_token_detail(Token::NewLine));
                            rest
                        }
                    }

                    '\n' | ';' => {
                        // new line
                        add_token_detail(&mut token_details, new_token_detail(Token::NewLine));
                        rest
                    }

                    '/' => {
                        if is_char('/', rest) {
                            // line comment
                            let post_rest = skip_line_comment(rest);
                            post_rest
                        } else if is_char('*', rest) {
                            /* comment */
                            let post_rest = skip_comment(rest)?;
                            post_rest
                        } else {
                            // `/`
                            add_token_detail(&mut token_details, new_token_detail(Token::Slash));
                            rest
                        }
                    }

                    '{' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::LeftBrace));
                        rest
                    }
                    '}' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::RightBrace));
                        rest
                    }
                    '=' => {
                        if is_char('=', rest) {
                            // `==`
                            add_token_detail(&mut token_details, new_token_detail(Token::Equal));
                            move_forword(rest, 1)
                        } else {
                            // `=`
                            add_token_detail(&mut token_details, new_token_detail(Token::Assign));
                            rest
                        }
                    }
                    '>' => {
                        if is_char('=', rest) {
                            // `>=`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::GreaterThanOrEqual),
                            );
                            move_forword(rest, 1)
                        } else {
                            // `>`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::GreaterThan),
                            );
                            rest
                        }
                    }
                    '|' => {
                        if is_char('|', rest) {
                            // `||`
                            add_token_detail(&mut token_details, new_token_detail(Token::LogicOr));
                            move_forword(rest, 1)
                        } else {
                            // `|`
                            add_token_detail(&mut token_details, new_token_detail(Token::Pipe));
                            rest
                        }
                    }
                    '&' => {
                        if is_char('&', rest) {
                            // `&&`
                            add_token_detail(&mut token_details, new_token_detail(Token::LogicAnd));
                            move_forword(rest, 1)
                        } else {
                            // `&`
                            add_token_detail(&mut token_details, new_token_detail(Token::Combine));
                            rest
                        }
                    }
                    '!' => {
                        if is_char('=', rest) {
                            // `!=`
                            add_token_detail(&mut token_details, new_token_detail(Token::NotEqual));
                            move_forword(rest, 1)
                        } else {
                            // `!`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::Exclamation),
                            );
                            rest
                        }
                    }
                    '<' => {
                        if is_char('=', rest) {
                            // `<=`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::LessThanOrEqual),
                            );
                            move_forword(rest, 1)
                        } else {
                            // `<`
                            add_token_detail(&mut token_details, new_token_detail(Token::LessThan));
                            rest
                        }
                    }
                    '+' => {
                        if is_char('+', rest) {
                            // `++`
                            add_token_detail(&mut token_details, new_token_detail(Token::Concat));
                            move_forword(rest, 1)
                        } else {
                            // `+`
                            add_token_detail(&mut token_details, new_token_detail(Token::Plus));
                            rest
                        }
                    }
                    '-' => {
                        if is_char('>', rest) {
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::OptionalAnd),
                            );
                            move_forword(rest, 1)
                        } else {
                            add_token_detail(&mut token_details, new_token_detail(Token::Minus));
                            rest
                        }
                    }
                    '*' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::Asterisk));
                        rest
                    }
                    '?' => {
                        if is_char('?', rest) {
                            // `??`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::OptionalOr),
                            );
                            move_forword(rest, 1)
                        } else {
                            // `?`
                            add_token_detail(&mut token_details, new_token_detail(Token::Unwrap));
                            rest
                        }
                    }
                    '^' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::Cast));
                        rest
                    }
                    '@' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::At));
                        rest
                    }
                    '.' => {
                        if is_chars(['.', '.'], rest) {
                            // `...`
                            add_token_detail(&mut token_details, new_token_detail(Token::Ellipsis));
                            move_forword(rest, 2)
                        } else if is_chars(['.', '='], rest) {
                            // `..=`
                            add_token_detail(
                                &mut token_details,
                                new_token_detail(Token::IntervalInclusive),
                            );
                            move_forword(rest, 2)
                        } else if is_char('.', rest) {
                            // `..`
                            add_token_detail(&mut token_details, new_token_detail(Token::Interval));
                            move_forword(rest, 1)
                        } else {
                            // `.`
                            add_token_detail(&mut token_details, new_token_detail(Token::Dot));
                            rest
                        }
                    }
                    '[' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::LeftBracket));
                        rest
                    }
                    ']' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::RightBracket));
                        rest
                    }
                    '(' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::LeftParen));
                        rest
                    }
                    ')' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::RightParen));
                        rest
                    }

                    ',' => {
                        add_token_detail(&mut token_details, new_token_detail(Token::Comma));
                        rest
                    }

                    '\'' => {
                        if is_chars(['\'', '\''], rest) {
                            // `'''...'''` ????????????
                            let (_, post_rest) = lex_document_comment(rest)?;
                            post_rest
                        } else {
                            // `'char'`
                            let (token_detail, post_rest) = lex_char(rest)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        }
                    }

                    '"' => {
                        if is_chars(['"', '"'], rest) {
                            // `"""..."""`
                            let (token_detail, post_rest) = lex_raw_string(rest)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        } else {
                            // `"string"`
                            let (token_detail, post_rest) = lex_string(rest)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        }
                    }

                    '`' => {
                        // `template string`
                        let (token_detail, post_rest) = lex_template_string(rest)?;
                        add_token_detail(&mut token_details, token_detail);
                        post_rest
                    }

                    '0' => {
                        if is_char('x', rest) {
                            // `0x...`??? ??????????????????
                            let (token_detail, post_rest) = lex_16_radix_integer(rest)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        } else if is_char('b', rest) {
                            // `0b...`??? ???????????????
                            let (token_detail, post_rest) = lex_2_radix_integer(rest)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        } else if is_char('.', rest) {
                            if is_char('.', rest) {
                                // ?????????????????????????????? `0` ??????????????????
                                add_token_detail(
                                    &mut token_details,
                                    new_token_detail(Token::Integer(0)),
                                );
                                rest
                            } else {
                                // `0.xx`??? ??????????????? 0 ????????????
                                let (token_detail, post_rest) = lex_zero_point_float(rest)?;
                                add_token_detail(&mut token_details, token_detail);
                                post_rest
                            }
                        } else {
                            match rest.first() {
                                Some(second_char)
                                    if is_valid_letter_of_identifier_or_keyword(*second_char) =>
                                {
                                    // ?????? 0 ??????????????????????????????????????????????????????????????????
                                    return Err(Error::LexerError(
                                        "invalid identifier".to_string(),
                                    ));
                                }
                                _ => {
                                    // ???????????? 0
                                    add_token_detail(
                                        &mut token_details,
                                        new_token_detail(Token::Integer(0)),
                                    );
                                    rest
                                }
                            }
                        }
                    }

                    '#' => {
                        match rest.first() {
                            Some(second_char)
                                if is_valid_first_letter_of_identifier_or_keyword(*second_char) =>
                            {
                                // `#hash_string`
                                let (token_detail, post_rest) = lex_hash_string(rest)?;
                                add_token_detail(&mut token_details, token_detail);
                                post_rest
                            }
                            Some(second_char) if *second_char == '[' => {
                                // `#[...]`
                                let (token_detail, post_rest) = lex_attribute(rest)?;
                                add_token_detail(&mut token_details, token_detail);
                                post_rest
                            }
                            _ => return Err(Error::LexerError("invalid char '#'".to_string())),
                        }
                    }

                    ':' => {
                        match rest.first() {
                            Some(second_char) => {
                                if *second_char == ':' {
                                    // `::`
                                    add_token_detail(
                                        &mut token_details,
                                        new_token_detail(Token::Separator),
                                    );
                                    move_forword(rest, 1)
                                } else if is_valid_first_letter_of_identifier_or_keyword(
                                    *second_char,
                                ) {
                                    match lex_named_operator(rest) {
                                        Ok((token_detail, post_rest)) => {
                                            // `:name_operator:`
                                            add_token_detail(&mut token_details, token_detail);
                                            post_rest
                                        }
                                        Err(_) => {
                                            // `:`
                                            add_token_detail(
                                                &mut token_details,
                                                new_token_detail(Token::Colon),
                                            ); // ":"
                                            rest
                                        }
                                    }
                                } else {
                                    // `:`
                                    add_token_detail(
                                        &mut token_details,
                                        new_token_detail(Token::Colon),
                                    );
                                    rest
                                }
                            }
                            None => {
                                // `:`
                                add_token_detail(
                                    &mut token_details,
                                    new_token_detail(Token::Colon),
                                );
                                rest
                            }
                        }
                    }

                    _ => {
                        if is_none_zero_number(*first) {
                            // ?????????????????????????????????
                            let (token_detail, post_rest) = lex_number(chars)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        } else if is_valid_first_letter_of_identifier_or_keyword(*first) {
                            // ????????????????????????
                            let (token_detail, post_rest) = lex_identifier_or_keyword(chars)?;
                            add_token_detail(&mut token_details, token_detail);
                            post_rest
                        } else {
                            // ??????????????????
                            return Err(Error::LexerError(format!("invalid char '{}'", first)));
                        }
                    }
                };
            }
            None => break,
        };
    }

    Ok(token_details)
}

fn skip_line_comment(source_chars: &[char]) -> &[char] {
    // ?????????
    // ???????????????????????????
    // - ?????????`\n`???`\r\n` ?????? `\r`???
    // - ????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => match *first {
                '\r' => {
                    if is_char('\n', rest) {
                        // `\r\n`
                        end_pos += 1;
                        break;
                    } else {
                        // `\r`
                        break;
                    }
                }
                '\n' => {
                    break;
                }
                _ => {
                    end_pos += 1;
                    rest
                }
            },
            None => {
                break;
            }
        }
    }

    // ???????????????????????????????????????????????????rest??????????????????????????? Token::NewLine
    &source_chars[end_pos..]
}

fn skip_comment(source_chars: &[char]) -> Result<&[char], Error> {
    // ????????????
    // ??????????????????????????????????????? `*/`

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => match *first {
                '*' => {
                    if is_char('/', rest) {
                        // `*/`
                        end_pos += 1;
                        break;
                    } else {
                        // ????????????????????????????????????
                        end_pos += 1;
                        rest
                    }
                }
                _ => {
                    end_pos += 1;
                    rest
                }
            },
            None => {
                // ?????????????????????????????????
                return Err(Error::LexerError(
                    "expected comment ending symbol".to_string(),
                ));
            }
        }
    }

    // /*foo bar*/
    //           ^-------- ?????????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok(rest)
}

fn lex_document_comment(source_chars: &[char]) -> Result<(String, &[char]), Error> {
    // ????????????
    // ?????? `????????????` ??????????????? `?????????`
    //
    // e.g.
    // ?????????foo bar?????????
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\'' => {
                        if is_chars(['\'', '\''], rest) {
                            // ????????? `'''`
                            end_pos += 2;
                            break;
                        } else {
                            // ??????????????????????????????????????????????????????
                            end_pos += 1;
                            rest
                        }
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected document comment ending symbol".to_string(),
                ));
            }
        }
    }

    let value_chars = &source_chars[2..end_pos - 2];
    let value = value_chars.iter().collect::<String>();

    // '''foo bar'''
    //             ^-------- ?????????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((value, rest))
}

fn lex_char(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ???????????????
    // ?????? `???????????????` ??????????????? `'`??????????????? `\'`
    //
    // e.g.
    // 'a'
    // '\n'
    // '\xHH'
    // '\u{H}' ~ '\u{HHHHHH}'
    // '\omega'
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if is_char('\'', rest) {
                            // ????????? '\''
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // ???????????????????????????
                            // todo::
                            end_pos += 1;
                            rest
                        }
                    }
                    '\'' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected char literal ending symbol".to_string(),
                ));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];

    // todo:: ??????????????????
    // todo:: ????????????????????????

    // ?????? end_pos ???????????? `'` ??????
    // ???????????????????????? `'` ??????????????????

    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::Char(value_chars[0])), rest))
}

fn lex_string(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ??????????????????
    // ?????? `??????????????????` ??????????????? `"`??????????????? `\"`
    //
    // e.g.
    // "foo bar"
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if is_char('"', rest) {
                            // ????????? '"'
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // ???????????????????????????
                            end_pos += 1;
                            rest
                        }
                    }
                    '\"' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected string literal ending symbol".to_string(),
                ));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // todo:: ??????????????????

    // ?????? end_pos ???????????? `"` ??????
    // ???????????????????????? `"` ??????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::GeneralString(value)), rest))
}

fn lex_raw_string(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ????????????????????????
    // ?????? `????????????????????????` ??????????????? `"""`
    //
    // e.g.
    // """foo bar"""
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '"' => {
                        if is_chars(['"', '"'], rest) {
                            // ????????? '"""'
                            end_pos += 2;
                            break;
                        } else {
                            // ??????????????????????????????????????????????????????
                            end_pos += 1;
                            rest
                        }
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected raw string literal ending symbol".to_string(),
                ));
            }
        }
    }

    // todo:: ?????????????????????????????????

    let value_chars = &source_chars[2..end_pos - 2];
    let value = value_chars.iter().collect::<String>();

    // """foo bar"""
    //             ^-------- ?????????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::GeneralString(value)), rest))
}

fn lex_template_string(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ????????????????????????
    // ?????? `????????????????????????` ??????????????? '`'??????????????? '`'
    //
    // e.g.
    // `foo bar`
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if is_char('`', rest) {
                            // ????????? '`'
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // ???????????????????????????
                            end_pos += 1;
                            rest
                        }
                    }
                    '`' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected template string literal ending symbol".to_string(),
                ));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // todo:: ??????????????????

    // todo:: ?????????????????????????????????

    // ?????? end_pos ???????????? '`' ??????
    // ???????????????????????? '`' ??????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::TemplateString(value)), rest))
}

fn lex_hash_string(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ???????????????
    // ?????????????????????
    //
    // e.g.
    // #foo_bar
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // ???????????????????????????????????????????????????????????????????????????????????????

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) if is_valid_letter_of_identifier_or_keyword(*first) => {
                end_pos += 1;
                rest
            }
            Some(_) => {
                break;
            }
            None => {
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // ?????? end_pos ??????????????????????????????????????????
    // ??????????????????????????????????????????????????????????????? end_pos ??????????????????
    let rest = move_forword(source_chars, end_pos);
    Ok((new_token_detail(Token::HashString(value)), rest))
}

fn lex_named_operator(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ???????????????
    // ??????????????????????????????????????? `:` ??????
    //
    // e.g.
    // :foo_bar:
    //  ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // ???????????????????????????????????????????????????????????????????????????????????????

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                if *first == ':' {
                    // ??????????????????
                    break;
                } else if is_valid_letter_of_identifier_or_keyword(*first) {
                    // ?????????????????????????????????
                    end_pos += 1;
                    rest
                } else {
                    // ??????????????????????????????
                    return Err(Error::LexerError("invalid identifier letter".to_string()));
                }
            }
            None => {
                // ????????????????????????????????????
                return Err(Error::LexerError(
                    "expected named operator ending symbol".to_string(),
                ));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // ?????? end_pos ???????????? `:` ??????
    // ???????????????????????? `:` ??????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::NamedOperator(value)), rest))
}

fn lex_attribute(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ???????????????attribute????????? Java ??? annotation???
    // ?????? `??????` ??????????????? `]`
    //
    // e.g.
    // "#[test]"
    //   ^-------- ?????????????????????

    // let mut chars = &source_chars[1..];
    // let mut end_pos: usize = 1;

    let end_pos = match source_chars.iter().position(|c| *c == ']') {
        Some(pos) => pos,
        None => {
            // ????????????????????????????????????
            return Err(Error::LexerError(
                "expected attribute ending symbol".to_string(),
            ));
        }
    };

    let value_chars = &source_chars[1..end_pos];
    let value = value_chars.iter().collect::<String>();

    // ?????? end_pos ???????????? `]` ??????
    // ???????????????????????? `]` ??????????????????
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token_detail(Token::Attribute(value)), rest))
}

fn lex_16_radix_integer(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    todo!()
}

fn lex_2_radix_integer(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    todo!()
}

fn lex_zero_point_float(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    todo!()
}

fn lex_number(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ?????????????????????????????????
    // ?????????????????????
    //
    // e.g.
    // 123
    // 1_234
    // 3i
    // 9.9i
    // 8'xff
    // 4'b01_10
    // 2.71828
    // 6.626e-34
    // ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // ???????????????????????????????????????????????????????????????????????????????????????

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                match *first {
                    '0'..='9' | '_' => {
                        // ???????????????????????????
                        end_pos += 1;
                        rest
                    }
                    '.' => {
                        if is_char('.', rest) {
                            // ?????????????????????????????? `..`
                            break;
                        } else {
                            // ??????????????????
                            return continue_lex_float_number(
                                source_chars[..end_pos].to_vec(),
                                rest,
                            );
                        }
                    }
                    '\'' => {
                        if is_chars(['\'', '\''], rest) {
                            // ?????????????????????
                            break;
                        } else {
                            // ??????????????????
                            return continue_lex_bit_number(source_chars[..end_pos].to_vec(), rest);
                        }
                    }
                    'i' => {
                        return continue_lex_imaginary_number(
                            source_chars[..end_pos].to_vec(),
                            rest,
                        );
                    }
                    'e' => {
                        return continue_lex_float_number_exponent(
                            source_chars[..end_pos].to_vec(),
                            rest,
                        );
                    }
                    _ => {
                        // ????????????????????????
                        break;
                    }
                }
            }
            None => {
                // ????????????
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value_string = value_chars
        .iter()
        .filter(|c| **c != '_') // ?????????????????????????????????
        .collect::<String>();

    // ???????????????????????????
    let value: i64 = value_string
        .parse()
        .map_err(|_| Error::LexerError("invalid integer number".to_string()))?;

    // ?????? end_pos ??????????????????????????????????????????
    // ???????????????????????????????????????????????????????????? end_pos ??????????????????
    let rest = move_forword(source_chars, end_pos);

    Ok((new_token_detail(Token::Integer(value)), rest))
}

fn extend_vec_with_with_separator_and_char_slice(
    mut left: Vec<char>,
    separator: char,
    right: &[char],
) -> Vec<char> {
    left.push(separator);
    left.extend_from_slice(right);
    left
}

fn continue_lex_float_number(
    previous_chars: Vec<char>,
    remain_chars: &[char],
) -> Result<(TokenDetail, &[char]), Error> {
    // ?????????????????????????????????
    // 123.456
    // ___ ___ remain_chars
    //   |____ previous_chars

    let mut chars = remain_chars;
    let mut end_pos: usize = 0;

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                match *first {
                    '0'..='9' | '_' => {
                        // ???????????????????????????
                        end_pos += 1;
                        rest
                    }
                    '.' => return Err(Error::LexerError("invalid float number".to_string())),
                    '\'' => return Err(Error::LexerError("invalid bit number".to_string())),
                    'i' => {
                        let extend_chars = extend_vec_with_with_separator_and_char_slice(
                            previous_chars,
                            '.',
                            &remain_chars[..end_pos],
                        );
                        return continue_lex_imaginary_number(extend_chars, rest);
                    }
                    'e' => {
                        let extend_chars = extend_vec_with_with_separator_and_char_slice(
                            previous_chars,
                            '.',
                            &remain_chars[..end_pos],
                        );
                        return continue_lex_float_number_exponent(extend_chars, rest);
                    }
                    _ => {
                        // ????????????????????????
                        break;
                    }
                }
            }
            None => {
                // ????????????
                break;
            }
        }
    }

    let value_chars = extend_vec_with_with_separator_and_char_slice(
        previous_chars,
        '.',
        &remain_chars[..end_pos],
    );

    let value_string = value_chars
        .iter()
        .filter(|c| **c != '_') // ?????????????????????????????????
        .collect::<String>();

    // ???????????????????????????
    let value: f64 = value_string
        .parse()
        .map_err(|_| Error::LexerError("invalid float number".to_string()))?;

    // ?????? end_pos ???????????????????????????????????????
    // ???????????????????????????????????????????????????????????? end_pos ??????????????????
    let rest = move_forword(remain_chars, end_pos);

    Ok((new_token_detail(Token::Float(value)), rest))
}

fn continue_lex_imaginary_number(
    previous_chars: Vec<char>,
    remain_chars: &[char],
) -> Result<(TokenDetail, &[char]), Error> {
    // ??????????????????
    // 123i...
    // ___ ___ remain_chars
    //   |____ previous_chars

    let value_string = previous_chars
        .iter()
        .filter(|c| **c != '_') // ?????????????????????????????????
        .collect::<String>();

    // ???????????????????????????
    let value: f64 = value_string
        .parse()
        .map_err(|_| Error::LexerError("invalid float number".to_string()))?;

    Ok((new_token_detail(Token::Imaginary(value)), remain_chars))
}

fn continue_lex_bit_number(
    previous_chars: Vec<char>,
    remain_chars: &[char],
) -> Result<(TokenDetail, &[char]), Error> {
    todo!()
}

fn continue_lex_float_number_exponent(
    previous_chars: Vec<char>,
    remain_chars: &[char],
) -> Result<(TokenDetail, &[char]), Error> {
    // ???????????? e ????????????
    // 123e-30
    // ___ ___ remain_chars
    //   |____ previous_chars

    let mut chars = remain_chars;
    let mut end_pos: usize = 0;

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                match *first {
                    '-' => {
                        if end_pos == 0 {
                            end_pos += 1;
                            rest
                        } else {
                            return Err(Error::LexerError("invalid exponent number".to_string()));
                        }
                    }
                    '0'..='9' | '_' => {
                        // ???????????????????????????
                        end_pos += 1;
                        rest
                    }
                    '.' => {
                        if is_char('.', rest) {
                            // ?????????????????????????????? `..`
                            break;
                        } else {
                            // ????????? "??????????????????" ????????????
                            return Err(Error::LexerError("unsupport float exponent".to_string()));
                        }
                    }
                    '\'' => return Err(Error::LexerError("invalid bit number".to_string())),
                    'i' => {
                        let extend_chars = extend_vec_with_with_separator_and_char_slice(
                            previous_chars,
                            'e',
                            &remain_chars[..end_pos],
                        );
                        return continue_lex_imaginary_number(extend_chars, rest);
                    }
                    'e' => return Err(Error::LexerError("invalid exponent number".to_string())),
                    _ => {
                        // ????????????????????????
                        break;
                    }
                }
            }
            None => {
                // ????????????
                break;
            }
        }
    }

    let value_chars = extend_vec_with_with_separator_and_char_slice(
        previous_chars,
        'e',
        &remain_chars[..end_pos],
    );

    let value_string = value_chars
        .iter()
        .filter(|c| **c != '_') // ?????????????????????????????????
        .collect::<String>();

    // ???????????????????????????
    let value: f64 = value_string
        .parse()
        .map_err(|_| Error::LexerError("invalid float number".to_string()))?;

    // ?????? end_pos ???????????????????????????????????????
    // ???????????????????????????????????????????????????????????? end_pos ??????????????????
    let rest = move_forword(remain_chars, end_pos);

    Ok((new_token_detail(Token::Float(value)), rest))
}

fn lex_identifier_or_keyword(source_chars: &[char]) -> Result<(TokenDetail, &[char]), Error> {
    // ????????????????????????
    // ?????????????????????
    //
    // e.g.
    // foo_bar
    // ^-------- ?????????????????????

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // ???????????????????????????????????????????????????????????????????????????????????????

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                if is_valid_letter_of_identifier_or_keyword(*first) {
                    // ?????????????????????????????????
                    end_pos += 1;
                    rest
                } else {
                    // ???????????????????????????????????????????????????
                    break;
                }
            }
            None => {
                // ????????????
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // ?????? end_pos ??????????????????????????????????????????
    // ??????????????????????????????????????????????????????????????? end_pos ??????????????????
    let rest = move_forword(source_chars, end_pos);

    match lookup_keyword(&value) {
        Some(token) => Ok((new_token_detail(token), rest)),
        None => Ok((new_token_detail(Token::Identifier(value)), rest)),
    }
}

fn is_none_zero_number(c: char) -> bool {
    match c {
        '1'..='9' => true,
        _ => false,
    }
}

// ??????????????????????????????????????????????????????
fn is_valid_first_letter_of_identifier_or_keyword(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

// ????????????????????????????????????????????????????????????????????????????????????
fn is_valid_letter_of_identifier_or_keyword(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }
}

fn is_char(expected: char, source_chars: &[char]) -> bool {
    match source_chars.first() {
        Some(first_char) if *first_char == expected => true,
        _ => false,
    }
}

fn is_chars(expected: [char; 2], source_chars: &[char]) -> bool {
    match source_chars.split_first() {
        Some((first, rest)) if *first == expected[0] => is_char(expected[1], rest),
        _ => false,
    }
}

fn move_forword(source_chars: &[char], count: usize) -> &[char] {
    &source_chars[count..]
}

fn add_token_detail(
    token_details: &mut Vec<TokenDetail>,
    token_detail: TokenDetail,
) -> &mut Vec<TokenDetail> {
    token_details.push(token_detail);
    token_details
}

fn new_token_detail(token: Token) -> TokenDetail {
    TokenDetail {
        location: new_location(),
        token,
    }
}

fn new_location() -> Location {
    // todo::
    // Location ?????????????????????????????????
    Location {
        file_id: 0,
        start: 0,
        end: 0,
    }
}

// ?????????????????????????????????????????????
fn lookup_keyword(name: &str) -> Option<Token> {
    match name {
        // ?????????
        "true" => Some(Token::Boolean(true)),
        "false" => Some(Token::Boolean(false)),

        // ?????????
        "do" => Some(Token::Do),
        "join" => Some(Token::Join),

        "let" => Some(Token::Let),
        "fn" => Some(Token::Fn),
        "sign" => Some(Token::Sign),

        "if" => Some(Token::If),
        "then" => Some(Token::Then),
        "else" => Some(Token::Else),
        "for" => Some(Token::For),
        "next" => Some(Token::Next),
        "each" => Some(Token::Each),
        "in" => Some(Token::In),

        "branch" => Some(Token::Branch),
        "match" => Some(Token::Match),
        "case" => Some(Token::Case),
        "default" => Some(Token::Default),
        "where" => Some(Token::Where),
        "only" => Some(Token::Only),
        // "as" => Some(Token::As),
        "into" => Some(Token::Into),
        "regular" => Some(Token::Regular),
        "template" => Some(Token::Template),

        "function" => Some(Token::Function),
        "type" => Some(Token::Type),
        "which" => Some(Token::Which),
        "empty" => Some(Token::Empty),
        "pattern" => Some(Token::Pattern),
        "limit" => Some(Token::Limit),

        // "namespace" => Some(Token::Namespace),
        "use" => Some(Token::Use),
        "const" => Some(Token::Const),
        "enum" => Some(Token::Enum),
        "struct" => Some(Token::Struct),
        "union" => Some(Token::Union),
        "trait" => Some(Token::Trait),
        "impl" => Some(Token::Impl),
        "alias" => Some(Token::Alias),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::new_location,
        token::{Token, TokenDetail},
    };

    use super::tokenize;

    // ????????????

    fn token_details_to_string(token_details: &[TokenDetail]) -> Vec<String> {
        let strings: Vec<String> = token_details.iter().map(|t| t.token.to_string()).collect();
        strings
    }

    #[test]
    fn test_whitespace() {
        let token_details = tokenize(" \t").unwrap();
        assert_eq!(token_details.len(), 0);
    }

    #[test]
    fn test_comment() {
        let tokens1 = tokenize("/").unwrap();
        assert_eq!(token_details_to_string(&tokens1), vec!["/"]);

        let tokens2 = tokenize("/ // comment").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["/"]);

        let tokens3 = tokenize("/ // comm/ent\n/").unwrap();
        assert_eq!(token_details_to_string(&tokens3), vec!["/", "\n", "/"]);

        // ??????????????????
        let tokens4 = tokenize("1/*com//me**nt*/2").unwrap();
        assert_eq!(token_details_to_string(&tokens4), vec!["1", "2"]);

        // ??????????????????
        let tokens5 = tokenize("1'''docu//ment''com'ment/*foo*/bar'''2").unwrap();
        assert_eq!(token_details_to_string(&tokens5), vec!["1", "2"]);
    }

    #[test]
    fn test_new_line() {
        let tokens1 = tokenize("\n \r\n").unwrap();
        assert_eq!(token_details_to_string(&tokens1), vec!["\n", "\n"]);

        let tokens2 = tokenize("; \n").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["\n", "\n"]);
    }

    #[test]
    fn test_identifier() {
        let tokens1 = tokenize("a ab a_b a123 _ _a a_").unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec!["a", "ab", "a_b", "a123", "_", "_a", "a_"]
        );
    }

    #[test]
    fn test_integer_literal() {
        let tokens1 = tokenize("123").unwrap();
        assert_eq!(
            tokens1,
            vec![TokenDetail {
                token: Token::Integer(123),
                location: new_location()
            }]
        );
        assert_eq!(token_details_to_string(&tokens1), vec!["123"]);

        let tokens2 = tokenize("1 100 1_234 1_2_3").unwrap();
        assert_eq!(
            token_details_to_string(&tokens2),
            vec!["1", "100", "1234", "123"]
        );

        // todo:: ?????? 16 ????????? 2 ????????????????????????
    }

    #[test]
    fn test_float_literal() {
        let tokens1 = tokenize("3.14").unwrap();
        assert_eq!(
            tokens1,
            vec![TokenDetail {
                token: Token::Float(3.14),
                location: new_location()
            }]
        );
        assert_eq!(token_details_to_string(&tokens1), vec!["3.14"]);

        let tokens2 = tokenize("27.182818").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["27.182818"]);

        let tokens3 = tokenize("5e2").unwrap();
        assert_eq!(token_details_to_string(&tokens3), vec!["500"]);

        let tokens4 = tokenize("1.6e2").unwrap();
        assert_eq!(token_details_to_string(&tokens4), vec!["160"]);

        let tokens5 = tokenize("1.6e-2").unwrap();
        assert_eq!(token_details_to_string(&tokens5), vec!["0.016"]);

        // ?????????????????????????????????????????????????????????
        let tokens6 = tokenize("1..10").unwrap();
        assert_eq!(token_details_to_string(&tokens6), vec!["1", "..", "10"]);

        let tokens7 = tokenize("1..=9").unwrap();
        assert_eq!(token_details_to_string(&tokens7), vec!["1", "..=", "9"]);

        let tokens8 = tokenize("[0..100]").unwrap();
        assert_eq!(
            token_details_to_string(&tokens8),
            vec!["[", "0", "..", "100", "]"]
        );
    }

    #[test]
    fn test_imaginary_literal() {
        let tokens1 = tokenize("5i").unwrap();
        assert_eq!(
            tokens1,
            vec![TokenDetail {
                token: Token::Imaginary(5.0),
                location: new_location()
            }]
        );
        assert_eq!(token_details_to_string(&tokens1), vec!["5i"]);

        let tokens2 = tokenize("3.14i").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["3.14i"]);

        let tokens3 = tokenize("5e2i").unwrap();
        assert_eq!(token_details_to_string(&tokens3), vec!["500i"]);

        let tokens4 = tokenize("1.6e2i").unwrap();
        assert_eq!(token_details_to_string(&tokens4), vec!["160i"]);

        let tokens5 = tokenize("1.6e-2i").unwrap();
        assert_eq!(token_details_to_string(&tokens5), vec!["0.016i"]);
    }

    #[test]
    fn test_bit_literal() {
        // todo::
    }

    #[test]
    fn test_boolean_literal() {
        let tokens1 = tokenize("true").unwrap();
        assert_eq!(
            tokens1,
            vec![TokenDetail {
                token: Token::Boolean(true),
                location: new_location()
            }]
        );
        assert_eq!(token_details_to_string(&tokens1), vec!["true"]);

        let tokens2 = tokenize("false").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["false"]);
    }

    #[test]
    fn test_char_literal() {
        let tokens1 = tokenize("'a' 'b'").unwrap();
        assert_eq!(token_details_to_string(&tokens1), vec!["'a'", "'b'"]);

        // todo:: ??????????????????
    }

    #[test]
    fn test_generall_string_literal() {
        let tokens1 = tokenize(r#""foo" "b'a`r" "a\"b""#).unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec!["\"foo\"", "\"b'a`r\"", "\"a\\\"b\""]
        );

        // ?????????????????????
        let tokens2 = tokenize("\"foo\n    bar\"").unwrap();
        assert_eq!(token_details_to_string(&tokens2), vec!["\"foo\n    bar\""]);

        // todo:: ??????????????????

        // ?????????????????????
        let tokens3 = tokenize(r#"11"""foo bar"""22"#).unwrap();
        assert_eq!(
            token_details_to_string(&tokens3),
            vec!["11", "\"foo bar\"", "22"]
        );

        // todo:: ??????????????????????????????????????????????????????
    }

    #[test]
    fn test_template_string_literal() {
        let tokens1 = tokenize(r#" `foo` `b'a"r` `a\`b` `user: {{name}}`"#).unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec!["`foo`", "`b'a\"r`", "`a\\`b`", "`user: {{name}}`"]
        );

        // todo:: ??????????????????

        // todo:: ??????????????????????????????????????????????????????
    }

    #[test]
    fn test_hash_string_literal() {
        let tokens1 = tokenize("\"foo\" #foo #_bar").unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec!["\"foo\"", "#foo", "#_bar"]
        );
    }

    #[test]
    fn test_named_operator() {
        let tokens1 = tokenize(":foo: :bar:").unwrap();
        assert_eq!(token_details_to_string(&tokens1), vec![":foo:", ":bar:"]);
    }

    #[test]
    fn test_attribute() {
        let tokens1 = tokenize("#[test] #[cfg(test)]").unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec!["#[test]", "#[cfg(test)]"]
        );
    }

    #[test]
    fn test_symbols_and_operators() {
        let tokens1 = tokenize("{ } = | || && == != > >= < <= -> ++ + - * /").unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec![
                "{", "}", "=", "|", "||", "&&", "==", "!=", ">", ">=", "<", "<=", "->", "++", "+",
                "-", "*", "/",
            ]
        );

        let tokens2 = tokenize("?? & ^ ? . [ ] ! ( ) : :: .. ... ,").unwrap();
        assert_eq!(
            token_details_to_string(&tokens2),
            vec!["??", "&", "^", "?", ".", "[", "]", "!", "(", ")", ":", "::", "..", "...", ",",]
        );
    }

    #[test]
    fn test_keywords() {
        let tokens1 =
            tokenize("do join let fn if then else for next each in branch match case default").unwrap();
        assert_eq!(
            token_details_to_string(&tokens1),
            vec![
                "do", "join", "let", "fn", "if", "then", "else", "for", "next", "each", "in", "branch", "match",
                "case", "default",
            ]
        );

        let tokens2 = tokenize("where only into regular template").unwrap();
        assert_eq!(
            token_details_to_string(&tokens2),
            vec!["where", "only", "into", "regular", "template",]
        );

        let tokens3 = tokenize("function type which empty pattern limit").unwrap();
        assert_eq!(
            token_details_to_string(&tokens3),
            vec!["function", "type", "which", "empty", "pattern", "limit",]
        );

        let tokens4 = tokenize("use const enum struct union trait impl alias").unwrap();
        assert_eq!(
            token_details_to_string(&tokens4),
            vec![
                //"namespace",
                "use",
                "const",
                "enum",
                "struct",
                "union",
                "trait",
                "impl",
                "alias",
            ]
        );
    }
}
