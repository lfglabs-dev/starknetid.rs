use lazy_static::lazy_static;
use num_traits::cast::ToPrimitive;
use starknet::core::types::{Felt, NonZeroFelt};

const BASIC_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789-";
const BIG_ALPHABET: &str = "这来";

lazy_static! {
    static ref BASIC_ALPHABET_SIZE: Felt = Felt::from(BASIC_ALPHABET.chars().count());
    static ref BIG_ALPHABET_SIZE: Felt = Felt::from(BIG_ALPHABET.chars().count());
}

#[derive(Debug)]
pub enum EncodingError {
    UnkwnownCharacter(char),
}

fn extract_stars(mut domain: &str) -> (&str, usize) {
    let mut k = 0;
    let last_char = BIG_ALPHABET.chars().last().unwrap();
    while domain.ends_with(last_char) {
        let mut chars = domain.chars();
        chars.next_back();
        domain = chars.as_str();
        k += 1;
    }
    (domain, k)
}

pub fn encode(domain: &str) -> Result<Felt, EncodingError> {
    let mut mul = Felt::ONE;
    let mut output = Felt::ZERO;
    let mut wip_domain: String;

    if domain.chars().count() >= 2
        && domain.chars().nth(domain.chars().count() - 2).unwrap()
            == BIG_ALPHABET.chars().next().unwrap()
        && domain.chars().last().unwrap() == BASIC_ALPHABET.chars().last().unwrap()
    {
        let mut chars = domain.chars();
        chars.next_back();
        chars.next_back();
        let (str, k) = extract_stars(chars.as_str());
        wip_domain = String::from(str);
        wip_domain.push_str(
            BIG_ALPHABET
                .chars()
                .last()
                .unwrap()
                .to_string()
                .repeat(2 * (k + 1))
                .as_str(),
        )
    } else {
        let (str, k) = extract_stars(domain);
        if k != 0 {
            wip_domain = String::from(str);
            wip_domain.push_str(
                BIG_ALPHABET
                    .chars()
                    .last()
                    .unwrap()
                    .to_string()
                    .repeat(1 + (2 * (k - 1)))
                    .as_str(),
            )
        } else {
            wip_domain = String::from(domain);
        }
    }

    for (i, c) in wip_domain.chars().enumerate() {
        if i == wip_domain.chars().count() - 1 && c == BASIC_ALPHABET.chars().next().unwrap() {
            output = output + *BASIC_ALPHABET_SIZE * mul;
        } else {
            let found_basic = BASIC_ALPHABET
                .chars()
                .position(|alphabet_c| alphabet_c == c);

            match found_basic {
                Some(index) => {
                    output = output + Felt::from(index) * mul;
                    mul *= *BASIC_ALPHABET_SIZE + Felt::ONE;
                }
                None => {
                    let found_big = BIG_ALPHABET.chars().position(|alphabet_c| alphabet_c == c);
                    match found_big {
                        Some(index) => {
                            output = output + *BASIC_ALPHABET_SIZE * mul;
                            mul *= *BASIC_ALPHABET_SIZE + Felt::ONE;

                            output = output
                                + Felt::from(
                                    mul * (Felt::from(index)
                                        + if i == wip_domain.chars().count() - 1 {
                                            Felt::ONE
                                        } else {
                                            Felt::ZERO
                                        }),
                                );
                            mul *= *BIG_ALPHABET_SIZE;
                        }
                        None => {
                            return Err(EncodingError::UnkwnownCharacter(c));
                        }
                    }
                }
            }
        }
    }

    Ok(output)
}

pub fn decode(mut felt: Felt) -> String {
    let mut decoded: String = String::new();
    let basic_plus: NonZeroFelt =
        NonZeroFelt::from_felt_unchecked(Felt::from(BASIC_ALPHABET.chars().count() + 1));
    let basic_len = Felt::from(BASIC_ALPHABET.chars().count());
    let big_plus: NonZeroFelt =
        NonZeroFelt::from_felt_unchecked(Felt::from(BIG_ALPHABET.chars().count() + 1));
    let big_len: NonZeroFelt =
        NonZeroFelt::from_felt_unchecked(Felt::from(BIG_ALPHABET.chars().count()));
    let last_big = BIG_ALPHABET.chars().last().unwrap();
    while felt != Felt::ZERO {
        let code = felt.mod_floor(&basic_plus);
        felt = felt.floor_div(&basic_plus);
        if code == basic_len {
            let next_felt = felt.floor_div(&big_plus);
            if next_felt == Felt::ZERO {
                let code2 = felt.mod_floor(&big_plus);
                felt = next_felt;
                decoded.push(if code2 == Felt::ZERO {
                    BASIC_ALPHABET.chars().next().unwrap()
                } else {
                    last_big
                });
            } else {
                decoded.push(
                    BIG_ALPHABET
                        .chars()
                        .nth((felt.mod_floor(&big_len)).to_biguint().to_usize().unwrap())
                        .unwrap(),
                );
                felt = felt.floor_div(&big_len);
            }
        } else {
            decoded.push(
                BASIC_ALPHABET
                    .chars()
                    .nth(code.to_biguint().to_usize().unwrap())
                    .unwrap(),
            );
        }

        let (decoded_str, k) = extract_stars(decoded.as_str());
        let mut decoded = String::from(decoded_str);
        if k != 0 {
            let star = last_big.to_string();
            if k % 2 == 0 {
                decoded.push_str(&str::repeat(&star, k / 2 - 1));
                decoded.push(BIG_ALPHABET.chars().next().unwrap());
                let mut basic_iter = BASIC_ALPHABET.chars();
                basic_iter.next();
                decoded.push(basic_iter.next().unwrap());
            } else {
                decoded.push_str(&str::repeat(&star, k / 2 + 1));
            }
        }
    }
    decoded
}
