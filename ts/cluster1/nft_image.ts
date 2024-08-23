import wallet from "../wba-wallet.json"
//import { Commitment } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
//const commitment: Commitment = "confirmed";
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {  
        //1. Load image             
        const image = await readFile("/Users/beatricethelisma/Q3T_SOL-BeatriceThelisma/ts/generug.png");
        //console.log(image)
        //2. Convert image to generic file.
        const file = createGenericFile(image, "generug", {contentType: "image/png"});
        //3. Upload image
        const [myUri] = await umi.uploader.upload([file]);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
