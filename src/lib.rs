pub mod siamese;
pub mod wirehair;

#[cfg(test)]
mod tests {
    use super::wirehair::*;

    #[test]
    fn basic_flow_works_wirehair() {
        assert!(wirehair_init().is_ok());

        let mut message = [0u8; 500];
        for i in 0..500 {
            message[i] = i as u8
        }

        let encoder = WirehairEncoder::new(&mut message, 500, 50);
        let decoder = WirehairDecoder::new(500, 50);

        let mut block_id = 0;

        loop {
            let mut block = [0u8; 50];
            let mut block_out_bytes: u32 = 0;
            let result = encoder.encode(block_id, &mut block, 50, &mut block_out_bytes);
            assert!(result.is_ok());

            if block_id % 5 == 0 {
                block_id += 1;
                continue;
            }

            let result = decoder.decode(block_id, &block, block_out_bytes);
            assert!(result.is_ok());

            block_id += 1;

            match result.unwrap() {
                WirehairResult::NeedMore => continue,
                WirehairResult::Success => break,
                _ => panic!(),
            }
        }

        let mut decoded_message = [0u8; 500];

        let result = decoder.recover(&mut decoded_message, 500);
        assert!(result.is_ok());

        assert!(wirehair_decoder_to_encoder(decoder).is_ok());
    }
}
