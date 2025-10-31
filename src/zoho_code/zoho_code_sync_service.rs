use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use anyhow::Result;
use crate::zoho_code::zoho_code_service::ZohoCodeService;
use crate::http::zoho::search_map_access_by_name;

pub struct ZohoCodeSyncService {
    zoho_code_service: Arc<ZohoCodeService>,
}

impl ZohoCodeSyncService {
    pub fn new(zoho_code_service: Arc<ZohoCodeService>) -> Self {
        Self {
            zoho_code_service,
        }
    }

  
    async fn fetch_current_zoho_code(&self) -> Result<String> {
        
        let map_access_name = std::env::var("ZOHO_MAP_ACCESS_NAME")
            .unwrap_or_else(|_| "default_access".to_string());
        println!("🔍 Searching for map access with name: {}", map_access_name);
        match search_map_access_by_name(&map_access_name).await {
            Ok(map_access) => {
                println!("🔍 Retrieved current Zoho code: {}", map_access.name);
                Ok(map_access.name)
            }
            Err(e) => {
                eprintln!("Failed to fetch current Zoho code: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn sync_zoho_code(&self) -> Result<()> {
        println!("Starting Zoho code synchronization...");

        let current_zoho_code = self.fetch_current_zoho_code().await?;

        match self.zoho_code_service.needs_update(&current_zoho_code) {
            Ok(needs_update) => {
                if needs_update {
                    println!("Zoho code changed, updating database...");
                    
                    match self.zoho_code_service.update_code(&current_zoho_code) {
                        Ok(new_code) => {
                            println!("Zoho code updated successfully: {} (ID: {})", 
                                new_code.code, new_code.id);
                        }
                        Err(e) => {
                            eprintln!("Failed to update Zoho code: {:?}", e);
                            return Err(anyhow::anyhow!("Failed to update Zoho code: {}", e));
                        }
                    }
                } else {
                    println!("Zoho code is up to date, no changes needed");
                }
            }
            Err(e) => {
                eprintln!("Failed to check if code needs update: {:?}", e);
                return Err(anyhow::anyhow!("Failed to check code update status: {}", e));
            }
        }

        Ok(())
    }

    pub async fn initialize_with_zoho(&self) -> Result<()> {
        println!("Initializing Zoho code service...");

        let current_zoho_code = self.fetch_current_zoho_code().await?;
        
        match self.zoho_code_service.initialize_code(&current_zoho_code) {
            Ok(code) => {
                println!("Zoho code service initialized with code: {} (ID: {})", 
                    code.code, code.id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to initialize Zoho code service: {:?}", e);
                Err(anyhow::anyhow!("Failed to initialize Zoho code service: {}", e))
            }
        }
    }

    pub async fn start_automatic_sync(&self, interval_minutes: u64) {
        println!("Starting automatic Zoho code sync every {} minutes", interval_minutes);

        loop {
            match self.sync_zoho_code().await {
                Ok(_) => println!("Zoho code sync completed successfully"),
                Err(e) => eprintln!("Zoho code sync failed: {:?}", e),
            }

            println!("Next Zoho code sync in {} minutes...", interval_minutes);
            time::sleep(Duration::from_secs(interval_minutes * 60)).await;
        }
    }
}