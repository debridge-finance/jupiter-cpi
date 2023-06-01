anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

pub mod jupiter_override {
    use super::Side;
    use super::SplitLeg;
    use anchor_lang::prelude::*;
    use anchor_lang::Discriminator;
    use anchor_lang::{AnchorSerialize, InstructionData};
    use std::io;
    use std::io::{ErrorKind, Write};

    #[derive(AnchorSerialize, AnchorDeserialize, Debug)]
    pub enum Swap {
        Saber,
        SaberAddDecimalsDeposit,
        SaberAddDecimalsWithdraw,
        TokenSwap,
        Sencha,
        Step,
        Cropper,
        Raydium,
        Crema,
        Lifinity,
        Mercurial,
        Cykura,
        Serum {
            side: Side,
        },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin {
            side: Side,
        },
        AldrinV2 {
            side: Side,
        },
        Whirlpool {
            a_to_b: bool,
        },
        Invariant {
            x_to_y: bool,
        },
        Meteora,
        GooseFX,
        DeltaFi {
            stable: bool,
        },
        Balansol,
        MarcoPolo {
            x_to_y: bool,
        },
        Dradex {
            side: Side,
        },
        LifinityV2,
        RaydiumClmm,
        Openbook {
            side: Side,
        },
        Phoenix {
            side: Side,
        },
        Symmetry {
            from_token_id: u64,
            to_token_id: u64,
        },
    }

    #[derive(Debug)]
    pub enum SwapLeg {
        Chain { swap_legs: Vec<SwapLeg> },
        Split { split_legs: Vec<SplitLeg> },
        Swap { swap: Swap },
    }

    impl AnchorSerialize for SwapLeg {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
            match self {
                SwapLeg::Chain { swap_legs } => {
                    0u8.serialize(writer)?;
                    swap_legs.serialize(writer)
                }
                SwapLeg::Split { split_legs } => {
                    1u8.serialize(writer)?;
                    split_legs.serialize(writer)
                }
                SwapLeg::Swap { swap } => {
                    2u8.serialize(writer)?;
                    swap.serialize(writer)
                }
            }
        }
    }

    impl AnchorDeserialize for SwapLeg {
        fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
            match buf[0] {
                0u8 => Ok(SwapLeg::Chain {
                    swap_legs: AnchorDeserialize::deserialize(&mut buf.split_at(1).1)?,
                }),
                1u8 => Ok(SwapLeg::Split {
                    split_legs: AnchorDeserialize::deserialize(&mut buf.split_at(1).1)?,
                }),
                2u8 => Ok(SwapLeg::Swap {
                    swap: AnchorDeserialize::deserialize(&mut buf.split_at(1).1)?,
                }),
                _ => Err(io::Error::new(
                    ErrorKind::NotFound,
                    "No recognized swap leg",
                )),
            }
        }
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Debug)]
    pub struct Route {
        pub swap_leg: SwapLeg,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    impl Discriminator for Route {
        const DISCRIMINATOR: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
    }

    impl InstructionData for Route {}

    #[derive(AnchorSerialize, AnchorDeserialize, Debug, PartialEq)]
    pub struct RouteMeta {
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    impl RouteMeta {
        pub const SIZE: usize = 19;
    }
}

#[cfg(test)]
mod serialization_tests {
    use crate::jupiter_override::RouteMeta;
    use crate::jupiter_override::{Route, SwapLeg};
    use anchor_lang::AnchorDeserialize;
    use std::os::raw;

    #[test]
    fn deserialize_test() {
        let raw_ix = vec![
            229, 23, 203, 151, 122, 227, 173, 42, 0, 1, 0, 0, 0, 1, 2, 0, 0, 0, 60, 2, 17, 1, 40,
            2, 11, 236, 213, 212, 1, 0, 0, 0, 0, 146, 177, 9, 0, 0, 0, 0, 0, 5, 0, 0,
        ];

        let meta = RouteMeta::deserialize(&mut raw_ix.split_at(raw_ix.len() - RouteMeta::SIZE).1)
            .expect("Failed to parse meta");

        assert_eq!(
            meta,
            RouteMeta {
                in_amount: 30725612,
                quoted_out_amount: 635282,
                slippage_bps: 5,
                platform_fee_bps: 0,
            }
        );
    }
}
