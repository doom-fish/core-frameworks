mod internal;

pub mod error;
pub mod media_type;

use core_foundation::{base::CFAllocatorRef, dictionary::CFDictionary};
use core_utils_rs::four_char_code::FourCharCode;
pub use internal::{CMFormatDescription, CMFormatDescriptionRef};

impl CMFormatDescription {
    pub fn create(
        allocator: CFAllocatorRef,
        media_type: media_type::CMMediaType,
        media_sub_type: FourCharCode,
        extensions: Option<CFDictionary>,
    ) -> Result<Self, error::CMFormatDescriptionError> {
        Self::internal_create(allocator, media_type, media_sub_type, extensions)
    }
}

// impl CMFormatDescriptionRef {
//     pub fn audio_format_description_get_stream_basic_description(
//         &self,
//     ) -> Option<&AudioStreamBasicDescription> {
//         unsafe {
//             let ptr = CMAudioFormatDescriptionGetStreamBasicDescription(self);
//             if ptr.is_null() {
//                 return None;
//             }
//             Some(&*ptr)
//         }
//     }
// }
//
// #[repr(C)]
// #[derive(Debug, Default, Copy, Clone)]
// pub struct AudioStreamBasicDescription {
//     pub sample_rate: f64,
//     pub format_id: ::std::os::raw::c_uint,
//     pub format_flags: ::std::os::raw::c_uint,
//     pub bytes_per_packet: ::std::os::raw::c_uint,
//     pub frames_per_packet: ::std::os::raw::c_uint,
//     pub bytes_per_frame: ::std::os::raw::c_uint,
//     pub channels_per_frame: ::std::os::raw::c_uint,
//     pub bits_per_channel: ::std::os::raw::c_uint,
//     pub reserved: ::std::os::raw::c_uint,
// }
//
// impl AudioStreamBasicDescription {
//     pub fn get_flag_names(&self) -> Vec<&'static str> {
//         let mut flag_strings = Vec::new();
//         let flags = self.format_flags;
//
//         if flags & kAudioFormatFlagIsFloat != 0 {
//             flag_strings.push("kAudioFormatFlagIsFloat");
//         }
//         if flags & kAudioFormatFlagIsBigEndian != 0 {
//             flag_strings.push("kAudioFormatFlagIsBigEndian");
//         }
//         if flags & kAudioFormatFlagIsSignedInteger != 0 {
//             flag_strings.push("kAudioFormatFlagIsSignedInteger");
//         }
//         if flags & kAudioFormatFlagIsPacked != 0 {
//             flag_strings.push("kAudioFormatFlagIsPacked");
//         }
//         if flags & kAudioFormatFlagIsAlignedHigh != 0 {
//             flag_strings.push("kAudioFormatFlagIsAlignedHigh");
//         }
//         if flags & kAudioFormatFlagIsNonInterleaved != 0 {
//             flag_strings.push("kAudioFormatFlagIsNonInterleaved");
//         }
//         if flags & kAudioFormatFlagIsNonMixable != 0 {
//             flag_strings.push("kAudioFormatFlagIsNonMixable");
//         }
//
//         // kAudioFormatFlagsAreAllClear flag is a special case and should be checked last.
//         if flags & kAudioFormatFlagsAreAllClear != 0 {
//             flag_strings.push("All flags are clear");
//         }
//
//         flag_strings
//     }
// }
// extern "C" {
//     pub static kAudioFormatFlagIsFloat: u32;
//     pub static kAudioFormatFlagIsBigEndian: u32;
//     pub static kAudioFormatFlagIsSignedInteger: u32;
//     pub static kAudioFormatFlagIsPacked: u32;
//     pub static kAudioFormatFlagIsAlignedHigh: u32;
//     pub static kAudioFormatFlagIsNonInterleaved: u32;
//     pub static kAudioFormatFlagIsNonMixable: u32;
//     pub static kAudioFormatFlagsAreAllClear: u32;
//
//     pub static kLinearPCMFormatFlagIsFloat: u32;
//     pub static kLinearPCMFormatFlagIsBigEndian: u32;
//     pub static kLinearPCMFormatFlagIsSignedInteger: u32;
//     pub static kLinearPCMFormatFlagIsPacked: u32;
//     pub static kLinearPCMFormatFlagIsAlignedHigh: u32;
//     pub static kLinearPCMFormatFlagIsNonInterleaved: u32;
//     pub static kLinearPCMFormatFlagIsNonMixable: u32;
//     pub static kLinearPCMFormatFlagsAreAllClear: u32;
//
//     pub static kAppleLosslessFormatFlag16BitSourceData: u32;
//     pub static kAppleLosslessFormatFlag20BitSourceData: u32;
//     pub static kAppleLosslessFormatFlag24BitSourceData: u32;
//     pub static kAppleLosslessFormatFlag32BitSourceData: u32;
//
//     pub static kAudioFormatLinearPCM: ::std::os::raw::c_uint;
//     pub static kAudioFormatAC3: ::std::os::raw::c_uint;
//     pub static kAudioFormat60958AC3: ::std::os::raw::c_uint;
//     pub static kAudioFormatAppleIMA4: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4CELP: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4HVXC: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4TwinVQ: ::std::os::raw::c_uint;
//     pub static kAudioFormatMACE3: ::std::os::raw::c_uint;
//     pub static kAudioFormatMACE6: ::std::os::raw::c_uint;
//     pub static kAudioFormatULaw: ::std::os::raw::c_uint;
//     pub static kAudioFormatALaw: ::std::os::raw::c_uint;
//     pub static kAudioFormatQDesign: ::std::os::raw::c_uint;
//     pub static kAudioFormatQDesign2: ::std::os::raw::c_uint;
//     pub static kAudioFormatQUALCOMM: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEGLayer1: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEGLayer2: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEGLayer3: ::std::os::raw::c_uint;
//     pub static kAudioFormatTimeCode: ::std::os::raw::c_uint;
//     pub static kAudioFormatMIDIStream: ::std::os::raw::c_uint;
//     pub static kAudioFormatParameterValueStream: ::std::os::raw::c_uint;
//     pub static kAudioFormatAppleLossless: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_HE: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_LD: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_ELD: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_ELD_SBR: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_ELD_V2: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_HE_V2: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEG4AAC_Spatial: ::std::os::raw::c_uint;
//     pub static kAudioFormatMPEGD_USAC: ::std::os::raw::c_uint;
//     pub static kAudioFormatAMR: ::std::os::raw::c_uint;
//     pub static kAudioFormatAMR_WB: ::std::os::raw::c_uint;
//     pub static kAudioFormatAudible: ::std::os::raw::c_uint;
//     pub static kAudioFormatiLBC: ::std::os::raw::c_uint;
//     pub static kAudioFormatDVIIntelIMA: ::std::os::raw::c_uint;
//     pub static kAudioFormatMicrosoftGSM: ::std::os::raw::c_uint;
//     pub static kAudioFormatAES3: ::std::os::raw::c_uint;
//     pub static kAudioFormatEnhancedAC3: ::std::os::raw::c_uint;
//     pub static kAudioFormatFLAC: ::std::os::raw::c_uint;
//     pub static kAudioFormatOpus: ::std::os::raw::c_uint;
//
// }
// extern "C" {
//
//     pub fn CMAudioFormatDescriptionGetStreamBasicDescription(
//         desc: CMFormatDescriptionRef,
//     ) -> *const AudioStreamBasicDescriptionRef;
// }
