import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ignition } from '../target/types/ignition';
import { Stake } from '../target/types/stake';
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("ignition", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.Ignition as Program<Ignition>;
  const stakeProgram = anchor.workspace.Stake as Program<Stake>;

  let now: number;
  let offerMint: PublicKey;
  let purchaseMint: PublicKey;
  let offerTokenAccount: PublicKey;
  let purchaseTokenAccount: PublicKey;

  const poolKeypair = Keypair.generate();
  const pool = poolKeypair.publicKey;


  const [authority, bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("authority"),
      pool.toBuffer()
    ],
    program.programId
  );

  const offerVault = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("offer-vault"),
      pool.toBuffer()
    ],
    program.programId
  )[0];

  const purchaseVault = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("purchase-vault"),
      pool.toBuffer()
    ],
    program.programId
  )[0];
  const owner = provider.wallet as NodeWallet;

  const buyer = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("buyer"),
      pool.toBuffer(),
      owner.publicKey.toBuffer()
    ],
    program.programId
  )[0];

  

  const offerDecimals = 9;
  const purchaseDecimals = 6;

  it("setup", async () => {
    const slot = await connection.getSlot();
    now = await connection.getBlockTime(slot);
    offerMint = await createMint(
      connection,
      owner.payer,
      owner.publicKey,
      owner.publicKey,
      offerDecimals
    );
    purchaseMint = await createMint(
      connection,
      owner.payer,
      owner.publicKey,
      owner.publicKey,
      purchaseDecimals
    );
    const offerAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      owner.payer,
      offerMint,
      owner.publicKey
    );

    offerTokenAccount = offerAccount.address;

    const purchaseAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      owner.payer,
      purchaseMint,
      owner.publicKey
    );

    purchaseTokenAccount = purchaseAccount.address;

    await mintTo(
      connection,
      owner.payer,
      offerMint,
      offerAccount.address,
      owner.payer,
      BigInt(1000000 * (10 ** offerDecimals))
    );

    await mintTo(
      connection,
      owner.payer,
      purchaseMint,
      purchaseAccount.address,
      owner.payer,
      BigInt(1000000 * (10 ** purchaseDecimals))
    );

  })

  it("create pool", async () => {
    const tx = await program.methods.createPool(
      [
        new BN(1000 * (10 ** purchaseDecimals)),
        new BN(100 * (10 ** purchaseDecimals)),
        new BN(1000 * (10 ** purchaseDecimals)),
        new BN(1000),
        new BN(1000),
        new BN(1000),
        new BN(5000),
        new BN(5000),
        new BN(2000 * (10 ** purchaseDecimals)),
        new BN(now + 1),
        new BN(now + 10),
        new BN(now + 20),
        new BN(100),
        new BN(now + 30),
        new BN(1000),
        new BN(1),
        new BN(1),
        new BN(2)
      ],
      offerDecimals,
      purchaseDecimals,
      false,
      false,
      bump
    ).accounts({
      offerMint,
      purchaseMint,
      authority,
      offerVault,
      pool
    }).signers([poolKeypair]).rpc().catch(e => console.log(e));
    console.log(tx);
  });

  it ("fund offer", async () => {
    const tx = await program.methods.fundOffer(
      new BN(1000000 * (10 ** offerDecimals))
    ).accounts({
      offerMint,
      ownerOfferToken: offerTokenAccount,
      authority,
      offerVault,
      pool
    }).rpc().catch(e => console.log(e));
    console.log(tx);
  })

  it("buy in early pool", async () => {
    const tx = await program.methods.buyInEarlyPool(
      new BN(100 * (10 ** purchaseDecimals))
    ).accounts({
      purchaseMint,
      userPurchaseToken: purchaseTokenAccount,
      authority,
      buyer,
      purchaseVault,
      pool,
    }).rpc();
    console.log(tx)
  });
})