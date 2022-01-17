# solana-riddleapp

In the process of creating a very simple riddle dApp with a smart contract on Solana. The idea is that there will be an initial riddle and answer to said riddle stored in an account owned by the program.
Users will be able to guess the answer to the riddle from a client interface. If they guess the correct answer, they will be able to replace the current riddle/answer with one of their own choosing.
Right now, anyone can replace the riddle/answer at any time regardless if they have correctly guessed the initial riddle, want to imporve upon the accesibility of that function in the future.


UPDATE: with my latest push, you can only change the riddle if you have correctly guesses the answer to the current one. This isn't the most ideal because anyone can update the riddle with an answer that no one would guess, therefore locking up the riddle forever. To combat this, I am thinking about putting a timer on said riddle so that if no one correctly answers in some arbitrary amount of time (1 day?) the riddle resets.

I also made the riddle answer a private variable so that no one can access it from outside the contract.


This was created from the same base code of the messengerapp I made earlier in this repo https://github.com/ixmorrow/solana-messengerapp
