use ripemd::{Digest, Ripemd160};
use sha1::Sha1;
use sha2::Sha256;
use std::collections::HashMap;

pub fn encode_num(num: i32) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }
    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = Vec::new();
    let mut remaining = abs_num;
    while remaining > 0 {
        result.push((remaining & 0xff) as u8);
        remaining >>= 8;
    }
    if result.last().unwrap() & 0x80 != 0 {
        if negative {
            result.push(0x80);
        } else {
            result.push(0);
        }
    } else if negative {
        let last = result.last_mut().unwrap();
        *last |= 0x80;
    }
    result
}

pub fn decode_num(element: &[u8]) -> i32 {
    if element.is_empty() {
        return 0;
    }
    let big_endian = element.iter().rev().cloned().collect::<Vec<_>>();
    let negative = big_endian[0] & 0x80 != 0;
    let mut result = if negative {
        (big_endian[0] & 0x7f) as i32
    } else {
        big_endian[0] as i32
    };
    for &c in &big_endian[1..] {
        result <<= 8;
        result += c as i32;
    }
    if negative {
        -result
    } else {
        result
    }
}

fn op_0(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(0));
    true
}

fn op_1negate(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(-1));
    true
}

fn op_1(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(1));
    true
}

fn op_2(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(2));
    true
}

fn op_3(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(3));
    true
}

fn op_4(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(4));
    true
}

fn op_5(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(5));
    true
}

fn op_6(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(6));
    true
}

fn op_7(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(7));
    true
}

fn op_8(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(8));
    true
}

fn op_9(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(9));
    true
}

fn op_10(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(10));
    true
}

fn op_11(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(11));
    true
}

fn op_12(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(12));
    true
}

fn op_13(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(13));
    true
}

fn op_14(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(14));
    true
}

fn op_15(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(15));
    true
}

fn op_16(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(16));
    true
}

#[allow(clippy::ptr_arg)]
fn op_nop(_stack: &mut Vec<Vec<u8>>) -> bool {
    true
}

fn op_if(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let mut true_items = Vec::new();
    let mut false_items = Vec::new();
    let mut current_array = &mut true_items;
    let mut found = false;
    let mut num_endifs_needed = 1;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }

    if !found {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        items.splice(0..0, false_items.into_iter());
    } else {
        items.splice(0..0, true_items.into_iter());
    }

    true
}

fn op_notif(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let mut true_items = Vec::new();
    let mut false_items = Vec::new();
    let mut current_array = &mut true_items;
    let mut found = false;
    let mut num_endifs_needed = 1;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }

    if !found {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        items.splice(0..0, true_items.into_iter());
    } else {
        items.splice(0..0, false_items.into_iter());
    }

    true
}

fn op_verify(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        return false;
    }

    true
}

#[allow(clippy::ptr_arg)]
fn op_return(_stack: &mut Vec<Vec<u8>>) -> bool {
    false
}

fn op_toaltstack(stack: &mut Vec<Vec<u8>>, altstack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    altstack.push(item);
    true
}

fn op_fromaltstack(stack: &mut Vec<Vec<u8>>, altstack: &mut Vec<Vec<u8>>) -> bool {
    if altstack.is_empty() {
        return false;
    }
    let item = altstack.pop().unwrap();
    stack.push(item);
    true
}

fn op_2drop(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.pop();
    stack.pop();
    true
}

fn op_2dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack[stack.len() - 2].clone();
    let item2 = stack[stack.len() - 1].clone();
    stack.push(item1);
    stack.push(item2);
    true
}

fn op_3dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack[stack.len() - 3].clone();
    let item2 = stack[stack.len() - 2].clone();
    let item3 = stack[stack.len() - 1].clone();
    stack.push(item1);
    stack.push(item2);
    stack.push(item3);
    true
}

fn op_2over(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 4 {
        return false;
    }
    let item1 = stack[stack.len() - 4].clone();
    let item2 = stack[stack.len() - 3].clone();
    stack.push(item1);
    stack.push(item2);
    true
}

