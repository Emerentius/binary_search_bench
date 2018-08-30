#![feature(test)]
// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
extern crate test;
use test::black_box;
use test::Bencher;

enum Cache {
    L1,
    L2,
    L3,
    TooLarge,
}

fn binary_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
    where F: Fn(usize) -> usize
{
    let size = match cache {
        Cache::L1 => 1000, // 8kb
        Cache::L2 => 10_000, // 80kb
        Cache::L3 => 1_000_000, // 8Mb
        Cache::TooLarge => 10_000_000, // 80 Mb
    };
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        //black_box(find(&v, i).is_some());
        //black_box(v.binary_search(&i).is_ok());
        black_box(old_binary_search(&v, i).is_ok());
    })
}

#[bench]
fn binary_search_l1(b: &mut Bencher) {
    binary_search(b, Cache::L1, |i| i * 2);
}

#[bench]
fn binary_search_l2(b: &mut Bencher) {
    binary_search(b, Cache::L2, |i| i * 2);
}

#[bench]
fn binary_search_l3(b: &mut Bencher) {
    binary_search(b, Cache::L3, |i| i * 2);
}

#[bench]
fn binary_search_too_large_for_cache(b: &mut Bencher) {
    binary_search(b, Cache::TooLarge, |i| i * 2);
}

#[bench]
fn binary_search_l1_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L1, |i| i / 16 * 16);
}

#[bench]
fn binary_search_l2_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L2, |i| i / 16 * 16);
}

#[bench]
fn binary_search_l3_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L3, |i| i / 16 * 16);
}

#[bench]
fn binary_search_too_large_for_cache_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::TooLarge, |i| i / 16 * 16);
}

/*
pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: Ord,
    S: AsRef<[T]>,
{
    _find(array.as_ref(), key)
}

// the problem is recursive and it's easy to check algorithm
// is correct for length = 1
// it suffices to check that it correctly reduces to subproblems for l > 1
// there are four cases to think of, namely all combinations of:
// 1. odd or even size of range
// 2. element is less than or (greater than or equal to) searched element
//
// odd case:
// 0 1 2
//   c      checked position, perfect middle
// l l      new range, if less
//   g g    new range, if greater or equal
//
// even case:
// 0 1 2 3
//     c    checked position, right one of middle elements
// l l      new range, if less
//     g g  new range, if greater or equal
//
// the searched for element is never left out
// but in one case a superfluous element is kept in, which doesn't matter
pub fn _find<T: Ord>(array: &[T], key: T) -> Option<usize>
{
    if array.len() == 0 { None? }
    let mut pivot = 0;
    {
        let mut array = array;
        while array.len() != 1 {
            let mid = array.len() / 2;
            /*
            use std::cmp::Ordering::*;
            match key.cmp(&array[mid]) {
                Less => array = &array[..mid],
                Greater => {
                    pivot += mid;
                    array = &array[mid..];
                }
                Equal => return Some(pivot+mid),
            }*/
            //size -= size / 2;
            /*
            if key < array[mid] {
                array = &array[..mid];
            } else {
                pivot += mid;
                array = &array[mid..];
            };
            */
        }
    }
    if key == array[pivot] { Some(pivot) } else { None }
}
*/

pub fn old_binary_search<T: Ord>(array: &[T], key: T) -> Result<usize, usize>
{
    let mut base = 0usize;
    let mut s = array;
    loop {
        let (head, tail) = s.split_at(s.len() >> 1);
        if tail.is_empty() {
            return Err(base)
        }
        use std::cmp::Ordering::*;
        match tail[0].cmp(&key) {
            Less => {
                base += head.len() + 1;
                s = &tail[1..];
            }
            Greater => s = head,
            Equal => return Ok(base + head.len()),
        }
    }
}
