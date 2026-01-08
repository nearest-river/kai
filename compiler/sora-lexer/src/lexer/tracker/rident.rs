
use crate::{
  prelude::*,
  lexer::token_hint::TokenHint,
};


pub struct RIdentLexTracker;


impl RIdentLexTracker {
  const PREFIX_LEN: usize=Ident::RAW_PREFIX.len();

  #[inline]
  pub fn seq_start(buf: &[u8])-> Option<Self> {
    if !buf.starts_with(Ident::RAW_PREFIX) {
      return None;
    }

    if buf.len()>Self::PREFIX_LEN && let Some(_)=RIdentLexTracker.seq_ends(&buf[Self::PREFIX_LEN..]) {
      None
    } else {
      Some(RIdentLexTracker)
    }
  }

  #[inline]
  pub fn seq_ends(&mut self,buf: &[u8])-> Option<TokenHint> {
    let s=match str::from_utf8(buf) {
      Ok(s)=> s,
      Err(err) if err.valid_up_to()==0=> return None,
      Err(err)=> {
        let reason=Some(Reason::Utf8Error(err));
        return Some(TokenHint::Illegal(reason));
      },
    };

    let ch0=s.chars().next()?;
    if unicode_ident::is_xid_continue(ch0) {
      None
    } else {
      Some(TokenHint::RIdent)
    }
  }

  pub fn prefix_len(&self)-> usize {
    Self::PREFIX_LEN
  }
}








