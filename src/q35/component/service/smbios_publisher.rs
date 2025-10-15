//! SMBIOS Table Publisher Component
//!
//! This component runs after all SMBIOS records have been added and publishes
//! the complete SMBIOS table to the UEFI Configuration Table so that tools
//! like `smbiosview` can access it.

use patina::{
    boot_services::StandardBootServices,
    component::{IntoComponent, service::Service},
    error::Result,
};
use patina_smbios::manager::SmbiosRecords;

#[derive(IntoComponent)]
pub struct SmbiosTablePublisher;

impl SmbiosTablePublisher {
    pub fn new() -> Self {
        Self
    }

    fn entry_point(
        self,
        smbios_service: Service<dyn SmbiosRecords<'static>>,
        boot_services: StandardBootServices,
    ) -> Result<()> {
        log::info!("=== Publishing SMBIOS Table to Configuration Table ===");
        
        // Publish the SMBIOS table to the configuration table
        match smbios_service.publish_table(&boot_services) {
            Ok((table_addr, entry_point_addr)) => {
                log::info!("✓ SMBIOS table published successfully");
                log::info!("  Entry Point: 0x{:X}", entry_point_addr);
                log::info!("  Table Data: 0x{:X}", table_addr);
                log::info!("✓ You can now use 'smbiosview' in UEFI Shell to view records");
            }
            Err(e) => {
                log::error!("Failed to publish SMBIOS table: {:?}", e);
                return Err(r_efi::efi::Status::ABORTED.into());
            }
        }
        
        Ok(())
    }
}
