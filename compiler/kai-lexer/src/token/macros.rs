
#[macro_export]
macro_rules! Token {
  [as]          => { $crate::token::Token::As };
  [async]       => { $crate::token::Token::Async };
  [await]       => { $crate::token::Token::Await };
  [break]       => { $crate::token::Token::Break };
  [const]       => { $crate::token::Token::Const };
  [continue]    => { $crate::token::Token::Continue };
  [this]        => { $crate::token::Token::This };
  [default]     => { $crate::token::Token::Default };
  [dyn]         => { $crate::token::Token::Dyn };
  [else]        => { $crate::token::Token::Else };
  [enum]        => { $crate::token::Token::Enum };
  [extern]      => { $crate::token::Token::Extern };
  [fn]          => { $crate::token::Token::Fn };
  [for]         => { $crate::token::Token::For };
  [if]          => { $crate::token::Token::If };
  [impl]        => { $crate::token::Token::Impl };
  [in]          => { $crate::token::Token::In };
  [let]         => { $crate::token::Token::Let };
  [loop]        => { $crate::token::Token::Loop };
  [macro]       => { $crate::token::Token::Macro };
  [match]       => { $crate::token::Token::Match };
  [mod]         => { $crate::token::Token::Mod };
  [move]        => { $crate::token::Token::Move };
  [mut]         => { $crate::token::Token::Mut };
  [pub]         => { $crate::token::Token::Pub };
  [raw]         => { $crate::token::Token::Raw };
  [ref]         => { $crate::token::Token::Ref };
  [return]      => { $crate::token::Token::Return };
  [Self]        => { $crate::token::Token::SelfType };
  [self]        => { $crate::token::Token::SelfValue };
  [static]      => { $crate::token::Token::Static };
  [struct]      => { $crate::token::Token::Struct };
  [super]       => { $crate::token::Token::Super };
  [trait]       => { $crate::token::Token::Trait };
  [type]        => { $crate::token::Token::Type };
  [typeof]      => { $crate::token::Token::Typeof };
  [union]       => { $crate::token::Token::Union };
  [unsafe]      => { $crate::token::Token::Unsafe };
  [use]         => { $crate::token::Token::Use };
  [where]       => { $crate::token::Token::Where };
  [while]       => { $crate::token::Token::While };
  [yeet]        => { $crate::token::Token::Yeet };
  [yield]       => { $crate::token::Token::Yield };
  [&]           => { $crate::token::Token::And };
  [&&]          => { $crate::token::Token::AndAnd };
  [&=]          => { $crate::token::Token::AndEq };
  [@]           => { $crate::token::Token::At }; // ill have to think about it.
  [^]           => { $crate::token::Token::Caret };
  [^=]          => { $crate::token::Token::CaretEq };
  [:]           => { $crate::token::Token::Colon };
  [,]           => { $crate::token::Token::Comma };
  [$]           => { $crate::token::Token::Dollar };
  [.]           => { $crate::token::Token::Dot };
  [..]          => { $crate::token::Token::DotDot };
  [...]         => { $crate::token::Token::DotDotDot };
  [..=]         => { $crate::token::Token::DotDotEq };
  [=]           => { $crate::token::Token::Eq };
  [==]          => { $crate::token::Token::EqEq };
  [=>]          => { $crate::token::Token::FatArrow };
  [>=]          => { $crate::token::Token::Ge };
  [>]           => { $crate::token::Token::Gt };
  [<-]          => { $crate::token::Token::LArrow };
  [<=]          => { $crate::token::Token::Le };
  [<]           => { $crate::token::Token::Lt };
  [-]           => { $crate::token::Token::Minus };
  [-=]          => { $crate::token::Token::MinusEq };
  [!=]          => { $crate::token::Token::Ne };
  [!]           => { $crate::token::Token::Not };
  [|]           => { $crate::token::Token::Or };
  [|=]          => { $crate::token::Token::OrEq };
  [||]          => { $crate::token::Token::OrOr };
  [::]          => { $crate::token::Token::PathSep };
  [%]           => { $crate::token::Token::Percent };
  [%=]          => { $crate::token::Token::PercentEq };
  [+]           => { $crate::token::Token::Plus };
  [+=]          => { $crate::token::Token::PlusEq };
  [#]           => { $crate::token::Token::Pound };
  [?]           => { $crate::token::Token::Question };
  [->]          => { $crate::token::Token::RArrow };
  [;]           => { $crate::token::Token::SemiColon };
  [<<]          => { $crate::token::Token::Shl };
  [<<=]         => { $crate::token::Token::ShlEq };
  [>>]          => { $crate::token::Token::Shr };
  [>>=]         => { $crate::token::Token::ShrEq };
  [/]           => { $crate::token::Token::Slash };
  [/=]          => { $crate::token::Token::SlashEq };
  [*]           => { $crate::token::Token::Star };
  [*=]          => { $crate::token::Token::StarEq };
  [_]           => { $crate::token::Token::Underscore };
}

#[macro_export]
macro_rules! impl_literal_tokens {
  ($($name:ident)*)=> {
    $(
    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))?;
        if f.alternate() {
          write!(f,"({:#?})",self.repr)
        } else {
          write!(f,"({:?})",self.repr)
        }
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,other: &$name)-> bool {
        self.repr==other.repr
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,state: &mut H) {
        self.repr.hash(state);
      }
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}


