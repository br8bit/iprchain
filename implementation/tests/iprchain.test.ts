import { MPL_CORE_PROGRAM_ID } from '@metaplex-foundation/mpl-core';
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { Program, web3, BN } from '@coral-xyz/anchor';
import { assert } from 'chai';
import { Iprchain } from '../target/types/iprchain';
import { createHash } from 'node:crypto';

const checkErrors = async (txId: any, provider) => {
  // Fetch the transaction details to check for errors
  const txDetails = await provider.connection.getTransaction(txId, {
    commitment: 'confirmed',
  });

  if (txDetails?.meta?.err) {
    console.error('Transaction failed:', txDetails.meta.err);
    throw new Error('Transaction failed');
  }
};

describe('IPRChain', async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Iprchain as Program<Iprchain>;

  const airdrop = async (user: PublicKey, sol: number = 2) => {
    const tx = await provider.connection.requestAirdrop(
      user,
      sol * LAMPORTS_PER_SOL
    );

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    // console.log(`Airdropped ${sol} SOL to ${user.toBase58()} 🪂`);
  };

  const generateIpHash = (content: string) =>
    Array.from(createHash('sha256').update(content).digest());

  let ipAccount: PublicKey;
  const admin = Keypair.generate();
  const creator = Keypair.generate();
  const metadataUri = 'https://arweave.net/abc123';
  const ipHash = generateIpHash('IP-123');

  const IP_ACCOUNT_SEED = Buffer.from('ip_account');
  const IP_REGISTRY_SEED = [
    Buffer.from('iprchain'),
    admin.publicKey.toBuffer(),
  ];
  const LICENSE_ACCOUNT_SEED = [
    Buffer.from('license'),
    creator.publicKey.toBuffer(),
  ];

  beforeEach(async () => {
    await airdrop(admin.publicKey);
    await airdrop(creator.publicKey);
  });

  it('Initializes IP Registry', async () => {
    const [ipRegistry] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('iprchain'), admin.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initialize(new BN(100))
      .accountsPartial({
        ipRegistry,
        admin: admin.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc({ commitment: 'confirmed' });

    const registryState = await program.account.ipRegistryState.fetch(
      ipRegistry
    );

    assert.equal(registryState.admin.toBase58(), admin.publicKey.toBase58());
    assert.equal(registryState.totalIps.toNumber(), 0);
  });

  describe('IP account with Core asset', async () => {
    const registerIp = async (ipHash: number[], metadataUri: string) => {
      const asset = Keypair.generate();

      const [ipRegistry] = web3.PublicKey.findProgramAddressSync(
        IP_REGISTRY_SEED,
        program.programId
      );

      const [ipAccount] = web3.PublicKey.findProgramAddressSync(
        [IP_ACCOUNT_SEED, Buffer.from(ipHash)],
        program.programId
      );

      const [treasury] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from('treasury'), ipRegistry.toBuffer()],
        program.programId
      );

      const txId = await program.methods
        .registerIp(ipHash, metadataUri)
        .accountsPartial({
          creator: creator.publicKey,
          asset: asset.publicKey,
          admin: admin.publicKey,
          ipAccount,
          ipRegistry,
          // treasury,
          collection: null,
          updateAuthority: null,
          authority: null,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([creator, admin, asset])
        .rpc({ commitment: 'confirmed' });

      checkErrors(txId, provider);

      return [ipAccount, ipRegistry];
    };

    it('Registers IP', async () => {
      [ipAccount] = await registerIp(ipHash, metadataUri);

      const account = await program.account.ipAccount.fetch(ipAccount);

      assert.exists(account);
    });

    it('Verifies IP hash and metadata', async () => {
      const account = await program.account.ipAccount.fetch(ipAccount);

      assert.equal(account.metadataUri, metadataUri);
      assert.deepEqual(account.ipHash, ipHash);
    });

    it('Verifies Core asset exists', async () => {
      const account = await program.account.ipAccount.fetch(ipAccount);
      const coreAsset = await provider.connection.getAccountInfo(
        account.coreAsset
      );
      assert.ok(coreAsset, 'Core asset account missing');
      assert.equal(
        coreAsset.owner.toBase58(),
        MPL_CORE_PROGRAM_ID,
        'Invalid Core asset owner'
      );
    });

    it('Prevents duplicate IP hash registration', async () => {
      // Attempt duplicate
      try {
        await registerIp(ipHash, metadataUri);
        assert.fail('Should have thrown duplicate error');
      } catch (err) {
        assert.include(err.message, 'DuplicateIpHash');
      }
    });

    it('Validates metadata URI length constraints', async () => {
      const longUri = 'a'.repeat(201); // MAX_URI_LENGTH = 200
      const content = `IP-${Date.now()}`;
      const ipHash = generateIpHash(content);

      try {
        await registerIp(ipHash, longUri);
        assert.fail('Should have rejected long URI');
      } catch (err) {
        assert.include(err.message, 'InvalidMetadataUriLength');
      }
    });

    it('Validates Total IPs count to be 1', async () => {
      const [ipRegistry] = web3.PublicKey.findProgramAddressSync(
        IP_REGISTRY_SEED,
        program.programId
      );

      const registryState = await program.account.ipRegistryState.fetch(
        ipRegistry
      );

      assert.equal(registryState.totalIps.toNumber(), 1);
    });

    it('Ensures the platform fee is within the range', async () => {
      const [ipRegistry] = web3.PublicKey.findProgramAddressSync(
        IP_REGISTRY_SEED,
        program.programId
      );

      const registryState = await program.account.ipRegistryState.fetch(
        ipRegistry
      );

      assert.isAtLeast(registryState.fee.toNumber(), 100);
      assert.isAtMost(registryState.fee.toNumber(), 200);
    });
  });

  describe('License account', async () => {
    let licenseAccount = null;

    async function createLicense() {
      const fee = new BN(100);
      const royalty = 10;

      const [licenseAccount] = web3.PublicKey.findProgramAddressSync(
        [...LICENSE_ACCOUNT_SEED, Buffer.from(ipHash)],
        program.programId
      );

      const [ipAccount] = web3.PublicKey.findProgramAddressSync(
        [IP_ACCOUNT_SEED, Buffer.from(ipHash)],
        program.programId
      );

      const currentTime = await provider.connection.getSlot();
      const currentTimestamp = await provider.connection.getBlockTime(
        currentTime
      );
      const startsAt = new BN(currentTimestamp + 60); // 60 seconds from now
      const expiresAt = new BN(currentTimestamp + 3600); // 1 hour from starts_at

      await program.methods
        .createLicense(fee, startsAt, expiresAt, royalty)
        .accountsPartial({
          creator: creator.publicKey,
          licenseAccount,
          ipAccount,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([creator])
        .rpc({ commitment: 'confirmed' });

      const account = await program.account.licenseAccount.fetch(
        licenseAccount
      );

      return account;
    }

    it('Initializes License Account', async () => {
      licenseAccount = await createLicense();

      assert.exists(licenseAccount);
      assert.equal(
        licenseAccount.creator.toBase58(),
        creator.publicKey.toBase58()
      );
    });

    it('Verifies License has no initial Licensee', async () => {
      assert.isNull(
        licenseAccount.licensee,
        'Licensee should be None initially'
      );
    });

    it('Validates License terms', async () => {
      assert.exists(licenseAccount.terms, "License terms don't exist");
      assert.equal(licenseAccount.terms.fee.toNumber(), 100);
      assert.equal(licenseAccount.terms.royaltyPercent, 10);
    });

    it('Validates License start and expiry timestamps', async () => {
      const currentTime = await provider.connection.getSlot();
      const currentTimestamp = await provider.connection.getBlockTime(
        currentTime
      );

      assert.isAtLeast(
        licenseAccount.terms.startsAt.toNumber(),
        currentTimestamp
      );
      assert.isAtMost(
        licenseAccount.terms.expiresAt.toNumber(),
        currentTimestamp + 3600
      );
    });

    it('Assign and verify licensee is set', async () => {
      const [licenseAccount] = web3.PublicKey.findProgramAddressSync(
        [...LICENSE_ACCOUNT_SEED, Buffer.from(ipHash)],
        program.programId
      );

      const licensee = Keypair.generate();
      await airdrop(licensee.publicKey);
      const txId = await program.methods
        .assignLicensee()
        .accountsPartial({
          licenseAccount,
          ipAccount,
          licensee: licensee.publicKey,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([licensee])
        .rpc({ commitment: 'confirmed' });

      const account = await program.account.licenseAccount.fetch(
        licenseAccount
      );

      assert.exists(account.licensee);
      assert.equal(
        account.licensee.toBase58(),
        licensee.publicKey.toBase58(),
        'Licensee should be set to the new user'
      );
    });
  });
});
