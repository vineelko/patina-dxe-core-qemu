//! SMBIOS Example Component for QEMU Q35
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;

use patina::{component::{IntoComponent, service::Service}, error::Result};
use patina_smbios::manager::{SmbiosRecords, SMBIOS_HANDLE_PI_RESERVED, SmbiosTableHeader};
use patina_smbios::smbios_record::{Type0PlatformFirmwareInformation, Type1SystemInformation, SmbiosRecordStructure};

#[derive(IntoComponent)]
pub struct SmbiosExamplePublisher;

impl SmbiosExamplePublisher {
    pub fn new() -> Self {
        Self
    }

    fn entry_point(self, smbios_service: Service<dyn SmbiosRecords<'static>>) -> Result<()> {
        log::info!("=== SMBIOS 3.9 Integration Validator ===");
        let (major, minor) = smbios_service.version();
        log::info!("SMBIOS Version: {}.{}", major, minor);
        if major == 3 && minor == 9 {
            log::info!("✓ SMBIOS 3.9 support confirmed");
        }
        
        // Add sample SMBIOS records using typed structs
        log::info!("Adding sample SMBIOS records...");
        
        // Type 0: BIOS Information
        let bios_info = Type0PlatformFirmwareInformation {
            header: SmbiosTableHeader { record_type: 0, length: 0, handle: SMBIOS_HANDLE_PI_RESERVED },
            vendor: 1,  // String index
            firmware_version: 2,  // String index
            bios_starting_address_segment: 0xE800,
            firmware_release_date: 3,  // String index
            firmware_rom_size: 0xFF,  // 16MB
            characteristics: 0x08,  // BIOS characteristics
            characteristics_ext1: 0x03,
            characteristics_ext2: 0x03,
            system_bios_major_release: 1,
            system_bios_minor_release: 0,
            embedded_controller_major_release: 0xFF,
            embedded_controller_minor_release: 0xFF,
            extended_bios_rom_size: 0,
            string_pool: vec![
                String::from("Patina Firmware"),
                String::from("1.0.0"),
                String::from("10/15/2025"),
            ],
        };
        
        let bios_bytes = bios_info.to_bytes();
        match smbios_service.add_from_bytes(None, &bios_bytes) {
            Ok(handle) => log::info!("✓ Added Type 0 (BIOS Info) with handle 0x{:04X}", handle),
            Err(e) => log::warn!("Failed to add Type 0: {:?}", e),
        }
        
        // Type 1: System Information
        let system_info = Type1SystemInformation {
            header: SmbiosTableHeader { record_type: 1, length: 0, handle: SMBIOS_HANDLE_PI_RESERVED },
            manufacturer: 1,  // String index
            product_name: 2,   // String index
            version: 3,        // String index
            serial_number: 4,  // String index
            uuid: [
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            ],
            wake_up_type: 0x06,  // Power Switch
            sku_number: 5,       // String index
            family: 6,           // String index
            string_pool: vec![
                String::from("OpenDevicePartnership"),
                String::from("Patina Q35 Platform"),
                String::from("1.0"),
                String::from("PAT-Q35-001"),
                String::from("SKU-001"),
                String::from("QEMU"),
            ],
        };
        
        let system_bytes = system_info.to_bytes();
        match smbios_service.add_from_bytes(None, &system_bytes) {
            Ok(handle) => log::info!("✓ Added Type 1 (System Info) with handle 0x{:04X}", handle),
            Err(e) => log::warn!("Failed to add Type 1: {:?}", e),
        }

        // Type 127: End-of-Table (Required by SMBIOS spec)
        // This is a mandatory marker indicating the end of the SMBIOS structure table
        let end_of_table: Vec<u8> = vec![
            127,  // Type 127 (End-of-Table)
            4,    // Length (header only, 4 bytes)
            0xFF, 0xFF,  // Handle (typically 0xFFFF for end marker)
            0x00, 0x00,  // Double-null terminator (no strings)
        ];
        match smbios_service.add_from_bytes(None, &end_of_table) {
            Ok(handle) => log::info!("✓ Added Type 127 (End-of-Table) with handle 0x{:04X}", handle),
            Err(e) => log::warn!("Failed to add Type 127: {:?}", e),
        }
        
        // Enumerate all SMBIOS records present in the system
        log::info!("Enumerating SMBIOS records:");
        log::info!("TIP: Use 'dmem 0x<address> 0x100' in UEFI Shell to view raw SMBIOS data");
        let mut handle: u16 = SMBIOS_HANDLE_PI_RESERVED;  // Start with reserved handle for first record
        let mut count = 0;
        
        loop {
            match smbios_service.get_next(&mut handle, None) {
                Ok((header, _producer)) => {
                    let type_name = match header.record_type {
                        0 => "BIOS Information",
                        1 => "System Information",
                        2 => "Baseboard Information",
                        3 => "System Enclosure",
                        4 => "Processor Information",
                        16 => "Physical Memory Array",
                        17 => "Memory Device",
                        19 => "Memory Array Mapped Address",
                        32 => "System Boot Information",
                        127 => "End-of-Table",
                        _ => "Other",
                    };
                    // Copy packed fields to local variables to avoid unaligned reference error
                    let length = header.length;
                    let handle_value = header.handle;
                    log::info!("  [{}] Type {}: {} (Length={}, Handle=0x{:04X})", 
                        count, header.record_type, type_name, length, handle_value);
                    count += 1;
                }
                Err(e) => {
                    if count == 0 {
                        log::error!("Failed to get first SMBIOS record: {:?}", e);
                    }
                    break;
                }
            }
        }
        
        if count == 0 {
            log::warn!("No SMBIOS records found in the system");
        } else {
            log::info!("Total SMBIOS records found: {}", count);
        }
        
        log::info!("=== SMBIOS Validation Complete ===");
        Ok(())
    }
}
