use crate::errors::Result;
use ethers_contract::builders::ContractCall;
use ethers_core::abi::{Detokenize, InvalidOutputType, Param, ParamType, Token};
use std::{fmt, marker::PhantomData, mem};

/// Stores the result of a call to decode it later.
#[must_use]
pub struct CallResult<V2, V3> {
    /// The output tokens.
    pub tokens: Vec<Token>,

    phantom: PhantomData<(V2, V3)>,
}

impl<V2, V3> Clone for CallResult<V2, V3> {
    fn clone(&self) -> Self {
        Self { tokens: self.tokens.clone(), phantom: PhantomData }
    }
}

impl<V2, V3> fmt::Debug for CallResult<V2, V3> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CallResult").field("tokens", &self.tokens).finish()
    }
}

impl<V2: Detokenize, V3: Detokenize> Detokenize for CallResult<V2, V3> {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
        Ok(Self::new(tokens))
    }
}

impl<V2, V3> CallResult<V2, V3> {
    /// Creates a new instance with the provided tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, phantom: PhantomData }
    }
}

impl<V2: Detokenize, V3> CallResult<V2, V3> {
    /// Consumes the tokens to decode them into `V2`.
    #[inline]
    pub fn v2(self) -> Result<V2> {
        V2::from_tokens(self.tokens).map_err(Into::into)
    }
}

impl<V2, V3: Detokenize> CallResult<V2, V3> {
    /// Consumes the tokens to decode them into `V3`.
    #[inline]
    pub fn v3(self) -> Result<V3> {
        V3::from_tokens(self.tokens).map_err(Into::into)
    }
}

/// Extra [ContractCall] methods.
pub trait CallExt<M>: private::Sealed {
    /// Change the output of a `ContractCall<M, D>`, which is `D`, to another type by changing the
    /// internal function's outputs.
    ///
    /// If `outputs` is empty, this method is just [`mem::transmute`].
    ///
    /// If you're looking to change the output into `()` use [`clear_output`][CallExt::clear_output]
    /// instead.
    ///
    /// # Safety
    ///
    /// It is up to the caller to make sure that the `outputs` Solidity types match the resulting
    /// `Dst` Rust types.
    ///
    /// # Example
    ///
    /// ```
    /// # use uniswap_rs::prelude::{*, _ethers::{*, abi::ParamType}};
    /// # use std::sync::Arc;
    /// # async fn foo<M: Middleware + 'static>(token: Erc20<M>) -> Result<(), Box<dyn std::error::Error>> {
    /// unsafe {
    ///     let result: Bytes = token                           // Erc20<M>
    ///         .contract()                                     // IERC20<M>
    ///         .name()                                         // ContractCall<M, String>
    ///         .change_output::<Bytes>(vec![ParamType::Bytes]) // ContractCall<M, Bytes>
    ///         .call()                                         // Future<Output = Result<Bytes, _>>
    ///         .await?;                                        // Bytes
    /// }
    /// # Ok(()) }
    /// ```
    unsafe fn change_output<Dst>(self, outputs: Vec<ParamType>) -> ContractCall<M, Dst>;

    /// Same as [`change_output`](CallExt::change_output), but with `()`.
    ///
    /// This is safe because there are no arguments.
    fn clear_output(self) -> ContractCall<M, ()>;
}

impl<M, Src> CallExt<M> for ContractCall<M, Src> {
    unsafe fn change_output<Dst>(mut self, outputs: Vec<ParamType>) -> ContractCall<M, Dst> {
        // the only thing checked after a call succeeds is the function's output types.
        if !outputs.is_empty() {
            let mut remaining = 0usize;
            let mut src = outputs.into_iter();
            let res = {
                let mut dst = self.function.outputs.iter_mut();
                loop {
                    match (dst.next(), src.next()) {
                        (None, None) => break None,
                        (Some(dst), Some(src)) => dst.kind = src,
                        (_, src) => break Some(src),
                    }
                    remaining += 1;
                }
            };
            match res {
                // same length -> all good
                None => {}
                // remaining self.function.outputs -> remove
                Some(None) => {
                    for _ in remaining..self.function.outputs.len() {
                        self.function.outputs.pop();
                    }
                }
                // remaining `outputs` -> push
                Some(first) => {
                    for kind in first.into_iter().chain(src) {
                        let param = Param { name: String::new(), internal_type: None, kind };
                        self.function.outputs.push(param);
                    }
                }
            }
        }

        mem::transmute::<ContractCall<M, Src>, ContractCall<M, Dst>>(self)
    }

    fn clear_output(mut self) -> ContractCall<M, ()> {
        self.function.outputs.clear();
        // SAFETY: See the function docs
        unsafe { mem::transmute::<ContractCall<M, Src>, ContractCall<M, ()>>(self) }
    }
}

mod private {
    pub trait Sealed {}

    impl<M, D> Sealed for ethers_contract::builders::ContractCall<M, D> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{contracts::bindings::i_uniswap_v2_pair::IUniswapV2Pair, CallResult};
    use ethers_core::types::Address;
    use ethers_providers::{Http, Provider};

    type CC = (u128, u128, u32);

    const fn assert_cc_output<D: Detokenize>(_cc: &ContractCall<Provider<Http>, D>) {}

    fn cc() -> ContractCall<Provider<Http>, CC> {
        let p = Provider::<Http>::try_from("http://example.com").unwrap();
        let c = IUniswapV2Pair::new(Address::zero(), p.into());
        let cc = c.get_reserves();
        assert_cc_output::<CC>(&cc);
        cc
    }

    #[test]
    fn test_transmute_zst() {
        let cc = cc();
        assert!(!cc.function.outputs.is_empty());
        let cc = cc.clear_output();
        assert_cc_output::<()>(&cc);
        assert!(cc.function.outputs.is_empty());
    }

    #[test]
    #[allow(unused_parens)]
    fn test_transmute() {
        unsafe {
            let cc = cc();

            macro_rules! test {
                ($($ty:ident),+) => { {
                    let outputs = vec![$(ParamType::$ty),+];
                    let cc = cc.clone().change_output::<($($ty),+)>(outputs.clone());
                    assert_cc_output::<($($ty),+)>(&cc);
                    assert_eq!(
                        cc.function.outputs.into_iter().map(|x| x.kind).collect::<Vec<_>>(),
                        outputs
                    );
                }};
            }

            assert_eq!(cc.function.outputs.len(), 3);

            // fully unchanged
            {
                let outputs = cc.function.outputs.clone();
                let cc = cc.clone().change_output::<CC>(vec![]);
                assert_cc_output::<CC>(&cc);
                assert_eq!(cc.function.outputs, outputs);
            }

            // fn outputs unchanged (bytes decoding the same), custom type
            {
                type Custom = CallResult<CC, ()>;
                let outputs = cc.function.outputs.clone();
                let cc = cc.clone().change_output::<Custom>(vec![]);
                assert_cc_output::<Custom>(&cc);
                assert_eq!(cc.function.outputs, outputs);
            }

            // new == old
            test!(Address, Address, Address);
            // new > old
            test!(Address, Address, Address, Address);
            test!(Address, Address, Address, Address, Address);
            test!(Address, Address, Address, Address, Address, Address);
            // new < old
            test!(Address, Address);
            test!(Address);
        }
    }
}
