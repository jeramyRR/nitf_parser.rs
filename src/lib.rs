//!
//! A NITF file consists of a Header and one or more Segments
//!
//! +----------------------------------------------------------------------------+
//! |                                  NITF FILE                                 |
//! +-------------+--------------------+--------------------+--------------------+
//! |             |      SEGEMENT      |      SEGEMENT      |      SEGEMENT      |
//! |             +----------+---------+----------+---------+----------+---------+
//! |    NITF     |          |         |          |         |          |         |
//! | FILE HEADER |  SUB-    |  DATA   |  SUB-    |  DATA   |  SUB-    |  DATA   |
//! |             |  HEADER  |  FIELD  |  HEADER  |  FIELD  |  HEADER  |  FIELD  |
//! |             |          |         |          |         |          |         |
//! +-------------+----------+---------+----------+---------+----------+---------+
//!
//!
//! ------------------------------------------------------------------------------
//! Acronyms:
//!
//! TRE = Tagged Record Extensions
//! DES =  Data Extension Segments
//! NPPBH = Number of Pixels Per Block Horizontal
//! NPPBV = Number of Pixels Per Block Vertical
//!
//! ------------------------------------------------------------------------------
//! References:
//! # [MIL-STD-2500C](http://www.gwg.nga.mil/ntb/baseline/docs/2500c/2500C.pdf)

#[macro_use]
extern crate nom;
extern crate memmap;

use std::str;
use std::fs::File;
use nom::{IResult, be_u8, be_u32};
use memmap::{Mmap, MmapOptions};

const FHDR_SIZE: usize = 4;
const FVER_SIZE: usize = 5;
const CLEVEL_SIZE: usize = 2;
const STYPE_SIZE: usize = 4;
const OSTAID_SIZE: usize = 10;
const FDT_SIZE: usize = 14;
const FTITLE_SIZE: usize = 80;
const FSCLASS_SIZE: usize = 1;
const FSCLSY_SIZE: usize = 2;
const FSCODE_SIZE: usize = 11;
const FSCTLH_SIZE: usize = 2;
const FSREL_SIZE: usize = 20;
const FSDCTP_SIZE: usize = 2;
const FSDCDT_SIZE: usize = 8;
const FSDCXM_SIZE: usize = 4;
const FSDG_SIZE: usize = 1;
const FSDGDT_SIZE: usize = 8;
const FSCLTX_SIZE: usize = 43;
const FSCATP_SIZE: usize = 1;
const FSCAUT_SIZE: usize = 40;
const FSCRSN_SIZE: usize = 1;
const FSSRDT_SIZE: usize = 8;
const FSCTLN_SIZE: usize = 15;
const FSCOP_SIZE: usize = 5;
const FSCPYS_SIZE: usize = 5;
const ENCRYP_SIZE: usize = 1;
const FBKGC_SIZE: usize = 3;


#[derive(Debug)]
pub struct RGB (u8, u8, u8);

#[derive(Debug)]
pub struct NitfHeader<'a> {
  fhdr: &'a [u8],
  fver: &'a [u8],
  clevel: &'a [u8],
  stype: &'a [u8],
  ostaid: &'a [u8],
  fdt: &'a [u8],
  ftitle: &'a [u8],
  fsclass: &'a [u8],
  fsclsy: &'a [u8],
  fscode: &'a [u8],
  fsctlh: &'a [u8],
  fsrel: &'a [u8],
  fsdctp: &'a [u8],
  fsdcdt: &'a [u8],
  fsdcxm: &'a [u8],
  fsdg: &'a [u8],
  fsdgdt: &'a [u8],
  fscltx: &'a [u8],
  fscatp: &'a [u8],
  fscaut: &'a [u8],
  fscrsn: &'a [u8],
  fssrdt: &'a [u8],
  fsctln: &'a [u8],
  fscop: &'a [u8],
  fscpys: &'a [u8],
  encryp: &'a [u8],
  fbkgc: RGB,
}


named!(
  parse_fbkgc <&[u8], RGB>,
  map!(
    take!(FBKGC_SIZE),
    |rgb: &[u8]| RGB(rgb[0], rgb[1], rgb[2])
  )
);


pub fn header(input: &[u8]) -> IResult<&[u8], NitfHeader> {
  do_parse!(input,
  fhdr: take!(FHDR_SIZE) >>
  fver: take!(FVER_SIZE) >>
  clevel: take!(CLEVEL_SIZE) >>
  stype: take!(STYPE_SIZE) >>
  ostaid: take!(OSTAID_SIZE) >>
  fdt: take!(FDT_SIZE) >>
  ftitle: take!(FTITLE_SIZE) >>
  fsclass: take!(FSCLASS_SIZE) >>
  fsclsy: take!(FSCLSY_SIZE) >>
  fscode: take!(FSCODE_SIZE) >>
  fsctlh: take!(FSCTLH_SIZE) >>
  fsrel: take!(FSREL_SIZE) >>
  fsdctp: take!(FSDCTP_SIZE) >>
  fsdcdt: take!(FSDCDT_SIZE) >>
  fsdcxm: take!(FSDCXM_SIZE) >>
  fsdg: take!(FSDG_SIZE) >>
  fsdgdt: take!(FSDGDT_SIZE) >>
  fscltx: take!(FSCLTX_SIZE) >>
  fscatp: take!(FSCATP_SIZE) >>
  fscaut: take!(FSCAUT_SIZE) >>
  fscrsn: take!(FSCRSN_SIZE) >>
  fssrdt: take!(FSSRDT_SIZE) >>
  fsctln: take!(FSCTLN_SIZE) >>
  fscop: take!(FSCOP_SIZE) >>
  fscpys: take!(FSCPYS_SIZE) >>
  encryp: take!(ENCRYP_SIZE) >>
  fbkgc: parse_fbkgc >>
  (
    NitfHeader {
      fhdr: fhdr,
      fver: fver,
      clevel: clevel,
      stype: stype,
      ostaid: ostaid,
      fdt: fdt,
      ftitle: ftitle,
      fsclass: fsclass,
      fsclsy: fsclsy,
      fscode: fscode,
      fsctlh: fsctlh,
      fsrel: fsrel,
      fsdctp: fsdctp,
      fsdcdt: fsdcdt,
      fsdcxm: fsdcxm,
      fsdg: fsdg,
      fsdgdt: fsdgdt,
      fscltx: fscltx,
      fscatp: fscatp,
      fscaut: fscaut,
      fscrsn: fscrsn,
      fssrdt: fssrdt,
      fsctln: fsctln,
      fscop: fscop,
      fscpys: fscpys,
      encryp: encryp,
      fbkgc: fbkgc,
  })
  )
}

#[test]
fn test_version() {
  let input = File::open("test/resources/i_3001a.ntf").expect("File does not exist");
  let mmap = unsafe { MmapOptions::new().map(&input).unwrap() };

  let (_, nitf_hdr) = header(&mmap).unwrap();

  let rgb = nitf_hdr.fbkgc;
  println!("{:#?}", rgb);




  let nitf_string = str::from_utf8(nitf_hdr.fhdr).unwrap();
  assert_eq!("NITF", nitf_string);
  assert_eq!("02.10", str::from_utf8(nitf_hdr.fver).unwrap());
  assert_eq!("03", str::from_utf8(nitf_hdr.clevel).unwrap());
  assert_eq!("BF01", str::from_utf8(nitf_hdr.stype).unwrap());

}
