/*
FaF is a high performance dns benchmarking tool
Copyright (C) 2023  James Bates

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(unused_imports)]
#![allow(dead_code)]

mod args;
mod statics;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(serde::Serialize)]
pub struct MessageSerdeJson {
   pub message: &'static str,
}

use nanoserde::SerJson;

#[derive(nanoserde::SerJson)]
pub struct MessageNanoserde {
   pub message: &'static str,
}

#[derive(simd_json_derive::Serialize)]
pub struct MessageSimdJsonDerive {
   pub message: &'static str,
}

#[repr(C, align(64))]
pub struct Timespec {
   pub tv_sec: i64,
   pub tv_nsec: i64,
}

extern "C" {
   // We use this function instead of a direct syscall because this function uses VDSO, which is faster
   fn clock_gettime(clk_id: i32, tp: *mut Timespec) -> i32;
}

const CLOCK_REALTIME: i32 = 0;
const HELLO_WORLD_JSON_BYTES: [u8; 26] =
   [123, 34, 109, 101, 115, 115, 97, 103, 101, 34, 58, 34, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 34, 125];

const HELLO_WORLD_JSON_STR: &str = r#"{"message":"Hello World!"}"#;

fn main() {
   {
      // Handle `about` argument
      if statics::ARGS.about {
         print_version();
         return;
      }
   }

   {
      // Handle `clear` argument
      if statics::ARGS.clear {
         //json_stats::StatsSet::clear();
         println!("Stats Cleared.");
         return;
      }
   }

   let duration_nanos: u64 = statics::ARGS.duration * 1_000_000_000u64;

   let mut ts: Timespec = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };

   {
      // serde_json to_vec
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         let out = serde_json::to_vec(&message).unwrap();
         assert_eq!(out.as_slice(), HELLO_WORLD_JSON_BYTES);
         let bytes_len = out.len();
         byte_count += bytes_len as u64;
      }

      print_output("serde_json to_vec", byte_count)
   }

   {
      // serde_json to_writer

      let mut writer = Vec::with_capacity(26);
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         serde_json::to_writer(&mut writer, &message).unwrap();
         let writer_slice = writer.as_slice();
         assert_eq!(writer_slice, HELLO_WORLD_JSON_BYTES);
         byte_count += writer_slice.len() as u64;
         writer.clear();
      }

      print_output("serde_json to_writer", byte_count)
   }

   {
      // serde_json_core to_vec

      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         let out = serde_json_core::to_vec::<MessageSerdeJson, 26>(&message).unwrap();
         assert_eq!(out.as_slice(), HELLO_WORLD_JSON_BYTES);
         byte_count += out.len() as u64;
      }

      print_output("serde_json_core to_vec", byte_count)
   }

   {
      // serde_json_core to_slice

      let mut buf: [u8; 26] = [0; 26];
      let buf_slice = buf.as_mut_slice();

      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         let bytes_len = serde_json_core::to_slice::<MessageSerdeJson>(&message, buf_slice).unwrap();
         assert_eq!(buf_slice, HELLO_WORLD_JSON_BYTES);
         byte_count += bytes_len as u64;
         // CANNOT writer.clear(); when using a vec instead of a byte array,
         //   the slice passed must be 'full' with values.
         // Seems like a bug but it works.
      }

      print_output("serde_json_core to_slice", byte_count)
   }

   {
      // nanoserde serialize_json

      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageNanoserde { message: "Hello World!" };
         let out = nanoserde::SerJson::serialize_json(&message);
         assert_eq!(out, HELLO_WORLD_JSON_STR);
         let bytes_len = out.len();
         byte_count += bytes_len as u64;
      }

      print_output("nanoserde serialize_json", byte_count)
   }

   {
      // nanoserde ser_json

      let mut state = nanoserde::SerJsonState { out: String::with_capacity(26) };
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageNanoserde { message: "Hello World!" };
         message.ser_json(26, &mut state);
         let bytes_len = state.out.len();
         assert_eq!(state.out, HELLO_WORLD_JSON_STR);
         byte_count += bytes_len as u64;
         state.out.clear();
      }

      print_output("nanoserde ser_json", byte_count)
   }

   {
      // simd_json serde::to_vec
      
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         let out = simd_json::serde::to_vec(&message).unwrap();
         assert_eq!(out, HELLO_WORLD_JSON_BYTES);
         let bytes_len = out.len();
         byte_count += bytes_len as u64;
      }

      print_output("simd_json serde::to_vec", byte_count)
   }

   {
      // simd_json to_writer

      let mut writer = Vec::with_capacity(26);
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSerdeJson { message: "Hello World!" };
         simd_json::serde::to_writer(&mut writer, &message).unwrap();
         let writer_slice = writer.as_slice();
         assert_eq!(writer_slice, HELLO_WORLD_JSON_BYTES);
         byte_count += writer_slice.len() as u64;
         writer.clear();
      }

      print_output("simd_json to_writer", byte_count)
   }

   {
      // simd_json_derive json_vec
      
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSimdJsonDerive { message: "Hello World!" };
         let out = simd_json_derive::Serialize::json_vec(&message).unwrap();
         assert_eq!(out, HELLO_WORLD_JSON_BYTES);
         let bytes_len = out.len();
         byte_count += bytes_len as u64;
      }

      print_output("simd_json_derive json_vec", byte_count)
   }

   {
      // simd_json_derive to_writer

      let mut writer = Vec::with_capacity(26);
      let mut byte_count = 0u64;
      let start_time_nanos = get_epoch_nanos(&mut ts);

      loop {
         if (get_epoch_nanos(&mut ts) - start_time_nanos) > duration_nanos {
            break;
         }

         let message = MessageSimdJsonDerive { message: "Hello World!" };
         simd_json_derive::Serialize::json_write(&message, &mut writer).unwrap();
         let writer_slice = writer.as_slice();
         assert_eq!(writer_slice, HELLO_WORLD_JSON_BYTES);
         byte_count += writer_slice.len() as u64;
         writer.clear();
      }

      print_output("simd_json_derive to_writer", byte_count)
   }
}

// SAFETY:
// current epoch time is 1694832834
// 1694832834 * 1_000_000_000 = 1,694,832,834,000,000,000
// u64::MAX is                 18,446,744,073,709,551,615
// So, we are safe for a while longer
fn get_epoch_nanos(ts: &mut Timespec) -> u64 {
   unsafe { clock_gettime(CLOCK_REALTIME, ts as *mut Timespec) };

   ts.tv_sec as u64 * 1_000_000_000u64 + ts.tv_nsec as u64
}

fn print_output(lib_name: &str, bytes_serialized: u64) {
   let bytes_per_second = format!("{:.0}", bytes_serialized as f64 / statics::ARGS.duration as f64)
      .as_bytes()
      .rchunks(3)
      .rev()
      .map(core::str::from_utf8)
      .collect::<Result<Vec<&str>, _>>()
      .unwrap()
      .join(",");
   println!("{:<26} {:>13} bytes/sec", lib_name, bytes_per_second);
}

fn print_version() {
   println!("{} v{} | repo: https://github.com/errantmind/faf-http-bench\n", statics::PROJECT_NAME, statics::VERSION,);
}

// impl<'a, B: &[u8]> std::io::Write for &[u8] {
//    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//        self.put_slice(buf);
//        Ok(buf.len())
//    }
//    fn flush(&mut self) -> io::Result<()> {
//        Ok(())
//    }
// }