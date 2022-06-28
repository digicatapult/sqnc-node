# Governance

This contains information about some of the governance procedures that take place in the Node. Initially this covers the voting process

## Create a Vote

In order to create a vote, in the Polkadot JS Apps frontend select the Governance tab, then `Tech. Comm`. You will then be presented with a screen with the current Governance members.

In order to start a new vote select proposals.

![Governance Tab](documentation/assets/governance/1.png)

On the proposal screen select submit proposal on the right hand side. This will bring up a new proposal which must be made from an initial account, for example, here Alice is creating a proposal. You may notice the use of the doAs pallet, for more information visit [the readme](README.md) on how it works.

![Committee Motion](documentation/assets/governance/2.png)

Once the proposal has been agreed and submit has been clicked another window will appear to confirm the transaction.

![Vote on proposal](documentation/assets/governance/3.png)

Click sign and submit. There wiill then be a voting round.

![Vote on proposal](documentation/assets/governance/4.png)

![Vote on proposal](documentation/assets/governance/5.png)

Once the voting round has been completed the proposal needs to be closed to enter the chain.

![Close vote](documentation/assets/governance/6.png)

You should see these for a successful vote.

![Succssful vote](documentation/assets/governance/7.png)

## Set Balance to 0

As we can see in the image below, Bob has nearly the same balance as Alice.

![Similar balances](documentation/assets/governance/8.png)

By following the voting rules above we can chance a users balance to 0.

![Vote to remove vote](documentation/assets/governance/2.png)

![Balance After](documentation/assets/governance/9.png)

## Can no longer transact

Now Bob has a balance of 0, if that party tries to create a proposal it will fail.

![Bob tries a vote](documentation/assets/governance/10.png)

This error appears in the right had side indicating that the use is not able to create a transaction.

![Bob cannot start a transaction](documentation/assets/governance/11.png)
