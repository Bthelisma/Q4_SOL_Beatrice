import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo, getAssociatedTokenAddressSync, createAssociatedTokenAccountIdempotentInstruction  } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", {commitment: "confirmed"});

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("<mint address>");

(async () => {
    try {
        // Create an ATA
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
        console.log(`This is your ATA: ${ata.address.toBase58()}`);

        // Mint to ATA
        let mintTx = await mintTo(connection, keypair, mint, ata.address, keypair.publicKey,  100000000,);
        console.log(`Succesfully Minted!. Transaction Here: https://explorer.solana.com/tx/${mintTx}?cluster=devnet`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
