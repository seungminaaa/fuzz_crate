#![feature(test)]

extern crate test;

use calamine::{open_workbook, Ods, Reader, Xls, Xlsb, Xlsx};
use std::fs::File;
use std::io::BufReader;
use test::Bencher;

fn count<R: Reader<BufReader<File>>>(path: &str) -> usize {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path);
    let mut excel: R = open_workbook(&path).expect("cannot open excel file");

    let sheets = excel.sheet_names().to_owned();
    let mut count = 0;
    for s in sheets {
        count += excel
            .worksheet_range(&s)
            .unwrap()
            .unwrap()
            .rows()
            .flat_map(|r| r.iter())
            .count();
    }
    count
}

#[bench]
fn bench_xls(b: &mut Bencher) {
    b.iter(|| count::<Xls<_>>("tests/issues.xls"));
}

#[bench]
fn bench_xlsx(b: &mut Bencher) {
    b.iter(|| count::<Xlsx<_>>("tests/issues.xlsx"));
}

#[bench]
fn bench_xlsb(b: &mut Bencher) {
    b.iter(|| count::<Xlsb<_>>("tests/issues.xlsb"));
}

#[bench]
fn bench_ods(b: &mut Bencher) {
    b.iter(|| count::<Ods<_>>("tests/issues.ods"));
}
