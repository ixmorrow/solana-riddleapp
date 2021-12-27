const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('riddleapp', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  
  //const messenger_account = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Riddleapp;
  const baseAccount  = anchor.web3.Keypair.generate();

  it("An account is initialized", async function() {
    await program.rpc.initialize("What gets wet while drying?", "A towel", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount]
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    //console.log('Data: ', account.riddle);
    assert.ok(account.riddle === "What gets wet while drying?");
    assert.ok(account.answer === "A towel");
    assert.ok(account.isCorrect === false);
  });

  it("Checks incorrect answer", async function() {
    await program.rpc.check("a fish", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      }
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.isCorrect === false);
  })

  it("Tries to update whithout correct answer", async function() {
    await program.rpc.update("Until I am measured, I am not known. Yet how you miss me, When I have flown.", "Time", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      }
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.riddle === "What gets wet while drying?");
  })

  it("Gives correct answer to riddle", async function() {
    await program.rpc.check("A towel", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      }
    })

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.isCorrect === true);
  })

  it("Updating riddle account after correctly answering initial riddle", async function() {
    await program.rpc.update("Until I am measured, I am not known. Yet how you miss me, When I have flown.", "Time", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      }
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.riddle === "Until I am measured, I am not known. Yet how you miss me, When I have flown.");
    //assert.ok(account.answer === "Time");
    assert.ok(account.isCorrect === false);
  })

  it("Check incorrect riddle after updating new riddle", async function() {
    await program.rpc.check("a fish", {
      accounts: {
        baseAccount: baseAccount.publicKey,
      }
    })
  
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    assert.ok(account.isCorrect === false);
  })

})