import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KozatskyBusiness } from "../target/types/kozatsky_business";
import { assert } from "chai";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("kozatsky_business", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.KozatskyBusiness as Program<KozatskyBusiness>;
  const user = provider.wallet;

  let playerPda: anchor.web3.PublicKey;
  let mintAuthorityPda: anchor.web3.PublicKey;
  let resourceMint: anchor.web3.PublicKey;
  let userResourceAccount: any;

  before(async () => {
    [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("player"), user.publicKey.toBuffer()],
      program.programId
    );

    [mintAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint-authority")],
      program.programId
    );

    resourceMint = await createMint(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      mintAuthorityPda,
      null,
      0
    );

    userResourceAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      resourceMint,
      user.publicKey
    );
  });

  it("Initialize player", async () => {
    await program.methods
      .initializePlayer()
      .accounts({
        player: playerPda,
        user: user.publicKey,
      } as any)
      .rpc();

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.ok(playerAccount.owner.equals(user.publicKey));
  });

  it("Search mints resource first time", async () => {
    await program.methods
      .searchResources(new anchor.BN(3))
      .accounts({
        player: playerPda,
        user: user.publicKey,
        resourceMint: resourceMint,
        userResourceAccount: userResourceAccount.address,
        mintAuthority: mintAuthorityPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      } as any)
      .rpc();

    const tokenAccount = await getAccount(
      provider.connection,
      userResourceAccount.address
    );

    assert.equal(Number(tokenAccount.amount), 3);
  });

  it("Search fails if called too early", async () => {
    try {
      await program.methods
        .search_resources(new anchor.BN(3))
        .accounts({
          player: playerPda,
          user: user.publicKey,
          resource_mint: resourceMint,
          user_resource_account: userResourceAccount.address,
          mint_authority: mintAuthorityPda,
          token_program: TOKEN_PROGRAM_ID,
        } as any)
        .rpc();

      assert.fail("Should have failed");
    } catch (err) {
      assert.ok(err);
    }
  });
});