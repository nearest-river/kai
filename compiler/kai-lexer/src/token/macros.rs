
#[macro_export]
macro_rules! TokenTy {
  [as]          => { $crate::token::As };
  [async]       => { $crate::token::Async };
  [await]       => { $crate::token::Await };
  [break]       => { $crate::token::Break };
  [const]       => { $crate::token::Const };
  [continue]    => { $crate::token::Continue };
  [this]        => { $crate::token::This };
  [default]     => { $crate::token::Default };
  [dyn]         => { $crate::token::Dyn };
  [else]        => { $crate::token::Else };
  [enum]        => { $crate::token::Enum };
  [extern]      => { $crate::token::Extern };
  [fn]          => { $crate::token::Fn };
  [for]         => { $crate::token::For };
  [if]          => { $crate::token::If };
  [impl]        => { $crate::token::Impl };
  [in]          => { $crate::token::In };
  [let]         => { $crate::token::Let };
  [loop]        => { $crate::token::Loop };
  [macro]       => { $crate::token::Macro };
  [match]       => { $crate::token::Match };
  [mod]         => { $crate::token::Mod };
  [move]        => { $crate::token::Move };
  [mut]         => { $crate::token::Mut };
  [pub]         => { $crate::token::Pub };
  [raw]         => { $crate::token::Raw };
  [ref]         => { $crate::token::Ref };
  [return]      => { $crate::token::Return };
  [Self]        => { $crate::token::SelfType };
  [self]        => { $crate::token::SelfValue };
  [static]      => { $crate::token::Static };
  [struct]      => { $crate::token::Struct };
  [super]       => { $crate::token::Super };
  [trait]       => { $crate::token::Trait };
  [type]        => { $crate::token::Type };
  [typeof]      => { $crate::token::Typeof };
  [union]       => { $crate::token::Union };
  [unsafe]      => { $crate::token::Unsafe };
  [use]         => { $crate::token::Use };
  [where]       => { $crate::token::Where };
  [while]       => { $crate::token::While };
  [yeet]        => { $crate::token::Yeet };
  [yield]       => { $crate::token::Yield };
  [&]           => { $crate::token::And };
  [&&]          => { $crate::token::AndAnd };
  [&=]          => { $crate::token::AndEq };
  [@]           => { $crate::token::At }; // ill have to think about it.
  [^]           => { $crate::token::Caret };
  [^=]          => { $crate::token::CaretEq };
  [:]           => { $crate::token::Colon };
  [,]           => { $crate::token::Comma };
  [$]           => { $crate::token::Dollar };
  [.]           => { $crate::token::Dot };
  [..]          => { $crate::token::DotDot };
  [...]         => { $crate::token::DotDotDot };
  [..=]         => { $crate::token::DotDotEq };
  [=]           => { $crate::token::Eq };
  [==]          => { $crate::token::EqEq };
  [=>]          => { $crate::token::FatArrow };
  [>=]          => { $crate::token::Ge };
  [>]           => { $crate::token::Gt };
  [<-]          => { $crate::token::LArrow };
  [<=]          => { $crate::token::Le };
  [<]           => { $crate::token::Lt };
  [-]           => { $crate::token::Minus };
  [-=]          => { $crate::token::MinusEq };
  [!=]          => { $crate::token::Ne };
  [!]           => { $crate::token::Not };
  [|]           => { $crate::token::Or };
  [|=]          => { $crate::token::OrEq };
  [||]          => { $crate::token::OrOr };
  [::]          => { $crate::token::PathSep };
  [%]           => { $crate::token::Percent };
  [%=]          => { $crate::token::PercentEq };
  [+]           => { $crate::token::Plus };
  [+=]          => { $crate::token::PlusEq };
  [#]           => { $crate::token::Pound };
  [?]           => { $crate::token::Question };
  [->]          => { $crate::token::RArrow };
  [;]           => { $crate::token::SemiColon };
  [<<]          => { $crate::token::Shl };
  [<<=]         => { $crate::token::ShlEq };
  [>>]          => { $crate::token::Shr };
  [>>=]         => { $crate::token::ShrEq };
  [/]           => { $crate::token::Slash };
  [/=]          => { $crate::token::SlashEq };
  [*]           => { $crate::token::Star };
  [*=]          => { $crate::token::StarEq };
  [_]           => { $crate::token::Underscore };
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





