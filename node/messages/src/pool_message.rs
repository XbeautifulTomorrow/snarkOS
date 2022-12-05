// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolRegisterRequest;

impl MessageTrait for PoolRegisterRequest {
    /// Returns the message name.
    #[inline]
    fn name(&self) -> String {
        "PoolRegisterRequest".to_string()
    }

    /// Serializes the message into the buffer.
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(writer.write_all(&[0])?)
    }

    /// Deserializes the given buffer into a message.
    #[inline]
    fn deserialize(mut bytes: BytesMut) -> Result<Self> {
        // Make sure a byte for the fork flag is available.
        if bytes.remaining() == 0 {
            bail!("Missing fork flag in a 'Pong'");
        }
        let _tag = bytes.get_u8();

        Ok(Self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolRegisterResponse(pub bool);

impl MessageTrait for PoolRegisterResponse {
    /// Returns the message name.
    #[inline]
    fn name(&self) -> String {
        "PoolRegisterResponse".to_string()
    }

    /// Serializes the message into the buffer.
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        let u = u8::from(self.0);
        Ok(writer.write_all(&[u])?)
    }

    /// Deserializes the given buffer into a message.
    #[inline]
    fn deserialize(mut bytes: BytesMut) -> Result<Self> {
        // Make sure a byte for the fork flag is available.
        if bytes.remaining() == 0 {
            bail!("Missing fork flag in a 'Pong'");
        }
        let tag = bytes.get_u8();
        let s = if tag == 1 { Self(true) } else { Self(false) };
        Ok(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolUpdateBlockHeight(pub u32);

impl MessageTrait for PoolUpdateBlockHeight {
    /// Returns the message name.
    #[inline]
    fn name(&self) -> String {
        "PoolUpdateBlockHeight".to_string()
    }

    /// Serializes the message into the buffer.
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.0.to_le_bytes()[..])?;
        Ok(())
    }

    /// Deserializes the given buffer into a message.
    #[inline]
    fn deserialize(mut bytes: BytesMut) -> Result<Self> {
        // Make sure a byte for the fork flag is available.
        if bytes.remaining() == 0 {
            bail!("Missing fork flag in a 'Pong'");
        }
        let block_height: u32 = bytes.get_u32_le();
        Ok(Self(block_height))
    }
}
