use std::os::raw::{c_char, c_int, c_uchar, c_void};

#[derive(PartialEq)]
#[repr(C)]
enum SiameseResultCode {
    Success = 0,
    InvalidInput = 1,
    NeedMoreData = 2,
    MaxPacketsReached = 3,
    DuplicateData = 4,
    Disabled = 5,
}

#[repr(C)]
struct SiameseOriginalPacket {
    PacketNum: u32,
    DataBytes: u32,
    Data: *const u8,
}

#[repr(C)]
struct SiameseRecoveryPacket {
    DataBytes: u32,
    Data: *const u8,
}

#[link(name = "siamese")]
extern "C" {
    fn siamese_init_(version: c_int) -> SiameseResultCode;

    fn siamese_encoder_create() -> *const c_void;

    fn siamese_encoder_free(encoder: *const c_void) -> c_void;

    fn siamese_encoder_is_ready(encoder: *const c_void) -> SiameseResultCode;

    fn siamese_encoder_add(
        encoder: *const c_void,
        original_packet: *mut SiameseOriginalPacket,
    ) -> SiameseResultCode;

    fn siamese_encoder_get(
        encoder: *const c_void,
        original_packet: *mut SiameseOriginalPacket,
    ) -> SiameseResultCode;

    fn siamese_encoder_remove_before(encoder: *const c_void, packet_num: u32) -> SiameseResultCode;

    fn siamese_encoder_ack(
        encoder: *const c_void,
        buffer: *const c_void,
        bytes: u32,
        next_expected_packet_num: *mut u32,
    );

    fn siamese_encoder_retransmit(
        encoder: *const c_void,
        original_packet: *mut SiameseOriginalPacket,
    ) -> SiameseResultCode;

    fn siamese_encode(
        encoder: *const c_void,
        recovery_packet: *mut SiameseRecoveryPacket,
    ) -> SiameseResultCode;

    fn siamese_encoder_stats(
        encoder: *const c_void,
        stats_out: *mut [u64],
        stats_count: u32,
    ) -> SiameseResultCode;

    fn siamese_decoder_create() -> *const c_void;

    fn siamese_decoder_free(decoder: *const c_void) -> c_void;

    fn siamese_decoder_add_original(
        decoder: *const c_void,
        original_packet: *const SiameseOriginalPacket,
    ) -> SiameseResultCode;

    fn siamese_decoder_add_recovery(
        decoder: *const c_void,
        recovery_packet: *const SiameseRecoveryPacket,
    ) -> SiameseResultCode;

    fn siamese_decoder_get(
        decoder: *const c_void,
        original_packet: *mut SiameseOriginalPacket,
    ) -> SiameseResultCode;

    fn siamese_decoder_is_ready(decoder: *const c_void) -> SiameseResultCode;

    fn siamese_decode(
        decoder: *const c_void,
        original_packets: *mut *mut SiameseOriginalPacket,
        count_out: *mut u32,
    ) -> SiameseResultCode;

    fn siamese_decoder_ack(
        decoder: *const c_void,
        buffer: *const c_void,
        byte_limit: u32,
        used_bytes: *mut u32,
    ) -> SiameseResultCode;

    fn siamese_decoder_stats(
        decoder: *const c_void,
        stats_out: *mut [u64],
        stats_count: u32,
    ) -> SiameseResultCode;
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::mem::MaybeUninit;

    use crate::siamese::*;

    #[test]
    fn basic_flow_works_fine_siamese() {
        unsafe {
            siamese_init_(5);
            let encoder = siamese_encoder_create();
            let decoder = siamese_decoder_create();

            let mut message = [0u8; 500];
            for i in 0..500 {
                message[i] = i as u8
            }

            let original = SiameseOriginalPacket {
                PacketNum: 0,
                Data: message.as_ptr(),
                DataBytes: 500,
            };

            let mut recovery: MaybeUninit<SiameseRecoveryPacket> = MaybeUninit::uninit();
            siamese_encode(encoder, recovery.as_mut_ptr());
            siamese_decoder_add_recovery(decoder, recovery.as_mut_ptr());

            if siamese_decoder_is_ready(decoder) == SiameseResultCode::Success {
                println!("Recovering");
                let mut recovered: MaybeUninit<*mut SiameseOriginalPacket> = MaybeUninit::uninit();
                let mut recovered_count = 0u32;
                siamese_decode(decoder, recovered.as_mut_ptr(), &mut recovered_count);
            }
        }
    }
}
