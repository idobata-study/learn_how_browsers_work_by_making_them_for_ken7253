use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnQuoted,
    SelfClosingStartTag,
    ScriptData,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    TemporaryBuffer,
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        loop {
            match self.state {
                State::Data => {}
                State::TagOpen => {}
                State::EndTagOpen => {}
                State::TagName => {}
                State::BeforeAttributeName => {}
                State::AttributeName => {}
                State::BeforeAttributeValue => {}
                State::AttributeValueDoubleQuoted => {}
                State::AttributeValueSingleQuoted => {}
                State::AttributeValueUnQuoted => {}
                State::SelfClosingStartTag => {}
                State::ScriptData => {}
                State::ScriptDataEndTagOpen => {}
                State::ScriptDataEndTagName => {}
                State::TemporaryBuffer => {}
                _ => {}
            }
        }
    }
}
