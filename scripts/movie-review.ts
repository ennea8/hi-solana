import {
  Keypair,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  LAMPORTS_PER_SOL,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { struct, u8, str } from "@coral-xyz/borsh";
import { writeFileSync } from "fs";
import dotenv from "dotenv";
import {
  initializeKeypair,
  addKeypairToEnvFile,
  getKeypairFromFile
} from "@solana-developers/helpers";

dotenv.config();


const movieInstructionLayout = struct([
  u8("variant"),
  str("title"),
  u8("rating"),
  str("description"),
]);

async function sendTestMovieReview(
  signer: Keypair,
  programId: PublicKey,
  connection: Connection
) {
  let buffer = Buffer.alloc(1000);
  const movieTitle = `Braveheart${Math.random() * 1000000}`;
  movieInstructionLayout.encode(
    {
      variant: 0,
      title: movieTitle,
      rating: 5,
      description: "A great movie",
    },
    buffer
  );

  buffer = buffer.subarray(0, movieInstructionLayout.getSpan(buffer));

  const [pda] = await PublicKey.findProgramAddressSync(
    [signer.publicKey.toBuffer(), Buffer.from(movieTitle)],
    programId
  );

  console.log("PDA is:", pda.toBase58());

  const transaction = new Transaction();

  const instruction = new TransactionInstruction({
    programId: programId,
    data: buffer,
    keys: [
      {
        pubkey: signer.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: pda,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
  });

  transaction.add(instruction);
  const tx = await sendAndConfirmTransaction(connection, transaction, [signer]);
  console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
}

(async () => {
  try {
    
    // const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const connection = new Connection("https://devnet.rpcpool.com", "confirmed");


    // console.log("Connection established",connection);

    // const signer = await initializeKeypair(connection, {
    //   //airdropAmount: LAMPORTS_PER_SOL,
    //   envVariableName: "PRIVATE_KEY",
    // });

    // console.log("signer",signer);

    const keyPair = await getKeypairFromFile("~/.config/solana/id.json");

    const movieProgramId = new PublicKey(
      "AEaTRsGXS5ruVmKZ6RS9dmtcD1nkMseiYnoDug6RnBE8"
    );
    await sendTestMovieReview(keyPair, movieProgramId, connection);

    console.log("Finished successfully");
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
})();