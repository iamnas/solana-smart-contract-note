import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Noter } from "../target/types/noter";
import { assert } from "chai";

describe("noter", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Noter as Program<Noter>;
  let note = anchor.web3.Keypair.generate();

  it("It Can Create a note!", async () => {
    const MESSAGE = "Test the connent";
    const tx = await program.methods
      .createNote(MESSAGE)
      .accounts({
        note: note.publicKey,
        user: provider.wallet.publicKey,
        // systemProgram:anchor.web3.SystemProgram.programId,
      })
      .signers([note])
      .rpc();

    let newNote = await program.account.note.fetch(note.publicKey);

    assert.strictEqual(newNote.content, MESSAGE);
    assert.strictEqual(
      newNote.user.toString(),
      provider.wallet.publicKey.toString()
    );
  });

  it("It Can delete a note!", async () => {
    await program.methods
      .deleteNote()
      .accounts({
        note: note.publicKey,
        // user: provider.wallet.publicKey,
        // systemProgram:anchor.web3.SystemProgram.programId,
      })
      .rpc();
    //.signers([note])

    let deletedNote = await program.account.note.fetchNullable(note.publicKey);

    assert.ok(deletedNote == null);
  });
});
