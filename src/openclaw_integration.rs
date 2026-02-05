/// OpenClaw integration - runs automatically with the node
/// Handles ceremony coordination and health monitoring in background
/// Spawns Python-based agents for security, network optimization, and monitoring

use tokio::task::JoinHandle;
use std::process::{Command, Child, Stdio};
use std::env;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

struct OpenClawAgents {
    security_guardian: Option<Child>,
    network_booster: Option<Child>,
    health_monitor: Option<Child>,
    ceremony_coordinator: Option<Child>,
}

pub async fn start_openclaw_background() -> Result<JoinHandle<()>, Box<dyn std::error::Error + Send + Sync>> {
    // Determine OpenClaw config path
    let config_path = env::var("AXIOM_OPENCLAW_CONFIG")
        .unwrap_or_else(|_| "./openclaw/bootstrap_server_config.json".to_string());
    
    // Get base directory for agents
    let base_dir = env::current_dir()?;
    
    // Spawn background task that manages all OpenClaw agents
    let handle = tokio::spawn(async move {
        match run_openclaw_daemon(&config_path, &base_dir).await {
            Ok(_) => println!("✅ OpenClaw agents terminated gracefully"),
            Err(e) => eprintln!("⚠️  OpenClaw error: {}", e),
        }
    });
    
    Ok(handle)
}

async fn run_openclaw_daemon(config_path: &str, base_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 OpenClaw daemon starting...");
    println!("📁 Config: {}", config_path);
    
    // Check if Python is available
    let python_check = Command::new("python3")
        .arg("--version")
        .output();
    
    match python_check {
        Ok(_) => println!("✅ Python3 found - agents will be launched"),
        Err(_) => {
            println!("⚠️  Python3 not found - agents will not start");
            println!("    Install Python3 to enable: sudo apt install python3");
            return Ok(());
        }
    }
    
    let mut agents = OpenClawAgents {
        security_guardian: None,
        network_booster: None,
        health_monitor: None,
        ceremony_coordinator: None,
    };
    
    // Start Security Guardian Agent
    agents.security_guardian = start_agent(
        base_dir,
        "security_guardian_agent.py",
        "🛡️  SECURITY GUARDIAN",
    );
    
    // Start Network Booster Agent
    agents.network_booster = start_agent(
        base_dir,
        "network_booster_agent.py",
        "🚀 NETWORK BOOSTER",
    );
    
    // Start Health Monitor Agent
    agents.health_monitor = start_agent(
        base_dir,
        "node_health_monitor.py",
        "🏥 HEALTH MONITOR",
    );
    
    // Start Ceremony Coordinator Agent
    agents.ceremony_coordinator = start_agent(
        base_dir,
        "ceremony_master.py",
        "📜 CEREMONY COORDINATOR",
    );
    
    // Keep agents running and restart if they crash
    loop {
        sleep(Duration::from_secs(10)).await;
        
        // Check each agent status
        if let Some(mut child) = agents.security_guardian.take() {
            match child.try_wait() {
                Ok(None) => {
                    agents.security_guardian = Some(child); // Still running
                },
                Ok(Some(status)) => {
                    println!("⚠️  Security Guardian crashed: {}", status);
                    agents.security_guardian = start_agent(base_dir, "security_guardian_agent.py", "🛡️  SECURITY GUARDIAN");
                },
                Err(e) => {
                    println!("⚠️  Error checking Security Guardian: {}", e);
                    agents.security_guardian = start_agent(base_dir, "security_guardian_agent.py", "🛡️  SECURITY GUARDIAN");
                }
            }
        } else {
            agents.security_guardian = start_agent(base_dir, "security_guardian_agent.py", "🛡️  SECURITY GUARDIAN");
        }
        
        // Check Network Booster
        if let Some(mut child) = agents.network_booster.take() {
            match child.try_wait() {
                Ok(None) => {
                    agents.network_booster = Some(child); // Still running
                },
                Ok(Some(status)) => {
                    println!("⚠️  Network Booster crashed: {}", status);
                    agents.network_booster = start_agent(base_dir, "network_booster_agent.py", "🚀 NETWORK BOOSTER");
                },
                Err(e) => {
                    println!("⚠️  Error checking Network Booster: {}", e);
                    agents.network_booster = start_agent(base_dir, "network_booster_agent.py", "🚀 NETWORK BOOSTER");
                }
            }
        } else {
            agents.network_booster = start_agent(base_dir, "network_booster_agent.py", "🚀 NETWORK BOOSTER");
        }
        
        // Check Health Monitor
        if let Some(mut child) = agents.health_monitor.take() {
            match child.try_wait() {
                Ok(None) => {
                    agents.health_monitor = Some(child); // Still running
                },
                Ok(Some(status)) => {
                    println!("⚠️  Health Monitor crashed: {}", status);
                    agents.health_monitor = start_agent(base_dir, "node_health_monitor.py", "🏥 HEALTH MONITOR");
                },
                Err(e) => {
                    println!("⚠️  Error checking Health Monitor: {}", e);
                    agents.health_monitor = start_agent(base_dir, "node_health_monitor.py", "🏥 HEALTH MONITOR");
                }
            }
        } else {
            agents.health_monitor = start_agent(base_dir, "node_health_monitor.py", "🏥 HEALTH MONITOR");
        }
        
        // Check Ceremony Coordinator
        if let Some(mut child) = agents.ceremony_coordinator.take() {
            match child.try_wait() {
                Ok(None) => {
                    agents.ceremony_coordinator = Some(child); // Still running
                },
                Ok(Some(status)) => {
                    println!("⚠️  Ceremony Coordinator crashed: {}", status);
                    agents.ceremony_coordinator = start_agent(base_dir, "ceremony_master.py", "📜 CEREMONY COORDINATOR");
                },
                Err(e) => {
                    println!("⚠️  Error checking Ceremony Coordinator: {}", e);
                    agents.ceremony_coordinator = start_agent(base_dir, "ceremony_master.py", "📜 CEREMONY COORDINATOR");
                }
            }
        } else {
            agents.ceremony_coordinator = start_agent(base_dir, "ceremony_master.py", "📜 CEREMONY COORDINATOR");
        }
    }
}

fn start_agent(base_dir: &std::path::Path, script_name: &str, agent_name: &str) -> Option<Child> {
    let script_path = base_dir.join("openclaw").join(script_name);
    
    if !Path::new(&script_path).exists() {
        println!("⚠️  {} agent not found at: {}", agent_name, script_path.display());
        return None;
    }
    
    match Command::new("python3")
        .arg(script_path.to_string_lossy().to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => {
            println!("✅ {} agent started (PID: {})", agent_name, child.id());
            Some(child)
        }
        Err(e) => {
            println!("❌ Failed to start {} agent: {}", agent_name, e);
            None
        }
    }
}
