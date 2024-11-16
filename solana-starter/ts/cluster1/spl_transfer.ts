import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", {commitment: "confirmed"});

// Mint address
const mint = new PublicKey("<mint address>");

// Recipient address
const to = new PublicKey("<receiver address>");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const from_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        );
    
        // Get the token account of the toWallet address, and if it does not exist, create it
        const to_ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to,
        );
        // Transfer the new token to the "toTokenAccount" we just created
        const mintTx = transfer(
            connection,
            keypair,
            from_ata.address,
            to_ata.address,
            keypair.publicKey,
            1000
        );
        console.log(`Succesfully Minted!. Transaction Here: https://explorer.solana.com/tx/${mintTx}?cluster=devnet`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();