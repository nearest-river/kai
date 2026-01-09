
#[macro_export]
macro_rules! TokenTy {
  [as]          => { $crate::token::tokens::As };
  [auto]        => { $crate::token::tokens::Auto };
  [async]       => { $crate::token::tokens::Async };
  [await]       => { $crate::token::tokens::Await };
  [break]       => { $crate::token::tokens::Break };
  [crate]       => { $crate::token::tokens::Crate };
  [const]       => { $crate::token::tokens::Const };
  [continue]    => { $crate::token::tokens::Continue };
  [this]        => { $crate::token::tokens::This };
  [default]     => { $crate::token::tokens::Default };
  [dyn]         => { $crate::token::tokens::Dyn };
  [else]        => { $crate::token::tokens::Else };
  [enum]        => { $crate::token::tokens::Enum };
  [extern]      => { $crate::token::tokens::Extern };
  [fn]          => { $crate::token::tokens::Fn };
  [for]         => { $crate::token::tokens::For };
  [if]          => { $crate::token::tokens::If };
  [impl]        => { $crate::token::tokens::Impl };
  [in]          => { $crate::token::tokens::In };
  [let]         => { $crate::token::tokens::Let };
  [loop]        => { $crate::token::tokens::Loop };
  [macro]       => { $crate::token::tokens::Macro };
  [match]       => { $crate::token::tokens::Match };
  [mod]         => { $crate::token::tokens::Mod };
  [move]        => { $crate::token::tokens::Move };
  [mut]         => { $crate::token::tokens::Mut };
  [pub]         => { $crate::token::tokens::Pub };
  [raw]         => { $crate::token::tokens::Raw };
  [ref]         => { $crate::token::tokens::Ref };
  [return]      => { $crate::token::tokens::Return };
  [Self]        => { $crate::token::tokens::SelfType };
  [self]        => { $crate::token::tokens::SelfValue };
  [static]      => { $crate::token::tokens::Static };
  [struct]      => { $crate::token::tokens::Struct };
  [super]       => { $crate::token::tokens::Super };
  [trait]       => { $crate::token::tokens::Trait };
  [type]        => { $crate::token::tokens::Type };
  [typeof]      => { $crate::token::tokens::Typeof };
  [union]       => { $crate::token::tokens::Union };
  [unsafe]      => { $crate::token::tokens::Unsafe };
  [use]         => { $crate::token::tokens::Use };
  [where]       => { $crate::token::tokens::Where };
  [while]       => { $crate::token::tokens::While };
  [yeet]        => { $crate::token::tokens::Yeet };
  [yield]       => { $crate::token::tokens::Yield };
  [&]           => { $crate::token::tokens::And };
  [&&]          => { $crate::token::tokens::AndAnd };
  [&=]          => { $crate::token::tokens::AndEq };
  [@]           => { $crate::token::tokens::At }; // ill have to think about it.
  [^]           => { $crate::token::tokens::Caret };
  [^=]          => { $crate::token::tokens::CaretEq };
  [:]           => { $crate::token::tokens::Colon };
  [,]           => { $crate::token::tokens::Comma };
  [$]           => { $crate::token::tokens::Dollar };
  [.]           => { $crate::token::tokens::Dot };
  [..]          => { $crate::token::tokens::DotDot };
  [...]         => { $crate::token::tokens::DotDotDot };
  [..=]         => { $crate::token::tokens::DotDotEq };
  [=]           => { $crate::token::tokens::Equal };
  [==]          => { $crate::token::tokens::EqualEqual };
  [=>]          => { $crate::token::tokens::FatArrow };
  [>=]          => { $crate::token::tokens::Ge };
  [>]           => { $crate::token::tokens::Gt };
  [<-]          => { $crate::token::tokens::LArrow };
  [<=]          => { $crate::token::tokens::Le };
  [<]           => { $crate::token::tokens::Lt };
  [-]           => { $crate::token::tokens::Minus };
  [-=]          => { $crate::token::tokens::MinusEq };
  [!=]          => { $crate::token::tokens::NotEq };
  [!]           => { $crate::token::tokens::Not };
  [|]           => { $crate::token::tokens::Or };
  [|=]          => { $crate::token::tokens::OrEq };
  [||]          => { $crate::token::tokens::OrOr };
  [::]          => { $crate::token::tokens::PathSep };
  [%]           => { $crate::token::tokens::Percent };
  [%=]          => { $crate::token::tokens::PercentEq };
  [+]           => { $crate::token::tokens::Plus };
  [+=]          => { $crate::token::tokens::PlusEq };
  [#]           => { $crate::token::tokens::Pound };
  [?]           => { $crate::token::tokens::Question };
  [->]          => { $crate::token::tokens::RArrow };
  [;]           => { $crate::token::tokens::SemiColon };
  [<<]          => { $crate::token::tokens::Shl };
  [<<=]         => { $crate::token::tokens::ShlEq };
  [>>]          => { $crate::token::tokens::Shr };
  [>>=]         => { $crate::token::tokens::ShrEq };
  [/]           => { $crate::token::tokens::Slash };
  [/=]          => { $crate::token::tokens::SlashEq };
  [*]           => { $crate::token::tokens::Star };
  [*=]          => { $crate::token::tokens::StarEq };
  [_]           => { $crate::token::tokens::Underscore };
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

    impl std::default::Default for $name {
      fn default()-> Self {
        Self::new(Span::default())
      }
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





