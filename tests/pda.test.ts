import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Pda } from "../target/types/pda";

describe("pda", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Pda as Program<Pda>;
    const wallet = provider.wallet;

    const [messagePda, messageBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("message"), wallet.publicKey.toBuffer()],
        program.programId
    );

    const [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), wallet.publicKey.toBuffer()],
        program.programId
    );
    

    it("Create Message Account", async () => {
        const message = "Hi Solana!";
        const txSig = await program.methods
            .create(message)
            .rpc({ commitment: "confirmed" });

        const messageAccount = await program.account.messageAccount.fetch(
            messagePda,
            "confirmed"
        );

        console.log(JSON.stringify(messageAccount, null, 2));
        console.log("Transaction Signature:", `https://solana.fm/tx/${txSig}?cluster=devnet-solana`);
    });

    it("Update Message Account", async () => {
        const message = "Hello Passenger!";
        const txSig = await program.methods
            .update(message)
            .accounts({
                messageAccount: messagePda,
                vaultAccount: vaultPda,
            })
            .rpc({ commitment: "confirmed" });

        const messageAccount = await program.account.messageAccount.fetch(
            messagePda,
            "confirmed"
        );

        console.log(JSON.stringify(messageAccount, null, 2));
        console.log("Transaction Signature:", `https://solana.fm/tx/${txSig}?cluster=devnet-solana`);
    });

    it("Delete Message Account", async () => {
        const txSig = await program.methods
            .delete()
            .accounts({
                messageAccount: messagePda,
                vaultAccount: vaultPda,
            })
            .rpc({ commitment: "confirmed" });

        const messageAccount = await program.account.messageAccount.fetchNullable(
            messagePda,
            "confirmed"
        );

        console.log(JSON.stringify(messageAccount, null, 2));
        console.log("Transaction Signature:", `https://solana.fm/tx/${txSig}?cluster=devnet-solana`);
    });
});
