use crate::lex::Token;
use crate::parse::ParseTokens;
use std::marker::PhantomData;

impl<T: ParseTokens + ?Sized> ParseTokens for Box<T> {
    type Output = Box<<T as ParseTokens>::Output>;
    fn parse_mut_tokens<'a>(tokens: &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
        Ok(Box::new(T::parse_mut_tokens(tokens)?))
    }
}

pub struct Repeat<T> {
    marker: PhantomData<T>,
}

impl<T: ParseTokens> ParseTokens for Repeat<T> {
    type Output = Vec<<T as ParseTokens>::Output>;
    fn parse_mut_tokens<'a>(tokens: &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
        let mut parsed_objects = vec![];
        while let Ok(parsed_object) = T::parse_mut_tokens(tokens) {
            parsed_objects.push(parsed_object);
        }
        Ok(parsed_objects)
    }
}

pub struct Optional<T> {
    marker: PhantomData<T>,
}

impl<T: ParseTokens> ParseTokens for Optional<T> {
    type Output = Option<<T as ParseTokens>::Output>;
    fn parse_mut_tokens<'a>(tokens: &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
        Ok(T::parse_mut_tokens(tokens).ok())
    }
}

macro_rules! impl_group_up_to {
    ( $T:ident ) => {
        impl_group_up_to! { @impl_trait $T }
    };
    ( $T:ident $($U:ident)+ ) => {
        impl_group_up_to! { $($U)+ }
        impl_group_up_to! { @impl_trait $T $($U)+ }
    };
    ( @impl_trait $($T:ident)* ) => {
        impl<$($T : ParseTokens, )*> ParseTokens for Group<($($T, )*)> {
            type Output = ($(<$T as ParseTokens>::Output, )*);
            fn parse_mut_tokens<'a>(tokens : &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
                let mut new_tokens : &[_] = tokens;
                let parsed = ($($T::parse_mut_tokens(&mut new_tokens)?, )*);
                *tokens = new_tokens;
                Ok(parsed)
            }
        }
    };
}

impl_group_up_to! { T8 T7 T6 T5 T4 T3 T2 T1 }

pub struct Group<T> {
    marker: PhantomData<T>,
}

macro_rules! define_any_variants_up_to {
    ( [$($EnumName:ident)*] [$($Variant:ident)*] [$($T:ident)*] ) => {
        pub mod any_variants {
            define_any_variants_up_to! { @inner [$($EnumName)*] [$($Variant)*] [$($T)*] }
        }
    };
    ( @inner [$EnumName:ident] [$Variant:ident] [$T:ident] ) => {
        define_any_variants_up_to!{ @reverse_args $EnumName [$Variant] [$T] [] [] }
    };
    ( @inner [$EnumName:ident $($tail1:tt)+] [$Variant:ident $($tail2:tt)+] [$T:ident $($tail3:tt)+] ) => {
        define_any_variants_up_to! { @inner [$($tail1)+] [$($tail2)+] [$($tail3)+] }
        define_any_variants_up_to! { @reverse_args $EnumName [$Variant $($tail2)+] [$T $($tail3)+] [] [] }
    };
    // we reverse the order here because the arguments were received as [ArgN ... Arg3 Arg2 Arg1]
    ( @reverse_args $EnumName:ident [$Variant1:ident] [$T1:ident] [$($NVariant:ident)*] [$($NT:ident)*] ) => {
        define_any_variants_up_to! { @define_variant $EnumName [$Variant1 $($NVariant)*] [$T1 $($NT)*] }
    };
    ( @reverse_args $EnumName:ident [$Variant1:ident $($Variant:ident)*] [$T1:ident $($T:ident)*] [$($NVariant:ident)*] [$($NT:ident)*] ) => {
        define_any_variants_up_to! { @reverse_args $EnumName [$($Variant)*] [$($T)*] [$Variant1 $($NVariant)*] [$T1 $($NT)*] }
    };
    ( @define_variant $EnumName:ident [$($Variant:ident)*] [$($T:ident)*] ) => {
        #[derive(Debug)]
        pub enum $EnumName<$($T, )*> {
            $($Variant($T),)*
        }
    };
}

macro_rules! impl_any_traits_up_to {
    ( [$EnumName:ident] [$Variant:ident] [$T:ident] ) => {
        impl_any_traits_up_to!{ reverse_args $EnumName [$Variant] [$T] [] [] }
    };
    ( [$EnumName:ident $($tail1:tt)+] [$Variant:ident $($tail2:tt)+] [$T:ident $($tail3:tt)+] ) => {
        impl_any_traits_up_to! { [$($tail1)+] [$($tail2)+] [$($tail3)+] }
        impl_any_traits_up_to! { reverse_args $EnumName [$Variant $($tail2)+] [$T $($tail3)+] [] [] }
    };
    // we reverse the order here because the arguments were received as [ArgN ... Arg3 Arg2 Arg1]
    ( reverse_args $EnumName:ident [$Variant1:ident] [$T1:ident] [$($NVariant:ident)*] [$($NT:ident)*] ) => {
        impl_any_traits_up_to!{ impl_trait $EnumName [$Variant1 $($NVariant)*] [$T1 $($NT)*] }
    };
    ( reverse_args $EnumName:ident [$Variant1:ident $($Variant:ident)*] [$T1:ident $($T:ident)*] [$($NVariant:ident)*] [$($NT:ident)*] ) => {
        impl_any_traits_up_to! { reverse_args $EnumName [$($Variant)*] [$($T)*] [$Variant1 $($NVariant)*] [$T1 $($NT)*] }
    };
    ( impl_trait $EnumName:ident [$($Variant:ident)*] [$($T:ident)*] ) => {
        impl<$($T : ParseTokens, )*> ParseTokens for Any<($($T, )*)> {
            type Output = any_variants::$EnumName<$($T::Output, )*>;
            fn parse_mut_tokens<'a>(tokens : &mut &'a [Token<'a>]) -> Result<Self::Output, String> {
                impl_any_traits_up_to! { if_cases tokens $EnumName [$($Variant)*] [$($T)*] }
            }
        }
    };
    ( if_cases $tokens:ident $EnumName:ident [$Variant1:ident $($Variant:ident)*] [$T1:ident $($T:ident)*] ) => {
        if let Ok(parsed) = $T1::parse_mut_tokens($tokens) {
            Ok(any_variants::$EnumName::$Variant1(parsed))
        }
        $(
        else if let Ok(parsed) = $T::parse_mut_tokens($tokens) {
            Ok(any_variants::$EnumName::$Variant(parsed))
        }
        )*
        else {
            Err("tokens didn't match any Any patterns".to_string())
        }
    };
}

macro_rules! impl_any_up_to {
    ( [$($EnumName:ident)*] [$($Variant:ident)*] [$($T:ident)*] ) => {
        define_any_variants_up_to! { [$($EnumName)*] [$($Variant)*] [$($T)*] }
        impl_any_traits_up_to! { [$($EnumName)*] [$($Variant)*] [$($T)*] }
    }
}

impl_any_up_to! {
    [AnyVariants8 AnyVariants7 AnyVariants6 AnyVariants5 AnyVariants4 AnyVariants3 AnyVariants2 AnyVariants1]
    [V8 V7 V6 V5 V4 V3 V2 V1]
    [T8 T7 T6 T5 T4 T3 T2 T1]
}

pub struct Any<T> {
    marker: PhantomData<T>,
}
