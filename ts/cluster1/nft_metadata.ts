import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //Follow this JSON structure
        //https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = createGenericFile("", "generug.png", {contentType: "image/jpg"})
        const metadata = {
            name: "Beauty",
            symbol: "Belle",
            description: "NFT Rug",
            image: "not yet working",
            attributes: [
                {trait_type: 'background', value: 'blue'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "not yet working"
                    },
                ]
            },
            creators: []
        };
        const [myUri] = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
