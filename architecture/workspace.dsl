workspace "IPRChain" "Decentralized IP Registry and Ownership Management" {

    !identifiers hierarchical

    model {
        #=========== Actors ============#
        creator = person "Creator" "Artists / Designers / Developers registering IP"
        startup = person "Startup" "Businesses managing IP Portfolio"
        lawyer = person "Lawyer" "Validates legal ownership"
        buyer = person "Buyer" "Discovers / Acquires IP"

        #=========== Software Systems ============#
        ipr_chain = softwareSystem "IPRChain" "Decentralized IP Registry and Ownership Management" {
            frontend = container "Frontend Application" {
              registration = component "Registry UI" "Registers / Uploads IP" {
                  tags "Frontend"
              } 
              dashboard = component "Dashboard UI" "Analytics / Management" { 
                  tags "Frontend" 
              }
              marketplace = component "Marketplace" "IP discovery portal" {
                  tags "Frontend"
              }
            }
            backend = container "Backend Service" "Node.js API"
            solana_program = container "Solana Program" "Smart Contracts / On-chain logic" {
                mint_program = component "Mint Program" "SPL Token Creation" 
                license_program = component "License Program" "Royalty Management"
                dao_program = component "DAO Program" "Dispute Resolution"
            }

            storage = container "Storage" "Decentralized Storage" {
                tags "Storage"
            }
        }

        solana_network = softwareSystem "Solana Network" "Blockchain infrastructure"
        oracle = softwareSystem "Pyth Oracle" "Price feeds" {
            tags "Oracle"
        }
        lending = softwareSystem "MarginFi" "DeFi Lending"
        wallet = softwareSystem "User Wallet" "Digital Wallet"

        #=========== Relationships ============#
        creator -> ipr_chain.frontend.registration "Registers / Mints IP"    
        startup -> ipr_chain.frontend.registration "Bulk registrations"
        lawyer -> ipr_chain.frontend.dashboard "Validates ownership"    
        buyer -> ipr_chain.frontend.marketplace "Discovers / Acquires IP"    
        
        ipr_chain.frontend.registration -> ipr_chain.backend "Submits metadata"
        ipr_chain.frontend.dashboard -> ipr_chain.backend "Sends API request"
        ipr_chain.frontend.marketplace -> ipr_chain.backend "Initiates license"

        ipr_chain.backend -> ipr_chain.storage "Reads from and writes to"
        ipr_chain.backend -> ipr_chain.solana_program.mint_program "Mints SPL Token"
        ipr_chain.backend -> ipr_chain.solana_program.license_program "Set Royalty"

        ipr_chain.solana_program.mint_program -> solana_network "On-chain records"
        ipr_chain.solana_program.license_program -> oracle "Price Data"
        ipr_chain.solana_program.dao_program -> lending "Collateral Management"
        ipr_chain.solana_program.dao_program -> wallet "Wallet Auth"
    }

    views {
        systemContext ipr_chain "Diagram1" {
            include *
            autolayout tb
        }

        container ipr_chain "Diagram2" {
            include *
            autolayout tb
        }

        component ipr_chain.frontend "Diagram3" {
            include *
            autolayout tb
        }

        component ipr_chain.solana_program "Diagram4" {
            include *
            autolayout tb
        }

        styles {
            element "Element" {
                color #ffffff
                stroke #eeeeee
            }
            element "Person" {
                background SeaGreen
                shape person
            }
            element "Software System" {
                background indigo
            }
            element "Container" {
                background olive 
            }
            element "Component" {
                background orangered
            }
            element "Storage" {
                shape cylinder
            }
            element "Oracle" {
                shape hexagon
            }
        }
    }

    configuration {
        scope softwaresystem
    }

}
