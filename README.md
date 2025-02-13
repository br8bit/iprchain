# IPRChain - Onchain Intellectual Property Registry

IPRChain is a decentralized platform on the Solana blockchain designed to enable individuals and organizations to securely register and verify ownership of their intellectual property and copyrights.

By utilizing blockchain’s immutability and transparency, IPRChain ensures that all processes are permanently recorded on-chain, providing indisputable proof of ownership.

The platform simplifies the intellectual property registration process, making it accessible globally and eliminating the reliance on traditional centralized organizations. Users will receive digital proof of their ownership, enhancing trust and reducing disputes in intellectual property claims.

## Architecture

### Core Components

1. **IP Registry**
   - Creator registers an IP, the protocol generates a unique identifier (IP ID).
   - This identifier is stored in the IP Registry (a PDA).
   - Metadata (title, creator, description, rights, etc.) is hashed and stored on-chain, while the full content can be stored off-chain (e.g., IPFS, Arweave).
   - The registered IP is linked to NFTs to represent verifiable ownership.
2. **Licensing**
   - The creator defines licensing terms (usage rights, royalty rules).
   - These licenses define:
     - Who can use the IP (public, private, specific entities)
     - How it can be used (commercial vs. non-commercial)
     - Revenue-sharing terms (percentage of royalties, upfront fees, etc.)
   - Other users discover the IP according to the set terms.
   - A licensee accepts the license and pays the required fees.
   - The protocol grants access to the IP.
3. **Derivative Tracking System**
   - A licensee creates a derivative work, registering it on the protocol.
   - A new IP ID is generated but remains linked to the original work for attribution.
4. **Economic & Incentive Layer**
   - The derivative work is listed on a marketplace for sale.
   - The marketplace reports sales to the protocol.
   - The protocol’s revenue system calculates royalty splits.
   - Funds are automatically distributed to the original creator and new contributor(s).

## Accounts

```mermaid
classDiagram

%% -- Core Accounts --
class IpAccount {
    owner: Pubkey
    mint: Pubkey
    ip_hash: [u8; 32]
    metadata_uri: String
    created_at: i64
}

class LicenseAccount {
    ip_account: Pubkey,
    licensee: Pubkey,
    terms: LicenseTerms,
    status: LicenseStatus,
    total_royalties_paid: u64,
    last_payment_date: i64,
}

%% -- Supporting Structs/Enums --
class LicenseTerms {
    fee: u64
    expiration: i64
    royalty_percent: u8
    commercial: bool
}

class LicenseStatus {
    <<enumeration>>
    Active
    Revoked
    Expired
}

%% -- External Connections --
class MetaplexTokenMetadata {
    mint: Pubkey
    uri: String
    royalties: RoyaltyConfig
}

class Arweave {
    content_hash: String
    json_uri: String
}

%% -- Royalty Configuration --
class RoyaltyConfig {
    royalty_basis_points: u16
    royalty_share: Pubkey
    recipients: Vec < RoyaltyRecipient >
    %% Metaplex-standard royalty structure
}

class RoyaltyRecipient {
    address: Pubkey,
    share: u8, // Percentage share (sum to 100)
}


%% -- Relationships --
IpAccount --> Arweave : "metadata_uri points\nto off-chain JSON"
IpAccount --> MetaplexTokenMetadata : "uses mint address\nfor ownership via NFT"
LicenseAccount --> IpAccount : "references\nlicensed IP"
LicenseAccount --> LicenseTerms : "contains"
LicenseAccount --> LicenseStatus : "tracks"
LicenseTerms --> MetaplexTokenMetadata : "royalties enforced\nvia Metaplex standard"
LicenseTerms --> RoyaltyConfig : "converted to\nbasis points (1% = 100)"
MetaplexTokenMetadata --> RoyaltyConfig : "stores"
RoyaltyConfig --> RoyaltyRecipient: "stores"
```

## Sequence Diagrams

### Creator

```mermaid
sequenceDiagram
    participant Creator
    participant Frontend as React Frontend
    participant Backend as Node.js Backend
    participant Solana as Solana Program
    participant Arweave as Arweave/IPFS
    participant Phantom as Phantom Wallet

    %% Creator Registration Flow
    Creator->>Frontend: Upload IP File
    activate Frontend
        Frontend->>Frontend: Compute SHA-256 Hash
        Frontend->>Backend: Check Hash Uniqueness (hash)
        activate Backend
            Backend->>Solana: PDA Exists?(hash)
            alt Hash Exists
                Solana-->>Backend: Error: Duplicate
                Backend-->>Frontend: Registration Denied
                Frontend-->>Creator: Show Error
            else
                Backend-->>Frontend: Proceed
                Frontend->>Arweave: Store File
                Arweave-->>Frontend: CID
                Frontend->>Phantom: Request Signature
                Phantom-->>Frontend: Signed TX
                Frontend->>Backend: Register IP (hash, CID)
                Backend->>Solana: Mint SPL Token (hash, CID)
                Solana->>Solana: Create PDA Account
                Solana-->>Backend: TX Confirmation
                Backend-->>Frontend: Success
                Frontend-->>Creator: Display Certificate
            end
        deactivate Backend
    deactivate Frontend
```

### Startup

```mermaid
sequenceDiagram
    participant Startup
    participant Frontend as React Frontend
    participant Backend as Node.js Backend
    participant Solana as Solana Program

    %% Bulk Upload Flow
    Startup->>Frontend: Bulk Upload CSV
    activate Frontend
        loop For Each IP
            Frontend->>Frontend: Validate Entry
            Frontend->>Backend: Process Bulk Entry
            Backend->>Solana: Batch Registration
        end
        Solana-->>Backend: Bulk TX Receipt
        Backend-->>Frontend: Bulk Complete
        Frontend-->>Startup: Summary Report
    deactivate Frontend
```

### Laywer

```mermaid
sequenceDiagram
    participant Lawyer
    participant Frontend as React Frontend
    participant Backend as Node.js Backend
    participant Solana as Solana Program
    participant Arweave as Arweave/IPFS

    %% Verification Flow
    Lawyer->>Frontend: Verify Ownership (CID)
    activate Frontend
        Frontend->>Backend: Request Verification
        activate Backend
            Backend->>Solana: Query PDA (CID)
            Solana-->>Backend: Registration Data
            Backend->>Arweave: Retrieve File (CID)
            Arweave-->>Backend: File Content
            Backend->>Backend: Verify Hash Match
            Backend-->>Frontend: Validation Report
        deactivate Backend
        Frontend-->>Lawyer: Display Legal Proof
    deactivate Frontend
```
