use crate::encode::{
    PacketEncode,
    EncodeBuf
};


macro impl_packetencode_for_tuples {

    ( $first:ident $( , $remaining:ident )* $(,)? ) => {
        impl_packetencode_for_tuple!( $first $( , $remaining )* );
        impl_packetencode_for_tuples!( $( $remaining , )* );
    },

    ( $(,)? ) => {
        impl_packetencode_for_tuple!();
    }

}

macro impl_packetencode_for_tuple($($generics:ident),* $(,)?) {
    unsafe impl< $( $generics , )* > PacketEncode for ( $( $generics , )* )
    where
        $( $generics : PacketEncode , )*
    {

        fn encode_len(&self) -> usize {
            #[allow(non_snake_case)]
            let ( $( $generics , )* ) = self;
            0 $( + $generics.encode_len() )*
        }

        unsafe fn encode(&self,
            #[allow(unused_variables)]
            buf : &mut EncodeBuf
        ) {
            #[allow(unused_unsafe)]
            unsafe {
                #[allow(non_snake_case)]
                let ( $( $generics , )* ) = self;
                $( $generics.encode(buf); )*
            }
        }

    }
}

impl_packetencode_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L,);