fn op_2rot(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 6 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let item4 = stack.pop().unwrap();
    let item5 = stack.pop().unwrap();
    let item6 = stack.pop().unwrap();
    stack.push(item3);
    stack.push(item4);
    stack.push(item1);
    stack.push(item2);
    stack.push(item5);
    stack.push(item6);
    true
}

fn op_2swap(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 4 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let item4 = stack.pop().unwrap();
    stack.push(item3);
    stack.push(item4);
    stack.push(item1);
    stack.push(item2);
    true
}

fn op_ifdup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack[stack.len() - 1].clone();
    if decode_num(&item) != 0 {
        stack.push(item);
    }
    true
}

fn op_depth(stack: &mut Vec<Vec<u8>>) -> bool {
    let depth = stack.len() as i32;
    stack.push(encode_num(depth));
    true
}

fn op_drop(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    stack.pop();
    true
}

fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack[stack.len() - 1].clone();
    stack.push(item);
    true
}

fn op_nip(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.remove(stack.len() - 2);
    true
}

fn op_over(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item = stack[stack.len() - 2].clone();
    stack.push(item);
    true
}

fn op_pick(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let n = decode_num(&item) as usize;
    if stack.len() < n {
        return false;
    }
    let item = stack[stack.len() - n].clone();
    stack.push(item);
    true
}

fn op_roll(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let n = decode_num(&item) as usize;
    if stack.len() < n {
        return false;
    }
    let item = stack.remove(stack.len() - n);
    stack.push(item);
    true
}

fn op_rot(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    stack.push(item2);
    stack.push(item1);
    stack.push(item3);
    true
}

fn op_swap(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    stack.push(item1);
    stack.push(item2);
    true
}

fn op_tuck(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    stack.push(item1.clone());
    stack.push(item2);
    stack.push(item1);
    true
}

fn op_size(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let size = item.len() as i32;
    stack.push(encode_num(size));
    true
}

fn op_equal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let result = if item1 == item2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_equalverify(stack: &mut Vec<Vec<u8>>) -> bool {
    if !op_equal(stack) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    if decode_num(&item) == 0 {
        return false;
    }
    true
}

fn op_1add(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num + 1));
    true
}

fn op_1sub(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num - 1));
    true
}

fn op_negate(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(-num));
    true
}

fn op_abs(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num.abs()));
    true
}

fn op_not(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    let result = if num == 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_0notequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    let result = if num == 0 { 0 } else { 1 };
    stack.push(encode_num(result));
    true
}

fn op_add(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 + num2));
    true
}

fn op_sub(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 - num2));
    true
}

fn op_booland(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != 0 && num2 != 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_boolor(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != 0 || num2 != 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_numequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 == num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_numequalverify(stack: &mut Vec<Vec<u8>>) -> bool {
    if !op_numequal(stack) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    if decode_num(&item) == 0 {
        return false;
    }
    true
}

fn op_numnotequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_lessthan(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 < num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_greaterthan(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 > num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_lessthanorequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 <= num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_greaterthanorequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 >= num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_min(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 < num2 { num1 } else { num2 };
    stack.push(encode_num(result));
    true
}

fn op_max(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 > num2 { num1 } else { num2 };
    stack.push(encode_num(result));
    true
}

fn op_within(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let num3 = decode_num(&item3);
    let result = if num2 <= num1 && num1 < num3 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}

fn op_ripemd160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let hash = Ripemd160::digest(item);
    stack.push(hash.to_vec());
    true
}

fn op_sha1(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&element);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}

fn op_sha256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}

fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    let mut hasher = Ripemd160::new();
    hasher.update(result);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}

fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(result);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}

#[allow(clippy::ptr_arg)]
fn op_checksig(_stack: &mut Vec<Vec<u8>>, _z: i64) -> bool {
    unimplemented!()
}

fn op_checksigverify(stack: &mut Vec<Vec<u8>>, z: i64) -> bool {
    op_checksig(stack, z) && op_verify(stack)
}

#[allow(clippy::ptr_arg)]
fn op_checkmultisig(_stack: &mut Vec<Vec<u8>>, _z: i64) -> bool {
    unimplemented!()
}

fn op_checkmultisigverify(stack: &mut Vec<Vec<u8>>, z: i64) -> bool {
    op_checkmultisig(stack, z) && op_verify(stack)
}

