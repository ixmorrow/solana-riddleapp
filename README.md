# solana-riddleapp

In the process of creating a very simple riddle dApp with a smart contract on Solana. The idea is that there will be an initial riddle and answer to said riddle stored in an account owned by the program.
Users will be able to guess the answer to the riddle from a client interface. If they guess the correct answer, they will be able to replace the current riddle/answer with one of their own choosing.
Right now, you can only change the riddle if you have correctly guessed the answer to the current one. This isn't the most ideal because anyone can update the riddle with an answer that no one would guess, therefore locking up the riddle forever. To combat this, I am thinking about putting a timer on said riddle so that if no one correctly answers in some arbitrary amount of time (1 day?) the riddle resets.

I also made the riddle answer a private variable so that no one can access it from outside the contract.

Additionally, now I have minted 1000 RIDDL tokens to be used by the protocol. When a user correctly guesses the answer to a riddle, they are rewarded with a riddl token. I would like to also incorporate a cost/deduction for incorrect guesses as well as somehow reward/punish users who create the current riddle. Still thinking about how to handle that.


This was created from the same base code of the messengerapp I made earlier in this repo https://github.com/ixmorrow/solana-messengerapp
