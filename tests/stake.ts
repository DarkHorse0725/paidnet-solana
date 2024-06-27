import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from '../target/types/stake';
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const connection = provider.connection;

const program = anchor.workspace.Stake as Program<Stake>;

const appState = PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode("app-state")
  ],
  program.programId
)[0];

const [authority, bump] = PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode("authority"),
    appState.toBuffer()
  ],
  program.programId
);

const rewardPot = PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode("reward-pot")
  ],
  program.programId
)[0];

const stakeVault = PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode("stake-vault")
  ],
  program.programId
)[0];

const owner = provider.wallet as NodeWallet;

export const staker = PublicKey.findProgramAddressSync(
  [
    anchor.utils.bytes.utf8.encode("staker"),
    owner.publicKey.toBuffer()
  ],
  program.programId
)[0]; 

export const initStake = async (
  rewardMint: PublicKey,
  stakeMint: PublicKey,
  rewardDecimals: number,
  stakeDecimals: number,
) => {
  const tx = await program.methods.initialize(
    new BN(100),
    rewardDecimals,
    stakeDecimals,
    false,
    bump
  ).accounts({
    rewardMint,
    stakeMint,
    appState,
    authority,
    rewardPot
  }).rpc();
  console.log(tx);
}

export const stake = async (
  stakeMint: PublicKey,
  stakeDecimals: number
) => {
  const userStakeToken = getAssociatedTokenAddressSync(
    stakeMint,
    owner.publicKey
  );
  const tx = await program.methods.stake(
    new BN(10000 * (10 ** stakeDecimals))
  ).accounts({
    stakeMint,
    userStakeToken,
    authority,
    stakeVault,
    appState,
    staker
  }).rpc();
  console.log(tx);
}