#[macro_export]
macro_rules! impl_repr_tokens {
  ($($name:ident)*)=> {
    $(
    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))?;
        if f.alternate() {
          write!(f,"({:#?})",self.repr)
        } else {
          write!(f,"({:?})",self.repr)
        }
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,other: &$name)-> bool {
        self.repr==other.repr
      }
    }
    impl<S: AsRef<str>> PartialEq<S> for $name {
      fn eq(&self,other: &S)-> bool {
        other.as_ref()==&*self.repr
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,state: &mut H) {
        self.repr.hash(state);
      }
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}



#[macro_export]
macro_rules! define_delimitirs {
  ($($token:literal pub struct $name:ident)*)=> {
    $(
    #[doc = concat!('`', $token, '`')]
    #[derive(Clone,Copy)]
    pub struct $name {
      pub span: Span,
      _marker: $crate::marker::ProcMacroAutoTraits,
    }

    impl $name {
      pub const fn new(span: Span)-> Self {
        Self {
          span,
          _marker: $crate::marker::MARKER
        }
      }
    }

    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,_other: &$name)-> bool {
        true
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,_state: &mut H) {}
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}

#[macro_export]
macro_rules! define_punctuation {
  ($($token:literal pub struct $name:ident/$len:tt #[doc = $usage:literal])*) => {
    $(
    #[doc = concat!('`', $token, '`')]
    #[derive(Clone,Copy)]
    pub struct $name {
      pub span: $crate::span::Span,
      _marker: $crate::marker::ProcMacroAutoTraits,
    }

    impl $name {
      #[inline]
      pub const fn new(span: Span)-> Self {
        Self {
          span,
          _marker: $crate::marker::MARKER
        }
      }

      #[inline]
      pub const fn as_str(&self)-> &str {
        $token
      }
    }

    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,_other: &$name)-> bool {
        true
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,_state: &mut H) {}
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}

#[macro_export]
macro_rules! define_keywords {
  ($($token:literal pub struct $name:ident)*)=> {
    $(
    #[doc = concat!('`', $token, '`')]
    #[derive(Clone,Copy)]
    pub struct $name {
      pub span: Span,
      _marker: $crate::marker::ProcMacroAutoTraits,
    }

    impl $name {
      pub const fn new(span: Span)-> Self {
        Self {
          span,
          _marker: $crate::marker::MARKER
        }
      }
    }

    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,_other: &$name)-> bool {
        true
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,_state: &mut H) {}
    }

    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}

#[macro_export]
macro_rules! define_punctuation_structs {
  ($($token:literal pub struct $name:ident/$len:tt #[doc = $usage:literal])*)=> {
    $(
    #[doc = concat!('`', $token, '`')]
    #[derive(Clone,Copy)]
    pub struct $name {
      pub span: Span,
      _marker: $crate::marker::ProcMacroAutoTraits,
    }

    impl $name {
      pub const fn new(span: Span)-> Self {
        Self {
          span,
          _marker: $crate::marker::MARKER
        }
      }
    }

    impl std::fmt::Debug for $name {
      fn fmt(&self,f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(stringify!($name))
      }
    }

    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,_other: &$name)-> bool {
        true
      }
    }

    impl std::hash::Hash for $name {
      fn hash<H: std::hash::Hasher>(&self,_state: &mut H) {}
    }


    impl $crate::token::TokenExt for $name {
      fn into_token(self)-> $crate::token::Token {
        $crate::token::Token::$name(self)
      }
    }
    )*
  };
}



#[macro_export]
macro_rules! impl_literal_partial_eqs {
  (
    $name:ident:
    $($ty:ty => $en:ident::$varient:ident,)*
  )=> {
    impl std::cmp::Eq for $name {}
    impl PartialEq for $name {
      fn eq(&self,other: &Self)-> bool {
        if let Some(k1)=self.kind && let Some(k2)=other.kind && k1!=k2 {
          return false;
        }

        self.repr==other.repr
      }
    }
    $(
    impl PartialEq<$ty> for $name {
      #[inline(always)]
      fn eq(&self,other: &$ty)-> bool {
        matches!(self.kind,Some($en::$varient)|None) && self.repr==*(other) as _
      }
    }
    )*
  };
}





