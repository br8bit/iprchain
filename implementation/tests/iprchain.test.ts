import { MPL_CORE_PROGRAM_ID, mplCore } from '@metaplex-foundation/mpl-core';
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

  const admin = Keypair.generate();
  const creator = Keypair.generate();
  const asset = Keypair.generate();

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

  describe('Creates IP account with Core asset', async () => {
    const content = `IP-${Date.now()}`;
    const ipHash = generateIpHash(content);
    const metadataUri = 'https://arweave.net/abc123';

    const [ipRegistry] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('iprchain'), admin.publicKey.toBuffer()],
      program.programId
    );

    const [ipAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('ip_account'), Buffer.from(ipHash)],
      program.programId
    );

    const [treasury] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from('treasury'), ipRegistry.toBuffer()],
      program.programId
    );

    it('Registers IP', async () => {
      // Execute registration
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
    });

    it('Verifies IP account', async () => {
      const account = await program.account.ipAccount.fetch(ipAccount);

      assert.equal(account.metadataUri, metadataUri);
      assert.deepEqual(account.ipHash, ipHash);
    });

    it('Verifies Core asset exists', async () => {
      const account = await program.account.ipAccount.fetch(ipAccount);
      const coreAsset = await provider.connection.getAccountInfo(
        account.coreAsset
      );
      assert.ok(asset, 'Core asset account missing');
      assert.equal(
        coreAsset.owner.toBase58(),
        MPL_CORE_PROGRAM_ID,
        'Invalid Core asset owner'
      );
    });
  });
});
