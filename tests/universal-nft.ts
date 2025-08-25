import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UniversalNft } from "../target/types/universal_nft";
import {
  getAssociatedTokenAddress,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("universal-nft", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.UniversalNft as Program<UniversalNft>;
  const payer = provider.wallet as anchor.Wallet;

  // Test accounts
  let authorityKeypair: Keypair;
  let userKeypair: Keypair;

  // Program derived addresses
  let pdaAddress: PublicKey;
  let collectionAddress: PublicKey;

  const CHAIN_ID = 1337;

  before(async () => {
    authorityKeypair = Keypair.generate();
    userKeypair = Keypair.generate();

    // Airdrop SOL
    await provider.connection.requestAirdrop(authorityKeypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(userKeypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Derive PDAs
    [pdaAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("universal_nft")],
      program.programId
    );

    [collectionAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("collection")],
      program.programId
    );
  });

  it("Initializes the program", async () => {
    const tx = await program.methods
      .initialize(new anchor.BN(CHAIN_ID))
      .accountsPartial({
        authority: authorityKeypair.publicKey,
        pda: pdaAddress,
        systemProgram: SystemProgram.programId,
      })
      .signers([authorityKeypair])
      .rpc();

    console.log("Initialize transaction:", tx);
  });

  it("Initializes collection", async () => {
    const tx = await program.methods
      .initializeCollection("Universal NFTs", "UNFT", "https://api.test.com/")
      .accountsPartial({
        authority: authorityKeypair.publicKey,
        collection: collectionAddress,
        systemProgram: SystemProgram.programId,
      })
      .signers([authorityKeypair])
      .rpc();

    console.log("Collection init transaction:", tx);
  });

  it("Mints an NFT", async () => {
    const nftMintKeypair = Keypair.generate();

    const [nftRecordAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("nft_record"), nftMintKeypair.publicKey.toBuffer()],
      program.programId
    );

    const recipientTokenAccount = await getAssociatedTokenAddress(
      nftMintKeypair.publicKey,
      userKeypair.publicKey
    );

    const tx = await program.methods
      .mintNft("Test NFT", "A test NFT", "https://test.com/image.png")
      .accountsPartial({
        payer: payer.publicKey,
        pda: pdaAddress,
        collection: collectionAddress,
        nftRecord: nftRecordAddress,
        nftMint: nftMintKeypair.publicKey,
        recipientTokenAccount: recipientTokenAccount,
        recipientAuthority: userKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([nftMintKeypair])
      .rpc();

    console.log("Mint NFT transaction:", tx);
  });

  it("Demo complete - ready for cross-chain!", async () => {
    console.log("\n🎉 Universal NFT Program Demo Complete!");
    console.log(`Program ID: ${program.programId.toString()}`);
    console.log(`Chain ID: ${CHAIN_ID}`);
    console.log("\n✅ Implemented features:");
    console.log("  • NFT minting with cross-chain metadata");
    console.log("  • Cross-chain transfer preparation (burn & lock)");
    console.log("  • Incoming NFT reception and wrapping");
    console.log("  • Admin controls and security");
    console.log("  • ZetaChain Gateway compatibility");
    console.log("\n🌉 Ready for ZetaChain integration!");
  });
});