fn op_checklocktimeverify(stack: &mut Vec<Vec<u8>>, locktime: u32, sequence: u32) -> bool {
    if sequence == 0xffffffff {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let element = decode_num(stack.last().unwrap());
    if element < 0 {
        return false;
    }
    if element < 500_000_000 && locktime > 500_000_000 {
        return false;
    }
    if locktime < element as u32 {
        return false;
    }
    true
}

fn op_checksequenceverify(stack: &mut Vec<Vec<u8>>, version: u32, sequence: u32) -> bool {
    if sequence & (1 << 31) == (1 << 31) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let element = decode_num(stack.last().unwrap());
    if element < 0 {
        return false;
    }

    if element as u32 & (1 << 31) == (1 << 31)
        && (version < 2
            || sequence & (1 << 31) == (1 << 31)
            || element as u32 & (1 << 22) != sequence & (1 << 22)
            || element as u32 & 0xffff > sequence & 0xffff)
    {
        return false;
    }
    true
}

pub fn create_op_code_functions() -> HashMap<u8, OpFunction> {
    let mut op_code_functions = HashMap::new();
    op_code_functions.insert(0, OpFunction::StackOp(op_0));
    op_code_functions.insert(79, OpFunction::StackOp(op_1negate));
    op_code_functions.insert(81, OpFunction::StackOp(op_1));
    op_code_functions.insert(82, OpFunction::StackOp(op_2));
    op_code_functions.insert(83, OpFunction::StackOp(op_3));
    op_code_functions.insert(84, OpFunction::StackOp(op_4));
    op_code_functions.insert(85, OpFunction::StackOp(op_5));
    op_code_functions.insert(86, OpFunction::StackOp(op_6));
    op_code_functions.insert(87, OpFunction::StackOp(op_7));
    op_code_functions.insert(88, OpFunction::StackOp(op_8));
    op_code_functions.insert(89, OpFunction::StackOp(op_9));
    op_code_functions.insert(90, OpFunction::StackOp(op_10));
    op_code_functions.insert(91, OpFunction::StackOp(op_11));
    op_code_functions.insert(92, OpFunction::StackOp(op_12));
    op_code_functions.insert(93, OpFunction::StackOp(op_13));
    op_code_functions.insert(94, OpFunction::StackOp(op_14));
    op_code_functions.insert(95, OpFunction::StackOp(op_15));
    op_code_functions.insert(96, OpFunction::StackOp(op_16));
    op_code_functions.insert(97, OpFunction::StackOp(op_nop));
    op_code_functions.insert(99, OpFunction::StackItemsOp(op_if));
    op_code_functions.insert(100, OpFunction::StackItemsOp(op_notif));
    op_code_functions.insert(105, OpFunction::StackOp(op_verify));
    op_code_functions.insert(106, OpFunction::StackOp(op_return));
    op_code_functions.insert(107, OpFunction::StackAltStackOp(op_toaltstack));
    op_code_functions.insert(108, OpFunction::StackAltStackOp(op_fromaltstack));
    op_code_functions.insert(109, OpFunction::StackOp(op_2drop));
    op_code_functions.insert(110, OpFunction::StackOp(op_2dup));
    op_code_functions.insert(111, OpFunction::StackOp(op_3dup));
    op_code_functions.insert(112, OpFunction::StackOp(op_2over));
    op_code_functions.insert(113, OpFunction::StackOp(op_2rot));
    op_code_functions.insert(114, OpFunction::StackOp(op_2swap));
    op_code_functions.insert(115, OpFunction::StackOp(op_ifdup));
    op_code_functions.insert(116, OpFunction::StackOp(op_depth));
    op_code_functions.insert(117, OpFunction::StackOp(op_drop));
    op_code_functions.insert(118, OpFunction::StackOp(op_dup));
    op_code_functions.insert(119, OpFunction::StackOp(op_nip));
    op_code_functions.insert(120, OpFunction::StackOp(op_over));
    op_code_functions.insert(121, OpFunction::StackOp(op_pick));
    op_code_functions.insert(122, OpFunction::StackOp(op_roll));
    op_code_functions.insert(123, OpFunction::StackOp(op_rot));
    op_code_functions.insert(124, OpFunction::StackOp(op_swap));
    op_code_functions.insert(125, OpFunction::StackOp(op_tuck));
    op_code_functions.insert(130, OpFunction::StackOp(op_size));
    op_code_functions.insert(135, OpFunction::StackHashOp(op_equal));
    op_code_functions.insert(136, OpFunction::StackHashOp(op_equalverify));
    op_code_functions.insert(139, OpFunction::StackOp(op_1add));
    op_code_functions.insert(140, OpFunction::StackOp(op_1sub));
    op_code_functions.insert(143, OpFunction::StackOp(op_negate));
    op_code_functions.insert(144, OpFunction::StackOp(op_abs));
    op_code_functions.insert(145, OpFunction::StackOp(op_not));
    op_code_functions.insert(146, OpFunction::StackOp(op_0notequal));
    op_code_functions.insert(147, OpFunction::StackOp(op_add));
    op_code_functions.insert(148, OpFunction::StackOp(op_sub));
    op_code_functions.insert(154, OpFunction::StackOp(op_booland));
    op_code_functions.insert(155, OpFunction::StackOp(op_boolor));
    op_code_functions.insert(156, OpFunction::StackOp(op_numequal));
    op_code_functions.insert(157, OpFunction::StackOp(op_numequalverify));
    op_code_functions.insert(158, OpFunction::StackOp(op_numnotequal));
    op_code_functions.insert(159, OpFunction::StackOp(op_lessthan));
    op_code_functions.insert(160, OpFunction::StackOp(op_greaterthan));
    op_code_functions.insert(161, OpFunction::StackOp(op_lessthanorequal));
    op_code_functions.insert(162, OpFunction::StackOp(op_greaterthanorequal));
    op_code_functions.insert(163, OpFunction::StackOp(op_min));
    op_code_functions.insert(164, OpFunction::StackOp(op_max));
    op_code_functions.insert(165, OpFunction::StackOp(op_within));
    op_code_functions.insert(166, OpFunction::StackOp(op_ripemd160));
    op_code_functions.insert(167, OpFunction::StackOp(op_sha1));
    op_code_functions.insert(168, OpFunction::StackOp(op_sha256));
    op_code_functions.insert(169, OpFunction::StackOp(op_hash160));
    op_code_functions.insert(170, OpFunction::StackOp(op_hash256));
    op_code_functions.insert(172, OpFunction::StackSigOp(op_checksig));
    op_code_functions.insert(173, OpFunction::StackSigOp(op_checksigverify));
    op_code_functions.insert(174, OpFunction::StackSigOp(op_checkmultisig));
    op_code_functions.insert(175, OpFunction::StackSigOp(op_checkmultisigverify));
    op_code_functions.insert(
        177,
        OpFunction::StackLocktimeSequenceOp(op_checklocktimeverify),
    );
    op_code_functions.insert(
        178,
        OpFunction::StackLocktimeSequenceOp(op_checksequenceverify),
    );
    op_code_functions
}

type StackOpFunc = fn(&mut Vec<Vec<u8>>, &mut Vec<Vec<u8>>) -> bool;

pub enum OpFunction {
    StackOp(fn(&mut Vec<Vec<u8>>) -> bool),
    StackItemsOp(fn(&mut Vec<Vec<u8>>, &mut Vec<u8>) -> bool),
    StackAltStackOp(StackOpFunc),
    StackHashOp(fn(&mut Vec<Vec<u8>>) -> bool),
    StackLocktimeSequenceOp(fn(&mut Vec<Vec<u8>>, u32, u32) -> bool),
    StackSigOp(fn(&mut Vec<Vec<u8>>, i64) -> bool),
}

pub fn create_op_code_names() -> HashMap<u8, &'static str> {
    let mut op_code_names = HashMap::new();
    op_code_names.insert(0, "OP_0");
    op_code_names.insert(79, "OP_1NEGATE");
    op_code_names.insert(81, "OP_1");
    op_code_names.insert(82, "OP_2");
    op_code_names.insert(83, "OP_3");
    op_code_names.insert(84, "OP_4");
    op_code_names.insert(85, "OP_5");
    op_code_names.insert(86, "OP_6");
    op_code_names.insert(87, "OP_7");
    op_code_names.insert(88, "OP_8");
    op_code_names.insert(89, "OP_9");
    op_code_names.insert(90, "OP_10");
    op_code_names.insert(91, "OP_11");
    op_code_names.insert(92, "OP_12");
    op_code_names.insert(93, "OP_13");
    op_code_names.insert(94, "OP_14");
    op_code_names.insert(95, "OP_15");
    op_code_names.insert(96, "OP_16");
    op_code_names.insert(97, "OP_NOP");
    op_code_names.insert(99, "OP_IF");
    op_code_names.insert(100, "OP_NOTIF");
    op_code_names.insert(105, "OP_VERIFY");
    op_code_names.insert(106, "OP_RETURN");
    op_code_names.insert(107, "OP_TOALTSTACK");
    op_code_names.insert(108, "OP_FROMALTSTACK");
    op_code_names.insert(109, "OP_2DROP");
    op_code_names.insert(110, "OP_2DUP");
    op_code_names.insert(111, "OP_3DUP");
    op_code_names.insert(112, "OP_2OVER");
    op_code_names.insert(113, "OP_2ROT");
    op_code_names.insert(114, "OP_2SWAP");
    op_code_names.insert(115, "OP_IFDUP");
    op_code_names.insert(116, "OP_DEPTH");
    op_code_names.insert(117, "OP_DROP");
    op_code_names.insert(118, "OP_DUP");
    op_code_names.insert(119, "OP_NIP");
    op_code_names.insert(120, "OP_OVER");
    op_code_names.insert(121, "OP_PICK");
    op_code_names.insert(122, "OP_ROLL");
    op_code_names.insert(123, "OP_ROT");
    op_code_names.insert(124, "OP_SWAP");
    op_code_names.insert(125, "OP_TUCK");
    op_code_names.insert(130, "OP_SIZE");
    op_code_names.insert(135, "OP_EQUAL");
    op_code_names.insert(136, "OP_EQUALVERIFY");
    op_code_names.insert(139, "OP_1ADD");
    op_code_names.insert(140, "OP_1SUB");
    op_code_names.insert(143, "OP_NEGATE");
    op_code_names.insert(144, "OP_ABS");
    op_code_names.insert(145, "OP_NOT");
    op_code_names.insert(146, "OP_0NOTEQUAL");
    op_code_names.insert(147, "OP_ADD");
    op_code_names.insert(148, "OP_SUB");
    op_code_names.insert(154, "OP_BOOLAND");
    op_code_names.insert(155, "OP_BOOLOR");
    op_code_names.insert(156, "OP_NUMEQUAL");
    op_code_names.insert(157, "OP_NUMEQUALVERIFY");
    op_code_names.insert(158, "OP_NUMNOTEQUAL");
    op_code_names.insert(159, "OP_LESSTHAN");
    op_code_names.insert(160, "OP_GREATERTHAN");
    op_code_names.insert(161, "OP_LESSTHANOREQUAL");
    op_code_names.insert(162, "OP_GREATERTHANOREQUAL");
    op_code_names.insert(163, "OP_MIN");
    op_code_names.insert(164, "OP_MAX");
    op_code_names.insert(165, "OP_WITHIN");
    op_code_names.insert(166, "OP_RIPEMD160");
    op_code_names.insert(167, "OP_SHA1");
    op_code_names.insert(168, "OP_SHA256");
    op_code_names.insert(169, "OP_HASH160");
    op_code_names.insert(170, "OP_HASH256");
    op_code_names.insert(172, "OP_CHECKSIG");
    op_code_names.insert(173, "OP_CHECKSIGVERIFY");
    op_code_names.insert(174, "OP_CHECKMULTISIG");
    op_code_names.insert(175, "OP_CHECKMULTISIGVERIFY");
    op_code_names.insert(177, "OP_CHECKLOCKTIMEVERIFY");
    op_code_names.insert(178, "OP_CHECKSEQUENCEVERIFY");
    op_code_names
}
