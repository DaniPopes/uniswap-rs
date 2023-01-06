/// Automatically derives Clone and Debug **without** generic type bounds.
///
/// Implements getters for the contract's client and address.
macro_rules! contract_struct {
    (
        $(#[$attr:meta])*
        pub struct $name:ident<M> {
            $(#[$cattr:meta])*
            $contract:ident: $cty:ty,
            $(
                $(#[$fattr:meta])*
                $vis:vis $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name<M> {
            $(#[$cattr])*
            $contract: $cty,
            $(
                $(#[$fattr])*
                $vis $field: $ty,
            )*
        }

        #[automatically_derived]
        impl<M> ::core::clone::Clone for $name<M> {
            fn clone(&self) -> Self {
                Self {
                    $contract: ::core::clone::Clone::clone(&self.$contract),
                    $($field: ::core::clone::Clone::clone(&self.$field)),*
                }
            }
        }

        #[automatically_derived]
        impl<M> ::core::fmt::Debug for $name<M> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("address", &self.$contract.address())
                    $(.field(stringify!($field), &self.$field))*
                    .finish()
            }
        }

        impl<M> $name<M> {
            /// Returns a reference to the factory contract.
            #[inline]
            pub fn contract(&self) -> &$cty {
                &self.$contract
            }

            /// Returns the pointer to the contract's client.
            #[inline]
            pub fn client(&self) -> ::std::sync::Arc<M> {
                self.$contract.client()
            }

            /// Returns the contract's address.
            #[inline]
            pub fn address(&self) -> ::ethers_core::types::Address {
                self.$contract.address()
            }
        }
    };
}
