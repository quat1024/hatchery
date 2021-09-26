use std::io;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[async_trait::async_trait]
pub trait McAsyncRead {
	async fn read_vari32(&mut self) -> Result<i32, io::Error>;
	async fn read_vari64(&mut self) -> Result<i64, io::Error>;
}

#[async_trait::async_trait]
pub trait McAsyncWrite {
	async fn write_vari32(&mut self, value: i32) -> Result<(), io::Error>;
	async fn write_vari64(&mut self, value: i64) -> Result<(), io::Error>;
}

#[async_trait::async_trait]
impl<R> McAsyncRead for R
where
	R: AsyncReadExt + Unpin + Send + ?Sized,
{
	async fn read_vari32(&mut self) -> Result<i32, io::Error> {
		let mut shift = 0i32;
		let mut result: i32 = 0;
		loop {
			let read = self.read_u8().await?;

			result |= ((read & 0b01111111) as i32) << shift;
			shift += 7;

			if read & 0b10000000 == 0 {
				break;
			}
		}
		Ok(result)
	}

	//This is the same function find-and-replaced to i64. Okay.
	//The only difference in wiki.vg pseudocode is varint throws after 5 bytes
	//and varlong throws after 10 bytes. I don't know if that is needed and it will muddy up the result types.
	async fn read_vari64(&mut self) -> Result<i64, io::Error> {
		let mut shift = 0i64;
		let mut result: i64 = 0;
		loop {
			let read = self.read_u8().await?;

			result |= ((read & 0b01111111) as i64) << shift;
			shift += 7;

			if read & 0b10000000 == 0 {
				break;
			}
		}
		Ok(result)
	}
}

#[async_trait::async_trait]
impl<W> McAsyncWrite for W
where
	W: AsyncWriteExt + Unpin + Send + ?Sized,
{
	async fn write_vari32(&mut self, value: i32) -> Result<(), io::Error> {
		let mut value = value;

		loop {
			let mut lower_seven: u8 = (value & 0b01111111) as u8;
			value = ((value as u32) >> 7) as i32;

			if value != 0 {
				lower_seven |= 0b10000000;
			}

			self.write_u8(lower_seven).await?;

			if value == 0 {
				return Ok(());
			}
		}
	}

	async fn write_vari64(&mut self, value: i64) -> Result<(), io::Error> {
		let mut value = value;

		loop {
			let mut lower_seven: u8 = (value & 0b01111111) as u8;
			value = ((value as u64) >> 7) as i64;

			if value != 0 {
				lower_seven |= 0b10000000;
			}

			self.write_u8(lower_seven).await?;

			if value == 0 {
				return Ok(());
			}
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;

	use super::*;

	// If you're looking at this like "Wow! negative numbers use so much space!"
	// "Isn't there something called zigzag encoding, from protobuf, that can help with this?"
	// To that I say: yes. You're right. Hey, not my protocol.

	// "Sample varints" from https://wiki.vg/Protocol#VarInt_and_VarLong
	fn i32_cases() -> Vec<(&'static [u8], i32)> {
		vec![
			(&[0x00], 0),
			(&[0x01], 1),
			(&[0x02], 2),
			(&[0x7f], 127),
			(&[0x80, 0x01], 128),
			(&[0xff, 0x01], 255),
			(&[0xff, 0xff, 0x7f], 2097151),
			(&[0xff, 0xff, 0xff, 0xff, 0x07], i32::MAX),
			(&[0xff, 0xff, 0xff, 0xff, 0x0f], -1),
			(&[0x80, 0x80, 0x80, 0x80, 0x08], i32::MIN),
		]
	}

	// "Sample varlongs" from https://wiki.vg/Protocol#VarInt_and_VarLong
	fn i64_cases() -> Vec<(&'static [u8], i64)> {
		vec![
			(&[0x00], 0),
			(&[0x01], 1),
			(&[0x02], 2),
			(&[0x7f], 127),
			(&[0x80, 0x01], 128),
			(&[0xff, 0x01], 255),
			(&[0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
			(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f], i64::MAX),
			(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01], -1),
			(&[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01], i32::MIN as i64),
			(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01], i64::MIN),
		]
	}

	#[tokio::test]
	async fn test_readvari32() {
		for (case, result) in i32_cases() {
			assert_eq!(Cursor::new(case).read_vari32().await.unwrap(), result);
		}
	}

	#[tokio::test]
	async fn test_readvari64() {
		use std::io::Cursor;

		use super::McAsyncRead;

		for (case, result) in i64_cases() {
			assert_eq!(Cursor::new(case).read_vari64().await.unwrap(), result);
		}
	}

	#[tokio::test]
	async fn test_writevari32() {
		for (case, result) in i32_cases() {
			let mut out: Vec<u8> = Vec::new();
			out.write_vari32(result).await.expect("write");
			assert_eq!(&out[..], case);
		}
	}

	#[tokio::test]
	async fn test_writevari64() {
		for (case, result) in i64_cases() {
			let mut out: Vec<u8> = Vec::new();
			out.write_vari64(result).await.expect("write");
			assert_eq!(&out[..], case);
		}
	}
}
